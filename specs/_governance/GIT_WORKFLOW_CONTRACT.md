---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "xonaix-library"
path: "specs/_governance/GIT_WORKFLOW_CONTRACT.md"
unit_id: "governance/library/git-workflow"
title: "Git Workflow Contract"
document_type: "contract"
language: "en"

# --- Version ---
version: "XGOV-1.0.0"
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
depends_on:
  - repo: "xonaix-library"
    ref: "LIBRARY_STANDARD_HEADER_CONTRACT.md"
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
last_updated: "2025-12-31T23:00:00Z"
---

# GIT WORKFLOW CONTRACT

**Scope:** All Xonaix repositories
**Purpose:** Define the standard Git workflow for all development

---

## 1. Purpose

This contract defines the mandatory Git workflow for all Xonaix repositories. The workflow is designed to:

- Prevent direct pushes to main
- Ensure human review of all changes
- Create clean, auditable commit history
- Enable future integration with ZeroPoint ledger

No repository may deviate from this workflow without explicit governance approval.

---

## 2. Core Principle

**Human approval is required for all changes to main.**

Agents and automation may:
- Create branches
- Make commits
- Open pull requests
- Wait for CI
- Merge ONLY with explicit human approval

---

## 3. Branch Naming Convention

All branches MUST follow the Conventional naming pattern:

```
type/short-description
```

### 3.1 Branch Types

| Type | Purpose | Example |
|------|---------|---------|
| `feat/` | New feature | `feat/add-header-validation` |
| `fix/` | Bug fix | `fix/crlf-line-endings` |
| `docs/` | Documentation only | `docs/update-readme` |
| `refactor/` | Code restructuring | `refactor/extract-module` |
| `test/` | Test additions/changes | `test/add-unit-tests` |
| `chore/` | Maintenance tasks | `chore/update-dependencies` |

### 3.2 Description Rules

- Use lowercase
- Use hyphens for spaces
- Keep short (3-5 words max)
- Be descriptive

---

## 4. Commit Message Format

All commits MUST follow the Conventional Commits specification:

```
type(scope): description

[optional body]

[optional footer]
```

### 4.1 Examples

```
feat(header): add v2.1 schema validation

Add header-validate command to Rust CLI.
Validates all document headers against v2.1 schema.

Closes #42
```

```
fix(enforce): handle CRLF detection on Windows

The CRLF check was failing on Windows due to line ending
normalization in the file read operation.
```

### 4.2 Type Reference

| Type | Version Impact | Description |
|------|---------------|-------------|
| `feat` | Minor bump | New feature |
| `fix` | Patch bump | Bug fix |
| `docs` | No bump | Documentation only |
| `refactor` | No bump | Code restructuring |
| `test` | No bump | Test changes |
| `chore` | No bump | Maintenance |
| `BREAKING CHANGE` | Major bump | Breaking change (in footer) |

---

## 5. Pull Request Workflow

### 5.1 Standard Flow

```
1. Create branch from main
   git checkout -b feat/my-feature

2. Make changes, commit (multiple commits OK)
   git add .
   git commit -m "feat(scope): description"

3. Push branch
   git push -u origin feat/my-feature

4. Create PR
   gh pr create --title "feat(scope): description" --body "..."

5. Wait for CI
   gh pr checks

6. Human reviews and approves
   (Human action required)

7. Squash merge to main
   gh pr merge --squash

8. Verify post-merge CI
   gh run watch
```

### 5.2 Multiple Commits on Branch

Branches may contain multiple commits:

```
feat/add-validation
  - "wip: initial structure"
  - "fix: missing import"
  - "fix: handle edge case"
  - "docs: add comments"
```

All commits are squashed into one on merge:

```
main
  - "feat(validation): add header validation (#42)"
```

---

## 6. Human Approval Requirement

### 6.1 Default Behavior

ALL pull requests require human approval before merge.

The human reviewer MUST:
- Review the changes
- Verify CI is green
- Click "Approve" in GitHub
- Click "Squash and merge"

### 6.2 Override Mechanism

In exceptional cases, a human may grant 1-time auto-merge permission.

**How to override:**
1. Human explicitly states in conversation: "Auto-merge this PR"
2. Agent merges when CI is green
3. The command becomes part of the audit trail

**Override rules:**
- 1-time only (per PR, not blanket)
- Cannot be delegated
- Cannot be scheduled ("auto-merge for the next hour")
- Even Founder follows these rules
- Future: Override recorded on ZeroPoint ledger

---

## 7. Branch Protection Rules

All Xonaix repositories MUST enable these branch protection rules on `main`:

| Rule | Setting | Purpose |
|------|---------|---------|
| Require PR | Yes | No direct push |
| Require approval | Yes (1) | Human eyeball |
| Require status checks | Yes | CI must pass |
| Require linear history | Yes | Squash only |
| Allow force push | No | Protect history |
| Allow deletions | No | Protect branch |
| Require up-to-date | No | Avoid rebase friction |

### 7.1 Status Checks Required

At minimum:
- Build passes
- Tests pass
- Enforcement checks pass

Additional checks as defined per repository.

---

## 8. CODEOWNERS

All repositories MUST have a CODEOWNERS file:

```
# .github/CODEOWNERS

# Default: Founder reviews everything
* @Founder

# Future: Team-specific ownership
# /src/auth/ @security-team
# /docs/ @docs-team
```

---

## 9. Merge Strategy

The ONLY permitted merge strategy is **Squash and Merge**.

This ensures:
- One commit per logical change
- Clean history on main
- Easy revert if needed
- Clear audit trail

Merge commits and rebase merges are NOT permitted.

---

## 10. CI Requirements

Before merge, CI MUST pass:

1. Build successfully
2. All tests pass
3. Enforcement checks pass (no forbidden tokens, etc.)
4. Header validation passes (schema compliance)
5. Manifest drift check passes

Future additions:
- Cortex Guardian analysis
- Security scanning
- Dependency audit

---

## 11. Post-Merge Verification

After squash merge, verify:

1. Post-merge CI is green
2. No unexpected failures

If post-merge CI fails:
1. Investigate immediately
2. Revert if necessary
3. Fix on new branch
4. Open new PR

---

## 12. Future: ZeroPoint Integration

When ZeroPoint is operational:

- Every PR approval will be signed
- Every merge will be anchored to ledger
- Every override will be recorded immutably
- Full audit trail of all decisions

---

## 13. Enforcement

This contract is enforced by:

- GitHub branch protection rules
- CI pipeline checks
- Agent behavior (will not merge without approval)
- Future: Cortex Guardian

Violations constitute governance debt and MUST be addressed.

---

## 14. Applicability

This contract applies to:

- xonaix-library
- All future Xonaix repositories
- All Org Blade repositories
- All customer engagement repositories

No exceptions without governance approval.

---

*Governance Contract*
*Canonical: `xonaix-library::specs/_governance/GIT_WORKFLOW_CONTRACT.md`*
*Authority: `xonaix-specs::XONAIX_SELF_GOVERNANCE_CONTRACT.md`*
