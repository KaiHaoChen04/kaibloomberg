#!/usr/bin/env bash
set -euo pipefail

# Non-intrusive completion signal for local sessions.
printf "\a" || true

if command -v osascript >/dev/null 2>&1; then
  osascript -e 'display notification "Claude task finished" with title "Kaibloomberg"' >/dev/null 2>&1 || true
fi

exit 0
