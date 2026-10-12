# Networking

## Network model

drift is built around local-network communication between two devices.

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

## What V1 implements

V1 implements a basic TCP transfer between two devices on the same reachable
local network.

One device listens for an incoming connection. The other device connects to that
listener and sends a file.

The Rust backend owns the connection, the streaming, and the file writing.

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

V1 implements a basic TCP sender and receiver with streaming and integrity
verification.

Current limitations:

- no encryption
- no authentication of the peer
- no automatic device discovery
- sender still needs the receiver address for V1
