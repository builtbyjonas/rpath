# Changelog

All notable changes to this project will be documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project aims to follow Semantic Versioning after the first stable release.

## [Unreleased]

## [0.1.0] - 2026-07-04

### Added

- Initial `rpath` Rust workspace with `rpath-core`, `rpath-shell`, `rpath-integrations`, and the `rpath` CLI.
- Cross-platform environment planning for Windows, macOS, and Linux.
- PATH collection from current process state, shell/profile files, common package-manager locations, Windows registry sources, WSL/Git Bash paths, and critical system directories.
- Safe PATH computation with duplicate removal, invalid-entry diagnostics, strict mode, repair planning, and critical-path preservation.
- Default refresh command that builds an environment plan and, when loaded through a wrapper, applies emitted shell commands to the current session.
- Shell emitters for PowerShell, PowerShell Core, cmd, bash, zsh, and fish.
- `doctor`, `diff`, `print`, `repair`, `init`, `install`, and `uninstall` commands.
- Manual snapshots with save, list, restore, and delete operations.
- Automatic environment versions with list, diff, and restore operations.
- Watch mode for tracking PATH changes, including user-scope service/task helpers.
- Local integrations for VS Code, Windows Explorer, WSL, and Git Bash.
- JSON output for automation and diagnostics across supported commands.
- `--dry-run`, `--shell`, `--json`, `--emit`, `--strict`, and `--no-dedupe` global options.
- `rpath upgrade` with latest-release checks, platform artifact selection, checksum verification, archive extraction, dry-run/check-only modes, and Windows delayed replacement.
- Hosted installer and uninstaller scripts for `get.rpath.dev`.
- User-local installer defaults for Unix/macOS (`~/.local/bin`) and Windows (`%LOCALAPPDATA%\Programs\rpath\bin`).
- Installer checksum verification against GitHub Release `.sha256` assets.
- Interactive shell-wrapper prompts in installers, with noninteractive and explicit wrapper modes.
- Full uninstall scripts that remove hosted-install binaries, installer PATH entries, and install metadata while keeping local state unless purged.
- VitePress documentation site with English and German guides, command reference, JSON reference, security page, installation docs, uninstall docs, and troubleshooting docs.
- `get.rpath.dev` Express/Vercel server for serving install and uninstall scripts.
- GitHub Actions CI for formatting, clippy, tests, JSON smoke checks, and shell wrapper dry-run smoke checks across Windows, macOS, and Linux.
- GitHub Actions release workflow for Linux x64/arm64, macOS x64/arm64, and Windows x64/arm64 archives with SHA-256 checksums.
- Open-source project files including license, contributing guide, code of conduct, security policy, support docs, Dependabot configuration, issue templates, and pull request template.

### Changed

- Documented that normal refresh stays offline while install, uninstall, and upgrade flows are explicit networked or persistent operations.
- Marked the v0.1 roadmap items as complete and moved remaining distribution hardening to future work.

### Security

- Refresh commands do not mutate system PATH or machine-wide environment settings.
- Shell wrapper installation creates profile backups before replacing rpath-managed blocks.
- Emit mode refuses to print mutation commands when the environment plan contains hard errors.
- Hosted installers verify release checksums before installing binaries.
- Persistent file/profile/registry/service changes are limited to explicit commands and user scope where possible.
