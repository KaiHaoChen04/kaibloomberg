# Kaibloomberg Claude Context

## Project Purpose
Kaibloomberg is a Rust terminal UI (TUI) app for tracking market headers, portfolio holdings, and options chains.

## Tech Stack
- Language: Rust (edition 2024)
- Toolchain: nightly (`rust-toolchain.toml`)
- TUI: `ratatui`, `crossterm`, `tui-piechart`
- Async/runtime: `tokio`
- Data/network: `reqwest` + Yahoo Finance endpoints
- Time/date: `chrono`, `chrono-tz`
- Serialization/errors: `serde`, `serde_json`, `thiserror`

## Repository Layout
- `src/main.rs`: entry point
- `src/app.rs`: app state, event handling, refresh loops
- `src/ui/`: rendering/layout code
- `src/app_data/`: market/options fetchers and data types
- `src/utils/`: shared errors and utility helpers

## Coding Standards
- Keep module boundaries clear: fetch/transform in `app_data`, rendering in `ui`, orchestration in `app`.
- Prefer small pure helper functions for parsing/transforming remote data.
- Avoid panics in runtime paths; propagate errors via `Result` and `thiserror` types.
- Preserve keyboard control behavior documented in `README.md`.
- Add focused tests near parsing and transformation logic when behavior changes.

## Rust Formatting And Linting
- Format with nightly rustfmt:
  - `cargo +nightly fmt --all`
- Lint with clippy:
  - `cargo clippy --all-targets --all-features -- -D warnings`

## Common Commands
- Run app:
  - `cargo run`
- Build release:
  - `cargo build --release`
- Run tests:
  - `cargo test --all`
- Quick check:
  - `cargo check --all-targets`

## Working Agreements For Claude
- Before major refactors, inspect module-level impacts (`app`, `ui`, `app_data`).
- Keep changes minimal and avoid unrelated formatting churn.
- If touching keybind logic, verify no regression in Main/Portfolio/Options navigation.
- Prefer updating docs when controls, behavior, or commands change.

## Environment Notes
- Network calls depend on Yahoo Finance availability/rate limits.
- Set `RUST_BACKTRACE=1` for debugging failures.
