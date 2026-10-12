# Drift

Local, offline-first file transfer for nearby devices.

## What it is

Drift is a desktop application that lets you transfer files directly between
nearby computers over a local network, without uploading them to the internet and
without needing a large USB drive.

The UI is built with React + TypeScript and Tailwind CSS. The desktop app is
Tauri. The transfer/core logic lives in Rust.

## Why it exists

The project started from a simple problem:

> Two brothers needed to move a large file between computers, but the available
> USB storage was not large enough.

Instead of buying another drive or uploading the file somewhere, the idea was to
transfer the file directly between the two machines over the local network.

That practical problem is the reason this project exists.

## Offline-first

The transfer path works on a local network even when there is no internet access.

Supported connection scenarios:

- Two devices on the same Wi-Fi network
- One device creating a hotspot and the other connecting to it
- Other compatible local-network setups

The application does not rely on cloud storage or a server for the core transfer
functionality.

## Current status

**Version 1.0 — Basic local transfer is now implemented**

**What works now:**

- Application builds and launches
- React + TypeScript + Tailwind CSS frontend with functional UI
- Tauri desktop shell
- Rust backend with complete V1 transfer engine
- Receiver mode: listen for incoming file transfers on a TCP port
- Sender mode: connect to a receiver and send a file
- File streaming with chunked I/O (large files supported)
- Transfer progress tracking
- SHA-256 integrity verification
- Cancellation support
- Error handling with user-facing messages
- Clean, functional UI for both sender and receiver roles

**Current V1 limitations:**

- No device discovery (manual IP address entry required)
- No encryption or authentication
- No resumable transfers
- No multiple-file transfers
- Single transfer at a time
- No session codes or QR pairing
- Desktop only (no mobile support yet)

These limitations are expected for V1 and will be addressed in future versions.

## Technology stack

- **Frontend:** React, TypeScript, Vite, Tailwind CSS
- **Desktop shell:** Tauri
- **Core/backend:** Rust with Tokio async runtime
- **Transfer protocol:** TCP with streaming and SHA-256 verification
- **Version control:** Git

## Architecture (high level)

The UI and the transfer engine are kept separate.

```
React + TypeScript
  -> Tauri IPC
    -> Rust transfer engine
      -> TCP networking / filesystem
```

The frontend handles user interaction, file selection, status display, and
transfer progress. The Rust layer owns the real transfer engineering: filesystem
interaction, transfer management, networking, file streaming, integrity
verification, transfer state, error handling, and security-related concerns.

## Project structure

Key directories:

- `src/` — React frontend
- `src-tauri/` — Tauri/Rust backend
- `docs/` — project documentation
- `test-data/` — test files for development

## Documentation

See the `docs/` directory for:

- `PROJECT_ORIGIN.md`
- `PROJECT_SPECIFICATION.md`
- `ARCHITECTURE.md`
- `NETWORKING.md`
- `PROTOCOL.md`
- `SECURITY.md`
- `DEVELOPMENT.md`
- `DEVELOPER_GUIDE.md`
- `ROADMAP.md`

## Getting started

### Prerequisites

- Node.js and npm
- Rust toolchain (rustc + cargo)
- Git

### Install dependencies

```bash
npm install
```

### Run in development

```bash
npm run tauri dev
```

The application will launch with the Drift interface. Choose "Receive a file"
or "Send a file" to start a transfer.

### Build for production

```bash
npm run tauri build
```

### Using drift

**To receive a file:**

1. Launch drift on the receiving computer
2. Click "Receive a file"
3. Choose a port (default 8765) and optionally a save directory
4. Click "Start listening"
5. Note your IP address (you'll need to provide this to the sender)
6. The application will wait for an incoming connection

**To send a file:**

1. Launch drift on the sending computer
2. Click "Send a file"
3. Browse and select the file you want to send
4. Enter the receiver's IP address (e.g., 192.168.1.100)
5. Enter the port (must match the receiver's port, default 8765)
6. Click "Send file"
7. Watch the progress as the file transfers

**Finding your IP address:**

- Windows: Open Command Prompt and run `ipconfig`
- Mac/Linux: Open Terminal and run `ifconfig` or `ip addr`
- Look for your local network IP (usually starts with 192.168.x.x or 10.x.x.x)

## Testing

V1 testing is primarily manual:

1. Launch two instances of drift (or run on two computers on the same network)
2. Start a receiver on one instance
3. Send a file from the other instance
4. Verify the file transfers successfully
5. Verify the integrity check passes
6. Test cancellation
7. Test error scenarios (wrong address, connection drop, etc.)

Automated tests can be added in future versions.

## Roadmap

Development is intended to be incremental.

- **Version 1 (complete):** basic local file transfer with manual IP entry
- **Version 2 (planned):** improved UX, multiple files, drag and drop, better progress
- **Version 3 (planned):** device discovery and pairing
- **Version 4 (planned):** resumable transfers and advanced reliability
- **Version 5 (planned):** mobile and cross-platform expansion
- **Version 6 (planned):** production readiness, encryption, packaging, release

A detailed roadmap lives in `docs/ROADMAP.md`.

## Known issues

- V1 requires manual IP address entry (no device discovery yet)
- V1 is not encrypted (security planned for V6)
- V1 does not authenticate devices (planned for later versions)
- Transfers are not resumable in V1 (planned for V4)
- The receiver always saves to `received_file` (better naming planned for V2)

## Original problem

Two brothers, a large file, and no sufficiently large USB drive.

That is the practical starting point for the whole project.

## Contributing

This project is currently in early development. Contributions, bug reports, and
feature suggestions are welcome.

## License

This project does not currently publish a chosen license. If you plan to use or
distribute it, choose a license first and document it explicitly.

---

**V1 is complete. V2 planning will begin after initial real-world usage and feedback.**
