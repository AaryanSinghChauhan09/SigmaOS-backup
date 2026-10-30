# SigmaOS-backup

Backup component for SigmaOS.

## Development

```bash
cargo check
cargo test
```

Backups must expose destination, retention, encryption, and verification state;
successful scheduling is not evidence that a restore works.
