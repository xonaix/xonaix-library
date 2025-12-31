---
schema: "xonaix-document-header"
schema_version: "2.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/cpp/STANDARDS_CPP.md"
unit_id: "library/standards/cpp"
title: "C++ Language Standard"
document_type: "standard"
language: "en"

# --- Version ---
version: "XLIB-1.0.0"
baseline: null
status: "active"

# --- Classification ---
trust_class: "L1/L2"
classification: "internal"
compliance: []

# --- Ownership ---
owner: "Founder"
approved_by: "Founder"
authority_tier: "T2"

# --- Authority ---
authority:
  repo: "xonaix-specs"
  ref: "THE_XONAIX_WAY.md"
  version: null

# --- Relationships ---
depends_on: []
supersedes: null
superseded_by: null
implements: []

# --- Integrity ---
integrity:
  hash_alg: null
  content_hash: null
  signature: null
  signed_by: null
  signed_at: null

# --- Constitutional Conformance ---
constitutional_conformance:
  constitution_version: null
  constitution_hash: null
  zero_point_version: null
  zero_point_hash: null
  deviations: []
  last_verified: null
  verified_by: null

# --- Lifecycle ---
created: "2025-12-31T00:00:00Z"
last_updated: "2025-12-31T22:00:00Z"
---

# C++ Language Standard

C++ is a systems programming language used for performance-critical components in Xonaix.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L1/L2 |
| Classification | Constitutional/Deterministic |

### What This Trust Class May Do

- Low-level memory operations with RAII patterns
- System interface bindings
- Performance-critical paths

### What This Trust Class May NOT Do

- Trust external input without validation
- Use raw pointers without explicit justification

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
*Canonical: \`xonaix-library::specs/standards/cpp/STANDARDS_CPP.md\`*
*Authority: \`xonaix-specs::THE_XONAIX_WAY.md\`*
