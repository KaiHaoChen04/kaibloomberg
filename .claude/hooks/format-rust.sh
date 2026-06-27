#!/usr/bin/env bash
set -euo pipefail

if command -v cargo >/dev/null 2>&1; then
  # Formatting should never block edits if rustfmt is unavailable.
  cargo +nightly fmt --all >/dev/null 2>&1 || true
fi

exit 0
