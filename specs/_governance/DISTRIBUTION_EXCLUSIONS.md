# DISTRIBUTION EXCLUSIONS

Scope: Xonaix Library
Authority Level: Binding (CI-Enforced)
Status: Active

The following paths are non-distributable. They must not exist in shipped scope.

---

## Excluded Paths

| Path | Reason |
|------|--------|
| `_deprecated/` | Deprecated baselines; historical reference only |
| `_reference/` | External ecosystem reference; not authoritative |
| `drafts/` | Work-in-progress content; not audit-ready |
| `quarantine/` | Flagged content pending review |
| `tools/legacy/` | Deprecated tooling; not part of current baseline |
| `target/` | Build artifacts; not source |

---

## _deprecated/ Policy

Deprecated baselines are:

- Excluded from default audit scope
- Excluded from distribution packages
- Excluded from baseline zip artifacts
- Treated as historical reference only
- Immutable (no modifications permitted except deprecation marker)

Inclusion in audit or distribution requires explicit Founder scope directive.

---

## _reference/ Policy

Reference materials are:

- Excluded from strict Rust-only token scans
- Excluded from default build and audit scope
- Non-authoritative (do not authorize implementation choices)

---

## Enforcement

CI must fail if any of the following conditions are true:

1. Any forbidden path (drafts/, quarantine/, tools/legacy/) exists in the repository
2. Any packaging step includes excluded paths in a release artifact
3. Any excluded path is referenced in current-scope manifests
4. _deprecated/ content appears in default packaging

---

## Machine-Parseable Policy

```yaml
policy_version: "2.0.0"
scope: "library"

excluded_from_distribution:
  - path: "_deprecated/"
    reason: "Deprecated baselines; historical reference only"
    policy: "immutable, reference-only"
  - path: "_reference/"
    reason: "External ecosystem reference; not authoritative"
    policy: "excluded from strict scans"
  - path: "drafts/"
    reason: "Work-in-progress content; not audit-ready"
    policy: "must not exist"
  - path: "quarantine/"
    reason: "Flagged content pending review"
    policy: "must not exist"
  - path: "tools/legacy/"
    reason: "Deprecated tooling; not part of current baseline"
    policy: "must not exist"
  - path: "target/"
    reason: "Build artifacts; not source"
    policy: "excluded from packaging"

enforcement:
  fail_on_forbidden_existence: true
  fail_on_packaging_inclusion: true
  fail_on_manifest_reference: true
```

---

## Enforcement Tool

Enforcement is handled by: `tools/xonaix-library-tools enforce`

Command: `xonaix-library-tools enforce --current-only`
