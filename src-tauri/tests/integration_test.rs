use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{timeout, Duration};

// Import from the library
use drift_lib::transfer::{
    run_receiver, run_sender, AppTransferControl, SendFileArgs, StartReceiverArgs, TransferState,
};

#[tokio::test]
async fn test_basic_file_transfer() {
    // Setup paths
    let test_dir = PathBuf::from("../test-data");
    let source_file = test_dir.join("test-small.txt");
    let receive_dir = test_dir.join("received");

    // Create receive directory
    fs::create_dir_all(&receive_dir).unwrap();

    // Create test file if it doesn't exist
    if !source_file.exists() {
        fs::write(&source_file, "Test content for transfer").unwrap();
    }

    // Setup receiver
    let receiver_state = Arc::new(RwLock::new(TransferState::default()));
    let receiver_control = Arc::new(AppTransferControl::new(receiver_state.clone()));
    let (receiver_cancel_tx, receiver_cancel_rx) = mpsc::unbounded_channel();

    receiver_control
        .set_cancel_tx(receiver_cancel_tx.clone())
        .await;

    let receiver_args = StartReceiverArgs {
        port: 18765,
        save_directory: receive_dir.to_string_lossy().to_string(),
    };

    let receiver_control_clone = receiver_control.clone();
    let receiver_handle = tokio::spawn(async move {
        run_receiver(receiver_args, receiver_control_clone, receiver_cancel_rx).await
    });

    // Give receiver time to start listening
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Setup sender
    let sender_state = Arc::new(RwLock::new(TransferState::default()));
    let sender_control = Arc::new(AppTransferControl::new(sender_state.clone()));
    let (_sender_cancel_tx, sender_cancel_rx) = mpsc::unbounded_channel();

    let sender_args = SendFileArgs {
        target_addr: "127.0.0.1:18765".to_string(),
        source_path: source_file.to_string_lossy().to_string(),
        port: 18765,
    };

    let sender_control_clone = sender_control.clone();
    let sender_handle = tokio::spawn(async move {
        run_sender(sender_args, sender_control_clone, sender_cancel_rx).await
    });

    // Wait for both to complete with timeout
    let receiver_result = timeout(Duration::from_secs(10), receiver_handle).await;
    let sender_result = timeout(Duration::from_secs(10), sender_handle).await;

    // Check results
    assert!(receiver_result.is_ok(), "Receiver timed out");
    assert!(sender_result.is_ok(), "Sender timed out");

    let receiver_final = receiver_result.unwrap().unwrap();
    let sender_final = sender_result.unwrap().unwrap();

    assert!(receiver_final.is_ok(), "Receiver failed: {:?}", receiver_final);
    assert!(sender_final.is_ok(), "Sender failed: {:?}", sender_final);

    // Verify the received file exists
    let received_file = receive_dir.join("received_file");
    assert!(received_file.exists(), "Received file does not exist");

    // Verify content matches
    let original_content = fs::read(&source_file).unwrap();
    let received_content = fs::read(&received_file).unwrap();
    assert_eq!(
        original_content, received_content,
        "File content mismatch"
    );

    // Cleanup
    fs::remove_file(received_file).ok();
}
