# COLD-START / NO-CARRYOVER AUDIT CONTRACT (Zero Debt)

Treat this audit as a **standalone engagement**: ignore any prior conversation context, memory, or assumptions. Use **only the contents of the audited Library baseline** as evidence. If something is not present, do not assume it exists.

---

## Library Structure

The Xonaix Library is organized as a unit-based standards repository:

| Component | Pattern | Path | Description |
|-----------|---------|------|-------------|
| Library | `LIB-X.Y.Z` | `specs/library/LIB-X.Y.Z/` | Library baseline |
| Standards | `library/<category>/<name>` | `specs/library/LIB-X.Y.Z/standards/<category>/<name>/` | Individual standard units |

Each unit has its own:
- UNIT.json identity
- Unit manifest
- Version and changelog
- BOM inclusion eligibility

---

## Scope

Default audit scope is **current-only**:

- `specs/_governance/**` (shared governance)
- `specs/library/<CURRENT_BASELINE>/**` (current Library baseline)

For Library audit:
- Audit **every non-git-control file** within the current Library baseline path.
- Exclude only git-control artifacts: `.git/`, `CODEOWNERS`, `.gitignore`, and other git metadata.
- **Do NOT exclude CI/build configuration** (`.github/` is in scope if present within domain).
- Governance docs in `specs/_governance/` must be audited.

---

## Excluded from Default Audit Scope

The following are excluded from default audit scope:

| Path | Reason | Policy |
|------|--------|--------|
| `_deprecated/` | Deprecated baselines | Reference only; hands-off |
| `_reference/` | External ecosystem reference | Non-authoritative |
| `target/` | Build artifacts | Not source |

### _deprecated/ Policy

- Deprecated baselines are immutable (no modifications permitted).
- Auditors must treat `_deprecated/` as historical reference only.
- Inclusion in audit requires explicit Founder scope directive.
- Never audit `_deprecated/` content unless explicitly instructed.

### _reference/ Policy

- Reference materials describe external ecosystems.
- They do NOT authorize their use within Xonaix Library.
- Excluded from strict Rust-only enforcement scans.

---

## Definition of DEBT (Hard Fail)

Debt is any of the following:
- Any contradiction across truth sources (terminology drift included).
- Any warning not resolved (warnings = debt = fail).
- Any unverifiable claim.
- Any non-canonical signature scheme.
- Any truncated hash/index.
- Any TODO/FIXME/TBD/CHANGEME/PLACEHOLDER/"intentionally blank" content.
- Any example credential/secret (dummy secrets forbidden).
- Any ambiguity in verification instructions (must be deterministic and reproducible).
- Any emoji in documentation (emojis are forbidden).

---

## Authority Lifecycle (Binding)

### PRE-SEAL (per baseline)
- Library baseline is **PRE-SEAL / CANDIDATE** until all required audits pass with **zero warnings**.
- **No cryptographic verification applies until the seal ceremony is complete.**
- **No signature artifacts may exist in shipped scope pre-seal** (no `*.asc`, no `*.sig`, no markdown signatures).

### POST-SEAL (per baseline, only after all audits pass)
- Canonical authority contract is:
  - **BOM.json + detached signature (`BOM.json.asc`)**
- Markdown is human-readable only and is **never** a verification surface.
- Any tooling or documentation that implies verification over markdown bytes is non-canonical and invalid.

---

## Truth-Source Order (for resolving conflicts)

1) Library spec text + verification scripts
2) `specs/_governance/` governance documents
3) BOM.json + sealing rules (post-seal only)
4) Derived indices/status summaries (must be reproducible outputs, never authoritative)

---

## Rainbow Team Lenses (Required Sections)

For Library audits, the following teams are most relevant:

- **White:** Audit coordination, arbitration
- **Green:** Governance compliance, sustainability
- **Cyan:** Data integrity, truth-source alignment
- **Yellow:** Code quality, maintainability
- **Rainbow:** Cross-team synthesis, holistic assessment

---

## Hard Gates (Must Explicitly PASS/FAIL)

1) **Inventory gate:** Complete inventory with SHA-256, size, type, and path (nothing skipped)
2) **Spec gate:** Run verification scripts; include exact commands and outputs
3) **Integrity gate:** Manifests/indices/hashes complete (no truncation) and match computed values
4) **Authority gate:** No sealed/signed claims pre-audit; post-seal contract only (BOM.json + `.json.asc`)
5) **Zero-warning gate:** Any warning (including "note/info" risk language) is a build-stopping defect
6) **Unit gate:** All units declared in registry, all manifests valid

---

## Deliverables

Per Library audit:
- Full inventory (`inventory_library_<baseline>.csv` / `.json`)
- `audit_report_library_<baseline>.md` with ranked findings and dependency-ordered fix plan
- Begin the report by restating this contract verbatim and confirming compliance

---

## Related Governance Documents

Auditors MUST also review:

| Document | Relevance |
|----------|-----------|
| `NO_DEBT_RULES.md` | Zero-debt enforcement requirements |
| `DISTRIBUTION_EXCLUSIONS.md` | Exclusion rules |
| `LIBRARY_SEALING_CONTRACT.md` | Sealing and BOM semantics |
| `XONAIX_SELF_GOVERNANCE_CONTRACT.md` | Constitutional authority |

---

*Global Governance Document - Applies to Xonaix Library*
*Canonical location: `specs/_governance/AUDIT_CONTRACT.md`*
