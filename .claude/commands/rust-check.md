Run and summarize Rust quality checks for this repo.

Steps:
1. `cargo +nightly fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all`

Output:
- Show failing command first.
- For each failure, report root cause and specific file locations.
- If all pass, report success with concise timings.
