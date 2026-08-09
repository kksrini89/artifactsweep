# 🧹ArtifactSweep

**Artifact Sweep** is a cross-platform Rust-based disk cleanup utility that safely removes generated development artifacts to reclaim storage space.

Use it in two ways:

- **Desktop app** — pick a folder, scan, select what to delete, clean  
- **CLI (`sweep`)** — same idea from the terminal  

---

## Install

Download the build for your system from  
**[Releases](https://github.com/kksrini89/artifactsweep/releases)**.

| Your system | What to download |
|-------------|------------------|
| Windows (64-bit) | `ArtifactSweep_*_x64-setup.exe` |
| Linux (Debian/Ubuntu 64-bit) | `ArtifactSweep_*_amd64.deb` |
| Mac (Apple Silicon) | `ArtifactSweep_*_aarch64.dmg` |

Optional: standalone `sweep-*` files if you only want the command line.

### After install

1. Open a **new** terminal (important on Windows so PATH updates).  
2. Check the CLI:

```bash
sweep --help
```

3. Open **ArtifactSweep** from the Start Menu (Windows), app menu (Linux), or Applications (Mac).

**Mac tip:** if macOS blocks the app, right-click → **Open**, or allow it in System Settings → Privacy & Security.

**Windows tip:** if both an old and new `sweep` exist, prefer the one under `AppData\Local\ArtifactSweep`, or remove an old Cargo install.

---

## Desktop app (quick start)

1. **Choose folder** — project or drive area to scan  
2. **Scan** — lists junk folders and sizes  
3. Check the rows you want to remove  
4. **Clean** — confirm, then delete only the selected folders  
5. The list refreshes when done  

Always review the list before cleaning. Some names (like `bin` or `build`) can appear in real projects.

---

## CLI (quick start)

```bash
# List junk under a path (safe — does not delete)
sweep scan .

# Preview deletes
sweep clean . --dry-run

# Delete junk under that path
sweep clean .
```

On Windows Git Bash, prefer paths like `/d/Projects/my-app` or `D:/Projects/my-app`.

---

## What gets detected?

Folders matched by **name**, for example:

- JS/TS: `node_modules`, `dist`, `.next`, `.angular`, …  
- Rust: `target`  
- Python: `__pycache__`  
- And other common caches (see the app/CLI after a scan)

The tool does not walk *inside* a matched folder for more junk names.

---

## Requirements

- **Desktop:** normal OS support (Windows 10/11, recent Ubuntu-style Linux, recent macOS on Apple Silicon).  
- **Windows desktop:** WebView2 (usually already installed).  
- **CLI-only:** just the `sweep` binary for your OS — no Node/Rust needed to *use* it.

---

## Screenshots

On one of my project folders alone, it reclaimed nearly 5 GB of storage.

![Tried clean with dry-run](screenshots/clean-with-dry-run.png "Clean with dry-run")

![Tried real clean](screenshots/real-clean.png "Real clean")

---

## License

MIT — see [LICENSE](LICENSE).
