# Security

## Security posture

drift is being built with security in mind, but it does not yet provide a
complete security story.

This document describes the current assumptions, the intended direction, and what
is not yet implemented.

## Current trust model

For now, the application is aimed at transfer between nearby devices on a local
network that the users control or understand.

That does not mean the transfer is automatically safe in every environment. It
means the project is oriented around a local, direct transfer use case and should
not pretend to be something it is not.

## What is not implemented yet

The following are planned or possible later, but are not current guarantees:

- strong transport encryption
- authenticated sessions
- access control beyond the basic transfer flow
- advanced threat modeling for hostile networks

If a feature is not implemented and tested, this document does not claim that it
exists.

## Security-relevant concerns

Even early in development, several things matter:

- Large files must be handled without unnecessary memory exposure.
- File paths and file access must be handled carefully.
- The transfer engine should not trust input blindly.
- Errors should not leak more information than necessary into the UI.
- Any future network exposure should be treated as a security boundary.

## Integrity

File integrity is a requirement.

The receiver should verify that the received file matches the sender's file. This
is both a reliability concern and a security-relevant concern.

## Privacy and network exposure

Because the application is local-network based, its exposure is different from a
cloud service. That does not remove the need for careful design.

Future work may need to consider:

- who can reach the receiver
- how sessions are established
- what information is visible to other devices on the network
- how to reduce unnecessary exposure

## Implementation rule

Do not claim security properties that have not been implemented and reviewed.

If encryption, authentication, or another security feature is planned, label it as
planned. If it is implemented, document what it does and what it does not do.

## Current status

Security work is planned for later versions.

For V1, the priority is a working, honest, and predictable transfer flow with
basic integrity verification and clear error handling.
