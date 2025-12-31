# LIBRARY STANDARD HEADER CONTRACT

**Status:** Binding Standard Contract
**Applies to:** All Xonaix Library standards, mini-standards, and templates
**Authority:** XONAIX_SELF_GOVERNANCE_CONTRACT.md
**Scope:** Library repository only
**Schema Version:** 1.0

---

## 1. Purpose

This contract defines the mandatory header format for all standards and mini-standards in the Xonaix Library.

The header is not decorative. It is a semantic control surface for:

- Machine parsing and validation
- Human comprehension
- Agent reasoning
- Long-term governance stability
- Enterprise compliance tracking

No standard is considered valid without a compliant header.

---

## 2. Applicability

This contract applies to:

- All Library standards
- All Library mini-standards
- All Library templates
- All future standards added to the Library

This contract does not apply to:

- Product specifications (B5, UX, Nexus, Web)
- Governance contracts (use simplified headers)
- Runtime documentation
- Reference-only external documents

---

## 3. Header Location

The header MUST appear at the top of the file, before any prose.

No content, comments, or whitespace may precede the header.

---

## 4. Header Format (MANDATORY)

The header MUST use YAML frontmatter delimited by triple dashes.

### 4.1 Canonical Format

```yaml
---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "{REPO_NAME}"
path: "{FILE_PATH}"
unit_id: "library/{category}/{name}"
title: "{Document Title}"
document_type: "{standard | mini-standard | template | contract}"
language: "en"

# --- Classification ---
trust_class: "{L1|L2|L3|L4|null}"
classification: "{public | internal | confidential | restricted}"
compliance: []

# --- Ownership ---
owner: "{Owner Identifier}"
approved_by: "{Approver Identifier}"

# --- Authority ---
authority:
  repo: "{AUTHORITY_REPO}"
  ref: "{AUTHORITY_FILE}"
  version: null

# --- Relationships ---
depends_on: []
supersedes: null
superseded_by: null

# --- Lifecycle ---
version: "XLIB-{MAJOR.MINOR.PATCH}"
status: "{draft | proposed | active | deprecated | superseded}"
created: "{ISO8601_UTC}"
last_updated: "{ISO8601_UTC}"
---
```

### 4.2 Section Comments

The section comments (`# --- Identity ---`, etc.) are REQUIRED for human readability but are NOT parsed semantically. They MUST appear exactly as shown.

---

## 5. Field Semantics (Binding)

### 5.1 Schema Identification

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema` | string | YES | MUST be `"xonaix-document-header"` |
| `schema_version` | string | YES | Schema version, currently `"1.0"` |

### 5.2 Identity Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repo` | string | YES | Repository name (e.g., `"xonaix-library"`) |
| `path` | string | YES | Full path from repo root |
| `unit_id` | string | YES | Unique identifier (library/category/name) |
| `title` | string | YES | Human-readable title |
| `document_type` | enum | YES | One of: `standard`, `mini-standard`, `template`, `contract` |
| `language` | string | YES | ISO 639-1 code (e.g., `"en"`) |

### 5.3 Classification Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trust_class` | string/null | YES | Trust classification: `L1`, `L2`, `L3`, `L4`, or `null` |
| `classification` | enum | YES | One of: `public`, `internal`, `confidential`, `restricted` |
| `compliance` | array | YES | Regulatory tags (e.g., `["SOC2", "FIPS"]`), may be empty |

### 5.4 Ownership Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `owner` | string | YES | Responsible party (e.g., `"Founder"`) |
| `approved_by` | string | YES | Who approved this version |

### 5.5 Authority Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authority.repo` | string | YES | Repository containing authority document |
| `authority.ref` | string | YES | Authority document filename |
| `authority.version` | string/null | YES | Pinned version (future use, currently `null`) |

### 5.6 Relationships Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `depends_on` | array | YES | Dependencies as `[{repo: "x", ref: "y.md"}]`, may be empty |
| `supersedes` | string/null | YES | Document this replaces |
| `superseded_by` | string/null | YES | Document that replaces this |

### 5.7 Lifecycle Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | string | YES | Format: `XLIB-MAJOR.MINOR.PATCH` |
| `status` | enum | YES | One of: `draft`, `proposed`, `active`, `deprecated`, `superseded` |
| `created` | string | YES | ISO 8601 UTC timestamp |
| `last_updated` | string | YES | ISO 8601 UTC timestamp |

---

## 6. Status Lifecycle

| Status | Description |
|--------|-------------|
| `draft` | Work in progress, not for reference |
| `proposed` | Ready for review and approval |
| `active` | Approved and authoritative |
| `deprecated` | Still valid but being phased out |
| `superseded` | Replaced by another document |

Deprecated standards remain authoritative unless explicitly superseded.

---

## 7. Classification Levels

| Level | Description |
|-------|-------------|
| `public` | Safe for external distribution |
| `internal` | Xonaix internal use only |
| `confidential` | Limited distribution, named recipients |
| `restricted` | Highest sensitivity, explicit approval required |

---

## 8. Trust Classes

| Class | Description |
|-------|-------------|
| `L1` | Constitutional — Can assert truth, sign, verify |
| `L2` | Deterministic — Trusted computation, no external authority |
| `L3` | Orchestration — Coordinates workflows, cannot assert truth |
| `L4` | Interface — UI/configuration, fully untrusted |
| `null` | Not applicable (templates, meta-documents) |

---

## 9. Forbidden Header Practices

The following are explicitly forbidden:

- Missing header fields
- Additional, undocumented fields
- Soft or advisory language in header values
- Ellipses (...)
- Emojis
- Placeholder values (e.g., `{TODO}`, `TBD`)
- Freeform metadata blocks
- Markdown headers in place of frontmatter
- Non-UTC timestamps
- Relative paths

---

## 10. Consistency Requirements

Header values MUST match:

- UNIT.json (when present)
- Unit manifest metadata
- BOM references (when applicable)

Inconsistencies are hard failures.

---

## 11. Enforcement

Compliance with this contract is enforced by:

- normalize-frontmatter tool
- CI --check mode
- Unit validation
- Doctor checks
- Schema validation

Any deviation constitutes governance debt and MUST block sealing and release.

---

## 12. Evolution

Changes to this contract require:

- Explicit Founder approval
- Schema version increment
- Regeneration of governance manifests
- Full CI verification pass
- Migration of existing documents

Backward compatibility is not guaranteed.

---

## 13. Final Assertion

A standard without a compliant header is not a standard.

The header is the identity anchor for machine and human reasoning.

Machine-first, human-readable. This is the Xonaix way.

---

*Governance Contract*
*Canonical location: \`specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md\`*
