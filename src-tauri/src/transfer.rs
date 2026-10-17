// V1 transfer engine.
//
// This module implements the first transfer flow:
// - a receiver listens on a local TCP port
// - a sender connects to that receiver and streams a file
// - the receiver writes to a temporary file and verifies size + SHA-256
//
// Large files are supported by streaming in chunks instead of buffering the
// whole file in memory.
//
// Design notes:
// - The sender computes the SHA-256 digest first (synchronously) so it can send
//   it in the transfer header. This is acceptable for V1 because the digest
//   work is bounded by file size and avoids mixing blocking file I/O into the
//   async send loop.
// - The send loop itself is fully async: it reads the file asynchronously and
//   writes to the TCP stream asynchronously, with a cancel branch in
//   tokio::select!.
// - The receive loop writes incoming data to a temporary file, computes the
//   received digest incrementally, and only finalizes the file after integrity
//   verification succeeds.
// - When the receiver accepts a connection, it stores that connection's cancel
//   sender on the app-level transfer state so the app's cancel command can stop
//   an in-flight transfer.
//
// This is intentionally simple for V1. It is not encrypted, not authenticated,
// and not resumable yet.

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};

// ---------------------------------------------------------------------------
// Public API types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferState {
    pub kind: String,
    pub status: String,
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub received_file_path: Option<String>,
    pub progress_bytes: u64,
    pub total_bytes: Option<u64>,
    pub sender_digest: Option<String>,
    pub error_message: Option<String>,
    pub port: Option<u16>,
}

impl Default for TransferState {
    fn default() -> Self {
        Self {
            kind: "unknown".into(),
            status: "idle".into(),
            file_name: None,
            file_size: None,
            received_file_path: None,
            progress_bytes: 0,
            total_bytes: None,
            sender_digest: None,
            error_message: None,
            port: None,
        }
    }
}

/// Arguments for starting a receiver listener.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartReceiverArgs {
    pub port: u16,
    pub save_directory: String,
}

/// Arguments for starting a sender transfer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendFileArgs {
    pub target_addr: String,
    pub source_path: String,
    pub port: u16,
}

// ---------------------------------------------------------------------------
// App-level transfer control
// ---------------------------------------------------------------------------

/// The app-level transfer control surface that the transfer module uses to
/// observe and update the current transfer state and cancel channel.
pub struct AppTransferControl {
    pub manager: Arc<RwLock<TransferState>>,
    pub cancel_tx: Arc<tokio::sync::Mutex<Option<mpsc::UnboundedSender<()>>>>,
}

impl AppTransferControl {
    pub fn new(manager: Arc<RwLock<TransferState>>) -> Self {
        Self {
            manager,
            cancel_tx: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    pub async fn set_cancel_tx(&self, tx: mpsc::UnboundedSender<()>) {
        let mut cancel_tx = self.cancel_tx.lock().await;
        *cancel_tx = Some(tx);
    }

    pub async fn take_cancel_tx(&self) -> Option<mpsc::UnboundedSender<()>> {
        let mut cancel_tx = self.cancel_tx.lock().await;
        cancel_tx.take()
    }
}

// ---------------------------------------------------------------------------
// Receiver
// ---------------------------------------------------------------------------

const CHUNK_BYTES: usize = 1_048_576; // 1 MB

/// Keep only a safe file name: no folders, no characters Windows rejects.
fn safe_file_name(raw: &str) -> String {
    // Take only the last path component, whichever slash the sender used.
    let last = raw.rsplit(|c| c == '/' || c == '\\').next().unwrap_or("");
    let cleaned: String = last
        .chars()
        .map(|c| {
            if c.is_control() || matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*') {
                '_'
            } else {
                c
            }
        })
        .collect();
    let cleaned = cleaned.trim().trim_end_matches('.').trim().to_string();
    if cleaned.is_empty() || cleaned == "." || cleaned == ".." {
        return "received_file".into();
    }
    // Windows reserved device names (CON, NUL, COM1, ...)
    let stem = cleaned.split('.').next().unwrap_or("").to_ascii_uppercase();
    let reserved = ["CON", "PRN", "AUX", "NUL"];
    let is_com_lpt = (stem.starts_with("COM") || stem.starts_with("LPT"))
        && stem.len() == 4
        && stem.chars().last().map_or(false, |c| c.is_ascii_digit());
    if reserved.contains(&stem.as_str()) || is_com_lpt {
        return format!("_{}", cleaned);
    }
    cleaned
}

/// Never overwrite an existing file: add " (1)", " (2)", ... before the extension.
fn unique_path(dir: &PathBuf, name: &str) -> PathBuf {
    let candidate = dir.join(name);
    if !candidate.exists() {
        return candidate;
    }
    let p = std::path::Path::new(name);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("received_file");
    let ext = p.extension().and_then(|s| s.to_str());
    for i in 1..10_000 {
        let new_name = match ext {
            Some(e) => format!("{} ({}).{}", stem, i, e),
            None => format!("{} ({})", stem, i),
        };
        let path = dir.join(&new_name);
        if !path.exists() {
            return path;
        }
    }
    candidate
}

pub async fn run_receiver(
    args: StartReceiverArgs,
    control: Arc<AppTransferControl>,
    mut cancel_rx: mpsc::UnboundedReceiver<()>,
) -> Result<(), String> {
    // Validate save directory early.
    let save_dir = PathBuf::from(&args.save_directory);
    if !save_dir.is_dir() {
        return Err(format!(
            "Save directory does not exist: {}",
            args.save_directory
        ));
    }

    let listener = TcpListener::bind(("0.0.0.0", args.port))
        .await
        .map_err(|e| format!("Could not listen on port {}: {}", args.port, e))?;

    {
        let mut state = control.manager.write().await;
        state.kind = "receiver".into();
        state.status = "listening".into();
        state.port = Some(args.port);
        state.error_message = None;
    }

    loop {
        tokio::select! {
            accept = listener.accept() => {
                match accept {
                    Ok((stream, remote)) => {
                        let control_for_spawn = control.clone();
                        let control_for_cancel = control.clone();
                        let (conn_cancel_tx, conn_cancel_rx) =
                            mpsc::unbounded_channel::<()>();
                        let conn_cancel_tx_for_app = conn_cancel_tx.clone();
                        let conn_cancel_rx_for_connection = conn_cancel_rx;
                        tokio::spawn(async move {
                            let control_for_error = control_for_spawn.clone();
                            if let Err(err) = handle_receiver_connection(
                                stream,
                                remote,
                                args.save_directory.clone(),
                                control_for_spawn,
                                conn_cancel_rx_for_connection,
                            )
                            .await
                            {
                                let mut state = control_for_error.manager.write().await;
                                state.status = "failed".into();
                                state.error_message = Some(err);
                            }
                        });
                        control_for_cancel.set_cancel_tx(conn_cancel_tx_for_app).await;
                        // V1 supports one active transfer at a time on the receiver.
                        return Ok(());
                    }
                    Err(e) => {
                        let mut state = control.manager.write().await;
                        state.status = "failed".into();
                        state.error_message = Some(format!("Accept failed: {}", e));
                        return Err(format!("Accept failed: {}", e));
                    }
                }
            }
            _ = cancel_rx.recv() => {
                let mut state = control.manager.write().await;
                state.status = "cancelled".into();
                return Ok(());
            }
        }
    }
}

async fn handle_receiver_connection(
    mut stream: TcpStream,
    remote: SocketAddr,
    save_directory: String,
    control: Arc<AppTransferControl>,
    mut cancel_rx: mpsc::UnboundedReceiver<()>,
) -> Result<(), String> {
    let mut buf = vec![0u8; CHUNK_BYTES];

    // Read the fixed-size header:
    // - 8 bytes: file size (little-endian u64)
    // - 4 bytes: digest length (little-endian u32)
    // - N bytes: base64-encoded SHA-256 digest ASCII
    let mut size_buf = [0u8; 8];
    read_exact_or_cancel(&mut stream, &mut size_buf, &mut cancel_rx)
        .await
        .map_err(|_| "Receiver: failed to read file-size header".to_string())?;
    let file_size = u64::from_le_bytes(size_buf);

    let mut len_buf = [0u8; 4];
    read_exact_or_cancel(&mut stream, &mut len_buf, &mut cancel_rx)
        .await
        .map_err(|_| "Receiver: failed to read digest-length header".to_string())?;
    let digest_len = u32::from_le_bytes(len_buf) as usize;

        if digest_len == 0 || digest_len > 256 {
        return Err("Receiver: invalid digest length".into());
    }

    let mut digest_bytes = vec![0u8; digest_len];
    read_exact_or_cancel(&mut stream, &mut digest_bytes, &mut cancel_rx)
        .await
        .map_err(|_| "Receiver: failed to read sender digest".to_string())?;

    // The sender sends the digest as base64 ASCII.
    let sender_digest =
        String::from_utf8(digest_bytes).map_err(|_| "Receiver: invalid digest encoding".to_string())?;

            let mut name_len_buf = [0u8; 4];
    read_exact_or_cancel(&mut stream, &mut name_len_buf, &mut cancel_rx)
        .await
        .map_err(|_| "Receiver: failed to read file-name length".to_string())?;
    let name_len = u32::from_le_bytes(name_len_buf) as usize;
    if name_len == 0 || name_len > 1024 {
        return Err("Receiver: invalid file name length".into());
    }
    let mut name_bytes = vec![0u8; name_len];
    read_exact_or_cancel(&mut stream, &mut name_bytes, &mut cancel_rx)
        .await
        .map_err(|_| "Receiver: failed to read file name".to_string())?;
    let file_name = safe_file_name(&String::from_utf8_lossy(&name_bytes));

    // Sanity-check the reported size to avoid surprising disk usage.
    if file_size > 100_000_000_000_000 {
        return Err("Receiver: reported file size is unreasonably large".into());
    }

    {
        let mut state = control.manager.write().await;
        state.kind = "receiver".into();
        state.status = "receiving".into();
        state.file_name = Some(file_name.clone());
        state.file_size = Some(file_size);
        state.sender_digest = Some(sender_digest.clone());
        state.total_bytes = Some(file_size);
        state.progress_bytes = 0;
    }

    // Write to a temporary file first. This lets us verify integrity before
    // exposing a received file to the user.
    let tmp_path = {
        let tmp_dir = PathBuf::from(&save_directory);
        let tmp_name = format!(
            ".drift-receiving-{:x}.tmp",
            // Use a stable-ish suffix based on remote port so concurrent
            // tests are less likely to collide.
            remote.port() ^ 0xFFFF
        );
        tmp_dir.join(tmp_name)
    };

    let mut file = File::create(&tmp_path).map_err(|e| {
        format!("Receiver: could not create temp file {}: {}", tmp_path.display(), e)
    })?;

    let mut received: u64 = 0;
    let mut hasher = Sha256::new();
    let mut remaining = file_size;

    while remaining > 0 {
        tokio::select! {
            _ = cancel_rx.recv() => {
                // Clean up the incomplete temp file.
                let _ = fs::remove_file(&tmp_path);
                {
                    let mut state = control.manager.write().await;
                    state.status = "cancelled".into();
                    state.error_message = Some("Transfer cancelled".into());
                }
                return Ok(());
            }
            n = stream.read(&mut buf) => {
                let n = n.map_err(|e| format!("Receiver: read error: {}", e))?;
                if n == 0 {
                    return Err("Receiver: connection closed before file completed".into());
                }
                file.write_all(&buf[..n]).map_err(|e| {
                    format!("Receiver could not write to temp file {}: {}", tmp_path.display(), e)
                })?;
                hasher.update(&buf[..n]);
                received += n as u64;
                remaining = remaining.saturating_sub(n as u64);

                {
                    let mut state = control.manager.write().await;
                    state.progress_bytes = received;
                }
            }
        }
    }

    // Flush and close the temp file.
    file.flush().map_err(|e| {
        format!(
            "Receiver could not flush temp file {}: {}",
            tmp_path.display(),
            e
        )
    })?;

    // Verify integrity.
    let received_digest = hasher.finalize();
    let received_digest_b64 = base64::engine::general_purpose::STANDARD.encode(received_digest);

    if !received_digest_b64.eq_ignore_ascii_case(&sender_digest) {
        let _ = fs::remove_file(&tmp_path);
        let _ = fs::remove_file(&tmp_path);
        {
            let mut state = control.manager.write().await;
            state.status = "failed".into();
            state.error_message = Some(format!(
                "Integrity check failed. Expected digest: {}. Received digest: {}.",
                sender_digest, received_digest_b64
            ));
        }
        return Err("Integrity check failed".into());
    }

        let final_path = unique_path(&PathBuf::from(&save_directory), &file_name);

    // Move temp file to final location.
        if let Err(e) = fs::rename(&tmp_path, &final_path) {
        let _ = stream.write_all(&[0u8]).await;
        return Err(format!(
            "Receiver: could not finalize file {}: {}",
            final_path.display(),
            e
        ));
    }

    {
        let mut state = control.manager.write().await;
        state.status = "finished".into();
        state.received_file_path = Some(final_path.to_string_lossy().to_string());
        state.progress_bytes = file_size;
    }
    // Tell the sender the file arrived intact and was saved.
    let _ = stream.write_all(&[1u8]).await;
    let _ = stream.flush().await;
    Ok(())
}

// ---------------------------------------------------------------------------
// Sender
// ---------------------------------------------------------------------------

pub async fn run_sender(
    args: SendFileArgs,
    control: Arc<AppTransferControl>,
    mut cancel_rx: mpsc::UnboundedReceiver<()>,
) -> Result<(), String> {
    let source = PathBuf::from(&args.source_path);

    if !source.is_file() {
        return Err(format!("Source path is not a file: {}", args.source_path));
    }

    let file_size = fs::metadata(&source)
        .map_err(|e| format!("Source file metadata error: {}", e))?
        .len();

    // Upper bound to avoid accidentally sending absurdly large files before
    // the app supports resume.
    if file_size > 100_000_000_000_000 {
        return Err("Source file is unreasonably large".into());
    }

    let file_name = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());

    // Compute the digest up front so the receiver can verify.
     {
        let mut state = control.manager.write().await;
        state.kind = "sender".into();
        state.status = "preparing".into();
        state.file_name = Some(file_name.clone());
        state.file_size = Some(file_size);
        state.total_bytes = Some(file_size);
        state.error_message = None;
        state.progress_bytes = 0;
    }

    let source_for_hash = source.clone();
    let hash_task = tokio::task::spawn_blocking(move || -> Result<String, String> {
        let mut file = File::open(&source_for_hash)
            .map_err(|e| format!("Could not open source file: {}", e))?;
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; CHUNK_BYTES];
        loop {
            let n = file
                .read(&mut buf)
                .map_err(|e| format!("Could not read source file: {}", e))?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
        Ok(base64::engine::general_purpose::STANDARD.encode(hasher.finalize()))
    });

    let sender_digest = tokio::select! {
        _ = cancel_rx.recv() => {
            let mut state = control.manager.write().await;
            state.status = "cancelled".into();
            state.error_message = Some("Transfer cancelled".into());
            return Ok(());
        }
        res = hash_task => {
            res.map_err(|e| format!("Hashing task failed: {}", e))??
        }
    };

    let addr = args
        .target_addr
        .parse::<SocketAddr>()
        .map_err(|_| format!("Invalid target address: {}", args.target_addr))?;

    {
        let mut state = control.manager.write().await;
        state.kind = "sender".into();
        state.status = "connecting".into();
        state.file_name = Some(file_name.clone());
        state.file_size = Some(file_size);
        state.total_bytes = Some(file_size);
        state.sender_digest = Some(sender_digest.clone());
        state.error_message = None;
        state.progress_bytes = 0;
    }

    let mut stream = match TcpStream::connect(addr).await {
        Ok(s) => s,
        Err(e) => {
            let mut state = control.manager.write().await;
            state.status = "failed".into();
            state.error_message = Some(format!(
                "Could not connect to the other device. Make sure it is listening on the same local network and that the address is correct."
            ));
            return Err(format!("Could not connect to the other device: {}", e));
        }
    };

    {
        let mut state = control.manager.write().await;
        state.status = "sending".into();
        state.error_message = None;
    }

    let mut file = tokio::fs::File::open(&source).await.map_err(|e| {
        format!("Could not open source file: {}", e)
    })?;
    let mut buf = vec![0u8; CHUNK_BYTES];
    let mut remaining = file_size;

    // Send header:
    // - 8 bytes: file size
    // - 4 bytes: digest length
    // - N bytes: digest ASCII
    stream
        .write_all(&file_size.to_le_bytes())
        .await
        .map_err(|e| format!("Failed to send file-size header: {}", e))?;
    let digest_bytes = sender_digest.as_bytes();
    stream
        .write_all(&(digest_bytes.len() as u32).to_le_bytes())
        .await
        .map_err(|e| format!("Failed to send digest-length header: {}", e))?;
    stream
        .write_all(digest_bytes)
        .await
        .map_err(|e| format!("Failed to send digest: {}", e))?;

        let name_bytes = file_name.as_bytes();
    stream
        .write_all(&(name_bytes.len() as u32).to_le_bytes())
        .await
        .map_err(|e| format!("Failed to send file-name length: {}", e))?;
    stream
        .write_all(name_bytes)
        .await
        .map_err(|e| format!("Failed to send file name: {}", e))?;

    let mut sent: u64 = 0;
    loop {
        tokio::select! {
            _ = cancel_rx.recv() => {
                // Try to close cleanly.
                let _ = stream.shutdown().await;
                {
                    let mut state = control.manager.write().await;
                    state.status = "cancelled".into();
                    state.error_message = Some("Transfer cancelled".into());
                }
                return Ok(());
            }
            read = file.read(&mut buf) => {
                let n = read.map_err(|e| format!("Failed to read source file: {}", e))?;
                if n == 0 {
                    break;
                }
                stream
                    .write_all(&buf[..n])
                    .await
                    .map_err(|e| format!("Failed to send data: {}", e))?;
                sent += n as u64;
                remaining = remaining.saturating_sub(n as u64);

                {
                    let mut state = control.manager.write().await;
                    state.progress_bytes = sent;
                }
            }
        }
    }

        // Close our sending side so the receiver knows no more data is coming.
    stream
        .shutdown()
        .await
        .map_err(|e| format!("Failed to close connection: {}", e))?;

    {
        let mut state = control.manager.write().await;
        state.progress_bytes = file_size;
    }

    // Wait for the receiver's verdict: 1 = verified and saved, 0 = failed.
    let mut reply = [0u8; 1];
    let verdict = tokio::select! {
        _ = cancel_rx.recv() => {
            let mut state = control.manager.write().await;
            state.status = "cancelled".into();
            state.error_message = Some("Transfer cancelled".into());
            return Ok(());
        }
        r = tokio::time::timeout(
            std::time::Duration::from_secs(120),
            stream.read_exact(&mut reply),
        ) => r,
    };

    match verdict {
        Ok(Ok(_)) if reply[0] == 1 => {
            let mut state = control.manager.write().await;
            state.status = "finished".into();
            state.progress_bytes = file_size;
            Ok(())
        }
        Ok(Ok(_)) => Err("The other device reported that the file did not arrive intact.".into()),
        Ok(Err(_)) => Err("The other device closed the connection before confirming the file arrived.".into()),
        Err(_) => Err("Timed out waiting for the other device to confirm the file.".into()),
    }
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// Read an exact number of bytes from a TCP stream while also watching a
/// cancel channel so we can abort without blocking forever.
async fn read_exact_or_cancel(
    stream: &mut TcpStream,
    buf: &mut [u8],
    cancel_rx: &mut mpsc::UnboundedReceiver<()>,
) -> io::Result<()> {
    let mut pos = 0;
    while pos < buf.len() {
        tokio::select! {
            _ = cancel_rx.recv() => {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled"));
            }
            n = stream.read(&mut buf[pos..]) => {
                let n = n?;
                if n == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "connection closed",
                    ));
                }
                pos += n;
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Kept from the scaffold so older frontend integrations still compile while we
// replace them.
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
