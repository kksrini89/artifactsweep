# 🧹artifactsweep

- **Artifact Sweep** is a cross-platform Rust-based disk cleanup utility that safely removes generated development artifacts to reclaim storage space.

- Generally, the artifcats are (`node_modules`, `target`, `dist`, and more) under a path you choose.

## Install

### Option A - Prebuilt binary (no Rust required)

1. Open [Releases](https://github.com/kksrini89/dev-clean/releases)
2. Download the asset for your OS:

| OS | Asset (example) |
|----|------------------|
| Windows x64 | `dev-clean-windows-x86_64.exe` |
| Linux x64 | `dev-clean-linux-x86_64` |
| macOS | `dev-clean-macos` |

3. (Linux/macOS) Make it executable and move it onto your `PATH` if you like:

   ```bash
   chmod +x dev-clean-linux-x86_64   # or dev-clean-macos
   sudo mv dev-clean-linux-x86_64 /usr/local/bin/dev-clean

4. On Windows, rename to dev-clean.exe if you want, then place it in a folder that is on your PATH, or run it by full path.

5. Check:

```dev-clean --help```


### Pre-requisite for Option B & C

- Rust / Cargo ([rustup](https://rustup.rs))

### Option B — From source (Rust / Cargo)

```bash
git clone https://github.com/kksrini89/dev-clean.git
cd dev-clean
cargo install --path .
dev-clean --help
```

### Option C — Build only

```bash
cargo build --release
# Windows: target/release/dev-clean.exe
# Linux/macOS: target/release/dev-clean
```

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
dev-clean scan /d/projects/opensource
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

```bash
dev-clean --help
dev-clean scan --help
dev-clean clean --help
```

#### What counts as junk?

Folders are matched by directory name only (anywhere under `PATH`). When a match is found, the tool does not walk inside it for further junk names.

Defined in [`src/junk.rs`](see `src/junk.rs`):

### JavaScript / TypeScript

- `node_modules`
- `dist`
- `build`
- `.angular`
- `.next`
- `.nuxt`
- `.turbo`
- `.cache`
- `.parcel-cache`
- `.svelte-kit`
- `.vite`

### Rust

- `target`

### Python

- `__pycache__`

### Java / Kotlin

- `.gradle`

### C# / .NET

- `bin`
- `obj`

### Flutter / Dart

- `.dart_tool`

### Generic

- `coverage`
- `out`
- `tmp`
- `temp`

> ***Warning***: Names like bin, obj, tmp, temp, out, and build appear in many stacks. 
> Always run scan or clean --dry-run before a live clean on a large or unfamiliar tree. Parent project folders are kept; only matching directories are removed.


### Notes

- Sizing uses parallel workers with `rayon` crate + progress bar with `indicatif` crate
- Prebuilt macOS builds come from GitHub’s macos-latest runner (often Apple Silicon). If a binary does not run on your Mac, build from source with Option B.

## License

MIT — see [LICENSE](LICENSE).