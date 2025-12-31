# Session Context - Git Workflow & Cortex Vision

## Completed This Session

1. Implemented header schema v2.1 with expanded status lifecycle
2. Built header-validate Rust command
3. Created _roadmap/ folder with governance specs
4. Migrated 17 documents to v2.1
5. CI is green

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

## Git Workflow Decisions

### Agreed
- PR-first workflow (no direct to main)
- Human approval required by default
- 1-time override only (explicit command in conversation)
- Conventional Commits for messages
- Squash merge to main
- Branch protection enabled
- CODEOWNERS file

### Override Mechanism
Option A: Explicit command in conversation
- Human says Auto-merge this PR
- Agent merges when CI green
- Command becomes part of audit trail

### Branch Protection Rules
- Require PR to merge: Yes
- Require 1 approval: Yes
- Require status checks: Yes
- Require linear history: Yes (squash only)
- Require up-to-date branch: No (avoids friction)
- Allow force push: No
- Require signed commits: No (add with ZeroPoint later)

## Completed Tasks

All governance tasks completed:

1. CORTEX_GUARDIAN_SPEC.md created in _roadmap
2. GIT_WORKFLOW_CONTRACT.md created in _governance
3. CLAUDE.md updated with PR workflow requirements
4. Branch protection enabled on xonaix-library
5. PR workflow tested (this PR)

## Branch Protection Active

The following rules are now enforced on main:

- Required status checks: build-tools, global-enforcement, test-tools
- Required approving reviews: 1
- Dismiss stale reviews: Yes
- Enforce admins: Yes (even admins must follow rules)
- Linear history: Yes (squash merge only)
- Force pushes: Disabled
- Branch deletions: Disabled

This is the first PR under the new workflow.
