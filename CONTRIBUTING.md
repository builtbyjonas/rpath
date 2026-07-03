# Contributing

Thanks for helping improve `rpath`.

## Development Setup

Install stable Rust, then run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Project Shape

- `rpath-core` owns environment discovery, PATH planning, diagnostics, and state.
- `rpath-shell` owns shell command emission and shell profile wrappers.
- `rpath-integrations` owns editor, Explorer, WSL, Git Bash, and watch-service glue.
- `rpath` is the CLI crate.

Keep platform-specific behavior isolated and add tests for shell output or PATH
rules whenever behavior changes.

## Pull Requests

- Keep changes focused.
- Add or update tests for behavior changes.
- Update README or command docs when the CLI changes.
- Do not add runtime network calls.
- Preserve the safety rule that refresh commands only mutate the current shell
  session through explicit emitted shell code.

## Commit Messages

Use short, imperative summaries, for example:

```text
Add fish wrapper installer
Fix Windows registry PATH parsing
Document snapshot restore flow
```
