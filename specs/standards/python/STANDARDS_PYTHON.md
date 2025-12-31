---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/python/STANDARDS_PYTHON.md"
unit_id: "library/standards/python"
title: "Python Language Standard"
document_type: "standard"
language: "en"

# --- Classification ---
trust_class: "L3"
classification: "internal"
compliance: []

# --- Ownership ---
owner: "Founder"
approved_by: "Founder"

# --- Authority ---
authority:
  repo: "xonaix-specs"
  ref: "THE_XONAIX_WAY.md"
  version: null

# --- Relationships ---
depends_on: []
supersedes: null
superseded_by: null

# --- Lifecycle ---
version: "XLIB-1.0.0"
status: "active"
created: "2025-12-31T00:00:00Z"
last_updated: "2025-12-31T20:00:00Z"
---

# Python Language Standard

Python is used for tooling, scripting, and AI/ML integration in Xonaix systems.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L3 |
| Classification | Orchestration |

### What This Trust Class May Do

- Scripting and automation
- AI/ML model integration
- Data processing pipelines

### What This Trust Class May NOT Do

- Assert authority without verification
- Perform cryptographic operations (delegate to XCLib)

---

## Security

- Parameterize all external inputs
- Prohibit eval/unsafe dynamic execution
- Require dependency pinning and SBOM

---

## Testing

- Unit tests required for critical paths
- Deterministic builds required

---

## Enforcement

CI MUST enforce linting, formatting, and security scanning for this language.

---

## Proof Artifacts

- Lint report
- Test report
- SBOM pointer

---

*Xonaix Library Standard*
*Canonical: \`xonaix-library::specs/standards/python/STANDARDS_PYTHON.md\`*
*Authority: \`xonaix-specs::THE_XONAIX_WAY.md\`*
