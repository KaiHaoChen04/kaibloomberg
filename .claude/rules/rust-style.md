---
applyTo: "src/**/*.rs"
description: "Rust coding conventions for Kaibloomberg source files"
---

- Keep functions focused and avoid deeply nested control flow.
- Prefer explicit types in public interfaces.
- Return `Result` for fallible operations; avoid `unwrap()` in app runtime paths.
- Keep error strings actionable (include symbol, endpoint, or operation context).
- Preserve existing keyboard control behavior unless the task explicitly changes UX.
