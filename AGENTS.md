# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust binary crate named `body-recorder`.

- `Cargo.toml` defines package metadata and dependencies.
- `Cargo.lock` is committed for reproducible application builds.
- `src/main.rs` is the executable entry point.
- `target/` contains local build output and should not be edited or committed.

As the project grows, keep modules under `src/` with focused files such as `src/config.rs`, `src/client.rs`, or `src/commands.rs`. Put integration tests in `tests/` and fixtures in `tests/fixtures/`.

## Build, Test, and Development Commands

- `cargo check` verifies the crate quickly without producing an optimized binary.
- `cargo build` compiles the debug binary.
- `cargo run` builds and runs the local executable.
- `cargo test` runs unit and integration tests.
- `cargo fmt` formats Rust source using `rustfmt`.
- `cargo clippy --all-targets --all-features` runs Rust lints across binaries, tests, and feature combinations.

Run `cargo check`, `cargo fmt`, and `cargo test` before opening a pull request. Use Clippy when changing behavior or module boundaries.

## Coding Style & Naming Conventions

Use standard Rust 2024 style with `rustfmt` defaults: 4-space indentation, `snake_case` for functions and modules, `PascalCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants.

Use `clap` derive types for command-line parsing, `serde` derives for JSON-compatible data structures, and `tokio` async APIs for network or file work that may block. Keep user-facing CLI messages concise and actionable.

## Testing Guidelines

Place unit tests next to the code they cover inside `#[cfg(test)] mod tests`. Put end-to-end or command-level scenarios in `tests/*.rs`. Name tests after the behavior being verified, for example `parses_config_path_argument` or `returns_error_for_invalid_json`.

Tests should avoid real network calls. Prefer dependency injection, local fixtures, or mocked HTTP behavior for code using `reqwest`.

## Commit & Pull Request Guidelines

This repository has no existing commit history, so use a clear conventional style going forward:

- `feat: add recorder command`
- `fix: handle missing config file`
- `test: cover invalid response parsing`

Pull requests should include a short description, the reason for the change, and the commands run for verification. Link related issues when available. Include screenshots or terminal output only when the CLI behavior or user-visible output changes.

## Security & Configuration Tips

Do not commit secrets, tokens, machine-specific paths, or generated build artifacts. Keep configuration examples sanitized, and prefer environment variables or user-local config directories for credentials.
