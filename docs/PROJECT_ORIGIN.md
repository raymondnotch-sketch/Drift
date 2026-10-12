# Project Origin

## The practical problem

The project began because the project owner and his brother needed to transfer a
large file between computers.

The available USB storage was not large enough for the file.

That created a practical question:

> How can two nearby computers transfer a large file directly, without requiring
> internet access or sufficiently large external storage?

## Why not just upload it

A natural modern answer is to upload the file somewhere and download it elsewhere.
For this situation, that was not the preferred answer.

The goal was a direct transfer between nearby devices on a local network, without
depending on an internet service for the core transfer.

## Why local network transfer

If two devices are on the same local network, they can in principle communicate
without internet access and without external storage.

That makes local-network file transfer a reasonable solution to the original
problem.

## The design intent

The application is meant to solve a concrete everyday problem first and grow from
there. It should be:

- offline-first
- local-network based
- simple enough for non-technical users
- capable of handling large files
- built in a way developers can understand and maintain

The long-term roadmap may include discovery, pairing, resume, encryption, mobile
support, and packaging, but none of that changes the original motivation.

## The original motivating scenario

Two brothers, a large file, and no sufficiently large USB drive.

That is the real origin of drift.
