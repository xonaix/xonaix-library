---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "xonaix-library"
path: "specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md"
unit_id: "governance/library/header-contract"
title: "Library Standard Header Contract"
document_type: "contract"
language: "en"

# --- Version ---
version: "XGOV-2.1.0"
baseline: null
status: "approved"

# --- Classification ---
trust_class: null
classification: "internal"
compliance: []

# --- Ownership ---
owner: "Founder"
approved_by: "Founder"
authority_tier: "T1"

# --- Authority ---
authority:
  repo: "xonaix-specs"
  ref: "XONAIX_SELF_GOVERNANCE_CONTRACT.md"
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
last_updated: "2025-12-31T23:30:00Z"
---

# LIBRARY STANDARD HEADER CONTRACT
**Scope:** All Xonaix repositories adopting the Universal Document Header
**Schema Version:** 2.1

---

## 1. Purpose

This contract defines the mandatory header format for all documents in the Xonaix ecosystem.

The header is not decorative. It is a semantic control surface for:

- Machine parsing and validation
- Human comprehension
- Agent reasoning
- Long-term governance stability
- Enterprise compliance tracking
- Cryptographic integrity verification
- Constitutional conformance auditing

No document is considered valid without a compliant header.

---

## 2. Applicability

This contract applies to:

- All Library standards
- All Library mini-standards
- All Library templates
- All governance contracts
- All future documents added to any Xonaix repository

---

## 3. Header Location

The header MUST appear at the top of the file, before any prose.

No content, comments, or whitespace may precede the header.

---

## 4. Header Format (MANDATORY)

The header MUST use YAML frontmatter delimited by triple dashes.

### 4.1 Canonical Format (Schema v2.1)

```yaml
---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "{REPO_NAME}"
path: "{FILE_PATH}"
unit_id: "{domain}/{category}/{name}"
title: "{Document Title}"
document_type: "{standard | mini-standard | template | contract}"
language: "en"

# --- Version ---
version: "{PREFIX}-{MAJOR}.{MINOR}.{PATCH}"
baseline: "{XBASE-X.Y.Z | null}"
status: "{draft | internal_review | proposed | approved | sealed | deprecated | superseded}"

# --- Classification ---
trust_class: "{L0 | L1 | L2 | L3 | L4 | null}"
classification: "{public | internal | confidential | restricted}"
compliance: []

# --- Ownership ---
owner: "{Owner Identifier}"
approved_by: "{Approver Identifier}"
authority_tier: "{T0 | T1 | T2 | T3}"

# --- Authority ---
authority:
  repo: "{AUTHORITY_REPO}"
  ref: "{AUTHORITY_FILE}"
  version: "{VERSION | null}"

# --- Relationships ---
depends_on: []
supersedes: null
superseded_by: null
implements: []

# --- Integrity ---
integrity:
  hash_alg: "{SHA3-512 | null}"
  content_hash: "{HASH | null}"
  signature: "{SIGNATURE | null}"
  signed_by: "{SIGNER_ID | null}"
  signed_at: "{ISO8601_UTC | null}"

# --- Constitutional Conformance ---
constitutional_conformance:
  constitution_version: "{VERSION | null}"
  constitution_hash: "{HASH | null}"
  zero_point_version: "{VERSION | null}"
  zero_point_hash: "{HASH | null}"
  deviations: []
  last_verified: "{ISO8601_UTC | null}"
  verified_by: "{VERIFIER_ID | null}"

# --- Lifecycle ---
created: "{ISO8601_UTC}"
last_updated: "{ISO8601_UTC}"
---
```

### 4.2 Section Comments

The section comments (`# --- Identity ---`, etc.) are REQUIRED for human readability but are NOT parsed semantically. They MUST appear exactly as shown.

---

## 5. Version Prefix System

All documents use domain-specific version prefixes to enable independent evolution.

### 5.1 Domain Prefixes

| Prefix | Domain | Description |
|--------|--------|-------------|
| `XZERO` | Zero Point | Constitutional foundation, immutable core principles |
| `XCORT` | Cortex | Core system specifications |
| `XCODE` | Code-Core | Code standards and implementations |
| `XNEX` | Nexus | Integration and networking specifications |
| `XBLADE` | Blade | UI/UX specifications |
| `XINFRA` | Infrastructure | Infrastructure and deployment |
| `XLIB` | Library | Language and protocol standards |
| `XGOV` | Governance | Governance contracts and policies |
| `XUX` | User Experience | User experience specifications |

### 5.2 Baseline Versions

Baselines aggregate multiple document versions into coherent releases:

- Format: `XBASE-MAJOR.MINOR.PATCH`
- Purpose: Cross-domain coordination
- Example: `XBASE-1.0.0` includes specific versions of XLIB, XGOV, etc.

### 5.3 Customer Releases

Customer-facing releases aggregate baselines:

- Format: `XREL-MAJOR.MINOR.PATCH`
- Purpose: External versioning
- Maps to internal baseline versions

---

## 6. Field Semantics (Binding)

### 6.1 Schema Identification

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema` | string | YES | MUST be `"xonaix-document-header"` |
| `schema_version` | string | YES | Schema version, currently `"2.1"` |

### 6.2 Identity Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repo` | string | YES | Repository name (e.g., `"xonaix-library"`) |
| `path` | string | YES | Full path from repo root |
| `unit_id` | string | YES | Unique identifier (domain/category/name) |
| `title` | string | YES | Human-readable title |
| `document_type` | enum | YES | One of: `standard`, `mini-standard`, `template`, `contract` |
| `language` | string | YES | ISO 639-1 code (e.g., `"en"`) |

### 6.3 Version Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | string | YES | Format: `{PREFIX}-MAJOR.MINOR.PATCH` |
| `baseline` | string/null | YES | Baseline version this document belongs to |
| `status` | enum | YES | One of: `draft`, `internal_review`, `proposed`, `approved`, `sealed`, `deprecated`, `superseded` |

### 6.4 Classification Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trust_class` | string/null | YES | Trust classification: `L0`, `L1`, `L2`, `L3`, `L4`, or `null` |
| `classification` | enum | YES | One of: `public`, `internal`, `confidential`, `restricted` |
| `compliance` | array | YES | Regulatory tags (e.g., `["SOC2", "FIPS"]`), may be empty |

### 6.5 Ownership Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `owner` | string | YES | Responsible party (e.g., `"Founder"`) |
| `approved_by` | string | YES | Who approved this version |
| `authority_tier` | enum | YES | One of: `T0`, `T1`, `T2`, `T3` |

### 6.6 Authority Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authority.repo` | string | YES | Repository containing authority document |
| `authority.ref` | string | YES | Authority document filename |
| `authority.version` | string/null | YES | Pinned version of authority document |

### 6.7 Relationships Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `depends_on` | array | YES | Dependencies as `[{repo: "x", ref: "y.md"}]`, may be empty |
| `supersedes` | string/null | YES | Document this replaces |
| `superseded_by` | string/null | YES | Document that replaces this |
| `implements` | array | YES | List of specifications this document implements |

### 6.8 Integrity Section (Cryptographic)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `integrity.hash_alg` | string/null | YES | Hash algorithm (e.g., `"SHA3-512"`) |
| `integrity.content_hash` | string/null | YES | Hash of document content (excluding header) |
| `integrity.signature` | string/null | YES | Cryptographic signature |
| `integrity.signed_by` | string/null | YES | Signer identifier |
| `integrity.signed_at` | string/null | YES | ISO 8601 UTC timestamp of signing |

### 6.9 Constitutional Conformance Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `constitutional_conformance.constitution_version` | string/null | YES | Version of constitution this conforms to |
| `constitutional_conformance.constitution_hash` | string/null | YES | Hash of constitution version |
| `constitutional_conformance.zero_point_version` | string/null | YES | Version of Zero Point |
| `constitutional_conformance.zero_point_hash` | string/null | YES | Hash of Zero Point version |
| `constitutional_conformance.deviations` | array | YES | Documented deviations from constitution |
| `constitutional_conformance.last_verified` | string/null | YES | When conformance was last verified |
| `constitutional_conformance.verified_by` | string/null | YES | Who verified conformance |

### 6.10 Lifecycle Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `created` | string | YES | ISO 8601 UTC timestamp |
| `last_updated` | string | YES | ISO 8601 UTC timestamp |

---

## 7. Trust Classes

| Class | Name | Description | May Do | May NOT Do |
|-------|------|-------------|--------|------------|
| `L0` | Constitutional | Foundational truth, bedrock principles | Define truth, establish authority | Be modified without ceremony |
| `L1` | Authority | Can assert truth, sign, verify | Sign, verify, canonicalize, hash | --- |
| `L2` | Deterministic | Trusted computation | Pure computation, transformation | Sign/verify without L1 delegation |
| `L3` | Orchestration | Coordinates workflows | Coordinate, transport, query | Assert truth, infer authority |
| `L4` | Interface | UI/configuration | Display, tooling, scripting | Any governance-affecting operation |
| `null` | Not Applicable | Templates, meta-documents | --- | --- |

---

## 8. Authority Tiers

| Tier | Name | Description | Examples |
|------|------|-------------|----------|
| `T0` | Constitutional | Foundational, rarely changes | Zero Point, Constitution |
| `T1` | Governance | Core policies and contracts | This contract, Sealing Contract |
| `T2` | Standards | Language and protocol standards | Rust Standard, TypeScript Standard |
| `T3` | Specifications | Product and feature specifications | B5, UX specs |

---

## 9. Status Lifecycle

Documents progress through a defined lifecycle:

| Status | Description | Integrity Requirement |
|--------|-------------|----------------------|
| `draft` | Work in progress, not for reference | None |
| `internal_review` | Ready for internal team review | None |
| `proposed` | Submitted for formal approval | None |
| `approved` | Content locked, awaiting seal | Content hash computed |
| `sealed` | Cryptographically signed and immutable | Signature required |
| `deprecated` | Still valid but being phased out | Maintains seal |
| `superseded` | Replaced by another document | Maintains seal |

### 9.1 Status Transitions

```
draft -> internal_review -> proposed -> approved -> sealed -> deprecated -> superseded
```

- **draft -> internal_review**: Author submits for team review
- **internal_review -> proposed**: Team approves for formal review
- **proposed -> approved**: Formal approval granted, content locked
- **approved -> sealed**: Cryptographic signature applied
- **sealed -> deprecated**: Document marked for phase-out
- **deprecated -> superseded**: Replacement document is sealed

### 9.2 Integrity by Status

| Status | hash_alg | content_hash | signature | CI Behavior |
|--------|----------|--------------|-----------|-------------|
| `draft` | null | null | null | Pass |
| `internal_review` | null | null | null | Pass |
| `proposed` | null | null | null | Pass |
| `approved` | SHA3-512 | REQUIRED | null | Warn if missing hash |
| `sealed` | SHA3-512 | REQUIRED | REQUIRED | Fail if missing sig |
| `deprecated` | SHA3-512 | REQUIRED | REQUIRED | Verify integrity |
| `superseded` | SHA3-512 | REQUIRED | REQUIRED | Verify integrity |

### 9.3 Governance Debt

Documents with `status: approved` but missing `content_hash` represent governance debt. CI will warn but not fail, allowing development to proceed while tracking unsigned documents.

---

## 10. Classification Levels

| Level | Description |
|-------|-------------|
| `public` | Safe for external distribution |
| `internal` | Xonaix internal use only |
| `confidential` | Limited distribution, named recipients |
| `restricted` | Highest sensitivity, explicit approval required |

---

## 11. Forbidden Header Practices

The following are explicitly forbidden:

- Missing header fields
- Soft or advisory language in header values
- Ellipses (...)
- Placeholder values (e.g., `{TODO}`, `TBD`)
- Freeform metadata blocks
- Markdown headers in place of frontmatter
- Non-UTC timestamps
- Relative paths
- Guessed or inferred authority

---

## 12. Consistency Requirements

Header values MUST match:

- UNIT.json (when present)
- Unit manifest metadata
- BOM references (when applicable)
- Authority chain references

Inconsistencies are hard failures.

---

## 13. Enforcement

Compliance with this contract is enforced by:

- normalize-frontmatter tool
- CI --check mode
- Unit validation
- Doctor checks
- Schema validation
- Constitutional conformance verification
- header-validate command

Any deviation constitutes governance debt and MUST block sealing and release.

---

## 14. Migration from v1.0

Documents using schema v1.0 MUST migrate to v2.0:

1. Update `schema_version` from `"1.0"` to `"2.0"`
2. Move `version` and `status` to Version section
3. Add `baseline` field (may be `null`)
4. Add `authority_tier` to Ownership section
5. Add `implements` to Relationships section
6. Add complete `integrity` section
7. Add complete `constitutional_conformance` section
8. Update version prefix as appropriate (XLIB, XGOV, etc.)

---

## 15. Migration from v2.0

Documents using schema v2.0 MUST migrate to v2.1:

1. Update `schema_version` from `"2.0"` to `"2.1"`
2. Update `status` values:
   - `active` -> `approved` (content locked, awaiting signature)
   - Or `active` -> `sealed` (if already signed)
3. Compute `content_hash` for documents with status `approved` or higher
4. Apply signature for documents with status `sealed`

The v2.1 schema introduces a refined status lifecycle that separates content approval from cryptographic sealing.

---

## 16. Evolution

Changes to this contract require:

- Explicit Founder approval
- Schema version increment
- Regeneration of governance manifests
- Full CI verification pass
- Migration of existing documents

Backward compatibility is not guaranteed.

---

## 17. Final Assertion

A document without a compliant header is not a valid Xonaix document.

The header is the identity anchor for machine and human reasoning.

Machine-first, human-readable, cryptographically verifiable. This is the Xonaix way.

---

*Governance Contract*
*Canonical: `xonaix-library::specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md`*
*Authority: `xonaix-specs::XONAIX_SELF_GOVERNANCE_CONTRACT.md`*
