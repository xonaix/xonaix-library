---
schema: "xonaix-document-header"
schema_version: "2.1"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/wasm/STANDARDS_WASM.md"
unit_id: "library/standards/wasm"
title: "WebAssembly Standard"
document_type: "standard"
language: "en"

# --- Version ---
version: "XLIB-1.0.0"
baseline: null
status: "approved"

# --- Classification ---
trust_class: "L1/L2"
classification: "internal"
compliance: []

# --- Ownership ---
owner: "Founder"
approved_by: "Founder"
authority_tier: "T2"

# --- Authority ---
authority:
  repo: "xonaix-specs"
  ref: "THE_XONAIX_WAY.md"
  version: null

# --- Relationships ---
depends_on: []
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
last_updated: "2025-12-31T22:26:48Z"
---

# WebAssembly Standard

WASM is the primary target for Cortex Nano and Lite agents requiring sovereign, offline, or resource-constrained execution.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

---

## Principle Mapping

| Principle | WASM Implementation |
|-----------|---------------------|
| 1. Truth Over Convenience | Deterministic execution only — no host variability |
| 2. Explicit Over Implicit | All capabilities must be injected or attested |
| 3. Immutable Record | No mutable global state across restarts |
| 4. Zero Trust | No direct OS or host access without explicit interface |
| 5. Self-Enforcement | Panic=abort on violation |
| 6. Constitutional Supremacy | Float ban, error boundary, capability classes enforced |
| 7. Adversarial Design | Agents assumed capable of exploiting any ambiguity |
| 8. Operational Autonomy | Offline-first, no network by default |
| 9. Nothing Lost, Ever | All state changes durable via host-provided store |
| 10. Carbon Accountable | Size budgets enforced |

---

## Target & Toolchain

| Item | Requirement |
|------|-------------|
| Target | `wasm32-unknown-unknown` (MUST) |
| Rust Flags | `-C opt-level=s` or `z`, `-C lto=yes` |
| Toolchain | Pinned via `rust-toolchain.toml` (exact version) |
| Post-Processing | `wasm-opt` + `wasm-snip` for size reduction |

---

## Core Requirements (MUST)

### 1. no_std Baseline

```rust
#![no_std]
#![no_main]
```

Standard library forbidden except explicit allows via features.

### 2. Panic Strategy

```toml
[profile.release]
panic = "abort"
```

Unwind forbidden — `panic = abort` required. This implements fail-closed semantics.

### 3. Memory & Allocation

- Static memory limit enforced by build
- No uncontrolled heap growth
- Allocation only via explicit, bounded allocator (injected or feature-gated)

### 4. Determinism Guarantees

| Requirement | Enforcement |
|-------------|-------------|
| **Float prohibition** | `#![forbid(float)]` or equivalent lint |
| No wall-clock time | `Instant::now`, `Date.now` forbidden |
| No OS entropy | Randomness MUST be injected deterministically |
| No locale-dependent operations | All string handling explicit |

### 5. Threads & Concurrency

- Threads forbidden by default
- Atomics and bulk memory allowed only with explicit justification
- Single-threaded execution model assumed

### 6. Cryptographic Operations

| Mode | Signing | Verification | Requirement |
|------|---------|--------------|-------------|
| Nano | Forbidden | Allowed | Verification only |
| Lite | With attestation | Allowed | Host capability attestation required |

- All crypto MUST use XCLib WASM bindings
- Host-provided crypto interface preferred
- No direct crypto primitive implementations

---

## WASI Interface (Explicit Allowlist)

WASM modules MUST use a **minimal, explicitly allowed interface**. The host provides only the following capabilities, all mediated through attested interfaces.

### Allowed Host Imports

| Category | Import | Purpose | Nano | Lite |
|----------|--------|---------|------|------|
| **Storage** | `xonaix_store_read` | Read from durable store | | |
| | `xonaix_store_write` | Write to durable store | | |
| | `xonaix_store_commit` | Commit pending writes | | |
| **Time** | `xonaix_monotonic_nanos` | Deterministic monotonic clock | | |
| **Randomness** | `xonaix_get_random` | Injected entropy | | |
| **Capability** | `xonaix_capability_check` | Runtime capability verification | | |
| | `xonaix_verify_proof` | Proof verification (XCLib) | | |
| **Observability** | `xonaix_log` | Structured logging | | |
| **Sync** | `xonaix_sync_complete` | Signal completion to host | | |

### Forbidden WASI Capabilities

MUST NOT import:
- Filesystem (`wasi:filesystem/*`)
- Network sockets (`wasi:sockets/*`)
- Environment variables (`wasi:environment/*`)
- Wall clocks (`wasi:clocks/wall-clock`)
- Process control (`wasi:proc/*`)
- Pre-opens or command-line arguments

### Interface Signatures

```rust
// Durable storage (Principle 9 - Nothing Lost)
extern "C" {
    fn xonaix_store_read(
        key_ptr: *const u8, 
        key_len: u32, 
        out_buf_ptr: *mut u8, 
        out_buf_len: u32
    ) -> i32;
    
    fn xonaix_store_write(
        key_ptr: *const u8, 
        key_len: u32, 
        value_ptr: *const u8, 
        value_len: u32
    ) -> i32;
    
    fn xonaix_store_commit() -> i32;
}

// Deterministic time
extern "C" {
    fn xonaix_monotonic_nanos() -> u64;
}

// Injected randomness
extern "C" {
    fn xonaix_get_random(out_ptr: *mut u8, out_len: u32) -> i32;
}

// Capability attestation
extern "C" {
    /// Returns 0 if capability granted, non-zero if denied
    fn xonaix_capability_check(required_class: u32) -> i32;
}

// Structured logging
extern "C" {
    /// Levels: 1=ERROR, 2=WARN, 3=INFO, 4=DEBUG
    fn xonaix_log(level: u32, msg_ptr: *const u8, msg_len: u32);
}

// Sync completion
extern "C" {
    fn xonaix_sync_complete();
}
```

### Interface Philosophy

- The host is the sole source of truth for time, randomness, storage, and capability
- WASM module is pure computation + verification
- All side effects mediated through explicit, attested host calls
- No implicit capabilities

---

## Size Budgets

| Mode | Max Size (gzipped) | Max Size (uncompressed) | Enforcement |
|------|-------------------|-------------------------|-------------|
| Nano | ≤ 512 KiB | ≤ 1.5 MiB | CI failure |
| Lite | ≤ 2 MiB | ≤ 6 MiB | CI failure |

Size checks are mandatory in CI. Builds exceeding budget MUST fail.

---

## Build & Verification Requirements

| Requirement | Specification |
|-------------|---------------|
| Reproducible builds | Hash matches across toolchains |
| Artifact signing | Via host (never inside WASM) |
| SBOM | Generated and signed |
| Provenance | Build attestation recorded |
| Import validation | CI verifies no forbidden imports |

Cross-reference: CODE_CORE §5 (Build Verification)

---

## Testing Requirements

| Test Type | Requirement |
|-----------|-------------|
| Unit | 100% coverage on public API |
| Property-Based | Determinism across injected inputs |
| Fuzz | Import boundary fuzzing |
| Size | Automated budget check |
| Conformance | XCLib verification vectors |
| Determinism | Rebuild and hash comparison |

---

## Security Considerations

- WASM sandbox assumed hostile — capability confinement mandatory
- No dynamic code loading or self-modification
- All external data validated against canonical profile
- Panic on any capability violation (fail-closed)
- No timing oracles — constant-time where applicable

---

## CI Pipeline Example

```yaml
name: WASM Build

steps:
  - name: Build WASM
    run: |
      cargo build --target wasm32-unknown-unknown --release
      
  - name: Optimize
    run: |
      wasm-opt -Oz target/wasm32-unknown-unknown/release/agent.wasm \
        -o optimized.wasm
        
  - name: Size Check (Nano)
    run: |
      size=$(gzip -c optimized.wasm | wc -c)
      if [ $size -gt 524288 ]; then
        echo "Size budget exceeded: $size > 512KiB"
        exit 1
      fi
      
  - name: Validate Imports
    run: |
      wasm-objdump -j Import optimized.wasm | \
        grep -v "xonaix_" && exit 1 || true
        
  - name: Determinism Test
    run: |
      cargo build --target wasm32-unknown-unknown --release
      diff optimized.wasm optimized2.wasm
      
  - name: Sign Artifact
    run: |
      # Host-side XCLib hybrid signature
      xonaix-sign --artifact optimized.wasm --output signed.wasm
```

---

## Reference Implementation

See CORTEX_NANO_EXAMPLE.rs for a minimal compliant agent demonstrating:
- `no_std`, `no_main`, `panic = abort`
- Capability check before processing
- Durable storage with commit
- Deterministic helpers only
- No floats, no threads, no OS calls

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| B-5.8.5 | 2025-12-22 | Initial reserved standard for Cortex Nano/Lite. Determinism, size, crypto constraints. Detailed host interface. Alignment with XCLIB_SPEC and CORTEX_PROTOCOL. |

---

---

*Xonaix Library Standard*
*Canonical: `xonaix-library::specs/standards/wasm/STANDARDS_WASM.md`*
*Authority: `xonaix-specs::THE_XONAIX_WAY.md`*
