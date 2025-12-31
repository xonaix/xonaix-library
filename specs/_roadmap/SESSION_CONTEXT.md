# Session Context - Claude Recovery Document

**Purpose:** Restore Claude context if session is lost (VS Code crash, etc.)

**Last Updated:** 2025-12-31T23:35:00Z

---

## Current State

### Repository
- **Repo:** xonaix-library
- **Branch:** main (after this commit)
- **Last Commit:** Adding VISION.md and context update

### Branch Protection Active
- PR required (but override granted for this specific push)
- CI must pass: build-tools, global-enforcement, test-tools
- Linear history (squash merge)
- No force pushes

### Account Strategy (Phase 1 Active)
- **Macman-1:** Founder, reviews/approves
- **Xonaix-Core:** (Future) Claude submits from this account
- Currently in Phase 1: single-party workflow, no review requirement

---

## Session History

### Completed This Session

1. Header schema v2.1 implemented (expanded status lifecycle)
2. header-validate Rust command built
3. 17 documents migrated to v2.1
4. GIT_WORKFLOW_CONTRACT.md created
5. CORTEX_GUARDIAN_SPEC.md created
6. Branch protection enabled on GitHub
7. PR workflow tested (PR #1, PR #2 - both merged)
8. Claude workflow patterns documented (proactive monitoring)
9. VISION.md created (mission and soul of Xonaix)

### Key Discussions Captured

1. **Thanksgiving 2025 Motivation**
   - AWS, Microsoft, Cloudflare all failed in one week
   - Bad configs, unreviewed changes, fat-fingers, rogue actions
   - Preventable human errors that slipped through

2. **The Mission**
   - "Shield the world from viruses, zero-days, and horrible commits"
   - Technology keeps people safe, happy, and protected

3. **The Brand Promise**
   - "The company the world trusts to do it right"
   - Library is the model for all repos

4. **The Scale Vision**
   - From dad at home with OpenWRT + Cortex-Lite to the DOD
   - Same principles, same rigor, scaled appropriately

5. **Trust Stack (5 Pillars)**
   - Everything requires approval
   - Everything is signed
   - Everything is on the ledger (ZeroPoint)
   - Even Founder follows rules
   - Cortex analyzes before merge

6. **Override Rules**
   - 1-time only
   - Must be explicit in conversation
   - Becomes part of audit trail
   - Even Founder follows this

7. **Future: OpenWRT**
   - Refactored by Xonaix Code-Core and Forge as Rust
   - Protected by Cortex-Lite

---

## Roadmap Documents

| Document | Purpose |
|----------|---------|
| VISION.md | Mission, motivation, soul of Xonaix |
| CORTEX_GUARDIAN_SPEC.md | AI-powered commit analysis |
| MULTI_PARTY_GOVERNANCE_SPEC.md | Multi-party approval for Blades |
| ZEROPOINT_INTEGRATION_SPEC.md | Cryptographic ledger integration |
| SESSION_CONTEXT.md | This document - Claude recovery |

---

## Architecture Summary



---

## Documents Needing Signature

19 documents in "approved" status awaiting XCLib/ZeroPoint:

- 3 governance contracts
- 2 meta documents  
- 14 language standards (Rust, TypeScript, Python, C, C++, etc.)

---

## Claude Workflow Patterns

### Pattern: Wait for Human Action
1. Create PR, wait for CI
2. Notify human: "PR ready for review"
3. Set todo to "waiting on human"
4. Watch for merge (poll in background)
5. When human merges, proceed automatically
6. Verify post-merge CI, clean up, report

### Pattern: Context Preservation
- Regularly update SESSION_CONTEXT.md
- Capture key discussions in roadmap docs
- Don't assume context persists between sessions

### Pattern: Override Handling
- Override must be explicit in conversation
- One-time only (not standing permission)
- Record in commit message for audit trail

---

## Recovery Instructions

If Claude needs to be restored:

1. Read this file first
2. Read VISION.md for mission context
3. Read CLAUDE.md for operational rules
4. Check git log for recent activity
5. Check GitHub for open PRs
6. Resume from current state

---

*Session Context Document*
*Canonical: `xonaix-library::specs/_roadmap/SESSION_CONTEXT.md`*
*Updated: 2025-12-31*
