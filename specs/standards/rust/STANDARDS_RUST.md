---
title: "Rust Language Standard"
unit_id: "library/standards/rust"
standard_type: "standard"
version: "1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# The Xonaix Way
## Language Standards: Rust

**Version:** B-5.8.5
**Status:** Active (Primary Language)
**Core-Compatible:** 5.7.0
**Trust Class:** L1 (restricted) / L2 (full)
**Created:** December 2025
**Last Reviewed:** December 2025

*This document implements The Xonaix Way B-5.8.5 principles for Rust projects.*

---

## Document Info

| Field | Value |
|-------|-------|
| Language | Rust |
| Status | **Active** (Primary Language) |
| Version | B-5.8.5 |
| Core-Compatible | 5.7.0 |
| Trust Class | L1 (restricted) / L2 (full) |
| Created | December 2025 |
| Last Reviewed | December 2025 |

**Prerequisites:** Read [THE_XONAIX_WAY.md](../THE_XONAIX_WAY.md) first. This document assumes familiarity with the 9 Principles.

**Language Policy:** Rust is the **primary language** for all Xonaix development. All core systems, governance code, security-critical components, and infrastructure MUST be written in Rust.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L1 (restricted subset) / L2 (full) |
| Classification | Constitutional / Deterministic |

### L1 Restricted Subset (Constitutional)

The following paths are L1 (Constitutional) and may touch canonical data, cryptography, and ledger:

- All code in `xonaix-xclib` crate
- All code in `xonaix-canonical` crate
- All signature/verification paths in `xonaix-ledger`
- All code handling private keys
- All code generating or verifying proofs

**L1 Requirements:**
- `panic = abort` in release builds
- Pass timing tests (dudect) for crypto operations
- Security audit required before production
- Capability attestation before crypto operations

### L2 Full (Deterministic Compute)

All other Rust code is L2 (Deterministic Compute). L2 code:
- May perform computation and transformation
- MUST delegate crypto to L1 code (XCLib)
- MUST NOT perform direct canonicalization without XCLib

### Trust Class Declaration

Every crate MUST declare its trust class:

```toml
# Cargo.toml
[package.metadata.xonaix]
trust_class = "L1"  # or "L2"
```

---

## XCLib Integration

**Authority:** Founder Ruling 2025-003(c) — Attested Capability

### XCLib Exclusivity Rule

All cryptographic operations in Xonaix Rust code MUST use XCLib:

| Operation | Required Module |
|-----------|-----------------|
| Canonicalization | `xonaix_xclib::canonical::*` |
| Hashing | `xonaix_xclib::hash::*` |
| Signing | `xonaix_xclib::sign::*` |
| Verification | `xonaix_xclib::verify::*` |

### Prohibited Direct Usage

Direct use of these crates outside XCLib is **FORBIDDEN**:

```toml
# deny.toml
[bans]
deny = [
  { name = "sha3", wrappers = ["xonaix-xclib"] },
  { name = "ed25519-dalek", wrappers = ["xonaix-xclib"] },
  { name = "pqcrypto-mldsa", wrappers = ["xonaix-xclib"] },
  { name = "blake3", wrappers = ["xonaix-xclib"] },
]
```

### Exception Process

Any deviation from XCLib exclusivity requires:
1. XONAIX_DEVIATION marker in code
2. Ledger-recorded approval (Founder + Council)
3. Security review

---

## Numeric Policy

**Authority:** Founder Ruling 2025-003(a) — Determinism

### Float Prohibition

Floating-point types are **PROHIBITED** in canonical/governance paths:

```rust
// FORBIDDEN in canonical paths
let rate: f64 = 1.5;        // NO
let amount: f32 = 100.0;    // NO

// REQUIRED representations
let rate_bps: u32 = 150;    // 150 basis points = 1.5%
let amount_cents: i64 = 10000;  // $100.00
let decimal: String = "1.5".to_string();  // String representation
```

### Enforcement

```rust
// lib.rs for L1 crates
#![forbid(clippy::float_arithmetic)]
#![forbid(clippy::float_cmp)]
```

### Permitted Numeric Types

| Use Case | Type | Example |
|----------|------|---------|
| Currency | `i64` (smallest unit) | `10000` = $100.00 |
| Percentages | `u32` (basis points) | `150` = 1.5% |
| Ratios | `Rational` struct | `Rational { num: 3, den: 2 }` |
| Arbitrary precision | `String` or `BigInt` | `"3.14159265358979"` |

---

## Capability & Posture Handling

**Authority:** Constitution Article I, §4 — Zero Trust

### Capability Verification Pattern

```rust
use xonaix_xclib::{CapabilityClass, Posture, CapabilityError};

/// Sign governance action with capability check
pub fn sign_governance_action(
    action: &GovernanceAction,
    posture: &Posture,
) -> Result<Signature, SignError> {
    // MUST verify capability before crypto operation
    posture.require_capability(CapabilityClass::SoftwareClassA)
        .map_err(|e| SignError::CapabilityDenied(e))?;
    
    // Proceed with signing only after capability verified
    xclib::sign(action)
}
```

### Prohibited Patterns

```rust
// FORBIDDEN: No capability check
fn sign_action(action: &Action) -> Signature {
    xclib::sign(action).unwrap()  // Missing posture verification!
}

// FORBIDDEN: Inferring authority from context
fn is_admin(user: &User) -> bool {
    user.role == "admin"  // Role claim, not cryptographic proof!
}
```

### Required Pattern

```rust
// CORRECT: Verify cryptographic authority
fn has_authority(
    actor: &Actor,
    required_scope: AuthorityScope,
) -> Result<bool, AuthError> {
    let proof = actor.authority_proof()?;
    let verified = xclib::verify_authority_proof(&proof, required_scope)?;
    Ok(verified)
}
```

---

## Generated Code Accountability

**Authority:** Constitutional Actor Model

### Requirements

Code generated by Forge, agents, or templates MUST:

1. **Declare Trust Class** in Cargo.toml metadata
2. **Declare Authority Scope** of operations performed
3. **Pass All CI Checks** — same as human-written code
4. **Include Provenance** — which agent/template/version generated it

### Provenance Marker

```rust
// XONAIX_GENERATED: forge:1.2.3:template:governance-action
// XONAIX_AUTHORITY_SCOPE: tier2:governance:read_write
// XONAIX_TRUST_CLASS: L2
```

### No Exceptions

"Generated" status does NOT:
- Reduce test coverage requirements
- Bypass linting rules
- Skip security review for L1 code
- Allow deviation from standards

---

## Principle Mapping

| Principle | Rust Implementation |
|-----------|---------------------|
| 1. Correct Over Fast | Strong type system, `Result` types, comprehensive tests, `clippy::pedantic` |
| 2. Explicit Over Implicit | Explicit types at boundaries, no hidden behavior, no magic macros |
| 3. Automated Over Vigilant | Clippy, rustfmt, cargo audit, cargo-vet, cargo-geiger in CI |
| 4. Secure By Default | `#![deny(unsafe_code)]`, memory safety, validated inputs, zeroize secrets |
| 5. Composable Over Clever | Small functions, trait composition, cognitive complexity ≤10 |
| 6. Fail Loud | Result/Option types, no unwrap, explicit error propagation |
| 7. X.I. Augments, Human Decides | X.I. generates; human reviews and approves |
| 8. Future-Proof Over Trend | Stable Rust only, vetted dependencies, 10-year decisions |
| 9. Nothing Lost, Ever | `sled`/`rocksdb` for persistence, `fsync` on writes, ACK-based messaging, WORM for audit |

---

## Deviation Recording (User Choice)

If a developer chooses to violate a **MUST** requirement (e.g., using `unsafe` outside FFI), they **MUST** explicitly mark it for the Security Ledger.

**Blade Enforcement:** Blade will block the build unless this pattern is found:

```rust
// XONAIX_DEVIATION: [Reason for deviation - be specific]
// LEDGER_ACK: [User_Signature_Hash]
#[allow(unsafe_code)]
unsafe {
    // Deviating code with detailed justification
}
```

**This triggers:**
1. Warning displayed to user
2. Acknowledgment required
3. Signature captured
4. Ledger record created
5. Artifact marked as "User Choice" (not Xonaix Certified)

---

## Required Tooling

### Rust Version Alignment

Developers MUST match CI Rust version to prevent formatting drift:

```bash
rustup update stable
rustup default stable
```

Before committing, ALWAYS run:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI uses `dtolnay/rust-toolchain@stable`. Local toolchain MUST match to avoid formatting discrepancies between Rust versions.

**Rationale:** Different Rust versions may apply different formatting rules. Matching versions prevents CI failures due to formatting drift.

### Compiler Configuration

All Rust projects MUST include in `lib.rs` or `main.rs`:

```rust
// === The Xonaix Way v5.1.0: RUST ENFORCEMENT ===

// Safety: No unsafe code (see Unsafe Exception Policy)
#![deny(unsafe_code)]

// Fail Loud: No silent error swallowing
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

// Explicit: No silent drops of Results
#![deny(unused_must_use)]

// Nothing Lost: No ignored results
#![deny(unused_results)]

// Documentation: All public items documented
#![deny(missing_docs)]

// Composable: Warn on complexity
#![warn(clippy::cognitive_complexity)]
```

### CI Configuration

```toml
# .cargo/config.toml
[build]
rustflags = ["-D", "warnings"]
```

### Test Module Exceptions

Test modules MAY relax certain lints for clarity and practicality:

```rust
#[cfg(test)]
#[allow(clippy::unwrap_used)]  // Tests may panic on failure
#[allow(clippy::expect_used)]  // Tests may panic on failure
#[allow(unused_results)]       // Test assertions don't need result handling
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        let result = do_thing().unwrap();  // OK in tests
        assert_eq!(result, expected);
    }
}
```

**Rationale:**
- Test failures are expected to panic — `unwrap()` is acceptable
- Tests may need relaxed rules for environment manipulation, mocking
- Production code remains strict; only test modules relaxed

**Requirements:**
- Attributes MUST be on the test module, not individual functions
- Production code MUST NOT use these exceptions
- Integration tests in `tests/` directory follow same rules

### Linting

```bash
# CI command - catches all warnings as errors
cargo clippy --all-targets --all-features -- -D warnings
```

### Formatting

```bash
cargo fmt --check
```

### Security Scanning

```bash
# Install if needed
cargo install cargo-audit --locked

# Run audit
cargo audit
```

---

## GitHub Actions

### MUST Use

```yaml
- uses: dtolnay/rust-toolchain@stable
```

### MUST NOT Use

- `dtolnay/rust-action` — Does not exist (common typo)
- Unpinned action versions for security-critical workflows
- Actions from unverified sources

### Standard Workflow

```yaml
jobs:
  rust-checks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Format Check
        run: cargo fmt --check
        
      - name: Lint
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Test
        run: cargo test --all-features
        
      - name: Security Audit
        run: |
          cargo install cargo-audit --locked
          cargo audit
```

---

## Error Handling

### MUST: Explicit Propagation

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum ProcessError {
    Fetch { source: Box<dyn Error + Send + Sync> },
    Parse { source: Box<dyn Error + Send + Sync> },
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch { source } => write!(f, "fetch failed: {}", source),
            Self::Parse { source } => write!(f, "parse failed: {}", source),
        }
    }
}

impl Error for ProcessError {}

fn process() -> Result<Output, ProcessError> {
    let data = fetch_data()
        .map_err(|e| ProcessError::Fetch { source: Box::new(e) })?;
    let parsed = parse(data)
        .map_err(|e| ProcessError::Parse { source: Box::new(e) })?;
    Ok(parsed)
}
```

### MUST NOT: Silent Swallowing

```rust
// VIOLATION - silently ignores error
let _ = might_fail();

// VIOLATION - panics instead of propagating
let value = might_fail().unwrap();

// VIOLATION - logging is not handling
if let Err(e) = might_fail() {
    log::error!("Failed: {e}");
    // continues as if nothing happened
}
```

### Exception: Provably Unreachable

```rust
// ALLOWED - but only with proof comment
unreachable!("enum is exhaustive; all variants handled above")
```

`unreachable!()` is the ONLY permitted intentional panic. It MUST have a comment explaining why the state is impossible.

---

## Function Constraints

| Metric | Limit | Enforcement |
|--------|-------|-------------|
| Cognitive complexity | ≤10 (≤15 with justification) | Clippy + Review |
| Parameters | ≤7 | Review |
| Nesting depth | ≤4 levels | Review |
| Function size | ≤60 lines MUST, ≤30 SHOULD | Review + Tooling |

**Cognitive complexity** measures how hard code is to understand. Clippy calculates this automatically.

**Exception:** Bounded recursive traversals may exceed complexity with documented justification.

---

## Async Patterns

### MUST: Timeout on External Calls

```rust
use tokio::time::{timeout, Duration};
use reqwest::Client;
use std::error::Error;

async fn fetch_with_timeout(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    
    let response = timeout(
        Duration::from_secs(30),
        client.get(url).send()
    )
    .await
    .map_err(|_| "Request timed out")?
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    
    let body = response.text().await?;
    Ok(body)
}
```

### MUST: Bounded Iteration

```rust
const MAX_ITEMS: usize = 10_000;

async fn process_stream<S, T, E>(mut stream: S) -> Result<Vec<T>, ProcessError>
where
    S: futures::Stream<Item = Result<T, E>> + Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    use futures::StreamExt;
    
    let mut results = Vec::new();
    let mut count = 0;
    
    while let Some(item) = stream.next().await {
        if count >= MAX_ITEMS {
            return Err(ProcessError::TooManyItems { 
                limit: MAX_ITEMS,
                processed: count,
            });
        }
        
        let value = item.map_err(|e| ProcessError::StreamError { 
            source: Box::new(e) 
        })?;
        
        results.push(value);
        count += 1;
    }
    
    Ok(results)
}
```

---

## Recursion

### MUST: Bounded with Depth Limit

```rust
const MAX_DEPTH: usize = 100;  // Rationale: Typical tree depth for governance structures

fn process_tree(node: &Node, depth: usize) -> Result<(), ProcessError> {
    // Precondition assertion
    debug_assert!(depth <= MAX_DEPTH, "Depth parameter exceeds MAX_DEPTH");
    
    if depth > MAX_DEPTH {
        return Err(ProcessError::MaxDepthExceeded {
            limit: MAX_DEPTH,
            actual: depth,
        });
    }
    
    for child in &node.children {
        process_tree(child, depth + 1)?;
    }
    
    // Postcondition: all children processed
    debug_assert!(node.children.iter().all(|c| c.is_processed()));
    
    Ok(())
}
```

**Requirements:**
- MUST have explicit depth limit with documented rationale
- MUST return Result (no panic on overflow)
- MUST have tests proving termination
- MUST have assertions for pre/postconditions

---

## Unsafe Code

### Default: Forbidden

```rust
#![deny(unsafe_code)]
```

### Exception: FFI and Crypto Only

Unsafe is permitted ONLY for:
- FFI bindings to audited external libraries
- Secure memory operations (zeroization) where safe alternatives insufficient

**For cryptographic operations:** MUST use audited safe crates (see Security section). Custom unsafe crypto requires external cryptographer review.

### Exception Requirements

1. Isolated module with `#![allow(unsafe_code)]` at module level only
2. Module header: `// SAFETY-CRITICAL: UNSAFE ALLOWED - [reason]`
3. Each unsafe block has `// SAFETY:` comment explaining invariants
4. Red Team review before merge
5. Additional testing: Miri, fuzzing
6. Re-review on any modification
7. Annual review for crypto code
8. Miri in CI: `cargo +nightly miri test`

### Tracking Unsafe in Dependencies

```bash
# Install cargo-geiger
cargo install cargo-geiger --locked

# Report unsafe usage in all dependencies
cargo geiger --all-features

# CI should warn/fail on unexpected unsafe
```

**Controlled classification MUST track and justify all unsafe code in dependencies.**

---

## Security & Cryptography

### MUST: Validate All External Input

All data crossing trust boundaries MUST be validated before use.

### MUST: Use Audited Crypto

```rust
// CORRECT - audited crates
use blake3;                    // Hashing (BLAKE3)
use ed25519_dalek;             // Ed25519 signatures (classical)
use ring::digest;              // Alternative hashing
use rustls::ClientConfig;      // TLS
use zeroize::Zeroize;          // Secret cleanup

// For PQC (Phase 2-3, 2026-2027):
// use pqcrypto_mldsa::mldsa65;  // ML-DSA-65 (FIPS 204)

// VIOLATION - rolling your own
fn my_hash(data: &[u8]) -> [u8; 32] { /* ... */ }
```

### Approved Cryptographic Crates

| Purpose | Crate | Notes |
|---------|-------|-------|
| Hashing | `blake3` | Primary for hash chains |
| Hashing | `ring::digest` | Alternative (SHA-256, etc.) |
| Signatures (Tier 1-2) | `ed25519-dalek` | Classical Ed25519 |
| Signatures (Tier 3-4, 2027) | `pqcrypto-mldsa` | **Hybrid with Ed25519 REQUIRED** |
| TLS | `rustls` | No OpenSSL dependency |
| Secret cleanup | `zeroize` | Zero memory on drop |
| Key derivation | `argon2` | Password hashing |
| Encryption | `chacha20poly1305` | Authenticated encryption |

**Adding new crypto crates requires security review.**

### Post-Quantum Cryptography (PQC)

**Phased Implementation Approach:**

| Phase | Timeline | Requirement | Description |
|-------|----------|-------------|-------------|
| **Phase 1: Design** | v5.1.0 (Now) | SHOULD | Design for hybrid compatibility. Use Ed25519 with abstraction layer. |
| **Phase 2: Simulate** | 2026 | SHOULD | Prototype hybrid in staging. Validate `pqcrypto-mldsa` stability. |
| **Phase 3: Production** | 2027 | MUST (Tier 3-4) | Full hybrid for Controlled. Mandatory for Tier 3-4 governance. |

**Target Algorithm:** ML-DSA-65 (FIPS 204, formerly CRYSTALS-Dilithium3)
**Reference Crate:** `pqcrypto-mldsa`

**Current Architecture (Phase 1 - Design for Hybrid):**

```rust
use ed25519_dalek::{SigningKey, Signature as Ed25519Signature, Signer, Verifier};

/// Signature abstraction supporting future hybrid
pub struct XonaixSignature {
    /// Classical signature (always present)
    pub classical: Ed25519Signature,
    /// Post-quantum signature (None until Phase 3)
    pub pqc: Option<Vec<u8>>,
}

/// Public key abstraction supporting future hybrid
pub struct XonaixPublicKey {
    /// Classical public key (always present)
    pub classical: ed25519_dalek::VerifyingKey,
    /// Post-quantum public key (None until Phase 3)
    pub pqc: Option<Vec<u8>>,
}

/// Sign message (Phase 1: Ed25519 only, architecture ready for hybrid)
pub fn sign(message: &[u8], signing_key: &SigningKey) -> XonaixSignature {
    let classical = signing_key.sign(message);
    
    XonaixSignature {
        classical,
        pqc: None,  // Will be populated in Phase 3
    }
}

/// Verify signature (Phase 1: Ed25519 only)
/// Phase 3: If PQC present, BOTH must verify
pub fn verify(
    message: &[u8],
    signature: &XonaixSignature,
    public_key: &XonaixPublicKey,
) -> Result<(), CryptoError> {
    // Classical verification (always required)
    public_key.classical
        .verify(message, &signature.classical)
        .map_err(|_| CryptoError::Ed25519VerificationFailed)?;
    
    // PQC verification (Phase 3 - when pqc fields are populated)
    if let (Some(pqc_sig), Some(pqc_pk)) = (&signature.pqc, &public_key.pqc) {
        verify_mldsa(message, pqc_sig, pqc_pk)?;
    }
    
    Ok(())
}

/// Placeholder for Phase 3 ML-DSA verification
#[allow(unused_variables)]
fn verify_mldsa(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<(), CryptoError> {
    // Phase 3 implementation:
    // use pqcrypto_mldsa::mldsa65::{verify, PublicKey, SignedMessage};
    // let pk = PublicKey::from_bytes(public_key)?;
    // let sm = SignedMessage::from_bytes(signature)?;
    // verify(&sm, &pk).map_err(|_| CryptoError::MlDsaVerificationFailed)
    
    // Phase 1-2: No-op (PQC not yet implemented)
    Ok(())
}
```

**Phase 3 Full Hybrid Implementation (2027):**

```rust
// This code will be enabled in Phase 3 when pqcrypto-mldsa is production-ready

use ed25519_dalek::{SigningKey, Signature as Ed25519Signature, Signer, Verifier};
use pqcrypto_mldsa::mldsa65::{
    keypair as mldsa_keypair,
    sign as mldsa_sign,
    open as mldsa_verify,
    PublicKey as MlDsaPublicKey,
    SecretKey as MlDsaSecretKey,
};
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};

/// Full hybrid keypair for Tier 3-4
pub struct HybridKeyPair {
    pub classical_signing: SigningKey,
    pub classical_verifying: ed25519_dalek::VerifyingKey,
    pub pqc_secret: MlDsaSecretKey,
    pub pqc_public: MlDsaPublicKey,
}

impl HybridKeyPair {
    /// Generate new hybrid keypair
    pub fn generate() -> Self {
        let classical_signing = SigningKey::generate(&mut rand::thread_rng());
        let classical_verifying = classical_signing.verifying_key();
        let (pqc_public, pqc_secret) = mldsa_keypair();
        
        Self {
            classical_signing,
            classical_verifying,
            pqc_secret,
            pqc_public,
        }
    }
    
    /// Sign with both algorithms
    pub fn sign(&self, message: &[u8]) -> HybridSignature {
        let classical = self.classical_signing.sign(message);
        let pqc_signed = mldsa_sign(message, &self.pqc_secret);
        
        HybridSignature {
            classical,
            pqc: pqc_signed.as_bytes().to_vec(),
        }
    }
}

/// Full hybrid signature
pub struct HybridSignature {
    pub classical: Ed25519Signature,
    pub pqc: Vec<u8>,
}

/// Verify hybrid signature - BOTH must pass
pub fn verify_hybrid(
    message: &[u8],
    signature: &HybridSignature,
    classical_pk: &ed25519_dalek::VerifyingKey,
    pqc_pk: &MlDsaPublicKey,
) -> Result<(), CryptoError> {
    // Classical verification
    classical_pk
        .verify(message, &signature.classical)
        .map_err(|_| CryptoError::Ed25519VerificationFailed)?;
    
    // Post-quantum verification
    let signed_msg = pqcrypto_mldsa::mldsa65::SignedMessage::from_bytes(&signature.pqc)
        .map_err(|_| CryptoError::InvalidMlDsaSignature)?;
    
    mldsa_verify(&signed_msg, pqc_pk)
        .map_err(|_| CryptoError::MlDsaVerificationFailed)?;
    
    Ok(())
}
```

**Rationale:** Quantum-capable adversaries not expected before 2030+, but "harvest now, decrypt later" attacks justify preparation. Phased approach balances security with budget.

### FIPS 140-3 Compliance Roadmap

| Phase | Requirement | Timeline |
|-------|-------------|----------|
| Phase 1 | Use crates wrapping FIPS-validated modules (e.g., `ring` with BoringSSL) | Immediate |
| Phase 2 | Document cryptographic module provenance in SBOM; Blade warns on non-FIPS paths | 2026 |
| Phase 3 | Full FIPS 140-3 validated module deployment for Controlled; Blade blocks non-compliant | 2027 |

**Controlled classification SHOULD use FIPS 140-3 validated modules. This graduates to MUST by 2027.**

### PQC Structural Requirement

**Phase 1-2 (Current):**
- `pqc` field MUST be structurally present in signature types
- Field MAY be `None` or contain simulation data
- Architecture validated for hybrid compatibility

**Phase 3 (2027, Controlled):**
- `is_phase_3()` configuration flag controls enforcement
- When true: `pqc.is_none()` SHALL fail verification for Controlled paths
- Both classical and PQC signatures MUST verify

**Blade Gate:**
- Controlled PRs without structural `pqc` field -> build warning (Phase 1-2)
- Controlled PRs without valid `pqc` signature -> build failure (Phase 3)

### MUST: Secure Secret Handling

```rust
use zeroize::Zeroizing;

fn process_secret() -> Result<(), Error> {
    let secret = Zeroizing::new(get_secret()?);
    
    // Use secret...
    do_something_with(&secret)?;
    
    // secret is zeroed when dropped
    Ok(())
}
```

### Hash Chain Implementation

When implementing hash chains (e.g., Cortex context chains):

```rust
use blake3::Hasher;
use uuid::Uuid;
use chrono::Utc;

/// Compute hash for chain linking with replay protection
fn compute_entry_hash(
    entry_id: &Uuid,
    content: &serde_json::Value,
    signature: &[u8],
    nonce: u64,
    timestamp: i64,
) -> Blake3Hash {
    // Precondition
    debug_assert!(!signature.is_empty(), "Signature must not be empty");
    
    let mut hasher = Hasher::new();
    hasher.update(entry_id.as_bytes());
    hasher.update(content.to_string().as_bytes());
    hasher.update(signature);
    hasher.update(&nonce.to_be_bytes());      // Prevent replay
    hasher.update(&timestamp.to_be_bytes());   // Temporal uniqueness
    
    let result = Blake3Hash(hasher.finalize().into());
    
    // Postcondition
    debug_assert!(result.0 != [0u8; 32], "Hash must not be zero");
    
    result
}

/// Each entry links to previous
#[derive(Debug, Clone)]
struct ChainEntry {
    id: Uuid,
    previous_hash: Option<Blake3Hash>,  // None only for genesis
    content_hash: Blake3Hash,
    nonce: u64,
    timestamp: i64,
}
```

**Requirements:**
- Genesis entry MUST have `previous_hash = None`
- All subsequent entries MUST reference previous hash
- Include nonce/timestamp to prevent replay attacks
- Hash formula MUST be consistent between write and verify
- Chain verification MUST be testable

---

## Nothing Lost, Ever (Principle 9)

### Persistence Requirements

```rust
use sled::Db;

/// Durable queue for governance messages
pub struct DurableQueue {
    db: Db,
}

impl DurableQueue {
    /// Enqueue with fsync - message persisted before return
    pub fn enqueue(&self, message: &GovernanceMessage) -> Result<MessageId, QueueError> {
        let id = MessageId::new();
        let serialized = bincode::serialize(message)?;
        
        // Insert with flush - guarantees durability
        self.db.insert(id.as_bytes(), serialized)?;
        self.db.flush()?;  // CRITICAL: fsync to disk
        
        Ok(id)
    }
    
    /// Dequeue only after ACK - message remains until acknowledged
    pub fn acknowledge(&self, id: &MessageId) -> Result<(), QueueError> {
        self.db.remove(id.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
    
    /// Recovery: return all unacknowledged messages
    pub fn recover_pending(&self) -> Result<Vec<GovernanceMessage>, QueueError> {
        self.db
            .iter()
            .map(|result| {
                let (_, value) = result?;
                bincode::deserialize(&value).map_err(QueueError::from)
            })
            .collect()
    }
}
```

**Requirements:**
- Governance messages MUST be persisted before acknowledgment
- `fsync` MUST be called for durability guarantees
- Recovery MUST return all unacknowledged messages
- Silent drops are PROHIBITED

---

## NASA/DOD Grade Requirements

*Per THE_XONAIX_WAY.md Part IX, Rust code must meet NASA/DOD grade standards for provable software.*

### Bounded Loops (NASA Rule 2)

All loops MUST have provable termination bounds:

```rust
// === BOUNDED LOOP PATTERNS ===

const MAX_ITERATIONS: usize = 10_000;

// Pattern 1: Iterator with take()
fn process_bounded_iter(items: &[Item]) -> Result<Vec<Output>, ProcessError> {
    items
        .iter()
        .take(MAX_ITERATIONS)
        .map(|item| process_item(item))
        .collect()
}

// Pattern 2: Explicit counter with error
fn process_bounded_explicit(items: &[Item]) -> Result<(), ProcessError> {
    // Precondition
    debug_assert!(items.len() <= MAX_ITERATIONS * 10, "Input suspiciously large");
    
    if items.len() > MAX_ITERATIONS {
        return Err(ProcessError::TooManyItems {
            limit: MAX_ITERATIONS,
            actual: items.len(),
        });
    }

    for item in items {
        process_item(item)?;
    }
    
    // Postcondition
    debug_assert!(items.iter().all(|i| i.is_processed()));
    
    Ok(())
}

// Pattern 3: While loop with counter
fn process_with_while(source: &mut impl Iterator<Item = Data>) -> Result<Vec<Output>, ProcessError> {
    let mut results = Vec::new();
    let mut count = 0;

    while let Some(item) = source.next() {
        if count >= MAX_ITERATIONS {
            return Err(ProcessError::IterationLimitExceeded {
                limit: MAX_ITERATIONS,
            });
        }
        results.push(process(item)?);
        count += 1;
    }

    Ok(results)
}
```

**Anti-Patterns (VIOLATIONS):**

```rust
// VIOLATION: Unbounded loop
loop {
    if should_stop() { break; }
    do_work();
}

// VIOLATION: While true without explicit bound
while true {
    // ...
}

// VIOLATION: External state determines termination
while !external_flag.load(Ordering::SeqCst) {
    process();
}
```

### Assertion Density (NASA Rule 5)

MUST have 2+ assertions per function for Production/Controlled:

```rust
fn transfer_funds(from: &Account, to: &Account, amount: u64) -> Result<Receipt, TransferError> {
    // Precondition 1: Amount must be positive
    debug_assert!(amount > 0, "Transfer amount must be positive");
    
    // Precondition 2: Source has sufficient balance
    debug_assert!(
        from.balance >= amount,
        "Insufficient balance: {} < {}",
        from.balance,
        amount
    );
    
    let receipt = execute_transfer(from, to, amount)?;
    
    // Postcondition 1: Receipt is finalized
    debug_assert!(receipt.is_finalized(), "Transfer must be finalized");
    
    // Postcondition 2: Balances updated correctly
    debug_assert!(
        from.balance + to.balance == from.original_balance + to.original_balance,
        "Conservation of funds violated"
    );
    
    Ok(receipt)
}
```

### Function Size (NASA Rule 4)

MUST ≤60 lines; SHOULD ≤30 lines.

**Refactoring Pattern:**

```rust
// BAD: 80+ line function
fn process_everything(data: &Data) -> Result<Output, Error> {
    // ... 80 lines of logic ...
}

// GOOD: Decomposed into focused functions
fn process_everything(data: &Data) -> Result<Output, Error> {
    let validated = validate_input(data)?;      // ~15 lines
    let transformed = transform(validated)?;     // ~20 lines
    let output = finalize(transformed)?;         // ~15 lines
    Ok(output)
}
```

---

## Formal Verification

### Kani for Cryptographic Operations

**Controlled classification MUST use Kani for crypto verification:**

```bash
# Install Kani
cargo install --locked kani-verifier
kani setup

# Run proofs
cargo kani
```

**Proof Pattern:**

```rust
#[cfg(kani)]
mod verification {
    use super::*;
    
    #[kani::proof]
    fn verify_hash_deterministic() {
        let input: [u8; 32] = kani::any();
        let hash1 = compute_hash(&input);
        let hash2 = compute_hash(&input);
        kani::assert(hash1 == hash2, "Hash must be deterministic");
    }
    
    #[kani::proof]
    fn verify_signature_roundtrip() {
        let key: [u8; 32] = kani::any();
        kani::assume(key != [0; 32]);  // Valid key assumption
        
        let message: [u8; 64] = kani::any();
        let signature = sign(&key, &message);
        let verified = verify(&derive_public(&key), &message, &signature);
        
        kani::assert(verified.is_ok(), "Valid signature must verify");
    }
}
```

### TLA+ for State Machines

**Controlled classification MUST use TLA+ for protocol/state machine verification:**

```tla
---- MODULE GovernanceProtocol ----
EXTENDS Integers, Sequences

VARIABLES state, messages, acknowledged

Init ==
    /\ state = "idle"
    /\ messages = <<>>
    /\ acknowledged = {}

SendMessage(m) ==
    /\ messages' = Append(messages, m)
    /\ UNCHANGED <<state, acknowledged>>

AcknowledgeMessage(m) ==
    /\ m \in Range(messages)
    /\ m \notin acknowledged
    /\ acknowledged' = acknowledged \cup {m}
    /\ UNCHANGED <<state, messages>>

\* INVARIANT: No message lost (Principle 9)
NothingLost == 
    \A m \in Range(messages) : 
        m \in acknowledged \/ m \in Range(messages)

====
```

### Loom for Concurrent Code

**Controlled classification SHOULD use Loom for concurrent code:**

> **Principle 6 Note:** Test code may use `unwrap()` per Test Module Exceptions (§Required Tooling). Test failures are expected to panic.

```rust
#[cfg(loom)]
mod concurrent_tests {
    use loom::sync::Arc;
    use loom::sync::atomic::{AtomicUsize, Ordering};
    use loom::thread;
    
    #[test]
    fn test_concurrent_counter() {
        loom::model(|| {
            let counter = Arc::new(AtomicUsize::new(0));
            
            let threads: Vec<_> = (0..2)
                .map(|_| {
                    let counter = Arc::clone(&counter);
                    thread::spawn(move || {
                        counter.fetch_add(1, Ordering::SeqCst);
                    })
                })
                .collect();
            
            for t in threads {
                t.join().unwrap();
            }
            
            assert_eq!(counter.load(Ordering::SeqCst), 2);
        });
    }
}
```

---

## Property-Based Testing

**Controlled classification MUST use property-based tests with custom shrinkers:**

```toml
[dev-dependencies]
proptest = "1"
```

**Core Patterns:**

```rust
use proptest::prelude::*;

proptest! {
    // Roundtrip property
    #[test]
    fn test_serialize_roundtrip(data in any::<GovernanceMessage>()) {
        let encoded = serialize(&data)?;
        let decoded: GovernanceMessage = deserialize(&encoded)?;
        prop_assert_eq!(data, decoded);
    }
    
    // Invariant property
    #[test]
    fn test_balance_never_negative(
        initial in 0u64..1_000_000,
        operations in prop::collection::vec(any::<Operation>(), 0..100)
    ) {
        let mut account = Account::new(initial);
        for op in operations {
            let _ = account.apply(op);  // May fail, that's OK
            prop_assert!(account.balance() >= 0, "Balance went negative");
        }
    }
    
    // Idempotence property
    #[test]
    fn test_normalize_idempotent(input in ".*") {
        let once = normalize(&input);
        let twice = normalize(&once);
        prop_assert_eq!(once, twice, "Normalize must be idempotent");
    }
}
```

**Custom Shrinker for Debugging:**

```rust
fn governance_message_strategy() -> impl Strategy<Value = GovernanceMessage> {
    (
        any::<Uuid>(),
        "[a-z]{1,50}",  // Simpler strings shrink better
        0u64..1_000_000,
    )
    .prop_map(|(id, content, timestamp)| GovernanceMessage {
        id,
        content,
        timestamp,
    })
    .prop_filter("Must be valid", |m| m.is_valid())
}
```

---

## Mutation Testing

**Controlled classification MUST achieve ≥95% mutation score with survivor justification:**

```bash
# Install
cargo install cargo-mutants --locked

# Run with parallel jobs
cargo mutants -j 4 --json > mutation-report.json

# Check score
jq '.summary.score' mutation-report.json
```

**CI Integration:**

```yaml
- name: Mutation Testing
  run: |
    cargo install cargo-mutants --locked
    cargo mutants -j 4 --json > mutation-report.json
    SCORE=$(jq '.summary.score // 0' mutation-report.json)
    echo "Mutation score: $SCORE%"
    if (( $(echo "$SCORE < 95" | bc -l) )); then
      echo "::error::Mutation score $SCORE% below 95% threshold"
      exit 1
    fi

- name: Upload Mutation Report
  uses: actions/upload-artifact@v4
  with:
    name: mutation-report
    path: mutation-report.json
```

**Surviving Mutant Review:**

Each surviving mutant MUST be categorized in `MUTATION_SURVIVORS.md`:

```markdown
# Mutation Survivors Review

## Equivalent Mutants (Cannot Kill)

### src/crypto.rs:45 - Changed `>=` to `>`
**Category:** Equivalent
**Reason:** Input is validated to never equal boundary value on line 42.

## Accepted Risk

### src/logging.rs:123 - Removed log statement  
**Category:** Accepted Risk
**Reason:** Log statement is observability, not correctness. Test would require log capture infrastructure.
**Reviewer:** @username
**Date:** 2025-12-19
```

---

## Code Coverage

| Classification | Line Coverage | Branch Coverage |
|----------------|---------------|-----------------|
| Development    | ≥60% SHOULD   | ≥50% SHOULD     |
| Production     | ≥80% MUST     | ≥70% MUST       |
| Controlled     | ≥95% MUST     | ≥90% MUST       |

**CI Integration:**

```yaml
- name: Coverage
  run: |
    cargo install cargo-tarpaulin --locked
    cargo tarpaulin --out Json --output-dir coverage/
    
- name: Check Coverage Thresholds
  run: |
    LINE_COV=$(jq '.coverage_percentage' coverage/tarpaulin-report.json)
    if (( $(echo "$LINE_COV < 95" | bc -l) )); then
      echo "::error::Line coverage $LINE_COV% below 95% threshold"
      exit 1
    fi
```

---

## Chaos Testing

**Controlled classification MUST implement chaos testing:**

> **Principle 6 Note:** Test code may use `unwrap()` per Test Module Exceptions (§Required Tooling). Test failures are expected to panic.

```rust
#[cfg(test)]
mod chaos_tests {
    use tokio::time::{timeout, Duration};
    
    /// Test behavior under network failure
    #[tokio::test]
    async fn test_network_failure_handling() {
        let service = TestService::new();
        
        // Inject network failure
        service.inject_fault(Fault::NetworkDrop);
        
        let result = timeout(
            Duration::from_secs(5),
            service.send_message(&test_message())
        ).await;
        
        // Should timeout gracefully, not panic
        assert!(result.is_err() || result.unwrap().is_err());
        
        // Message should be queued for retry (Nothing Lost)
        assert!(service.has_pending_messages());
    }
    
    /// Test behavior under disk failure
    #[tokio::test]
    async fn test_disk_failure_handling() {
        let service = TestService::new();
        
        // Inject disk full error
        service.inject_fault(Fault::DiskFull);
        
        let result = service.persist_message(&test_message()).await;
        
        // Should return error, not panic or lose data
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::StorageFull));
    }
}
```

**Fault Categories to Test:**
- Network: latency, drops, partitions
- Storage: disk full, I/O errors, corruption
- Service: crash, hang, slow response
- Resources: memory pressure, CPU exhaustion

---

## Dependency Vetting

**Controlled classification MUST vet all dependencies:**

```bash
# Install cargo-vet
cargo install cargo-vet --locked

# Initialize
cargo vet init

# Check all dependencies
cargo vet

# Import trusted audits
cargo vet trust --all Mozilla
cargo vet trust --all Google

# Record manual audit
cargo vet certify my-dependency 1.2.3
```

**CI Integration:**

```yaml
- name: Dependency Vetting
  run: |
    cargo install cargo-vet --locked
    cargo vet --locked
```

**Also use cargo-deny:**

```bash
cargo install cargo-deny --locked
cargo deny check
```



## Reproducible Builds (Controlled)

Controlled classification builds SHALL be reproducible:

**Required Build Flags:**
```bash
export CARGO_INCREMENTAL=0
export SOURCE_DATE_EPOCH="$(git log -1 --pretty=%ct)"
export RUSTFLAGS="-C embed-bitcode=no --remap-path-prefix=$(pwd)=."
cargo build --release --locked
```

**CI Verification:**
```bash
# Build twice and compare
cargo build --release --locked
sha256sum target/release/<binary> > build1.sha
cargo clean
cargo build --release --locked
sha256sum target/release/<binary> > build2.sha
diff build1.sha build2.sha || exit 1
```

**SBOM & Provenance:**
- Generate SBOM for each release build
- Sign SBOM with release key
- Attach provenance attestation (build environment, inputs, outputs)

**Weekly Drift Detection:**
```bash
# Scheduled CI job
cargo update --dry-run 2>&1 | tee dependency-drift.log
cargo deny check 2>&1 | tee -a dependency-drift.log
# Alert on potential breaking changes
```

**Violations:**
- Build hash mismatch -> fail release, investigate
- Unsigned SBOM -> block deployment
- Drift detection findings -> review before next release

---

## Testing


### MUST

- All public functions have unit tests
- All error paths have tests
- Hash chain integrity verified in tests
- Security-critical modules have fuzz tests
- Parsing/serialization has property-based tests
- Line coverage ≥95% for Controlled
- Branch coverage ≥90% for Controlled

### SHOULD

- Miri runs for any unsafe code
- Integration tests for cross-module flows
- Benchmarks for performance-critical paths
- Loom tests for concurrent code

### Hash Chain Testing Pattern

> **Principle 6 Note:** Test code may use `unwrap()` per Test Module Exceptions (§Required Tooling). Test failures are expected to panic.

```rust
#[test]
fn test_chain_integrity() {
    let genesis = create_genesis_entry();
    assert!(genesis.previous_hash.is_none());
    
    let now = Utc::now().timestamp();
    let entry1 = create_entry(&genesis, 1, now);
    assert_eq!(entry1.previous_hash, Some(genesis.content_hash));
    
    let entry2 = create_entry(&entry1, 2, now + 1);
    assert_eq!(entry2.previous_hash, Some(entry1.content_hash));
    
    // Verify chain
    assert!(verify_chain(&[genesis, entry1, entry2]).is_ok());
}

#[test]
fn test_chain_detects_tampering() {
    let mut entries = create_valid_chain(5);
    
    // Tamper with middle entry
    entries[2].content_hash = Blake3Hash([0u8; 32]);
    
    // Verification must fail
    let result = verify_chain(&entries);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ChainError::IntegrityViolation { index: 2, .. }));
}

#[test]
fn test_replay_attack_prevented() {
    let genesis = create_genesis_entry();
    let entry1 = create_entry_with_nonce(&genesis, 1, 12345, now);
    let entry2 = create_entry_with_nonce(&entry1, 1, 12345, now);  // Same nonce
    
    // Should reject duplicate nonce
    assert!(validate_entry(&entry2, &entry1).is_err());
}
```

---

## MCP Server Development

When building MCP (Model Context Protocol) servers:

### Tool Implementation

```rust
#[tool(description = "Get context summary for an agent")]
async fn cortex_get_summary(
    &self,
    #[arg(description = "Name of the agent")] agent_name: String,
) -> Result<ContextSummary, McpError> {
    // Input validation (Secure By Default)
    if agent_name.is_empty() || agent_name.len() > 256 {
        return Err(McpError::InvalidInput("Agent name must be 1-256 characters".into()));
    }
    
    if !agent_name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(McpError::InvalidInput("Agent name contains invalid characters".into()));
    }
    
    self.client
        .get_summary_by_name(&agent_name)
        .await
        .map_err(|e| McpError::Internal(e.to_string()))
}
```

### Requirements

| Requirement | Rationale |
|-------------|-----------|
| All tools MUST return Result types | Fail Loud principle |
| Tool registration MUST use `#[tool]` macro | Explicit Over Implicit |
| Input validation before API calls | Secure By Default |
| Descriptive error messages | Debugging support |

### Development Workflow

1. Add tool implementation with `#[tool]` macro
2. Add corresponding client method
3. Build and test: `cargo build && cargo test`
4. Rebuild release binary: `cargo build --release -p <mcp-crate>`
5. **Restart MCP host** (Claude Code, Claude Desktop) to pick up new tools

**Note:** MCP tool discovery happens at session start. Binary rebuild alone is insufficient — the host application must restart to see new tools.

---

## CI Pipeline

```yaml
name: Xonaix Rust CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust-checks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Format Check
        run: cargo fmt --check
        
      - name: Lint
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Test
        run: cargo test --all-features
        
      - name: Security Audit
        run: |
          cargo install cargo-audit --locked
          cargo audit
          
      - name: Dependency Vetting
        run: |
          cargo install cargo-vet --locked
          cargo vet --locked
          
      - name: Unsafe Tracking
        run: |
          cargo install cargo-geiger --locked
          cargo geiger --all-features
          
      - name: Coverage
        run: |
          cargo install cargo-tarpaulin --locked
          cargo tarpaulin --out Xml
          
      - name: Mutation Testing
        run: |
          cargo install cargo-mutants --locked
          cargo mutants -j 4 --json > mutation-report.json
          SCORE=$(jq '.summary.score // 0' mutation-report.json)
          if (( $(echo "$SCORE < 95" | bc -l) )); then
            echo "::error::Mutation score $SCORE% below 95%"
            exit 1
          fi
          
      - name: Documentation
        run: cargo doc --no-deps
        
      - name: Prototype Check
        # Note: Blade provides semantic/AST-based detection
        run: |
          if grep -rE "XONAIX_PROTOTYPE|// PROTOTYPE|//PROTOTYPE" src/; then
            echo "ERROR: Prototype markers detected"
            exit 1
          fi

  # Controlled classification only
  controlled-checks:
    runs-on: ubuntu-latest
    if: contains(github.event.pull_request.labels.*.name, 'controlled')
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Kani Verification
        run: |
          cargo install --locked kani-verifier
          kani setup
          cargo kani
          
      - name: Miri (Unsafe Check)
        run: |
          rustup +nightly component add miri
          cargo +nightly miri test
```

---

## X.I. Prompt Appendix

*Add to base prompt when working with Rust.*

```
RUST v5.1.0 REQUIREMENTS:

NOTE: Rust is the PRIMARY LANGUAGE for Xonaix development.
All core systems, governance, and security code MUST be Rust.

TOOLCHAIN:
- Match CI Rust version (stable)
- Run cargo fmt --all before committing
- Run cargo clippy -- -D warnings before committing
- Run cargo test before committing

FORBIDDEN:
- .unwrap() — use ? operator with proper error types
- .expect() — same as unwrap
- unsafe blocks — except FFI/crypto with XONAIX_DEVIATION marker
- panic!() — use Result types instead
- let _ = fallible(); — silent error dropping
- Unbounded loops — ALL loops MUST have MAX_* constants

REQUIRED:
- All errors via Result<T, E> with context
- unreachable!() only with proof comment (only permitted panic)
- Cognitive complexity ≤10 per function
- Function size ≤60 lines MUST, ≤30 SHOULD
- Timeouts on ALL external calls
- Bounded iteration on external data
- Bounded recursion with depth limits
- All public items documented
- 2+ assertions per function (preconditions + postconditions)
- Hash chains verified in tests

CRYPTO:
- Use blake3 for hashing
- Use ed25519-dalek for signatures (all tiers now)
- Use zeroize for secret cleanup
- Never roll your own crypto
- Phase 3 (2027): pqcrypto-mldsa for ML-DSA-65 hybrid signatures

NASA/DOD GRADE:
- Bounded loops: ALL loops MUST have MAX_* constants
- Assertion density: 2+ assertions per function
- Function size: ≤60 lines MUST, ≤30 lines SHOULD
- Line coverage: ≥95% for Controlled
- Branch coverage: ≥90% for Controlled
- Property tests: MUST for Controlled (use proptest with shrinkers)
- Mutation testing: ≥95% score for Controlled (justify survivors)
- Formal verification: MUST for crypto (Kani), MUST for state machines (TLA+)
- Chaos testing: MUST for Controlled

VERIFICATION TOOLS:
- Kani: cargo install --locked kani-verifier && kani setup
- proptest: Add to [dev-dependencies]
- cargo-mutants: cargo install cargo-mutants --locked
- cargo-vet: cargo install cargo-vet --locked
- cargo-geiger: cargo install cargo-geiger --locked
- cargo-tarpaulin: cargo install cargo-tarpaulin --locked

PRINCIPLE 9 (NOTHING LOST):
- Use sled/rocksdb for persistence
- Call fsync() after critical writes
- ACK-based message processing
- Recovery must preserve pending work
- No fire-and-forget for governance messages

CONTROLLED CLASSIFICATION CHECKLIST:
[ ] All loops bounded with MAX_* constant
[ ] 2+ assertions per function (preconditions + postconditions)
[ ] Functions ≤60 lines
[ ] Line coverage ≥95%
[ ] Branch coverage ≥90%
[ ] Property-based tests with proptest
[ ] Kani proofs for crypto operations
[ ] TLA+ specs for state machines
[ ] Mutation score ≥95% with survivors justified
[ ] Chaos tests with fault injection
[ ] Dependencies vetted with cargo-vet
[ ] Unsafe tracked with cargo-geiger

FLAG THESE VIOLATIONS:
NO .unwrap() or .expect() in production code
NO let _ = might_fail();
NO unsafe without // SAFETY: comment and XONAIX_DEVIATION marker
NO Unbounded loops over external data
NO Recursion without explicit depth limit
NO Missing error context on ? operator
NO Complexity > 10 without documented justification
NO Hash chain without integrity tests
NO dtolnay/rust-action (use rust-toolchain)
NO Functions > 60 lines
NO Missing assertions in Production/Controlled code
NO Controlled crypto without Kani proofs
NO Fire-and-forget messaging for governance
NO Coverage below thresholds
NO Mutation score below 95% for Controlled
```

---

## Quick Reference

### Allowed Error Handling

```rust
// Propagate with context
fetch().map_err(|e| MyError::Fetch(e))?;

// Match and handle all cases
match result {
    Ok(v) => use_value(v),
    Err(e) => return Err(e.into()),
}

// Unreachable with proof
unreachable!("checked non-empty on line 42")
```

### Forbidden Error Handling

```rust
// Unwrap
value.unwrap();

// Expect  
value.expect("should exist");

// Silent drop
let _ = fallible();

// Panic
panic!("something wrong");
```

### Pre-Commit Checklist

```bash
# MUST run before every commit
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

---

## Common Criteria EAL4 Design Compatibility

**Controlled systems SHOULD be designed for CC EAL4 evaluation compatibility:**

- Maintain Security Target documentation
- Document all security functions with functional specifications
- Keep design documentation traceable to requirements
- Collect test evidence systematically
- Perform ongoing vulnerability analysis

**Note:** This ensures design compatibility, not immediate certification.

---

## Prototype Marker

```rust
// XONAIX_PROTOTYPE - NOT FOR PRODUCTION
// This file will be deleted, not merged
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L1/L2 classification)
- **MAJOR:** Added XCLib Integration section (exclusivity rule)
- **MAJOR:** Added Numeric Policy section (float prohibition)
- **MAJOR:** Added Capability & Posture Handling section
- **MAJOR:** Added Generated Code Accountability section
- **UPDATED:** Core-Compatible to 5.7.0
- **ALIGNED:** Cross-language requirements per STANDARDS_INDEX B-5.8.5
- **Source:** Red-Blue-Black Team synthesis with Founder approval

### v5.1.0 (December 2025)
- **RATIFIED:** Rainbow Team consensus (Red-Blue-White-Green)
- **ADDED:** Reproducible Builds section (Controlled classification)
- **ADDED:** PQC Structural Requirement (Phase flag, envelope presence)
- **ADDED:** Weekly drift detection pattern
- **ADDED:** SBOM signing and provenance requirements
- **ALIGNED:** Synchronized with The Xonaix Way v5.1.0
- **ADDED:** Principle 9 (Nothing Lost, Ever) implementation (sled, fsync, ACK queues)
- **ADDED:** Deviation Recording syntax (XONAIX_DEVIATION marker)
- **ADDED:** Post-Quantum Cryptography with phased approach (ML-DSA-65 via `pqcrypto-mldsa`)
- **UPDATED:** PQC from pqcrypto-dilithium to pqcrypto-mldsa (FIPS 204 compliant)
- **ADDED:** FIPS 140-3 compliance roadmap (phased SHOULD -> MUST by 2027)
- **ADDED:** Common Criteria EAL4 design compatibility section
- **ADDED:** Formal verification expansion (TLA+ for state machines, Loom for concurrency)
- **ADDED:** Code coverage requirements (95% line, 90% branch for Controlled)
- **ADDED:** Dependency vetting (cargo-vet MUST for Controlled)
- **ADDED:** Unsafe tracking (cargo-geiger MUST for Controlled)
- **ADDED:** Chaos testing section with fault injection patterns
- **ADDED:** Mutation survivor justification requirement
- **UPDATED:** Mutation score threshold (≥95% for Controlled)
- **FIXED:** Async timeout examples (proper imports, no double `?`)
- **FIXED:** Error handling examples (proper Error trait implementation)
- **UPDATED:** CI pipeline with all new tooling
- **UPDATED:** X.I. Prompt Appendix with v5.1.0 requirements
- **Source:** Rainbow Team synthesis (Red, Blue, White, Green, Yellow, Purple)

### v1.4.0 (December 2025)
- Added: NASA/Google Grade Requirements section
- Added: Formal Verification section (Kani)
- Added: Property-Based Testing section (proptest)
- Added: Mutation Testing section (cargo-mutants)

### v1.3.0 (December 2025)
- Added: Test Module Exceptions section

### v1.2.0 (December 2025)
- Added: Rust Version Alignment section
- Added: GitHub Actions section
- Added: Approved Cryptographic Crates table
- Added: Hash Chain Implementation guidance
- Added: MCP Server Development section

### v1.1.0 (December 2025)
- Updated: Core-Version to 1.1.0
- Added: Language Policy note

### v1.0.0 (December 2025)
- Initial release

---

*Rust Standards B-5.8.5 — Part of The Xonaix Way B-5.8.5*

*"Rust is the foundation. XCLib is the trust root."*

*Xonaix, Inc. — Intelligence, evolved.*
