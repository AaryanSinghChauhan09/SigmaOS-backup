# SigmaOS Backup

Rust types and state models for backup jobs, snapshots, recovery, and backup
destinations. The crate exposes managers for backup job state and snapshot
records.

## Current behavior and limits

Several prominent APIs are simulations rather than backup operations. For
example, `SovereignBackupToolEngine::create_backup_archive()` estimates a fixed
deduplication ratio and appends an in-memory record; it does not read source
files, write an archive, or encrypt data. Its `encrypted` field reflects a
configuration flag, not cryptographic processing. `restore_archive()` checks
whether a record exists; it does not restore data.

Do not rely on these models to protect or recover user data. A real backup path
needs a documented source and destination, authenticated encryption, integrity
checks, retention behavior, interruption handling, and a restore drill that
verifies restored contents.

## Development

The Rust validation workflow runs `cargo test --all-targets --locked`; inspect
the result for the commit under review. Unit tests of record models are not
evidence of successful backup or restore behavior.
