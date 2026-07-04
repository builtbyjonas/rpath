<div align="center">
<img src="./assets/readme-header.png" width="75%" height="auto"></img>
</div>

# rpath

Refresh your shell environment instantly — without restarting your terminal.

rpath is a cross-platform environment manager that rebuilds and safely applies your PATH and environment variables to the current shell session, similar to sourcing `.bashrc` on Linux but primarily built for Windows, also works on macOS, and Linux.

> [!NOTE]
> Think: “source ~/.bashrc” — but for PATH changes everywhere.

---

## Why rpath exists

On Windows, PATH changes are fragmented and painful:

- You install a tool → PATH updates don’t apply
- You restart terminals constantly
- You log out just to refresh environment variables
- Each shell behaves differently

rpath fixes this by:
- rebuilding your environment from all system sources
- computing a safe, deduplicated PATH
- applying updates directly into the current shell session

No restarts. No hacks. No manual copying.

## Features

- One-command refresh: `rpath`
- Cross-platform: Windows, Linux, macOS
- Shell support: PowerShell, cmd, bash, zsh, fish
- PATH diagnostics (duplicates, invalid entries, risks)
- Diff, repair, snapshot & restore system state
- Environment version tracking
- Integrations: VS Code, Explorer, WSL, Git Bash
- Optional watcher/service mode
- Offline refresh path; hosted install and `rpath upgrade` use explicit GitHub Release downloads
- Safe execution model (no silent system mutation)

## Install

### Windows (PowerShell)

```powershell
irm https://get.rpath.dev/install.ps1 | iex
```

### Linux / macOS

```bash
curl -fsSL https://get.rpath.dev/install.sh | sh
```

The hosted installer verifies release checksums, installs a user-local binary, and asks whether to install shell integration for the detected shell.

## Quick Start

Install shell integration:

```bash
rpath install
```

Or explicitly:

```bash
rpath install --shell powershell
rpath install --shell cmd
rpath install --shell bash
```

Restart your terminal once, then:

```bash
rpath
```

From now on, PATH refresh works instantly.

## Uninstall

```powershell
irm https://get.rpath.dev/uninstall.ps1 | iex
```

```bash
curl -fsSL https://get.rpath.dev/uninstall.sh | sh
```

The uninstall scripts remove the hosted-install binary and PATH entry. Local snapshots and versions are kept unless you pass the purge option.

## How it works

rpath cannot directly modify a parent shell process (this is a system limitation).

Instead it:

1. Builds a safe environment plan
2. Emits shell-compatible commands
3. Executes them via a small installed wrapper

Example:

```bash
eval "$(rpath --emit --shell bash)"
```

PowerShell:

```powershell
Invoke-Expression (& rpath --emit --shell pwsh)
```

Fish:

```fish
rpath --emit --shell fish | source
```

## Commands

```text
rpath                         Rebuild environment plan
rpath --emit                  Emit shell update commands
rpath print                   Print computed PATH
rpath doctor                  Diagnose environment issues
rpath diff                    Show PATH differences
rpath repair                  Fix PATH issues safely

rpath snapshot save           Save current state
rpath snapshot list           List snapshots
rpath snapshot restore        Restore snapshot

rpath version list            Show tracked environment versions
rpath version diff            Compare versions

rpath upgrade                 Upgrade the installed binary
rpath install                 Install shell integration
rpath uninstall               Remove shell integration

rpath watch                   Track environment changes
rpath integrate               Connect external tools
```

## Safety model

rpath is designed to be non-destructive:

* `❌` Never modifies system PATH directly during refresh
* `❌` Never silently writes system-wide changes
* `❌` Never performs network calls during refresh
* `✅` Only emits shell-safe commands
* `✅` Requires explicit install, uninstall, or upgrade commands for persistent changes
* `✅` Creates backups before modifying profiles
* `✅` Refuses invalid or unsafe PATH plans

## Platform sources

rpath builds PATH from multiple trusted sources:

### Windows

* User + system registry PATH
* Current process environment
* Git for Windows / MSYS (if present)
* Critical system directories preserved automatically

### Linux / macOS

* Shell profiles (`.bashrc`, `.zshrc`, etc.)
* `/etc/environment`
* Package managers (Homebrew, Nix, Snap, Flatpak)
* User-local bin paths

## State system

rpath stores:

* snapshots (manual saves)
* version history (automatic tracking)
* integration markers
* watcher state

Stored locally in platform data directories.

## JSON mode

For scripting and automation:

```bash
rpath doctor --json
rpath snapshot list --json
rpath diff --json
```

## Development

```bash
cargo build --workspace
cargo run -- --help
```

### Quality checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

### Workspace

* `rpath-core` – environment model & PATH planner
* `rpath-shell` – shell emitters
* `rpath-integrations` – external integrations
* `rpath` – CLI binary

## Troubleshooting

If changes don’t apply:

```bash
rpath install
```

Then restart your terminal.

For debugging:

```bash
rpath doctor --verbose
rpath diff
rpath repair
```

## Contributing

See:

* [CONTRIBUTING.md](https://github.com/builtbyjonas/rpath/blob/main/CONTRIBUTING.md)
* [CODE_OF_CONDUCT.md](https://github.com/builtbyjonas/rpath/blob/main/CODE_OF_CONDUCT.md)
* [SECURITY.md](https://github.com/builtbyjonas/rpath/blob/main/SECURITY.md)
