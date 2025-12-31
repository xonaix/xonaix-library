---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "xonaix-library"
path: "specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md"
unit_id: "library/meta/language-template"
title: "Language Standard Template"
document_type: "template"
language: "en"

# --- Version ---
version: "XLIB-2.0.0"
baseline: null
status: "approved"

# --- Classification ---
trust_class: null
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
last_updated: "2025-12-31T22:26:48Z"
---

# Language Standard Template

This is the canonical template for creating new language standards in the Xonaix Library.

---

## Header Schema Reference (v2.0)

All Xonaix documents MUST use the v2.0 header schema. See LIBRARY_STANDARD_HEADER_CONTRACT.md for the complete specification.

### Version Prefixes

| Prefix | Domain | Use For |
|--------|--------|---------|
| XLIB | Library | Language and protocol standards |
| XGOV | Governance | Contracts and policies |
| XZERO | Zero Point | Constitutional documents |
| XCORT | Cortex | Core specifications |
| XCODE | Code-Core | Code standards |
| XNEX | Nexus | Integration specs |
| XBLADE | Blade | UI/UX specs |
| XINFRA | Infrastructure | Infrastructure specs |
| XUX | User Experience | UX specs |

### Status Lifecycle

| Status | Description |
|--------|-------------|
| draft | Work in progress, not for reference |
| proposed | Ready for review and approval |
| active | Approved and authoritative |
| deprecated | Still valid but being phased out |
| superseded | Replaced by another document |

### Authority Tiers

| Tier | Description | Examples |
|------|-------------|----------|
| T0 | Constitutional | Zero Point, Constitution |
| T1 | Governance | Contracts, policies |
| T2 | Standards | Language standards |
| T3 | Specifications | Product specs |

---

## Footer Format

All standards MUST end with:

---

*Xonaix Library Standard*
*Canonical: xonaix-library::specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md*
*Authority: xonaix-specs::THE_XONAIX_WAY.md*
