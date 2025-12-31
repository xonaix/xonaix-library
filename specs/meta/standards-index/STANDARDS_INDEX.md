---
schema: "xonaix-document-header"
schema_version: "1.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/meta/standards-index/STANDARDS_INDEX.md"
unit_id: "library/meta/standards-index"
title: "Standards Index"
document_type: "standard"
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
last_updated: "2025-12-31T21:00:00Z"
---

# Xonaix Library Standards Index

This document defines the cross-cutting requirements and classification system for all language standards in the Xonaix Library.

**For principles and philosophy:** See THE_XONAIX_WAY.md in xonaix-specs.

**For the authoritative catalog of units:** See UNIT_REGISTRY.json in _governance.

**For language-specific implementation:** See the individual standards in specs/standards/.

---

## Part 1: Trust Classes

Trust Classes define what operations a language may safely perform within Xonaix systems. They define capability boundaries, not language restrictions.

### Trust Class Definitions

| Class | Name | May Do | May NOT Do |
|-------|------|--------|------------|
| **L1** | Constitutional | Sign, verify, canonicalize, hash (via XCLib) | — |
| **L2** | Deterministic Compute | Pure computation, transformation | Sign/verify without L1 delegation |
| **L3** | Orchestration | Coordinate, transport, query | Assert truth, infer authority |
| **L4** | Interface | Display, tooling, scripting | Any governance-affecting operation |

### Classification Rules

1. **No Promotion Without Standard** — A language cannot operate above its standard's trust class
2. **Explicit Declaration** — Code must declare its intended trust class
3. **Downgrade Permitted** — L1 code may operate as L2/L3/L4
4. **Audit Requirement** — L1 code paths require security audit

---

## Part 2: Cross-Language Requirements

These implementation requirements apply to ALL language standards. Individual standards MUST implement these for their specific language.

*These requirements implement principles defined in THE_XONAIX_WAY.md.*

### Requirement 1: XCLib Exclusivity

Any code performing canonicalization, hashing, signing, or verification of Xonaix proofs MUST use XCLib or its official bindings.

| Trust Class | Requirement |
|-------------|-------------|
| L1 | Direct XCLib use required |
| L2 | XCLib bindings or delegation to L1 |
| L3 | Must delegate crypto to L1/L2 |
| L4 | No crypto operations permitted |

### Requirement 2: Numeric Canonicalization

Any numeric value influencing governance, hashing, signing, or verification MUST be represented as integer or fixed-decimal. Floats are forbidden in canonical paths.

| Language | Forbidden | Required |
|----------|-----------|----------|
| Rust | `f32`, `f64` in canonical paths | Integer types, decimal crates |
| TypeScript | `number` for canonical data | `string`, `BigInt` |
| SQL | `FLOAT`, `DOUBLE`, `REAL` | `INTEGER`, `DECIMAL`, `NUMERIC` |

### Requirement 3: No Silent Authority

No code may infer authority, trust, or posture from context, environment, or configuration. Authority must always be explicit and cryptographically proven.

### Requirement 4: Bounded Error Surfaces

Error behavior must not leak information or create side channels. Each language must define explicit error handling that maps to bounded public error codes.

### Requirement 5: Generated Code Accountability

Code produced by Forge, agents, or templates MUST pass the same standards as human-written code. "Generated" status does not bypass standards.

---

## Part 3: Language Policy

> **Rust-first. TypeScript for interfaces. Everything else requires justification.**

### Primary Languages

| Language | Role | Trust Class |
|----------|------|-------------|
| **Rust** | Primary systems language | L1/L2 |
| **TypeScript** | Secondary, interfaces only | L3 |

### Supporting Languages

| Language | Role | Trust Class |
|----------|------|-------------|
| **SQL** | Data persistence | L2 |
| **NATS** | Messaging | L2 |
| **WASM** | Sandboxed execution | L1/L2 |

### Framework Standards

| Framework | Role | Trust Class |
|-----------|------|-------------|
| **Tauri** | Desktop shell | L3/L4 |
| **SvelteKit** | UI framework | L4 |

### Configuration Formats

| Format | Role | Trust Class |
|--------|------|-------------|
| **YAML** | Configuration | L4 |
| **TOML** | Rust configuration | L4 |
| **JSON** | Data interchange | L4 |

### Additional Languages

| Language | Role | Trust Class |
|----------|------|-------------|
| **C** | FFI bindings | L1/L2 |
| **C++** | FFI bindings | L1/L2 |
| **Node.js** | Build tooling | L3 |
| **Python** | Scripting, AI/ML | L3 |

---

## Part 4: Certification Levels

| Level | Requirements | Designation |
|-------|--------------|-------------|
| **Xonaix Certified** | All MUST + all SHOULD | "Built with The Xonaix Way" |
| **Xonaix Compatible** | All MUST | "Xonaix Compatible" |
| **User Choice** | Deviations from MUST | No certification (recorded) |

---

## Part 5: Creating New Standards

Use TEMPLATE_LANGUAGE_STANDARD.md as the base for any new language standard.

### Process

1. Copy the template from `specs/meta/language-template/`
2. Fill in language-specific implementations
3. Assign appropriate Trust Class
4. Register in `specs/_governance/UNIT_REGISTRY.json`
5. Run doctor to verify
6. Commit and push, wait for green CI

### Justification Required

When proposing a new language, document:

- Why Rust cannot be used
- Trust Class assignment with rationale
- Implementation of all Cross-Language Requirements
- Migration path to Rust if applicable

---

## References

- **Principles:** THE_XONAIX_WAY.md (xonaix-specs)
- **Header Format:** LIBRARY_STANDARD_HEADER_CONTRACT.md
- **Unit Registry:** UNIT_REGISTRY.json
- **Template:** TEMPLATE_LANGUAGE_STANDARD.md

---

*Xonaix Library Standard*
*Canonical: `xonaix-library::specs/meta/standards-index/STANDARDS_INDEX.md`*
*Authority: `xonaix-specs::THE_XONAIX_WAY.md`*
