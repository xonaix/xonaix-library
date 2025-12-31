# NO DEBT RULES

Scope: Xonaix Library (current-only)
Authority Level: Binding (CI-Enforced)
Status: Active

These rules are non-negotiable. Violations fail CI unconditionally.

## Enforcement Scope

Strict enforcement applies to **current-only** scope:
- `specs/_governance/**`
- `specs/library/<CURRENT_BASELINE>/**`

Excluded from strict enforcement:
- `_deprecated/` (historical reference; immutable)
- `_reference/` (external ecosystem reference)

---

## Binding Rules

### Core Rules

1. **Warnings = Debt** — All compiler/linter warnings are treated as errors. CI must fail on any warning.

2. **No Placeholders** — No incomplete markers anywhere in shipped scope. CI fails on detection.

3. **No Contradictions** — Conflicting claims across docs, specs, or tools is a hard fail. Terminology drift included.

4. **No Pre-Seal Signatures** — No `*.asc` or `*.sig` files in shipped scope until post-seal.

5. **No SEALED/SIGNED Claims Pre-Audit** — Baseline must remain CANDIDATE/PRE-SEAL until audits pass.

6. **Single Post-Seal Authority Contract** — BOM.json + detached signature (`BOM.json.asc`). Markdown is never a verification surface.

7. **No Non-Canonical Verification** — Any documentation or tooling that implies verification over markdown bytes is forbidden. Only BOM.json + `.json.asc` is canonical post-seal.

8. **No Emojis** — Emojis are forbidden in all shipped documentation. Documentation must be 100% pure and professional.

### Scope Controls

9. **Non-Distributable Paths** — `/drafts/`, `/quarantine/`, `/tools/legacy/` are excluded from audit scope and must not exist in shipped repos.

10. **Exceptions Require Founder Approval** — Any exception to these rules requires explicit Founder approval recorded in-repo.

---

## Machine-Parseable Policy

```yaml
policy_version: "1.0.0"
baseline: "LIB-1.0.0"
status: "PRE-SEAL"

warnings_are_errors: true

forbidden_tokens:
  - "TODO"
  - "TBD"
  - "FIXME"
  - "CHANGEME"
  - "PLACEHOLDER"
  - "INTENTIONALLY LEFT BLANK"
  - "..."  # stub ellipses as incomplete content

forbidden_paths:
  - "/drafts/"
  - "/quarantine/"
  - "/tools/legacy/"

forbidden_file_globs:
  - "*.asc"
  - "*.sig"

# Emojis are forbidden in all documentation
forbidden_emoji_ranges:
  - "U+1F300-U+1F9FF"  # Miscellaneous Symbols and Pictographs
  - "U+2600-U+26FF"    # Miscellaneous Symbols
  - "U+2700-U+27BF"    # Dingbats
  - "U+1F600-U+1F64F"  # Emoticons
  - "U+1F680-U+1F6FF"  # Transport and Map Symbols
  - "U+2300-U+23FF"    # Miscellaneous Technical

# Non-canonical verification patterns (MUST NOT appear in shipped scope)
forbidden_verification_patterns:
  - "gpg --verify"
  - ".md.asc"
  - "verify_authority_statement_hardened"

pre_seal_forbidden_claims:
  - "Status: SEALED"
  - "Status: SIGNED"

post_seal_contract:
  format: "JSON"
  signature_extension: ".json.asc"
  verification_surface: "BOM.json bytes only"
  markdown_authoritative: false

exception_policy:
  requires: "Founder approval"
  recorded_in_repo: true
```

---

## Enforcement

CI must parse the YAML block above and fail on any violation.

Enforcement tool: `tools/xonaix-library-tools enforce --current-only`

### Required Governance Documents

The following governance documents MUST exist in `specs/_governance/`:

| Document | Purpose |
|----------|---------|
| `NO_DEBT_RULES.md` | Zero-debt enforcement rules |
| `AUDIT_CONTRACT.md` | Audit requirements and gates |
| `DISTRIBUTION_EXCLUSIONS.md` | Non-distributable path policy |
| `LIBRARY_SEALING_CONTRACT.md` | Sealing and BOM semantics |
| `XONAIX_SELF_GOVERNANCE_CONTRACT.md` | Constitutional authority |

Missing governance documents fail CI unconditionally.

### Rust-Only Policy

Xonaix Library tooling is Rust-only. In current-only scope:
- No Python files (`.py`)
- No shell scripts (`.sh`)
- No Python token references (`python`, `python3`, `pip`, `conda`, `venv`, `pyenv`, `virtualenv`)

Enforcement command: `xonaix-library-tools enforce --current-only`
