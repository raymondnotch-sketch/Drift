import { useEffect, useState, useRef } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { homeDir } from "@tauri-apps/api/path";
import { Upload, Download, CheckCircle } from "lucide-react";
import {
  startReceiver,
  sendFile,
  cancelTransfer,
  getTransferState,
  resetTransfer,
} from "./api";
import type { TransferState } from "./types";
import {
  Button,
  Card,
  ScreenHeader,
  ProgressBar,
  StatusMessage,
  FileRow,
} from "./components";
import { formatBytes, mapErrorToFriendlyMessage } from "./utils";
import "./App.css";

type Mode = "home" | "send" | "receive";

function App() {
  const [mode, setMode] = useState<Mode>("home");
  const [state, setState] = useState<TransferState | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Sender state
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [destination, setDestination] = useState<string>("");

  // Receiver state
  const [receiverPort, setReceiverPort] = useState<string>("7000");
  const [saveDirectory, setSaveDirectory] = useState<string>("");

  // Speed state
  const [speed, setSpeed] = useState(0);
  const lastSample = useRef<{ bytes: number; time: number } | null>(null);

  // Poll transfer state (throttled to avoid too many UI updates)
  useEffect(() => {
    if (mode === "home") return;

    let lastUpdate = 0;
    const throttleMs = 100; // Update UI max every 100ms

    const interval = setInterval(async () => {
      const now = Date.now();
      if (now - lastUpdate < throttleMs) return;

      try {
        const transferState = await getTransferState();
        setState(transferState);
                const active =
          transferState.status === "sending" || transferState.status === "receiving";
        if (!active) {
          lastSample.current = null;
          setSpeed(0);
        } else {
          const t = Date.now();
          const prev = lastSample.current;
          if (!prev) {
            lastSample.current = { bytes: transferState.progress_bytes, time: t };
          } else if (t - prev.time >= 500) {
            const bps =
              ((transferState.progress_bytes - prev.bytes) / (t - prev.time)) * 1000;
            setSpeed((old) => (old === 0 ? bps : old * 0.7 + bps * 0.3));
            lastSample.current = { bytes: transferState.progress_bytes, time: t };
          }
        }
        lastUpdate = now;
      } catch (err) {
        console.error("Failed to get transfer state:", err);
      }
    }, 50);

    return () => clearInterval(interval);
  }, [mode]);

  const handleChooseFile = async () => {
    try {
      const file = await open({
        multiple: false,
        directory: false,
      });

      if (file) {
        setSelectedFile(file as string);
        setError(null);
      }
    } catch (err) {
      setError(`Failed to select file: ${err}`);
    }
  };

  const handleChooseSaveDirectory = async () => {
    try {
      const dir = await open({
        multiple: false,
        directory: true,
      });

      if (dir) {
        setSaveDirectory(dir as string);
        setError(null);
      }
    } catch (err) {
      setError(`Failed to select directory: ${err}`);
    }
  };

  const validateDestination = (dest: string): boolean => {
    const pattern = /^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d+$/;
    return pattern.test(dest);
  };

  const handleStartReceiver = async () => {
    try {
      setError(null);
      const port = parseInt(receiverPort, 10);
      if (isNaN(port) || port < 1 || port > 65535) {
        setError("Port must be between 1 and 65535");
        return;
      }

      let dir = saveDirectory;
      if (!dir) {
        dir = await homeDir();
      }

      await startReceiver({
        port,
        save_directory: dir,
      });
    } catch (err) {
      setError(mapErrorToFriendlyMessage(String(err)));
    }
  };

  const handleSendFile = async () => {
    try {
      setError(null);

      if (!selectedFile) {
        setError("Please select a file first");
        return;
      }

      if (!destination.trim()) {
        setError("Enter an address like 192.168.1.15:7000");
        return;
      }

      if (!validateDestination(destination)) {
        setError("Enter an address like 192.168.1.15:7000");
        return;
      }

      const [_ip, portStr] = destination.split(":");
      const port = parseInt(portStr, 10);

      await sendFile({
        target_addr: destination,
        source_path: selectedFile,
        port,
      });
    } catch (err) {
      setError(mapErrorToFriendlyMessage(String(err)));
    }
  };

  const handleCancel = async () => {
    if (
      !window.confirm(
        "Are you sure you want to cancel this transfer?"
      )
    ) {
      return;
    }

    try {
      await cancelTransfer();
    } catch (err) {
      setError(mapErrorToFriendlyMessage(String(err)));
    }
  };

  const handleTryAgain = async () => {
  try {
    await resetTransfer();
  } catch (err) {
    console.error("Reset failed:", err);
  }
  setState(null);
  setError(null);
};

  const handleBack = () => {
    const isActive =
      state &&
      (state.status === "listening" ||
        state.status === "receiving" ||
        state.status === "preparing" ||
        state.status === "connecting" ||
        state.status === "sending");

    if (isActive) {
      if (
        window.confirm(
          "A transfer is currently active. Going back will cancel it. Continue?"
        )
      ) {
        handleCancel();
        setTimeout(() => resetToHome(), 100);
      }
    } else {
      resetToHome();
    }
  };

  const resetToHome = () => {
  resetTransfer().catch(() => {});
  setMode("home");
  setState(null);
  setError(null);
  setSelectedFile(null);
  setDestination("");
};

  const getFileName = (path: string): string => {
    return path.split(/[/\\]/).pop() || path;
  };

  // Home screen
  if (mode === "home") {
    return (
      <div className="min-h-screen bg-slate-50 flex items-center justify-center p-8 dark:bg-slate-900">
        <div className="max-w-2xl w-full">
          {/* Header */}
          <div className="text-center mb-12">
            <h1 className="text-5xl font-bold text-slate-900 mb-3 dark:text-white">
              Drift
            </h1>
            <p className="text-lg text-slate-600 dark:text-slate-400">
              Move files. Stay local.
            </p>
          </div>

          {/* Error */}
          {error && (
            <StatusMessage type="error" className="mb-6">
              {error}
            </StatusMessage>
          )}

          {/* Action cards */}
          <div className="grid md:grid-cols-2 gap-4 mb-10">
            <button
              onClick={() => setMode("send")}
              className="group text-left bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-2xl p-6 hover:border-indigo-600 dark:hover:border-indigo-500 hover:shadow-lg transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500"
            >
              <div className="flex items-center gap-3 mb-3">
                <div className="w-12 h-12 rounded-xl bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center">
                  <Upload className="w-6 h-6 text-indigo-600 dark:text-indigo-400" />
                </div>
                <h2 className="text-2xl font-semibold text-slate-900 dark:text-white">
                  Send files
                </h2>
              </div>
              <p className="text-slate-600 dark:text-slate-400 leading-relaxed">
                Send files directly to another device.
              </p>
            </button>

            <button
              onClick={() => setMode("receive")}
              className="group text-left bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-2xl p-6 hover:border-indigo-600 dark:hover:border-indigo-500 hover:shadow-lg transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500"
            >
              <div className="flex items-center gap-3 mb-3">
                <div className="w-12 h-12 rounded-xl bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center">
                  <Download className="w-6 h-6 text-indigo-600 dark:text-indigo-400" />
                </div>
                <h2 className="text-2xl font-semibold text-slate-900 dark:text-white">
                  Receive files
                </h2>
              </div>
              <p className="text-slate-600 dark:text-slate-400 leading-relaxed">
                Receive files from another device.
              </p>
            </button>
          </div>

          {/* Footer */}
          <p className="text-center text-xs text-slate-400 dark:text-slate-500">
            Version 1.0
          </p>
        </div>
      </div>
    );
  }

  // Send screen
  if (mode === "send") {
        const isActive =
      state &&
      (state.status === "preparing" ||
        state.status === "connecting" ||
        state.status === "sending");
    const isFinished = state && state.status === "finished";
    const isFailed = state && state.status === "failed";
    const isCancelled = state && state.status === "cancelled";
    const showSetup = !state || state.status === "idle";
    const showProgress = isActive;
    const showComplete = isFinished;
    const showError = isFailed || isCancelled;

    return (
      <div className="min-h-screen bg-slate-50 dark:bg-slate-900 p-8">
        <div className="max-w-2xl mx-auto">
          <ScreenHeader
            title="Send files"
            description="Choose the files you want to send."
            onBack={handleBack}
          />

          {error && (
            <StatusMessage type="error" className="mb-6">
              {mapErrorToFriendlyMessage(error)}
            </StatusMessage>
          )}

          {showSetup && (
            <Card>
              <div className="space-y-6">
                {/* File selection */}
                <div>
                  <label className="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
                    Select files
                  </label>
                  {selectedFile ? (
                    <FileRow
                      name={getFileName(selectedFile)}
                      size="File selected"
                      onRemove={() => setSelectedFile(null)}
                    />
                  ) : (
                    <Button variant="secondary" onClick={handleChooseFile} className="w-full">
                      Choose file
                    </Button>
                  )}
                </div>

                {/* Destination */}
                <div>
                  <label className="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
                    Destination
                  </label>
                  <input
                    type="text"
                    value={destination}
                    onChange={(e) => setDestination(e.target.value)}
                    placeholder="192.168.1.15:7000"
                    className="w-full px-4 py-3 border border-slate-200 dark:border-slate-700 rounded-xl bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
                  />
                  {destination && !validateDestination(destination) && (
                    <p className="mt-2 text-sm text-red-600">
                      Enter an address like 192.168.1.15:7000
                    </p>
                  )}
                </div>

                {/* Send button */}
                <Button
                  variant="primary"
                  size="large"
                  onClick={handleSendFile}
                  disabled={!selectedFile || !validateDestination(destination)}
                  className="w-full"
                >
                  Send files
                </Button>
              </div>
            </Card>
          )}

          {showProgress && state && (
            <Card>
              <div className="space-y-4">
                <div className="text-center pb-4 border-b border-slate-100 dark:border-slate-700">
                  <h2 className="text-2xl font-semibold text-slate-900 dark:text-white mb-1">
                    {state.status === "preparing"
                    ? "Preparing file..."
                    : state.status === "connecting"
                    ? "Connecting..."
                    : "Sending files..."}
                  </h2>
                  {state.file_name && (
                    <p className="text-slate-600 dark:text-slate-400 break-all">{state.file_name}</p>
                  )}
                </div>

                {state.total_bytes && state.status === "sending" && (
                  <>
                    <ProgressBar
                      value={state.progress_bytes}
                      max={state.total_bytes}
                    />
                    <div className="flex justify-between text-sm">
                      <span className="text-slate-600 dark:text-slate-400">
                        {formatBytes(state.progress_bytes)} / {formatBytes(state.total_bytes)}
                      </span>
                      <span className="font-semibold text-slate-900 dark:text-white">
                        {((state.progress_bytes / state.total_bytes) * 100).toFixed(1)}%
                      </span>
                    </div>
                    <div className="text-sm text-slate-600 dark:text-slate-400">
                                            {speed > 0 && state.total_bytes && (
                        <div>
                          {formatBytes(speed)}/s · about{" "}
                          {Math.max(
                            1,
                            Math.round((state.total_bytes - state.progress_bytes) / speed)
                          )}
                          s left
                        </div>
                      )}
                    </div>
                  </>
                )}

                <Button variant="danger" onClick={handleCancel} className="w-full">
                  Cancel transfer
                </Button>
              </div>
            </Card>
          )}

          {showComplete && state && (
            <Card>
              <div className="text-center space-y-6">
                <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-green-100 dark:bg-green-900">
                  <CheckCircle className="w-8 h-8 text-green-600 dark:text-green-400" />
                </div>
                <div>
                  <h2 className="text-2xl font-semibold text-slate-900 dark:text-white mb-2">
                    Transfer complete
                  </h2>
                  <p className="text-slate-600 dark:text-slate-400 break-all">
                    {state.file_name && state.file_size && (
                      <>{state.file_name} · {formatBytes(state.file_size)} transferred successfully.</>
                    )}
                  </p>
                  {state.sender_digest && (
                    <p className="text-sm text-green-600 dark:text-green-400 mt-2">
                      Integrity verified
                    </p>
                  )}
                </div>
                <Button variant="primary" size="large" onClick={resetToHome} className="w-full">
                  Back to home
                </Button>
              </div>
            </Card>
          )}

          {showError && state && (
            <Card>
              <div className="text-center space-y-6">
                <StatusMessage type="error">
                  {state.error_message
                    ? mapErrorToFriendlyMessage(state.error_message)
                    : isCancelled
                    ? "Transfer cancelled."
                    : "Transfer failed."}
                </StatusMessage>
                <div className="flex gap-3">
                  <Button variant="secondary" onClick={resetToHome} className="flex-1">
                    Back to home
                  </Button>
                  <Button variant="primary" onClick={handleTryAgain} className="flex-1">
                    Try again
                  </Button>
                </div>
              </div>
            </Card>
          )}
        </div>
      </div>
    );
  }

  // Receive screen
  if (mode === "receive") {
    // const isActive = state && (state.status === "listening" || state.status === "receiving");
    const isFinished = state && state.status === "finished";
    const isFailed = state && state.status === "failed";
    const isCancelled = state && state.status === "cancelled";
    const showSetup = !state || state.status === "idle";
    const showWaiting = state && state.status === "listening";
    const showProgress = state && state.status === "receiving";
    const showComplete = isFinished;
    const showError = isFailed || isCancelled;

    return (
      <div className="min-h-screen bg-slate-50 dark:bg-slate-900 p-8">
        <div className="max-w-2xl mx-auto">
          <ScreenHeader
            title="Receive files"
            onBack={handleBack}
          />

          {error && (
            <StatusMessage type="error" className="mb-6">
              {mapErrorToFriendlyMessage(error)}
            </StatusMessage>
          )}

          {showSetup && (
            <Card>
              <div className="space-y-6">
                {/* Save location */}
                <div>
                  <label className="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
                    Save location
                  </label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={saveDirectory || "Home directory"}
                      readOnly
                      className="flex-1 px-4 py-3 border border-slate-200 dark:border-slate-700 rounded-xl bg-slate-50 dark:bg-slate-800 text-slate-700 dark:text-slate-300"
                    />
                    <Button variant="secondary" onClick={handleChooseSaveDirectory}>
                      Change folder
                    </Button>
                  </div>
                </div>

                {/* Port */}
                <div>
                  <label className="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
                    Port
                  </label>
                  <input
                    type="number"
                    value={receiverPort}
                    onChange={(e) => setReceiverPort(e.target.value)}
                    placeholder="7000"
                    className="w-full px-4 py-3 border border-slate-200 dark:border-slate-700 rounded-xl bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
                  />
                </div>

                {/* Start button */}
                <Button
                  variant="primary"
                  size="large"
                  onClick={handleStartReceiver}
                  className="w-full"
                >
                  Start listening
                </Button>
              </div>
            </Card>
          )}

          {showWaiting && state && (
            <Card>
              <div className="text-center space-y-6">
                <div className="inline-block">
                  <div className="w-16 h-16 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin" />
                </div>
                <div>
                  <h2 className="text-2xl font-semibold text-slate-900 dark:text-white mb-2">
                    Waiting for sender...
                  </h2>
                  <p className="text-slate-600 dark:text-slate-400 mb-4">
                    Make sure the sender connects to:
                  </p>
                  {state.port && (
                    <div className="inline-block px-6 py-3 bg-slate-100 dark:bg-slate-800 rounded-xl">
                      <code className="text-lg font-mono font-semibold text-slate-900 dark:text-white">
                        {/* Backend doesn't expose local IP, show port only */}
                        Port {state.port}
                      </code>
                    </div>
                  )}
                  <p className="text-sm text-slate-500 dark:text-slate-400 mt-4">
                    The sender needs your local IP address and this port.
                  </p>
                </div>
                <Button variant="danger" onClick={handleCancel} className="w-full">
                  Cancel
                </Button>
              </div>
            </Card>
          )}

          {showProgress && state && (
            <Card>
              <div className="space-y-4">
                <div className="text-center pb-4 border-b border-slate-100 dark:border-slate-700">
                  <h2 className="text-2xl font-semibold text-slate-900 dark:text-white mb-1">
                    Receiving files...
                  </h2>
                  {state.file_name && (
                    <p className="text-slate-600 dark:text-slate-400 break-all">{state.file_name}</p>
                  )}
                </div>

                {state.total_bytes && (
                  <>
                    <ProgressBar
                      value={state.progress_bytes}
                      max={state.total_bytes}
                    />
                    <div className="flex justify-between text-sm">
                      <span className="text-slate-600 dark:text-slate-400">
                        {formatBytes(state.progress_bytes)} / {formatBytes(state.total_bytes)}
                      </span>
                      <span className="font-semibold text-slate-900 dark:text-white">
                        {((state.progress_bytes / state.total_bytes) * 100).toFixed(1)}%
                      </span>
                    </div>
                    <div className="text-sm text-slate-600 dark:text-slate-400">
                                            {speed > 0 && state.total_bytes && (
                        <div>
                          {formatBytes(speed)}/s · about{" "}
                          {Math.max(
                            1,
                            Math.round((state.total_bytes - state.progress_bytes) / speed)
                          )}
                          s left
                        </div>
                      )}
                    </div>
                  </>
                )}

                <Button variant="danger" onClick={handleCancel} className="w-full">
                  Cancel transfer
                </Button>
              </div>
            </Card>
          )}

          {showComplete && state && (
            <Card>
              <div className="text-center space-y-6">
                <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-green-100 dark:bg-green-900">
                  <CheckCircle className="w-8 h-8 text-green-600 dark:text-green-400" />
                </div>
                <div>
                  <h2 className="text-2xl font-semibold text-slate-900 dark:text-white mb-2">
                    Transfer complete
                  </h2>
                  <p className="text-slate-600 dark:text-slate-400 break_all">
                    {state.file_name && state.file_size && (
                      <>{state.file_name} · {formatBytes(state.file_size)} transferred successfully.</>
                    )}
                  </p>
                  {state.sender_digest && (
                    <p className="text-sm text-green-600 dark:text-green-400 mt-2">
                      Integrity verified
                    </p>
                  )}
                  {state.received_file_path && (
                    <p className="text-xs text-slate-500 dark:text-slate-400 mt-4 font-mono break-all">
                      {state.received_file_path}
                    </p>
                  )}
                </div>
                <Button variant="primary" size="large" onClick={resetToHome} className="w-full">
                  Back to home
                </Button>
              </div>
            </Card>
          )}

          {showError && state && (
            <Card>
              <div className="text-center space-y-6">
                <StatusMessage type="error">
                  {state.error_message
                    ? mapErrorToFriendlyMessage(state.error_message)
                    : isCancelled
                    ? "Transfer cancelled."
                    : "Transfer failed."}
                </StatusMessage>
                <div className="flex gap-3">
                  <Button variant="secondary" onClick={resetToHome} className="flex-1">
                    Back to home
                  </Button>
                  <Button variant="primary" onClick={handleTryAgain} className="flex-1">
                    Try again
                  </Button>
                </div>
              </div>
            </Card>
          )}
        </div>
      </div>
    );
  }

  return null;
}

export default App;