# Protocol

## Purpose of this document

This document describes the intended communication model between a sender and a
receiver in drift.

It is deliberately conceptual for now. The exact wire format and command set will
be refined as V1 is implemented.

## Roles

There are two roles:

- **Sender:** the device that chooses a file and sends it
- **Receiver:** the device that accepts the file and writes it to disk

A transfer is between one sender and one receiver.

## Basic interaction model

The transfer flow is expected to look something like this:

1. One device starts a receiver and listens on a local TCP port.
2. The other device starts a sender and connects to the receiver's address.
3. The sender streams the chosen file to the receiver in chunks.
4. The receiver writes incoming data to a temporary file as it arrives.
5. When the transfer completes, the receiver verifies the file and moves it into
   its final location.
6. Both sides report success or failure to the user.

For V1 this is intentionally simple. There is no encrypted channel, no device
discovery, and no session code yet.

## Streaming

The transfer streams the file rather than loading it entirely into memory.

That means:

- the sender reads and sends the file in chunks
- the receiver writes chunks as they arrive
- memory usage stays bounded for large files

Large-file support is a core requirement, so V1 is built around chunked I/O
from the start. An approximately 11 GB file is a practical stress target, but
it does not need to be transferred during every development step.

## File identity and verification

V1 verifies a transferred file on the receiving side.

At minimum, the receiver checks that the received file matches the source file.

V1 integrity verification uses:

- file size comparison
- a SHA-256 digest of the complete file compared between sender and receiver

The receiver should not silently accept a wrong or corrupted file.

## Transfer state

V1 models transfer state as:

- idle
- listening
- connecting
- sending
- receiving
- finished
- failed
- cancelled

The UI displays that state. The Rust backend is responsible for producing it.

## Error handling

The protocol should fail in a controlled way.

Examples of things that should be handled gracefully:

- a receiver not being reachable
- a connection dropping during transfer
- a file being unreadable on the sender side
- a write failure on the receiver side

Failure should produce a clear user-facing explanation rather than a raw crash or
obscure network error string.

## Planned improvements

Future protocol work may add:

- multiple-file transfers
- resumable transfers
- interrupted-transfer recovery
- automatic retry
- chunk-level integrity checks
- session codes or QR-based pairing
- device discovery
- stronger security

Those are planned for later versions, not part of V1.

## Current status

V1 implements a basic TCP listen/connect transfer with streaming and file
integrity verification.

Current limitations:

- V1 is not encrypted
- V1 does not authenticate the other device
- V1 does not yet include device discovery
- V1 does not yet include resumable transfers
- V1 still expects the user to provide the receiver address when acting as a
  sender

This document describes the V1 transfer behavior, not a finalized future
protocol.
