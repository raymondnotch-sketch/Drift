# Development

## Stack

- React + TypeScript + Tailwind CSS + Vite
- Tauri
- Rust
- SQLite where persistent metadata is actually needed

## What "done" means here

A feature is not done just because code exists.

A feature is done when:

- the implementation exists
- it fits the architecture
- it builds
- relevant tests pass
- it has been manually checked where appropriate
- errors are handled reasonably
- documentation is updated
- the README and status notes remain accurate

## Working style

Development is incremental.

The project tries to add the smallest useful thing that moves the current version
forward, then verify it, then document it.

Large rewrites are avoided unless there is a strong reason.

## Documentation rule

Documentation is updated as the project changes.

If something is planned but not implemented, say so plainly.

For example:

- planned for a later version

not:

- the app now supports this

## Git hygiene

Use Git from the start.

- make meaningful commits
- avoid committing build artifacts, secrets, credentials, test data, and other
  generated or personal files
- keep changes logically grouped

## Testing approach

Testing is risk-based and focused.

Early priority is:

1. build verification
2. launch verification
3. local connection
4. core file transfer
5. file integrity
6. critical failure handling

After that, test what is relevant to the feature being implemented.

Do not burn time on exhaustive edge-case testing before the core flow is working.

## Large-file testing

Large-file support is important.

It should be validated with progressively larger representative files as the
transfer flow improves.

An approximately 11 GB file is an important eventual target, but it does not need
to be transferred during every development step.

## Environment notes

This project is developed with a standard Node + Rust toolchain.

If you are setting up a new checkout, install dependencies with:

```bash
npm install
```

If you need to run the app in development:

```bash
npm run tauri dev
```

## Contribution

Contributions should preserve the project's constraints:

- offline-first
- local-network based
- separation between UI and transfer engine
- maintainability
- honest documentation

If a change would materially alter the architecture, raise it before landing it.
