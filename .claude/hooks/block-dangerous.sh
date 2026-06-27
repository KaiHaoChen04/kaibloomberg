#!/usr/bin/env bash
set -euo pipefail

# Best-effort guard for obviously dangerous shell operations.
input="${1:-}"

if [[ "$input" == *"rm -rf /"* ]] || [[ "$input" == *":(){:|:&};:"* ]]; then
  echo "Blocked dangerous command pattern." >&2
  exit 2
fi

exit 0
