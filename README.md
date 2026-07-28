# dev-clean

- Disk clean-up utility for developers, not a file deletion tool.

- Scan or remove common developer junk folders (`node_modules`, `target`, `dist`, `build`, `.angular`, `.next`, `.cache`, `__pycache__`) under a path you choose.

## Requirements

- Rust / Cargo ([rustup](https://rustup.rs))

## Build & run

From the project directory:

```bash
cargo build
cargo run -- scan <PATH>
cargo run -- clean <PATH> --dry-run
cargo run -- clean <PATH>
```

Release binary:

```bash
cargo build --release
./target/release/dev-clean scan <PATH>    # Git Bash / Unix-style
```

Optional install (puts dev-clean on your PATH):

```bash
cargo install --path .
dev-clean scan <PATH>
```

## Commands

#### scan

List junk folders under PATH (read-only).

```bash
dev-clean scan .
dev-clean scan /d/Personal/projects/opensource
```

#### clean

Delete the same junk folders scan would find.

Preview first (recommended):

```bash
dev-clean clean <PATH> --dry-run
```

Delete for real:
```bash
dev-clean clean <PATH>
```

#### Paths in Git Bash

Prefer forward-slash or Git Bash paths so \ is not eaten by the shell:

```bash
dev-clean scan /d/Personal/projects/my-app
dev-clean scan D:/Personal/projects/my-app
dev-clean scan 'D:\Personal\projects\my-app'   # quoted backslashes OK
```

#### Help

dev-clean --help
dev-clean scan --help
dev-clean clean --help

### Notes

• Only folders whose names match the built-in junk list are touched.
• Parent project folders are kept; only matching junk directories are removed.
• Always run scan or clean --dry-run before a live clean on large trees.