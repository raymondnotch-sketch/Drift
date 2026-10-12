import { invoke } from "@tauri-apps/api/core";
import type { TransferState, StartReceiverArgs, SendFileArgs } from "./types";

/**
 * Start listening for incoming file transfers on the specified port.
 */
export async function startReceiver(args: StartReceiverArgs): Promise<TransferState> {
  return await invoke<TransferState>("drift_listen", { args });
}

/**
 * Connect to a receiver and send a file.
 */
export async function sendFile(args: SendFileArgs): Promise<TransferState> {
  return await invoke<TransferState>("drift_connect", { args });
}

/**
 * Cancel the current transfer.
 */
export async function cancelTransfer(): Promise<TransferState> {
  return await invoke<TransferState>("drift_cancel");
}

/**
 * Get the current transfer state.
 */
export async function getTransferState(): Promise<TransferState> {
  return await invoke<TransferState>("drift_state");
}
