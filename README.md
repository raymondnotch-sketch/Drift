# freebuff

Local, offline-first file transfer for nearby devices.

## What it is

freebuff is a desktop application that lets you transfer files directly between
nearby computers over a local network, without uploading them to the internet and
without needing a large USB drive.

The UI is built with React + TypeScript and Tailwind CSS. The desktop app is
Tauri. The transfer/core logic will live in Rust.

## Why it exists

The project started from a simple problem:

> Two brothers needed to move a large file between computers, but the available
> USB storage was not large enough.

Instead of buying another drive or uploading the file somewhere, the idea was to
transfer the file directly between the two machines over the local network.

That practical problem is the reason this project exists.

## Offline-first

The transfer path is meant to work on a local network even when there is no
internet access.

Supported connection scenarios (conceptual, not all implemented yet):

- Two devices on the same Wi-Fi network
- One device creating a hotspot and the other connecting to it
- Other compatible local-network setups

The application does not rely on cloud storage or a server for the core transfer
functionality.

## Current status

- **Version:** 0.1.0 (scaffold / initialization milestone)
- **Primary goal right now:** a clean, buildable, launching application with the
  approved tech stack and initial documentation in place.

**What works now:**

- Application scaffold builds and launches
- React + TypeScript + Tailwind CSS frontend
- Tauri desktop shell
- Rust backend crate compiles cleanly
- Repository structure and initial documentation exist

**What is NOT implemented yet:**

- File transfer
- Device connection / session logic
- Local network discovery
- Transfer progress, resume, encryption, mobile support, and similar features

Those are planned for later versions.

## Technology stack

- **Frontend:** React, TypeScript, Vite, Tailwind CSS
- **Desktop shell:** Tauri
- **Core/backend:** Rust
- **Local persistence:** SQLite will be used where application metadata truly
  needs to persist
- **Version control:** Git

## Architecture (high level)

The UI and the transfer engine are kept separate.

Conceptually:

```
React + TypeScript
  -> Tauri
    -> Rust application core
      -> local network / filesystem
```

The frontend is responsible for user interaction, file selection, status display,
transfer progress, settings, and user-facing error messages.

The Rust layer should own the real transfer engineering:

- filesystem interaction
- transfer management
- networking
- file streaming
- integrity verification
- transfer state
- error handling
- security-related concerns

This separation matters because large-file transfer logic should not live in the
frontend, and streaming/integrity work is much more natural in Rust.

## Project structure

Key directories:

- `src/` — React frontend
- `src-tauri/` — Tauri/Rust backend
- `docs/` — project documentation
- `test-data/` — optional place for test files used during development

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

### Build for production

```bash
npm run tauri build
```

### Running tests

Right now the project does not have a large automated test suite. Early
development is focused on build verification, launch verification, and manual
testing of the actual transfer flow once it exists.

## Roadmap

Development is intended to be incremental.

- **Version 1:** basic local file transfer between two devices
- **Version 2:** better UX, multiple files, speed/time estimates, drag and drop
- **Version 3:** device discovery and pairing
- **Version 4:** resumable and more reliable transfers
- **Version 5:** broader cross-platform/mobile reach where practical
- **Version 6:** production readiness, stronger security, packaging, and release

A detailed roadmap lives in `docs/ROADMAP.md`.

## Original problem

Two brothers, a large file, and no sufficiently large USB drive.

That is the practical starting point for the whole project.

## License

This project does not currently publish a chosen license. If you plan to use or
distribute it, choose a license first and document it explicitly.
