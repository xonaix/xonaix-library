# Claude Code Instructions for xonaix-library

## Repository Overview

This is the **Xonaix Library** - the canonical source of truth for all Xonaix standards, specifications, and governance documents. It is a reference-only library that other Xonaix repositories depend on.

## Critical Rules

### 1. NEVER Push Directly to Main

All changes MUST go through a Pull Request:

```bash
# Create branch
git checkout -b feat/my-feature

# Make changes, commit
git add .
git commit -m "feat(scope): description"

# Push branch
git push -u origin feat/my-feature

# Create PR
gh pr create --title "feat(scope): description" --body "Description of changes"

# Wait for CI
gh pr checks

# Human approves and merges (you cannot merge without approval)
```

### 2. Human Approval Required

You CANNOT merge PRs without human approval. The workflow is:

1. You create PR and wait for CI green
2. You notify human: "PR ready for review"
3. Human reviews, approves, and clicks merge
4. You verify post-merge CI is green

**Override:** Only if human explicitly says "Auto-merge this PR" can you merge without clicking.

### 3. Regenerate Manifests After Governance Changes

When modifying files in `specs/_governance/`, you MUST regenerate the governance manifest:

```bash
./bin/xonaix-library-tools generate-manifest --governance
```

### 4. Run Doctor Before Committing

Always verify the environment is healthy:

```bash
./bin/xonaix-library-tools doctor
```

## Git Workflow

See `specs/_governance/GIT_WORKFLOW_CONTRACT.md` for full details.

### Branch Naming

```
type/short-description

feat/add-validation     # New feature
fix/crlf-issue          # Bug fix
docs/update-readme      # Documentation
refactor/extract-mod    # Restructuring
chore/update-deps       # Maintenance
```

### Commit Messages (Conventional Commits)

```
type(scope): description

feat(header): add v2.1 schema validation
fix(enforce): handle CRLF on Windows
docs(claude): update workflow instructions
```

### PR Flow

```
1. git checkout -b feat/my-feature
2. Make changes, commit (multiple commits OK)
3. git push -u origin feat/my-feature
4. gh pr create
5. Wait for CI green
6. Human reviews and approves
7. Human clicks "Squash and merge"
8. Verify post-merge CI
```

### Proactive Workflow

After creating a PR and CI passes:

1. Notify human: "PR ready for review"
2. Set todo to "waiting on human"
3. **Watch for merge** (poll PR status in background)
4. When human merges, **proceed automatically**
5. Verify post-merge CI
6. Clean up branch
7. Report completion

Do NOT wait for human to say "continue" - detect the merge and proceed.

See `specs/_roadmap/SESSION_CONTEXT.md` for full workflow patterns.

## Document Header Schema

All standards use the `xonaix-document-header` schema v2.1. See:
- `specs/meta/language-template/TEMPLATE_LANGUAGE_STANDARD.md` - Master template
- `specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md` - Official contract

Required header sections:
- Schema identification (`schema`, `schema_version`)
- Identity (`repo`, `path`, `unit_id`, `title`, `document_type`, `language`)
- Version (`version`, `baseline`, `status`)
- Classification (`trust_class`, `classification`, `compliance`)
- Ownership (`owner`, `approved_by`, `authority_tier`)
- Authority (`authority.repo`, `authority.ref`, `authority.version`)
- Relationships (`depends_on`, `supersedes`, `superseded_by`, `implements`)
- Integrity (`hash_alg`, `content_hash`, `signature`, `signed_by`, `signed_at`)
- Constitutional Conformance (constitution and zero point references)
- Lifecycle (`created`, `last_updated`)

## Directory Structure

```
specs/
  _governance/           # Governance contracts and manifests
    manifests/           # Generated SHA256 manifests (DO NOT EDIT)
    GIT_WORKFLOW_CONTRACT.md
    LIBRARY_SEALING_CONTRACT.md
    LIBRARY_STANDARD_HEADER_CONTRACT.md
    UNIT_REGISTRY.json
  _roadmap/              # Future specifications
  standards/             # Language standards
  mini-standards/        # Smaller focused standards
  meta/                  # Templates and meta-documents

tools/
  xonaix-library-tools/  # Rust CLI
```

## Trust Classes

| Class | Description |
|-------|-------------|
| L0 | Constitutional - Foundational truth, bedrock principles |
| L1 | Authority - Can assert truth, sign, verify |
| L2 | Deterministic - Trusted computation |
| L3 | Orchestration - Coordinates workflows |
| L4 | Interface - UI/configuration, untrusted |

## Status Lifecycle

```
draft -> internal_review -> proposed -> approved -> sealed -> deprecated -> superseded
```

| Status | Description | Integrity Requirement |
|--------|-------------|----------------------|
| draft | Work in progress | None |
| internal_review | Ready for team review | None |
| proposed | Submitted for approval | None |
| approved | Content locked | content_hash (warn if missing) |
| sealed | Signed and immutable | signature required |
| deprecated | Being phased out | Maintains seal |
| superseded | Replaced | Maintains seal |

## CI Pipeline

The CI runs:
1. `build-tools` - Compiles the Rust CLI
2. `global-enforcement` - Runs enforce, header-validate, unit-validate, graph-verify, doctor, manifest check
3. `test-tools` - Runs Rust tests

All three must pass for green CI.

## Forbidden Practices

- Never push directly to main
- Never merge without human approval
- Never edit files in `specs/_governance/manifests/` manually
- Never use placeholder values in headers
- Never skip CI verification
- Never commit without running doctor first
- Never use non-UTC timestamps
