# Roadmap

Development is intended to be incremental.

The roadmap is a plan, not a promise. Items may be reordered, refined, or delayed
as the project evolves.

## Version 1 — Basic local file transfer

Status: in progress

Goal:
Transfer a file successfully between two computers over a local network without
requiring internet access.

Planned V1 capabilities:

- application startup
- local-network communication
- basic sender and receiver roles
- file selection on the sender side
- file transfer
- transfer progress
- receiver-side file saving
- basic error handling
- basic file-integrity verification

V1 is intentionally simple. It should not try to build every future feature at
once.

## Version 2 — Improved user experience

Status: planned for a later version

Possible features:

- multiple-file transfers
- improved file selection
- drag and drop
- better transfer progress
- transfer speed
- estimated remaining time
- improved cancellation
- better error messages
- better transfer states

## Version 3 — Device discovery and pairing

Status: planned for a later version

Possible features:

- local device discovery
- device names
- session codes
- QR-based pairing where appropriate
- reduced dependence on manually entering addresses
- better connection handling

Any discovery or pairing technology should be chosen based on real requirements
and platform compatibility, not added because it sounds modern.

## Version 4 — Advanced transfer reliability

Status: planned for a later version

Possible features:

- chunked transfers
- resumable transfers
- interrupted-transfer recovery
- automatic retry
- chunk-level integrity checks
- multiple simultaneous transfers
- transfer history
- improved handling of very large files

## Version 5 — Mobile and cross-platform expansion

Status: planned for a later version

Possible goals:

- mobile-friendly experience
- Android support where technically practical
- phone-to-laptop transfers
- laptop-to-phone transfers
- phone-to-phone transfers
- expanded platform support

Cross-platform support should not compromise the stability of the core transfer
architecture.

## Version 6 — Production-ready platform

Status: planned for a later version

Possible goals:

- stronger security
- encryption
- persistent transfer sessions
- advanced transfer history
- comprehensive settings
- advanced recovery
- performance optimization
- production packaging
- release infrastructure
- public distribution

Features should be introduced based on demonstrated requirements rather than
speculation.

## Current reality

Right now the project is at the initialization/scaffold stage.

What exists:

- approved tech stack
- repository structure
- documentation
- a buildable and launchable scaffold

What does not exist yet:

- the actual transfer engine and flow
- discovery and pairing
- resume and advanced reliability
- mobile support
- production packaging

The next step after this scaffold is to begin implementing the V1 transfer flow.
