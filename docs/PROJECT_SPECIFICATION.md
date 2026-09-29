# Project Specification

## Purpose

freebuff is a desktop application for transferring files directly between nearby
devices over a local network.

It is designed to work without requiring internet access for the core transfer
functionality.

## Product goals

- Transfer files between nearby devices on the same local network
- Work offline on a local network
- Handle large files without loading the entire file into memory unnecessarily
- Give users a simple interface even though the underlying system may be
  technically sophisticated
- Remain maintainable and understandable to developers

## Core user requirements

- Choose a file to send
- Identify the receiving device or session
- Send the file
- See transfer progress
- Save the received file
- Understand what went wrong when something fails

## Non-functional requirements

- Offline-first behavior for local transfer
- Reasonable memory usage for large files
- Streaming or chunked handling where appropriate
- Clear user-facing error messages instead of raw technical noise
- Separation between UI and transfer/engine logic

## Supported connection scenarios

The application should be prepared to work in situations such as:

- Two devices on the same Wi-Fi network
- One device hosting a hotspot and the other connecting to it
- Other compatible local-network configurations

Network creation itself is not the application's first responsibility. The app
focuses on communication over an accessible local network.

## V1 scope

Version 1 establishes the smallest useful transfer flow.

V1 should include:

- Application startup
- Local-network communication
- Basic sender/receiver roles
- File selection on the sender side
- File transfer
- Transfer progress
- Receiver-side file saving
- Basic error handling
- Basic file-integrity verification

V1 is intentionally simple. It should not try to solve every future problem at
once.

## Out of scope for V1

The following are planned or possible later, but not required for V1:

- Multiple-file transfers as a polished flow
- Drag and drop
- Device discovery
- Session codes or QR pairing
- Resumable transfers
- Encryption
- Mobile support
- Accounts or cloud storage
- Advanced transfer history

## Success criteria

For V1, success means:

1. The app builds
2. The app launches
3. Two devices can communicate over the intended local network
4. A sender can transfer a file to a receiver
5. The received file matches the original
6. Common failures are handled without crashing or corrupting the transfer

## Non-goals

- Becoming a generic cloud file service
- Replacing external storage for every use case
- Depending on internet access for local transfer
- Solving every possible network topology on day one
