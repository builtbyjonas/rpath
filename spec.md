# rpath — Environment Reload Tool

> Instantly refresh your shell environment across Windows, Linux, and macOS.

`rpath` is a single-command CLI tool that rebuilds and reloads the current shell environment from system + user sources, ensuring PATH and environment changes are applied immediately without restarting the terminal.

---

# 1. Overview

## Problem

On modern systems, environment variable changes (especially PATH) are:
- not reflected in already-open terminals
- inconsistent across shells
- fragmented between OS-specific mechanisms (registry, shell profiles, package managers)

## Solution

`rpath` provides a unified command:

```bash
rpath
````

Which:

* detects the current shell
* rebuilds PATH from system sources
* merges user and system environment variables
* removes duplicates and invalid entries
* applies changes to the current session

---

# 2. Core Principle

> One command should fix your environment instantly.

```bash
rpath
```

No flags required.

Everything else is optional tooling.

---

# 3. Supported Platforms

| OS      | Shells Supported                 |
| ------- | -------------------------------- |
| Windows | cmd, PowerShell, PowerShell Core |
| Linux   | bash, zsh, fish                  |
| macOS   | bash, zsh, fish                  |

---

# 4. Core Behavior

When executed, `rpath` performs:

## 4.1 Environment Source Collection

### Windows

* HKCU:\Environment (User PATH)
* HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment (Machine PATH)

### Linux/macOS

* `/etc/environment`
* `/etc/profile`
* `~/.profile`
* `~/.bashrc`
* `~/.zshrc`
* `~/.config/fish/config.fish`
* Homebrew paths (if detected)
* Nix profiles (if present)

---

## 4.2 PATH Construction Rules

* Merge system PATH first, then user PATH (default)
* Remove duplicates
* Normalize separators:

  * Windows: `;`
  * Unix: `:`
* Expand variables:

  * Windows: `%VAR%`
  * Unix: `$VAR`, `${VAR}`
* Validate entries:

  * optionally remove non-existent paths

---

## 4.3 Environment Update

`rpath` updates the CURRENT SHELL session:

### Windows CMD

```cmd
set PATH=...
```

### PowerShell

```powershell
$env:Path = "..."
```

### Bash/Zsh

```bash
export PATH="..."
```

### Fish

```fish
set -gx PATH ...
```

---

# 5. Execution Modes

## 5.1 Default Mode (Required)

```bash
rpath
```

* Fully reload environment
* Applies changes immediately
* Minimal output

Example:

```
✓ Environment refreshed
✓ PATH updated (42 → 51 entries)
✓ 3 duplicates removed
```

---

## 5.2 Shell Evaluation Mode (Internal)

Used when needed for compatibility:

```bash
eval "$(rpath --emit)"
```

Outputs shell-specific commands instead of applying them directly.

---

# 6. Shell Integration Strategy

Because a child process cannot directly modify a parent shell, `rpath` supports two integration modes:

## 6.1 Direct Injection Mode

Supported in:

* PowerShell (via session context)
* fish shell (via function injection)

## 6.2 Eval Mode

Used in:

* bash
* zsh
* cmd.exe

Example:

```bash
eval "$(rpath)"
```

or in CMD:

```cmd
for /f "delims=" %i in ('rpath --cmd') do @%i
```

---

# 7. CLI Interface

## 7.1 Primary Command

```bash
rpath
```

Reloads environment.

---

## 7.2 Optional Commands

### Diagnostics

```bash
rpath doctor
```

* detects broken PATH entries
* duplicates
* invalid directories
* security risks (optional flag)

---

### Diff Mode

```bash
rpath diff
```

Shows:

* added paths
* removed paths
* reordered entries

---

### Snapshot Mode

```bash
rpath snapshot save
rpath snapshot restore
```

* saves current environment state
* restores previous known-good state

---

### Print Mode

```bash
rpath print
```

Outputs computed PATH only.

---

### Emit Mode

```bash
rpath --emit
```

Returns shell-compatible commands without applying them.

---

# 8. Path Validation Rules

## 8.1 Invalid Entry Detection

An entry is considered invalid if:

* directory does not exist
* path is empty
* malformed variable expansion exists

## 8.2 Optional Cleanup

If enabled:

* remove invalid paths automatically
* log removed entries

---

# 9. Platform-Specific Behavior

## 9.1 Windows

* Reads registry environment variables
* Handles %SystemRoot% expansion
* Supports CMD + PowerShell dual output
* Respects UAC-separated environments (user vs machine)

---

## 9.2 Linux

* Re-sources shell configuration files
* Supports system-wide environment files
* Handles package manager paths (apt, snap, flatpak, nix)

---

## 9.3 macOS

* Includes Homebrew paths automatically:

  * `/opt/homebrew/bin`
  * `/usr/local/bin`
* Supports Apple Silicon vs Intel differences

---

# 10. Performance Requirements

* Execution time: < 50ms typical
* No network calls
* Fully offline
* No daemon required

---

# 11. Safety Rules

* Never overwrite PATH if parsing fails
* Always preserve last known valid environment
* Never remove system-critical paths without explicit flag
* Never mutate environment outside current shell session

---

# 12. Output Design Principles

* Minimal by default
* Human-readable
* No spam logs
* Optional verbose mode only

---

# 13. Optional Flags (Advanced Use)

These are NOT required for normal usage:

```bash
rpath --verbose
rpath --dry-run
rpath --no-dedupe
rpath --strict
rpath --json
```

---

# 14. Future Extensions

Planned capabilities:

## Environment Management

* env diff tracking over time
* automatic PATH repair
* environment versioning

## System Integration

* Windows Explorer integration
* VS Code terminal auto-refresh hook
* Git Bash / WSL synchronization

## Background Mode (optional future)

* detect registry changes
* notify user or auto-refresh shell

---

# 15. Design Goals

* Zero configuration required
* Works instantly after install
* Cross-shell consistent behavior
* Safe by default
* Extensible without breaking simplicity

---

# 16. Summary

`rpath` is a single-command environment refresh tool:

```bash
rpath
```

It ensures that any environment change on the system is immediately reflected in the current terminal session — across Windows, Linux, and macOS — without restarting the shell.