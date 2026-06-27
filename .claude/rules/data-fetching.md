---
applyTo: "src/app_data/**/*.rs"
description: "Rules for Yahoo Finance fetch/parsing code"
---

- Separate HTTP request logic from parsing/transformation logic.
- Validate optional fields defensively; remote data may be sparse.
- Include endpoint and symbol in propagated errors.
- Keep network retries conservative to avoid rate-limit amplification.
