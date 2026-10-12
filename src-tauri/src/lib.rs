pub mod transfer;

use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{AppHandle, Manager};
use transfer::{AppTransferControl, TransferState};

// ---------------------------------------------------------------------------
// App-scoped transfer storage
// ---------------------------------------------------------------------------

pub struct AppTransferState {
    pub control: Arc<AppTransferControl>,
}

/// If the caller needs the cancel channel or manager directly, it should call
/// `app_transfer_control` instead, because that helper keeps the read guard alive
/// for the duration of the returned clone.
fn app_transfer_state(app: &AppHandle) -> Result<Arc<RwLock<AppTransferState>>, String> {
    app.try_state::<Arc<RwLock<AppTransferState>>>()
        .map(|state| state.inner().clone())
        .ok_or_else(|| "Transfer state not initialized".to_string())
}

async fn app_transfer_control(app: &AppHandle) -> Result<Arc<AppTransferControl>, String> {
    let app_state = app_transfer_state(app)?;
    let app_state = app_state.read().await;
    Ok(app_state.control.clone())
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

#[tauri::command]
async fn drift_listen(
    app: AppHandle,
    args: transfer::StartReceiverArgs,
) -> Result<TransferState, String> {
    let control = app_transfer_control(&app).await?;

    {
        let existing = control.manager.read().await;
        if existing.status != "idle" && existing.status != "finished" && existing.status != "failed" {
            return Err("A transfer is already active on this side".into());
        }
    }

    let (cancel_tx, cancel_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    control.set_cancel_tx(cancel_tx).await;

    let control2 = control.clone();
    tokio::spawn(async move {
        // Reset state for the listener task.
        {
            let mut state = control2.manager.write().await;
            *state = TransferState::default();
            state.kind = "receiver".into();
            state.status = "listening".into();
            state.port = Some(args.port);
        }

        let control3 = control2.clone();
        if let Err(err) = transfer::run_receiver(args, control2, cancel_rx).await {
            let mut state = control3.manager.write().await;
            state.status = "failed".into();
            state.error_message = Some(err);
        }
    });

    let state = app_transfer_control(&app).await?.manager.read().await.clone();
    Ok(state)
}

#[tauri::command]
async fn drift_connect(
    app: AppHandle,
    args: transfer::SendFileArgs,
) -> Result<TransferState, String> {
    let control = app_transfer_control(&app).await?;

    {
        let existing = control.manager.read().await;
        if existing.status != "idle" && existing.status != "finished" && existing.status != "failed" {
            return Err("A transfer is already active on this side".into());
        }
    }

    let (cancel_tx, cancel_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    control.set_cancel_tx(cancel_tx).await;

    let control2 = control.clone();
    tokio::spawn(async move {
        let send_args = transfer::SendFileArgs {
            target_addr: args.target_addr,
            source_path: args.source_path,
            port: args.port,
        };
        // Reset state for the sender task.
        {
            let mut state = control2.manager.write().await;
            *state = TransferState::default();
            state.kind = "sender".into();
            state.status = "connecting".into();
        }

        let control3 = control2.clone();
        if let Err(err) = transfer::run_sender(send_args, control2, cancel_rx).await {
            let mut state = control3.manager.write().await;
            state.status = "failed".into();
            state.error_message = Some(err);
        }
    });

    let state = app_transfer_control(&app).await?.manager.read().await.clone();
    Ok(state)
}

#[tauri::command]
async fn drift_cancel(app: AppHandle) -> Result<TransferState, String> {
    let control = app_transfer_control(&app).await?;

    let tx = control.take_cancel_tx().await;
    match tx {
        Some(tx) => {
            let _ = tx.send(());
            Ok(app_transfer_control(&app).await?.manager.read().await.clone())
        }
        None => Err("No active transfer to cancel".into()),
    }
}

#[tauri::command]
async fn drift_state(app: AppHandle) -> Result<TransferState, String> {
    let control = app_transfer_control(&app).await?;
    let state = {
        let manager = control.manager.read().await;
        manager.clone()
    };
    Ok(state)
}

// ---------------------------------------------------------------------------
// App entry point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Arc::new(RwLock::new(AppTransferState {
                control: Arc::new(AppTransferControl::new(
                    Arc::new(RwLock::new(TransferState::default())),
                )),
            })));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            transfer::greet,
            drift_listen,
            drift_connect,
            drift_cancel,
            drift_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
