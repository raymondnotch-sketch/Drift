# Networking

## Network model

freebuff is built around local-network communication between two devices.

The two devices may be connected through different local-network arrangements,
including:

- an existing Wi-Fi network
- one device acting as a hotspot and the other connecting to it
- other compatible local-network configurations

Internet access is not required for the core transfer path.

## What the network layer is responsible for

The Rust backend owns the networking work. That includes:

- establishing a connection between sender and receiver
- transferring bytes efficiently
- handling connection errors
- closing connections cleanly
- supporting the transfer state that the UI displays

The UI should not implement its own transfer networking. It should ask the
backend to connect or accept, and then show what the backend reports.

## What the application does not own yet

The application is not initially responsible for creating the operating system's
hotspot or otherwise managing the network itself.

Network creation and file transfer are separate concerns. The app focuses on
communicating over a local network that already exists or that the user has set up.

Local device discovery, automatic peer detection, and similar features are future
work, not current behavior.

## Why streaming matters here

Local transfer is not just about sending bytes. It is about sending potentially
large files without turning the transfer into a memory problem.

That is why the networking path is tied closely to streaming and chunked handling
in the backend, rather than dumping whole files into memory.

## Security boundary

The networking layer should eventually be treated as a trust and security boundary.
Any future encryption, authentication, or access-control work belongs with the
networking and transfer logic, not as a UI concern.

Right now, the main requirement is that the app does not pretend to be more secure
than it is. Planned security work is planned, not delivered.

## Current state

Today, networking is planned and scaffolded, but the actual transfer protocol and
connection flow are being implemented as V1 work.

This document describes the intended model and constraints.
