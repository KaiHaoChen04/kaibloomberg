---
name: rust-quality-gate
description: "Run format, lint, and tests for this Rust repo and summarize failures with actionable fixes"
---

# Rust Quality Gate

## Use When
- User asks to validate code quality before commit.
- User asks why CI failed for formatting, clippy, or tests.

## Workflow
1. Run: `cargo +nightly fmt --all -- --check`
2. Run: `cargo clippy --all-targets --all-features -- -D warnings`
3. Run: `cargo test --all`
4. Summarize:
   - First failing step
   - Root cause
   - Targeted fix options

## Guardrails
- Do not auto-fix broadly without user request.
- Keep edits minimal and scoped to failing checks.
