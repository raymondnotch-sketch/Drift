// Type definitions matching the Rust backend structures

export interface TransferState {
  kind: string;
  status: string;
  file_name: string | null;
  file_size: number | null;
  received_file_path: string | null;
  progress_bytes: number;
  total_bytes: number | null;
  sender_digest: string | null;
  error_message: string | null;
  port: number | null;
}

export interface StartReceiverArgs {
  port: number;
  save_directory: string;
}

export interface SendFileArgs {
  target_addr: string;
  source_path: string;
  port: number;
}
