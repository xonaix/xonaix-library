# LIBRARY STANDARD HEADER CONTRACT

**Status:** Binding Standard Contract
**Applies to:** All Xonaix Library standards and mini-standards
**Authority:** XONAIX_SELF_GOVERNANCE_CONTRACT.md
**Scope:** Library repository only

---

## 1. Purpose

This contract defines the mandatory header format for all standards and mini-standards in the Xonaix Library.

The header is not decorative. It is a semantic control surface for:

- Human comprehension
- Mechanical verification
- Agent reasoning
- Long-term governance stability

No standard is considered valid without a compliant header.

---

## 2. Applicability

This contract applies to:

- All Library standards
- All Library mini-standards
- All future standards added to the Library

This contract does not apply to:

- Product specifications (B5, UX, Nexus, Web)
- Governance contracts
- Runtime documentation
- Reference-only external documents

---

## 3. Header Location

The header MUST appear at the top of the file, before any prose.

No content, comments, or whitespace may precede the header.

---

## 4. Header Format (MANDATORY)

The header MUST use YAML frontmatter delimited by triple dashes.

### Canonical Format

```yaml
---
title: "<Standard Title>"
unit_id: "library/<category>/<name>"
standard_type: "<standard | mini-standard>"
version: "XLIB-<MAJOR.MINOR.PATCH>"
status: "<active | deprecated>"
owner: "<Owner Identifier>"
last_updated: "<YYYY-MM-DD>"
---
```

---

## 5. Field Semantics (Binding)

### 5.1 title

- Human-readable name of the standard
- MUST be descriptive
- MUST NOT include version numbers

### 5.2 unit_id

- MUST exactly match the unit's declared UNIT.json
- Path-derived inference is forbidden
- Immutable once published

### 5.3 standard_type

MUST be one of:

- `standard`
- `mini-standard`

No other values permitted.

### 5.4 version

- MUST use format: `XLIB-MAJOR.MINOR.PATCH`
- XLIB prefix identifies this as a Xonaix Library document
- MUST match the version declared in UNIT.json
- Example: `XLIB-1.0.0`, `XLIB-1.1.0`, `XLIB-2.0.0`

### 5.5 status

MUST be one of:

- `active`
- `deprecated`

Deprecated standards remain authoritative unless explicitly replaced.

### 5.6 owner

- MUST identify the responsible authority
- MAY be a role (e.g., Founder)
- Ownership changes require explicit update

### 5.7 last_updated

- ISO date format only: YYYY-MM-DD
- Updated only when content meaningfully changes

---

## 6. Forbidden Header Practices

The following are explicitly forbidden:

- Missing header fields
- Additional, undocumented fields
- Soft or advisory language in header values
- Ellipses (...)
- Emojis
- Placeholder values
- Freeform metadata blocks
- Markdown headers in place of frontmatter

---

## 7. Consistency Requirements

Header values MUST match:

- UNIT.json
- Unit manifest metadata
- BOM references (when applicable)

Inconsistencies are hard failures.

---

## 8. Enforcement

Compliance with this contract is enforced by:

- normalize-frontmatter
- CI --check mode
- Unit validation
- Doctor checks

Any deviation constitutes governance debt and MUST block sealing and release.

---

## 9. Evolution

Changes to this contract require:

- Explicit Founder approval
- Regeneration of governance manifests
- Full CI verification pass

Backward compatibility is not guaranteed.

---

## 10. Final Assertion

A standard without a compliant header is not a standard.

The header is the identity anchor for human and machine reasoning.

Clarity here prevents ambiguity everywhere else.
