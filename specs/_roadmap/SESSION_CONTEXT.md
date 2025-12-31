# Session Context - Git Workflow & Cortex Vision

## Completed This Session

1. Implemented header schema v2.1 with expanded status lifecycle
2. Built header-validate Rust command
3. Created _roadmap/ folder with governance specs
4. Migrated 17 documents to v2.1
5. CI is green
6. Created GIT_WORKFLOW_CONTRACT.md
7. Created CORTEX_GUARDIAN_SPEC.md
8. Updated CLAUDE.md with PR workflow
9. Set up branch protection on main

## Xonaix Vision: Trust Infrastructure

### The Problem (Thanksgiving 2025)
AWS, Microsoft, Cloudflare all went down in one week - all due to bad decisions.
Bad configs, unreviewed changes, fat-fingers, rogue actions.

### The Solution: Xonaix Trust Stack

1. **Everything requires approval** - No unilateral actions
2. **Everything is signed** - Cryptographic proof of who/when
3. **Everything is on the ledger** - ZeroPoint immutable audit trail
4. **Even Founder follows rules** - Leading by example, no exceptions
5. **Cortex analyzes before merge** - AI guardian catches problems before damage

### Cortex Guardian Vision

Xonaix Cortex analyzes commits and advises if something will cause harm:
- Bad configs that could cause outages
- Security vulnerabilities (viruses, zero-days)
- Code patterns that break systems
- Hardware configs that destabilize

**Scale:** From dad at home with OpenWRT + Cortex-Lite to the DOD
**Goal:** Shield the world so technology keeps people safe, happy, and protected

Future: OpenWRT refactored by Xonaix Code-Core and Forge as Rust

## Git Workflow - Phased Implementation

### Phase 1: Single-Party Workflow (Current)

Branch protection active with:
- PR required (no direct push to main)
- CI must pass (build-tools, global-enforcement, test-tools)
- Linear history (squash merge only)
- No force pushes
- No branch deletions
- Review requirement: DISABLED (single-person workflow)

Macman-1 (Founder) creates PRs, waits for CI, merges.

### Phase 2: Two-Account Workflow (Next)

When Xonaix Core account is set up:
- **Xonaix-Core**: Claude submits PRs from this account
- **Macman-1**: Reviews and approves PRs
- Review requirement: RE-ENABLED

This enables proper separation:
- AI agent submits work
- Human reviews and approves
- Full audit trail of who did what

### Phase 3: Multi-Party Workflow (Future)

When team grows or customer engagements begin:
- Multiple reviewers
- CODEOWNERS file
- Role-based approvals
- See MULTI_PARTY_GOVERNANCE_SPEC.md

## Branch Protection Rules (Phase 1)

| Rule | Setting |
|------|---------|
| Require PR to merge | Yes |
| Require status checks | Yes (strict) |
| Required checks | build-tools, global-enforcement, test-tools |
| Require linear history | Yes |
| Enforce admins | Yes |
| Allow force push | No |
| Allow deletions | No |
| Require approving reviews | No (Phase 1 only) |

## Account Strategy

| Account | Role | Purpose |
|---------|------|---------|
| Macman-1 | Founder | Reviews, approves, owns repos |
| Xonaix-Core | AI Agent | Claude submits PRs from this account |

This separation ensures:
- Clear audit trail (human vs AI actions)
- Proper review workflow (AI submits, human approves)
- No self-approval issues
- Scales to team workflow later

## Governance Documents Created

1. **GIT_WORKFLOW_CONTRACT.md** - Git workflow governance
2. **CORTEX_GUARDIAN_SPEC.md** - AI commit analysis roadmap
3. **MULTI_PARTY_GOVERNANCE_SPEC.md** - Future multi-party workflow
4. **ZEROPOINT_INTEGRATION_SPEC.md** - Cryptographic ledger integration
