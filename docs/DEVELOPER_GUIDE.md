# Developer Guide

This guide explains how drift is put together and why.

It is written so that someone who did not write the original code can read it and
understand the system.

## The big picture

drift is a desktop app for transferring files between nearby devices over a
local network.

The app is split into two main concerns:

1. **Frontend**
   - React
   - TypeScript
   - Tailwind CSS
   - Vite

2. **Backend / core**
   - Rust
   - Tauri

The frontend talks to the backend through Tauri.

The frontend is for the user. The backend is for the transfer engine.

## Repository layout

### `src/`

The React frontend.

This is where you will find:

- the main app component
- UI pages/components
- frontend state related to the UI
- calls into the Rust backend through the Tauri API

### `src-tauri/`

The Rust side of the app.

This is where you will find:

- the Tauri application configuration
- the Rust crate
- Tauri commands
- backend logic for the transfer engine as it is built

This is the place for filesystem work, networking, streaming, integrity checks,
and transfer state.

### `docs/`

Project documentation.

Documentation is treated as a first-class part of the project, not an optional
afterthought.

### `public/`

Static assets served by the frontend dev server or bundled into the app.

### `test-data/`

An optional location for test files during development.

Large test files should usually live here only when they are actually useful for
a current test or validation step.

## Why the project is split this way

A file-transfer application has two very different jobs.

The UI job is:

- let the user pick a file
- show progress
- explain status
- present errors in plain language

The engine job is:

- read and write files efficiently
- move data over the network
- keep memory usage under control
- verify integrity
- manage state and failure

Those jobs do not belong in the same layer.

React is good for the UI. Rust is a strong choice for the engine.

That is why the frontend remains thin and the backend owns the real transfer
work.

## Frontend/backend communication

The frontend does not implement file transfer directly.

Instead:

1. The UI asks the backend to do something, such as:
   - start listening as a receiver
   - connect to a receiver as a sender
   - begin sending a chosen file
   - cancel a transfer
2. The backend does the work.
3. The backend reports back progress, status, and errors.
4. The UI renders that information.

This keeps the transfer logic in Rust, where it is easier to handle streaming and
filesystem concerns correctly.

## Rust architecture

The Rust side should eventually contain the core responsibilities:

- file system interaction
- transfer management
- networking
- device communication
- file streaming
- integrity verification
- transfer state
- error handling
- security-related functionality

The exact module structure will evolve as V1 is implemented.

For now, the important principle is:

- keep transfer logic in Rust
- keep the frontend focused on UI and orchestration
- do not let the UI become the place where large-file handling lives

## Transfer architecture

A transfer has two sides:

- **sender**
- **receiver**

The sender chooses a file and sends it. The receiver accepts it and writes it to
disk.

The system is designed around streaming. That means the file is sent and received
in chunks rather than being loaded entirely into memory.

Streaming matters because large files are a core requirement. An approximately 11
GB file is a practical stress test.

The backend should:

- read the source file efficiently
- send data in chunks
- write received data efficiently
- verify the result
- avoid unnecessary copies and buffering

## Network communication

Networking belongs in the Rust backend.

The application is designed to work over a local network without requiring internet
access.

Possible environments include:

- the same Wi-Fi network
- a device hotspot
- other compatible local-network configurations

The app is not initially responsible for creating the operating system's hotspot.
Network creation and transfer are separate concerns.

## File handling

File handling should be deliberate.

Key rules:

- do not load huge files into memory unnecessarily
- stream when possible
- handle missing files, permission problems, and write failures gracefully
- give the user a usable error instead of internal noise

The backend is the right place for this logic.

## Transfer lifecycle

In V1, a transfer lifecycle looks like this:

1. **Preparation**
   - sender chooses a file
   - receiver starts listening on a TCP port

2. **Connection**
   - sender connects to the receiver's address

3. **Transfer**
   - sender streams the file in chunks
   - receiver writes chunks to a temporary file
   - sender reports progress

4. **Completion**
   - receiver finishes writing the temporary file
   - receiver computes the SHA-256 digest
   - receiver verifies size and digest against the sender's values
   - if verification succeeds, the receiver moves the file to its final location

5. **Result**
   - the UI shows success or failure

The lifecycle is explicit on purpose. That makes the transfer observable and
easier to debug.

## Error handling

Error handling should be two-layered:

1. **Internal errors**
   - handled in the backend where possible
   - converted into usable status before reaching the UI

2. **User-facing errors**
   - expressed in plain language
   - helpful about what the user can check

For example, instead of showing raw network jargon, the UI should say something
like:

> Could not connect to the other device. Make sure both devices are connected to
> the same local network.

Technical details can still be available through logs or diagnostics if needed.

## Integrity verification

Integrity verification is a requirement, not a nice-to-have.

In V1, the receiver confirms that the received file matches the source file using
two checks:

- file size comparison
- SHA-256 digest comparison

The sender computes the SHA-256 digest of the file it is sending. The receiver
computes the SHA-256 digest of the file it received and compares it with the
sender's digest.

This protects against silent corruption and makes the transfer trustworthy.

Integrity verification is done in the Rust backend, not in the UI.

## State management

There are two kinds of state:

- **transient transfer state**
  - belongs in memory during a transfer
  - describes what is happening right now

- **persistent application metadata**
  - belongs in SQLite when durability is actually needed
  - used only where persistence is genuinely useful

Do not put everything into a database just because a database exists.

## Configuration

Right now, the main configuration is the Tauri app configuration and the frontend
build configuration.

As the project grows, configuration may include things like:

- app window settings
- build settings
- optional user preferences

Any new configuration should be justified by a real need.

## How to run the project

### Install dependencies

```bash
npm install
```

### Run in development

```bash
npm run tauri dev
```

This starts the Tauri development flow with the Vite frontend.

### Build for production

```bash
npm run tauri build
```

## How to build the Rust side

From the repository root, you can check the Rust crate with:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

Or work from inside `src-tauri/` if you prefer.

## How to test the project

Testing is risk-based.

For V1, the priority is:

1. build verification
2. launch verification
3. local connection
4. core file transfer
5. file integrity
6. critical failure handling

Manual testing is expected and appropriate for the transfer flow, especially early
on.

Use progressively larger test files as the transfer logic improves. An
approximately 11 GB file is an eventual validation target, not a requirement for
every iteration.

## How to debug common problems

### Frontend does not appear

- check that the Vite dev server or built frontend is serving correctly
- check the Tauri dev flow is actually running
- look at browser devtools if the UI is visible but wrong

### Rust build issues

- read the cargo error output carefully
- check dependency versions
- confirm the Rust toolchain is installed

### Transfer does not work

- confirm both devices are on the same reachable local network
- confirm the receiver is listening
- confirm the sender is connecting to the right target
- check for firewall or network isolation issues
- look at backend logs if available

### Large file problems

- check memory behavior
- check that streaming is actually happening
- check that chunking is implemented where needed
- avoid full-file buffering

## How to add a new feature

A good order is:

1. Decide where the feature belongs.
2. If it is transfer-related, put the logic in Rust.
3. If it is UI-related, put the UI in React.
4. Add the smallest working version.
5. Verify it builds.
6. Verify it manually where appropriate.
7. Update documentation.

If the feature would change the core architecture, pause and discuss it before
landing it.

## Important technical constraints

- Keep the transfer engine in Rust.
- Keep the UI in React/TypeScript.
- Stream large files instead of loading them fully into memory.
- Use SQLite only for persistent metadata that genuinely needs it.
- Do not add dependencies unless they solve a real problem.

## Security considerations

Security is important, but claims about security must match reality.

Do not claim encryption, authentication, or other security properties until they
are actually implemented and reviewed.

Integrity verification is part of the core requirements.

Future security work may include:

- encryption
- authentication or pairing
- reduced network exposure
- stronger handling of session and transfer state

## Performance considerations

Performance matters because the app is meant for large files.

Avoid:

- full-file buffering
- unnecessary memory copies
- loading huge data structures
- disk duplication
- obvious network bottlenecks

Prefer streaming and efficient I/O.

Do not optimize prematurely. Measure when it matters.

## Reading this guide together with the docs

This guide is the implementation map. The other docs provide the context:

- `PROJECT_ORIGIN.md`
- `PROJECT_SPECIFICATION.md`
- `ARCHITECTURE.md`
- `NETWORKING.md`
- `PROTOCOL.md`
- `SECURITY.md`
- `ROADMAP.md`

If you are new to the project, start with the origin and specification, then read
the architecture and developer guide.
