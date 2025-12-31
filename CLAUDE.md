# Claude Code Instructions for xonaix-library

## Repository Overview

This is the **Xonaix Library** - the canonical source of truth for all Xonaix standards, specifications, and governance documents. It is a reference-only library that other Xonaix repositories depend on.

## Critical Rules

### 1. Never Declare Done Until CI is Green

After any push, ALWAYS verify CI passes before declaring complete:

```bash
gh run watch --exit-status
```

If CI fails, fix the issue and push again. Repeat until green.

### 2. Regenerate Manifests After Governance Changes

When modifying files in `specs/_governance/`, you MUST regenerate the governance manifest:

```bash
./bin/xonaix-library-tools generate-manifest --governance
```

Then commit and push the updated manifest.

### 3. Run Doctor Before Committing

Always verify the environment is healthy:

```bash
cargo run --manifest-path tools/xonaix-library-tools/Cargo.toml -- doctor
```

Or if the binary is built:
```bash
./bin/xonaix-library-tools doctor
```

## Document Header Schema

All standards use the `xonaix-document-header` schema v1.0. See:
- `specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md` - Master template
- `specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md` - Official contract

Required header sections:
- Schema identification (`schema`, `schema_version`)
- Identity (`repo`, `path`, `unit_id`, `title`, `document_type`, `language`)
- Classification (`trust_class`, `classification`, `compliance`)
- Ownership (`owner`, `approved_by`)
- Authority (`authority.repo`, `authority.ref`, `authority.version`)
- Relationships (`depends_on`, `supersedes`, `superseded_by`)
- Lifecycle (`version`, `status`, `created`, `last_updated`)

## Directory Structure

```
specs/
├── _governance/           # Governance contracts and manifests
│   ├── manifests/         # Generated SHA256 manifests (DO NOT EDIT MANUALLY)
│   ├── LIBRARY_SEALING_CONTRACT.md
│   ├── LIBRARY_STANDARD_HEADER_CONTRACT.md
│   └── UNIT_REGISTRY.json
├── standards/             # Language standards (one per language)
│   ├── rust/
│   ├── typescript/
│   └── ...
├── mini-standards/        # Smaller focused standards
└── meta/                  # Templates and meta-documents
    └── language-template/

tools/
└── xonaix-library-tools/  # Rust CLI for validation, manifests, doctor
```

## Common Tasks

### Adding a New Standard

1. Copy template from `specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md`
2. Fill in all header fields (no placeholders)
3. Add content following the template structure
4. Add entry to `specs/_governance/UNIT_REGISTRY.json`
5. Run doctor to verify
6. Commit and push, wait for green CI

### Updating a Standard

1. Update content
2. Update `last_updated` timestamp in header (ISO 8601 UTC)
3. Bump `version` if substantive change
4. Run doctor
5. Commit and push, wait for green CI

### Modifying Governance Documents

1. Make changes
2. Regenerate manifest: `./bin/xonaix-library-tools generate-manifest --governance`
3. Commit both the document AND the manifest
4. Push and wait for green CI

## Versioning

- Library versions: `XLIB-MAJOR.MINOR.PATCH`
- All timestamps: ISO 8601 UTC format (`2025-12-31T20:00:00Z`)

## Trust Classes

| Class | Description |
|-------|-------------|
| L1 | Constitutional — Can assert truth, sign, verify |
| L2 | Deterministic — Trusted computation |
| L3 | Orchestration — Coordinates workflows |
| L4 | Interface — UI/configuration, untrusted |

## CI Pipeline

The CI runs:
1. `build-tools` - Compiles the Rust CLI
2. `global-enforcement` - Runs all validation checks
3. `test-tools` - Runs Rust tests

All three must pass for green CI.

## Forbidden Practices

- Never edit files in `specs/_governance/manifests/` manually
- Never use placeholder values in headers
- Never skip CI verification
- Never commit without running doctor first
- Never use non-UTC timestamps
