---
title: "C Language Standard"
unit_id: "library/standards/c"
standard_type: "standard"
version: "XLIB-1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# STANDARDS C

## Purpose
Defines baseline standards for this language/runtime within Xonaix Core.

## Security
- Parameterize all external inputs.
- Prohibit eval/unsafe dynamic execution.
- Require dependency pinning and SBOM.

## Testing
- Unit tests required for critical paths.
- Deterministic builds required.

## Enforcement
CI MUST enforce linting, formatting, and security scanning for this language.

## Proof Artifacts
- Lint report
- Test report
- SBOM pointer
