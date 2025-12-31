# Session Context - Header Schema v2.1 Implementation

## Current Status

Session continuation implementing v2.1 header schema with expanded status lifecycle.

## Completed Tasks

1. Created _roadmap/ folder structure
2. Created MULTI_PARTY_GOVERNANCE_SPEC.md - future multi-party approval chains for Org Blades
3. Created ZEROPOINT_INTEGRATION_SPEC.md - cryptographic ledger integration roadmap
4. Updated LIBRARY_STANDARD_HEADER_CONTRACT.md to v2.1 with:
   - Expanded status lifecycle: draft -> internal_review -> proposed -> approved -> sealed -> deprecated -> superseded
   - Integrity requirements by status (approved = hash, sealed = hash + signature)
   - Governance debt concept (approved without hash = warning, not failure)
   - Migration guide from v2.0 to v2.1

## In Progress

5. Building header-validate Rust command
   - Added serde_yaml to Cargo.toml
   - Creating header.rs module for validation
   - Will validate all headers against v2.1 schema
   - Checks: schema, status, integrity requirements, timestamps, etc.

## Remaining Tasks

6. Build governance-report Rust command
7. Update CI to run new validations
8. Migrate all documents to v2.1 headers
9. Regenerate governance manifest
10. Run doctor and verify
11. Commit and push changes
12. Verify CI is green

## Key v2.1 Changes

Status Lifecycle:
- draft: Work in progress
- internal_review: Ready for team review
- proposed: Submitted for formal approval
- approved: Content locked, awaiting seal (content_hash required)
- sealed: Signed and immutable (signature required)
- deprecated: Being phased out (maintains seal)
- superseded: Replaced by another doc (maintains seal)

Integrity Requirements:
- draft/internal_review/proposed: No integrity fields required
- approved: content_hash SHOULD be present (warn if missing)
- sealed+: content_hash AND signature MUST be present (error if missing)

## Files Modified This Session

- specs/_roadmap/MULTI_PARTY_GOVERNANCE_SPEC.md (new)
- specs/_roadmap/ZEROPOINT_INTEGRATION_SPEC.md (new)
- specs/_governance/LIBRARY_STANDARD_HEADER_CONTRACT.md (updated to v2.1)
- tools/xonaix-library-tools/Cargo.toml (added serde_yaml)
- tools/xonaix-library-tools/src/header.rs (in progress)

## To Resume

Continue with creating header.rs module for header-validate command.
The module should:
1. Parse YAML frontmatter from .md files
2. Validate against v2.1 schema
3. Check status-specific integrity requirements
4. Report errors and warnings
5. Exit with failure if any errors found
