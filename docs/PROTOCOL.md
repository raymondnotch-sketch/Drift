# Protocol

## Purpose of this document

This document describes the intended communication model between a sender and a
receiver in freebuff.

It is deliberately conceptual for now. The exact wire format and command set will
be refined as V1 is implemented.

## Roles

There are two roles:

- **Sender:** the device that chooses a file and sends it
- **Receiver:** the device that accepts the file and writes it to disk

A transfer is between one sender and one receiver.

## Basic interaction model

The transfer flow is expected to look something like this:

1. One device acts as the receiver and becomes reachable on the local network.
2. The other device acts as the sender.
3. The sender connects to the receiver.
4. The sender streams the file to the receiver.
5. The receiver writes the incoming data to disk.
6. The receiver verifies the file matches the source.
7. Both sides report success or failure to the user.

Exact setup and connection details are implementation work, not a fixed public
spec at this stage.

## Streaming

The protocol should support streaming the file rather than loading it entirely
into memory.

That means:

- the sender reads and sends the file in chunks
- the receiver writes chunks as they arrive
- memory usage stays bounded for large files

This is important because large-file support is a core requirement, and an
approximately 11 GB file is a meaningful practical target.

## File identity and verification

A transferred file should be verifiable on the receiving side.

At minimum, the receiver should be able to confirm that the file it received
matches the file the sender intended to send.

Integrity verification may include things like:

- size comparison
- checksum comparison
- or another integrity mechanism chosen during implementation

The exact mechanism is an implementation detail. The requirement is that the
receiver not silently accept a wrong or corrupted file.

## Transfer state

Both sides should be able to describe what is happening:

- waiting
- connecting
- transferring
- finishing
- succeeded
- failed
- cancelled

The UI displays that state. The backend is responsible for producing it.

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

- multi-file transfers
- resumable transfers
- chunk-level integrity checks
- retry behavior
- session management
- stronger security

Those are planned for later versions, not part of the current protocol spec.

## Current status

The protocol is being shaped during V1 implementation.

This document describes the intended behavior and constraints, not a completed
wire specification.
