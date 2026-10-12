# Drift V1 Manual Testing Guide

This document describes how to manually test the Drift V1 file transfer functionality.

## Prerequisites

- Two instances of Drift running (either on the same machine or two different machines on the same network)
- A test file to transfer (can use `test-data/test-small.txt` or any file)

## Test 1: Basic File Transfer

### Setup

1. Launch two instances of Drift
   - On Windows: Run `npm run tauri dev` in two separate terminals
   - Or run on two different computers on the same network

### Receiver Side

1. Click "Receive a file"
2. Keep the default port (8765) or choose a different one
3. Optionally select a save directory (defaults to home directory)
4. Click "Start listening"
5. Note the status changes to "listening"
6. Find your IP address:
   - Windows: `ipconfig` in Command Prompt
   - Look for IPv4 Address (e.g., 192.168.1.100)

### Sender Side

1. Click "Send a file"
2. Click "Browse" and select a test file
3. Enter the receiver's IP address (e.g., 192.168.1.100 or 127.0.0.1 if testing locally)
4. Enter the port (must match receiver's port, default 8765)
5. Click "Send file"

### Expected Results

- Sender status changes: connecting → sending → finished
- Receiver status changes: listening → receiving → finished
- Progress bar shows transfer progress on both sides
- File size and name displayed correctly
- SHA-256 digest computed and verified
- Received file saved to the specified directory
- Both sides show "finished" status
- File content matches original (manually verify if needed)

## Test 2: Transfer Cancellation

### Steps

1. Start a receiver as in Test 1
2. Start sending a large file (>10MB)
3. While transfer is in progress, click "Cancel" on either side
4. Verify:
   - Status changes to "cancelled"
   - Transfer stops immediately
   - Temporary files are cleaned up
   - Both sides can start new transfers

## Test 3: Error Handling - Wrong Address

### Steps

1. Start a sender
2. Select a file
3. Enter an unreachable IP address (e.g., 192.168.1.254)
4. Click "Send file"
5. Verify:
   - Status shows "failed"
   - Clear error message displayed
   - Application doesn't crash
   - Can retry with correct address

## Test 4: Error Handling - No Receiver

### Steps

1. Start a sender
2. Select a file
3. Enter correct IP but port where no receiver is listening
4. Click "Send file"
5. Verify:
   - Connection fails with clear error message
   - Application remains responsive

## Test 5: Large File Transfer

### Setup

Create a large test file (100MB or more):
```bash
# On Windows PowerShell:
fsutil file createnew test-data\large-file.bin 104857600  # 100 MB

# On Mac/Linux:
dd if=/dev/zero of=test-data/large-file.bin bs=1M count=100
```

### Steps

1. Transfer the large file following Test 1 steps
2. Verify:
   - Progress bar updates smoothly
   - Memory usage remains reasonable
   - Transfer completes successfully
   - File integrity verified (SHA-256 match)
   - Received file size matches original

## Test 6: Different Port

### Steps

1. Start receiver on port 9000 (instead of default 8765)
2. Start sender and specify port 9000
3. Transfer a file
4. Verify successful transfer

## Success Criteria

All tests should:
- Complete without application crashes
- Show appropriate status updates
- Display clear error messages when failures occur
- Preserve file integrity (SHA-256 verification passes)
- Clean up temporary files
- Allow restarting transfers after completion or failure

## Known V1 Limitations (Expected Behavior)

- Receiver always saves as "received_file" (not original filename)
- Only one transfer at a time per instance
- No device discovery - manual IP entry required
- No encryption or authentication
- Transfers cannot be resumed if interrupted
- No automatic retry on failure

These are expected V1 limitations documented in the roadmap.

## Reporting Issues

If you encounter issues not covered by the known limitations:
1. Note the exact steps to reproduce
2. Record any error messages displayed
3. Check console output for detailed logs
4. Document expected vs. actual behavior
