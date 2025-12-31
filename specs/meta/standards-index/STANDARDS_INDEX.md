---
title: "Standards Index"
unit_id: "library/meta/standards-index"
standard_type: "standard"
version: "XLIB-1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# Xonaix Library Standards Index

This document serves as the authoritative catalog of language standards in the Xonaix Library.

**For principles and philosophy:** See THE_XONAIX_WAY.md in xonaix-specs/core.

**For language-specific implementation:** See the individual standards listed below.

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

| Language | Role | Trust Class | Status |
|----------|------|-------------|--------|
| **Rust** | Primary | L1/L2 | Active |
| **TypeScript** | Secondary | L3 | Active |
| **SQL** | Data | L3 | Active |
| **NATS** | Messaging | L3 | Active |
| **Tauri** | Desktop | L3/L4 | Active |
| **SvelteKit** | UI Framework | L4 | Active |
| **WASM** | Sandboxed Execution | L1/L2 | Active |
| **YAML** | Configuration | L4 | Active |
| **TOML** | Configuration | L4 | Active |
| **JSON** | Data Format | L4 | Active |
| **C** | FFI | L2 | Active |
| **C++** | FFI | L2 | Active |
| **Node.js** | Runtime | L3 | Active |
| **Python** | Legacy | L4 | Deprecated |

---

## Part 4: Certification Levels

| Level | Requirements | Designation |
|-------|--------------|-------------|
| **Xonaix Certified** | All MUST + all SHOULD | "Built with The Xonaix Way" |
| **Xonaix Compatible** | All MUST | "Xonaix Compatible" |
| **User Choice** | Deviations from MUST | No certification (recorded) |

---

## Part 5: Standards Catalog

### Language Standards

| Standard | Language | Trust Class | Location |
|----------|----------|-------------|----------|
| STANDARDS_RUST.md | Rust | L1/L2 | `specs/standards/rust/` |
| STANDARDS_TYPESCRIPT.md | TypeScript | L3 | `specs/standards/typescript/` |
| STANDARDS_SQL.md | SQL | L3 | `specs/standards/sql/` |
| STANDARDS_NATS.md | NATS | L3 | `specs/standards/nats/` |
| STANDARDS_TAURI.md | Tauri | L3/L4 | `specs/standards/tauri/` |
| STANDARDS_SVELTEKIT.md | SvelteKit | L4 | `specs/standards/sveltekit/` |
| STANDARDS_WASM.md | WebAssembly | L1/L2 | `specs/standards/wasm/` |
| STANDARDS_YAML.md | YAML | L4 | `specs/standards/yaml/` |
| STANDARDS_TOML.md | TOML | L4 | `specs/standards/toml/` |
| STANDARDS_JSON.md | JSON | L4 | `specs/standards/json/` |
| STANDARDS_C.md | C | L2 | `specs/standards/c/` |
| STANDARDS_CPP.md | C++ | L2 | `specs/standards/cpp/` |
| STANDARDS_NODEJS.md | Node.js | L3 | `specs/standards/nodejs/` |
| STANDARDS_PYTHON.md | Python | L4 | `specs/standards/python/` |

### Meta Standards

| Standard | Purpose | Location |
|----------|---------|----------|
| STANDARDS_INDEX.md | This catalog | `specs/meta/standards-index/` |
| TEMPLATE_LANGUAGE_STANDARD.md | Template for new standards | `specs/meta/language-template/` |

---

## Creating New Standards

Use TEMPLATE_LANGUAGE_STANDARD.md as the base for any new language standard.

### Process

1. Copy the template
2. Fill in language-specific implementations
3. Assign appropriate Trust Class
4. Add to this catalog
5. Register in UNIT_REGISTRY.json

### Justification Required

When proposing a new language, document:
- Why Rust cannot be used
- Trust Class assignment with rationale
- Implementation of all Cross-Language Requirements
- Migration path to Rust if applicable

---

## References

- **Principles:** THE_XONAIX_WAY.md (xonaix-specs/core)
- **Header Format:** LIBRARY_STANDARD_HEADER_CONTRACT.md
- **Unit Registry:** UNIT_REGISTRY.json

---

*Xonaix Library Standards Index v1.0.0*

*"Standards define what 'good' looks like — for new code and legacy transformation alike."*
