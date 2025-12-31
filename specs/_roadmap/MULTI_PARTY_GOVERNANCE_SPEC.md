---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "xonaix-library"
path: "specs/_roadmap/MULTI_PARTY_GOVERNANCE_SPEC.md"
unit_id: "roadmap/multi-party-governance"
title: "Multi-Party Governance Specification"
document_type: "standard"
language: "en"

# --- Version ---
version: "XLIB-0.1.0"
baseline: null
status: "draft"

# --- Classification ---
trust_class: null
classification: "internal"
compliance: []

# --- Ownership ---
owner: "Founder"
approved_by: null
authority_tier: "T2"

# --- Authority ---
authority:
  repo: "xonaix-library"
  ref: "LIBRARY_STANDARD_HEADER_CONTRACT.md"
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

# Multi-Party Governance Specification

**Status:** ROADMAP - Future specification for Org Blade implementations
**Target:** Header Schema v3.0+
**Timeline:** When Blade specifications begin

---

## 1. Purpose

This document captures the architectural vision for multi-party governance in Xonaix Org Blades. While xonaix-library uses a single-party model (Xonaix only), customer Blade deployments require multi-party approval chains.

This is a **roadmap document** - it describes what we will build, not what exists today.

---

## 2. Use Cases

### 2.1 Customer Engagement

A bank (Bank X) contracts Xonaix to refactor their legacy system. Before work begins:

- Xonaix authors specifications
- Bank X reviews and approves
- Both parties sign off
- Work proceeds only when all required approvals are in place

### 2.2 Regulatory Compliance

A healthcare org requires:

- Xonaix approval (author)
- Customer approval (owner)
- Compliance officer approval (regulatory)
- External auditor approval (third-party verification)

### 2.3 Multi-Vendor Collaboration

Multiple vendors collaborate on a system:

- Vendor A owns Module 1
- Vendor B owns Module 2
- Customer owns the integration spec
- Each party signs their respective documents

---

## 3. Proposed Header Extension (v3.0)

```yaml
# --- Approval Chain ---
approvals:
  - party: "Xonaix"
    role: "author"
    status: "sealed"
    approved_by: "Founder"
    approved_at: "2025-12-31T22:00:00Z"
    signature: "xsig_abc123..."
    
  - party: "Bank X"
    role: "customer"
    status: "sealed"
    approved_by: "CTO"
    approved_at: "2025-12-31T23:00:00Z"
    signature: "xsig_def456..."
    
  - party: "External Auditor"
    role: "auditor"
    status: "pending"
    approved_by: null
    approved_at: null
    signature: null

# --- Approval Requirements ---
approval_requirements:
  minimum_approvals: 2
  required_roles: ["author", "customer"]
  optional_roles: ["auditor", "regulator"]
```

---

## 4. Approval Statuses

| Status | Meaning |
|--------|---------|
| `pending` | Awaiting review |
| `in_review` | Currently being reviewed |
| `approved` | Approved, signature pending |
| `sealed` | Approved and signed |
| `rejected` | Rejected with reason |
| `withdrawn` | Withdrawn by approver |

---

## 5. Signature Model

Each approval includes:

- `signature`: Cryptographic signature of document content
- Links to ZeroPoint ledger for proof anchoring
- Timestamp of signing
- Identity of signer

Signatures are computed over:
- Document content (excluding header)
- All prior approval signatures (chain integrity)

---

## 6. Engagement Registry

For tracking customer engagements:

```json
{
  "engagements": {
    "bank-x-legacy-refactor": {
      "customer": "Bank X",
      "status": "active",
      "created": "2025-12-31T00:00:00Z",
      "contract_ref": "CONTRACT-2025-001",
      "required_specs": [
        "library/standards/rust",
        "library/standards/typescript",
        "blade/bank-x/api-schema"
      ],
      "approval_matrix": {
        "library/standards/rust": {
          "required": ["xonaix"],
          "optional": ["customer"]
        },
        "blade/bank-x/api-schema": {
          "required": ["xonaix", "customer"],
          "optional": ["auditor"]
        }
      },
      "readiness": {
        "all_required_sealed": false,
        "blocking_documents": ["blade/bank-x/api-schema"]
      }
    }
  }
}
```

---

## 7. Integration Points

### 7.1 With ZeroPoint

- All sealed documents anchored to ZeroPoint ledger
- Merkle proofs for audit trails
- Cross-party verification without trust assumptions

### 7.2 With Org Blade

- Engagement registry lives in customer's Blade
- Xonaix standards referenced, not copied
- Customer-specific specs in their Blade

### 7.3 With Forge

- Forge reads approval status before generating code
- Blocks generation if required specs not sealed
- Audit log of which specs were used for generation

---

## 8. Implementation Notes

This specification will be implemented when:

1. Blade specifications are being authored
2. First customer engagement requires multi-party approval
3. ZeroPoint ledger is operational

Until then, xonaix-library uses the single-party v2.1 model.

---

## 9. Open Questions

- How do we handle approval expiration?
- How do we handle document updates after multi-party seal?
- How do we handle disputes between parties?
- How do we handle key rotation for signers?

These will be addressed as we build toward v3.0.

---

*Roadmap Specification*
*Canonical: `xonaix-library::specs/_roadmap/MULTI_PARTY_GOVERNANCE_SPEC.md`*
*Status: Draft - Future Implementation*
