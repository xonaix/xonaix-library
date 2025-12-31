---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md"
unit_id: "library/meta/language-template"
title: "Language Standard Template"
document_type: "template"
language: "en"

# --- Classification ---
trust_class: null
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

# Language Standard Template

This is the canonical template for creating new language standards in the Xonaix Library.

---

## Header Schema Reference

All Xonaix documents MUST use the following header schema:

\`\`\`yaml
---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "{REPO_NAME}"                    # Repository containing this document
path: "{FILE_PATH}"                    # Full path from repo root
unit_id: "{UNIT_ID}"                   # Unique identifier (library/category/name)
title: "{DOCUMENT_TITLE}"              # Human-readable title
document_type: "{TYPE}"                # standard | mini-standard | template | contract
language: "en"                         # ISO 639-1 language code

# --- Classification ---
trust_class: "{L1|L2|L3|L4|null}"      # Trust classification
classification: "{LEVEL}"              # public | internal | confidential | restricted
compliance: []                         # Regulatory tags: ["SOC2", "FIPS", "HIPAA"]

# --- Ownership ---
owner: "{OWNER}"                       # Responsible party
approved_by: "{APPROVER}"              # Who approved this version

# --- Authority ---
authority:
  repo: "{AUTHORITY_REPO}"             # Repository containing authority document
  ref: "{AUTHORITY_FILE}"              # Authority document filename
  version: null                        # Pin to specific version (future use)

# --- Relationships ---
depends_on: []                         # Dependencies: [{repo: "x", ref: "y.md"}]
supersedes: null                       # Document this replaces
superseded_by: null                    # Document that replaces this

# --- Lifecycle ---
version: "XLIB-{MAJOR.MINOR.PATCH}"    # Semantic version with XLIB prefix
status: "{STATUS}"                     # draft | proposed | active | deprecated | superseded
created: "{ISO8601_UTC}"               # Creation timestamp
last_updated: "{ISO8601_UTC}"          # Last modification timestamp
---
\`\`\`

### Status Lifecycle

| Status | Description |
|--------|-------------|
| draft | Work in progress, not for reference |
| proposed | Ready for review and approval |
| active | Approved and authoritative |
| deprecated | Still valid but being phased out |
| superseded | Replaced by another document |

### Classification Levels

| Level | Description |
|-------|-------------|
| public | Safe for external distribution |
| internal | Xonaix internal use only |
| confidential | Limited distribution, named recipients |
| restricted | Highest sensitivity, explicit approval required |

---

## Template: [LANGUAGE_NAME] Language Standard

*Copy from here when creating a new language standard.*

\`\`\`markdown
---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/{language}/STANDARDS_{LANGUAGE}.md"
unit_id: "library/standards/{language}"
title: "{Language} Language Standard"
document_type: "standard"
language: "en"

# --- Classification ---
trust_class: "{L1|L2|L1/L2|null}"
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
created: "{YYYY-MM-DD}T00:00:00Z"
last_updated: "{YYYY-MM-DD}T00:00:00Z"
---

# {Language} Language Standard

{Brief description of the language's role in Xonaix development.}

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | {L1/L2/L3/L4} |
| Classification | {Constitutional/Deterministic/Orchestration/Interface} |

### What This Trust Class May Do

- {Capability 1}
- {Capability 2}

### What This Trust Class May NOT Do

- {Restriction 1}
- {Restriction 2}

---

## XCLib Integration

**Authority:** {Ruling reference}

### XCLib Exclusivity Rule

{Description of how this language integrates with XCLib}

| Operation | Required Module |
|-----------|-----------------|
| Canonicalization | {module} |
| Hashing | {module} |
| Signing | {module} |
| Verification | {module} |

---

## Numeric Policy

**Authority:** {Ruling reference}

### Float Prohibition

{Description of float handling in this language}

\`\`\`{language}
// FORBIDDEN
{example of forbidden pattern}

// REQUIRED
{example of required pattern}
\`\`\`

---

## Principle Mapping

| Principle | {Language} Implementation |
|-----------|---------------------------|
| 1. Correct Over Fast | {implementation} |
| 2. Explicit Over Implicit | {implementation} |
| 3. Automated Over Vigilant | {implementation} |
| 4. Secure By Default | {implementation} |
| 5. Composable Over Clever | {implementation} |
| 6. Fail Loud | {implementation} |
| 7. X.I. Augments, Human Decides | {implementation} |
| 8. Future-Proof Over Trend | {implementation} |
| 9. Nothing Lost, Ever | {implementation} |
| 10. Clarity Above All | {implementation} |

---

{... Additional sections as needed ...}

---

*Xonaix Library Standard*
*Canonical: \`xonaix-library::{path}\`*
*Authority: \`xonaix-specs::THE_XONAIX_WAY.md\`*
\`\`\`

---

## Footer Format

All standards MUST end with:

\`\`\`markdown
---

*Xonaix Library Standard*
*Canonical: \`{repo}::{path}\`*
*Authority: \`{authority.repo}::{authority.ref}\`*
\`\`\`

---

*Xonaix Library Standard*
*Canonical: \`xonaix-library::specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md\`*
*Authority: \`xonaix-specs::THE_XONAIX_WAY.md\`*
