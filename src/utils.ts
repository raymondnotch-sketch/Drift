export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
}

export function formatSpeed(bytesPerSecond: number): string {
  return `${formatBytes(bytesPerSecond)}/s`;
}

export function mapErrorToFriendlyMessage(error: string): string {
  // Map common backend errors to friendly messages
  if (error.includes("Could not connect")) {
    return "Couldn't connect. Check that both devices are on the same network and the address is correct.";
  }
  if (error.includes("cancelled") || error.includes("canceled")) {
    return "Transfer cancelled.";
  }
  if (error.includes("Integrity check failed")) {
    return "File integrity check failed. The received file doesn't match the original.";
  }
  if (error.includes("Port")) {
    return "Couldn't bind to port. The port may already be in use.";
  }
  if (error.includes("Source file")) {
    return "Couldn't read the source file. Make sure it still exists.";
  }
  
  // Return original if no mapping found
  return error;
}
