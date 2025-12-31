---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "xonaix-library"
path: "specs/_roadmap/ZEROPOINT_INTEGRATION_SPEC.md"
unit_id: "roadmap/zeropoint-integration"
title: "ZeroPoint Integration Specification"
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
  - repo: "xonaix-library"
    ref: "_roadmap/MULTI_PARTY_GOVERNANCE_SPEC.md"
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

# ZeroPoint Integration Specification

**Status:** ROADMAP - Future specification for cryptographic ledger integration
**Target:** Header Schema v3.0+ with ZeroPoint anchoring
**Timeline:** When ZeroPoint ledger is operational

---

## 1. Purpose

ZeroPoint is the Xonaix cryptographic proof ledger. It provides:

- Immutable anchoring of document hashes
- Mathematical proof of document state at a point in time
- Cross-party verification without trust assumptions
- Audit trail that cannot be tampered with

This document describes how Xonaix documents integrate with ZeroPoint.

This is a **roadmap document** - it describes the architectural vision.

---

## 2. The Handoff Model

### 2.1 What We Build Now (v2.1)

```yaml
integrity:
  hash_alg: "SHA3-512"        # Algorithm choice
  content_hash: "..."         # We compute
  signature: "..."            # We store (local signing)
  signed_by: "..."            # Identity
  signed_at: "..."            # Timestamp
```

This is the "front end" - document-level integrity.

### 2.2 What ZeroPoint Adds (v3.0+)

```yaml
zeropoint_anchor:
  ledger_id: "xzp-main"                    # Which ledger
  block_height: 12345                       # Block number
  merkle_root: "..."                        # Root hash of block
  merkle_proof: ["...", "...", "..."]       # Proof path
  anchored_at: "2025-12-31T23:30:00Z"       # When anchored
  anchor_ref: "xzp://main/12345/doc/abc"    # Canonical reference
```

This is the "back end" - ledger-level proof.

---

## 3. Cryptographic Model

### 3.1 Hash Algorithm

All Xonaix documents use **SHA3-512** for content hashing.

Why SHA3-512:
- NIST standard (FIPS 202)
- Resistant to length-extension attacks
- 512-bit output provides long-term security
- Different internal structure from SHA-2 (defense in depth)

### 3.2 Signature Algorithm

All Xonaix signatures use **Ed25519** (EdDSA with Curve25519).

Why Ed25519:
- Fast signing and verification
- Small signatures (64 bytes)
- Deterministic (no random number generator needed)
- Resistant to side-channel attacks
- Widely supported

### 3.3 Content Hash Computation

The content hash is computed over:

1. Document content AFTER the closing `---` of the header
2. Normalized to LF line endings
3. Trimmed of leading/trailing whitespace
4. UTF-8 encoded

The header is EXCLUDED from the content hash. This allows header updates (like adding anchor info) without invalidating the content hash.

---

## 4. ZeroPoint Ledger Model

### 4.1 Block Structure

```
Block N
├── block_height: N
├── timestamp: "..."
├── prev_hash: hash(Block N-1)
├── merkle_root: hash(all documents in block)
└── documents:
    ├── doc_1: { hash, signature, metadata }
    ├── doc_2: { hash, signature, metadata }
    └── doc_N: { hash, signature, metadata }
```

### 4.2 Merkle Proof

To verify a document is in a block:

1. Compute document hash
2. Obtain merkle_proof from ledger
3. Walk proof path to compute root
4. Compare computed root to block's merkle_root
5. If match, document was in block at that time

### 4.3 Verification Without Trust

Anyone can verify:
- Download the block header (public)
- Compute the merkle proof
- No need to trust Xonaix or ZeroPoint
- Mathematical proof only

---

## 5. Anchoring Workflow

### 5.1 Document Sealing (Current)

```
1. Author completes document
2. Status set to "approved"
3. Content hash computed (SHA3-512)
4. Signature computed (Ed25519)
5. Status set to "sealed"
6. Document is locally sealed
```

### 5.2 ZeroPoint Anchoring (Future)

```
7. Sealed document submitted to ZeroPoint
8. ZeroPoint batches documents into block
9. Block is finalized
10. Anchor info returned to document
11. zeropoint_anchor section populated
12. Document is ledger-anchored
```

### 5.3 Verification (Future)

```
1. Obtain document
2. Verify local signature (Ed25519)
3. Verify content hash matches content
4. Obtain ZeroPoint anchor
5. Verify merkle proof against block
6. Document integrity proven
```

---

## 6. Header Evolution

### 6.1 Current (v2.1)

```yaml
integrity:
  hash_alg: "SHA3-512"
  content_hash: null          # Filled when sealed
  signature: null             # Filled when sealed
  signed_by: null
  signed_at: null
```

### 6.2 Future (v3.0)

```yaml
integrity:
  hash_alg: "SHA3-512"
  content_hash: "a1b2c3..."
  signature: "x1y2z3..."
  signed_by: "Founder"
  signed_at: "2025-12-31T23:00:00Z"

zeropoint_anchor:
  ledger_id: "xzp-main"
  block_height: 12345
  merkle_root: "m1n2o3..."
  merkle_proof:
    - "p1..."
    - "p2..."
    - "p3..."
  anchored_at: "2025-12-31T23:30:00Z"
  anchor_ref: "xzp://main/12345/doc/abc123"
```

---

## 7. XCLib Integration

All cryptographic operations MUST use XCLib (Xonaix Cryptographic Library):

| Operation | XCLib Module |
|-----------|--------------|
| SHA3-512 hashing | `xclib::hash::sha3_512` |
| Ed25519 signing | `xclib::sign::ed25519` |
| Ed25519 verification | `xclib::verify::ed25519` |
| Merkle proof computation | `xclib::merkle::prove` |
| Merkle proof verification | `xclib::merkle::verify` |

No other cryptographic libraries permitted for governance operations.

---

## 8. Trust Model

### 8.1 What We Trust

- The mathematics (hash functions, signatures)
- Our own key management
- The ZeroPoint ledger protocol

### 8.2 What We Don't Trust

- Network transport (always verify)
- Other parties' claims (require proofs)
- Timestamps without anchoring (use ledger time)

### 8.3 Constitutional Conformance

All documents reference the Constitution and Zero Point.
When sealed, the hashes of these foundational documents are recorded.
This creates an unbroken chain from any document to the constitutional foundation.

---

## 9. Implementation Phases

### Phase 1: Local Integrity (Current - v2.1)
- Content hashing
- Local signatures
- Status lifecycle
- Governance reports

### Phase 2: Signature Tooling (Next)
- XCLib integration
- Key management
- Signing ceremonies
- Verification commands

### Phase 3: ZeroPoint Integration (Future)
- Ledger client
- Anchoring workflow
- Merkle proof handling
- Cross-party verification

---

## 10. Open Questions

- How do we handle key rotation?
- How do we handle ledger forks (if any)?
- What's the block interval for ZeroPoint?
- How do we handle offline verification?
- How do we handle document updates after anchoring?

These will be addressed as ZeroPoint is built.

---

*Roadmap Specification*
*Canonical: `xonaix-library::specs/_roadmap/ZEROPOINT_INTEGRATION_SPEC.md`*
*Status: Draft - Future Implementation*
