---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/json/STANDARDS_JSON.md"
unit_id: "library/standards/json"
title: "JSON Data Format Standard"
document_type: "standard"
language: "en"

# --- Classification ---
trust_class: "L4"
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

# JSON Data Format Standard

JSON is used for data interchange and configuration in Xonaix systems.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L4 |
| Classification | Interface |

### What This Trust Class May Do

- Data serialization/deserialization
- Configuration storage
- API response formats

### What This Trust Class May NOT Do

- Assert authority without verification
- Store secrets in plaintext

---

## Security

- Validate all JSON input against schemas
- Prohibit dynamic JSON construction from untrusted input
- Require schema validation

---

## Testing

- Schema validation tests required
- Deterministic serialization required

---

## Enforcement

CI MUST enforce JSON schema validation and linting.

---

## Proof Artifacts

- Schema validation report
- Lint report

---

*Xonaix Library Standard*
*Canonical: \`xonaix-library::specs/standards/json/STANDARDS_JSON.md\`*
*Authority: \`xonaix-specs::THE_XONAIX_WAY.md\`*
