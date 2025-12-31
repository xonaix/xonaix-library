# LIBRARY SEALING CONTRACT

Status: BINDING
Authority Level: Governance Contract
Applies to: Xonaix Library Repository
Authority: XONAIX_SELF_GOVERNANCE_CONTRACT.md

---

## 1. Purpose

This contract defines the authoritative sealing, batching, and verification model for the Xonaix Library.

The Xonaix Library exists to provide durable, reference-only standards used by Xonaix systems, agents, and humans.
Its integrity must be preserved independently of any product, baseline, or deployment.

This contract ensures:

- Deterministic provenance of standards
- Minimal resealing surface
- Incremental evolution without document sprawl
- Verifiability in connected and air-gapped environments

---

## 2. Scope

This contract governs:

- All standards and mini-standards in the Xonaix Library
- All Library releases (LIB-X.Y.Z)
- All sealing ceremonies for Library artifacts
- All verification requirements for sealed Library releases

This contract does not govern:

- Product specifications (B5, UX, Nexus, Web)
- Runtime code
- Enterprise permissions
- Cryptographic key management implementation details

---

## 3. Authority Model

### 3.1 Chain of Custody

Trust Aggregation (bottom-up):

```
File Hashes
  -> Unit Manifests
    -> Library BOM
      -> Signature
```

Verification (top-down):

```
Signature
  -> Library BOM
    -> Unit Manifests
      -> File Hashes
```

The Library BOM is the authoritative aggregation point for sealing.

---

## 4. Units

### 4.1 Definition

A Library Unit is a discrete standard or mini-standard with:

- A declared unit_id
- A semantic version
- A unit-scoped manifest enumerating authoritative files

Each unit is independently versioned and traceable.

### 4.2 Unit Identity

- unit_id is mandatory
- Path-derived identity is forbidden
- Identity is permanent once published
- Deprecated units retain identity but must be explicitly marked

---

## 5. Library BOM (Bill of Materials)

### 5.1 Definition

A Library BOM is a snapshot artifact that enumerates:

- Library version (LIB-X.Y.Z)
- All included unit_ids
- Unit versions
- Hash of each unit's manifest

The BOM defines exactly what constitutes a Library release.

### 5.2 Authority

The BOM is the only artifact that is sealed.

Sealing the BOM implicitly seals:

- All referenced unit manifests
- All referenced files via hash transitively

---

## 6. Sealing Ceremonies

### 6.1 Initial Sealing (One-Time Exception)

The first Library release (LIB-1.0.0):

- MAY include all existing standards
- SHALL be sealed as a single batch
- Produces one signature
- Establishes the initial trust anchor

This exception is permitted once only.

### 6.2 Subsequent Sealing (Standard Rule)

For all future Library releases:

- Only units that are new or version-changed are included
- Unchanged units are referenced, not resealed
- A new BOM is generated
- One signature per release

This model mirrors deterministic version control:

```
stage -> batch -> seal
```

---

## 7. Batching Semantics

### 7.1 What Constitutes a Batch

A batch consists of:

- One or more unit version changes
- Supporting files necessary for those units
- A regenerated BOM

### 7.2 Forbidden Practices

The following are forbidden:

- Resealing unchanged units
- Partial sealing of a BOM
- Sealing individual files outside unit context
- Silent inclusion of units in a BOM

---

## 8. Verification Requirements

A sealed Library release is valid only if:

- The BOM hash matches the sealed artifact
- All referenced unit manifests exist
- All unit manifest hashes match file contents
- No extra or missing files exist
- The signer is authorized per AUDIT_CONTRACT.md

Verification MUST fail on any discrepancy.

---

## 9. Versioning

### 9.1 Library Versioning

Library releases follow semantic versioning:

```
LIB-MAJOR.MINOR.PATCH
```

Library versioning is independent of:

- B5
- UX
- Core
- Nexus
- Web

### 9.2 Unit Versioning

Each unit maintains its own version and changelog.

Unit version changes do not imply Library version changes unless included in a BOM.

---

## 10. Change Logs

### 10.1 Unit Change Logs

Each unit SHALL maintain a CHANGELOG.md describing:

- What changed
- Why it changed
- Compatibility notes

### 10.2 Library Change Log

Each Library release SHALL include a Library-level CHANGELOG.md summarizing:

- Included unit changes
- Added units
- Deprecated units

---

## 11. Air-Gapped and Sovereign Verification

The Library sealing model is designed to support:

- Offline verification
- Air-gapped environments
- Sovereign custody

No network access is required to verify a sealed Library release.

---

## 12. Enforcement

This contract is:

- Binding upon commit
- Enforced by CI
- Required by governance manifests
- Audited under Rainbow Team procedures

Any violation constitutes governance debt and MUST be remediated before further releases.

---

## 13. Amendments

Changes to this contract require:

- Explicit Founder signoff
- Regeneration of governance manifests
- Full verification pass

No silent amendments are permitted.

---

## 14. Final Assertion

The Xonaix Library is not a convenience artifact.
It is a durable authority surface.

Sealing is not ceremony.
It is truth preservation.

---

*Governance Contract*
*Canonical location: `specs/_governance/LIBRARY_SEALING_CONTRACT.md`*
