# Architecture

## High-level shape

freebuff is split into two main layers:

1. **Frontend**
   - React
   - TypeScript
   - Tailwind CSS
   - Vite

2. **Backend / core**
   - Rust
   - Tauri

The two layers communicate through the Tauri bridge.

## Why this split

The frontend should be good at user interaction and status display. The backend
should be good at the things a file-transfer engine actually needs:

- filesystem access
- streaming large files
- networking
- transfer state
- integrity checks
- error handling
- security-sensitive logic

Keeping the transfer engineering in Rust avoids putting heavy file and network
logic into the UI layer.

## Data flow concept

A simplified transfer flow looks like this:

```
[Sender UI]
  -> choose file
  -> ask backend to start sending

[Backend on sender]
  -> serve or push the file over local network
  -> stream the file in chunks
  -> report progress

[Backend on receiver]
  -> accept the connection
  -> write incoming data to disk
  -> verify integrity
  -> report progress and result
```

The UI does not stream the file itself. It tells the backend what to do and
displays the result.

## Frontend responsibilities

- File selection
- Sending/receiving mode or session UI
- Progress display
- Status messages
- Error presentation
- Settings and preferences
- Launching transfer actions through Tauri commands

## Backend responsibilities

- File system access
- Starting and managing transfers
- Listening for incoming connections
- Connecting to a receiver
- Streaming file data efficiently
- Verifying that the received file matches the source
- Managing transfer state
- Reporting progress and errors back to the frontend

## Large-file handling

Large files are a core requirement, not a later add-on.

The system should avoid loading an entire large file into memory at once. Instead,
it should stream or chunk the file as it is sent and written.

This matters because the original motivating file was large, and an approximately
11 GB file is an important practical stress target.

## Persistence

SQLite should be used only when persistent application metadata is genuinely
required.

Not every piece of state needs a database. Transient transfer state can live in
memory during a transfer. SQLite is for durable metadata where that actually
makes sense.

## Network assumptions

The application expects to communicate over a reachable local network.

Possible environments include:

- existing Wi-Fi network
- device hotspot
- other compatible local-network configurations

The app should not assume internet access is present.

## Extensibility

The architecture is meant to leave room for later work:

- discovery and pairing
- resumable transfers
- stronger security
- transfer history
- broader platform support

Those features are added when they are justified, not because the project started
with them.

## Current architecture status

Right now the project is at the scaffold stage.

What exists:

- frontend shell
- Tauri backend shell
- Rust crate that compiles
- documentation

What does not exist yet:

- the actual transfer engine
- the front-end transfer flow
- discovery
- pairing
- persistence beyond the scaffold
