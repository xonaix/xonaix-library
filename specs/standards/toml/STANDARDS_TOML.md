---
title: "TOML Configuration Standard"
unit_id: "library/standards/toml"
standard_type: "standard"
version: "1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# The Xonaix Way
## Standards: TOML

**Version:** B-5.8.5
**Status:** Active
**Core-Compatible:** 5.7.0
**Trust Class:** L4
**Created:** December 2025
**Last Reviewed:** December 2025

*This document implements The Xonaix Way B-5.8.5 principles for TOML configuration files.*

---

## Document Info

| Field | Value |
|-------|-------|
| Domain | Configuration |
| Status | **Active** |
| Version | B-5.8.5 |
| Core-Compatible | 5.7.0 |
| Trust Class | L4 (Configuration) |
| Created | December 2025 |
| Last Reviewed | December 2025 |
| Specification | TOML v1.0.0 |
| Primary Use | Cargo.toml, Rust configuration |
| Related Standards | STANDARDS_YAML.md, STANDARDS_JSON.md, STANDARDS_RUST.md |

**Prerequisites:** Read [THE_XONAIX_WAY.md](../THE_XONAIX_WAY.md) first. This document assumes familiarity with the 9 Principles.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L4 |
| Classification | Configuration |

TOML files are L4 (Configuration/Tooling). They define settings but cannot assert authority.

### Security-Affecting Values

Configuration values that affect security posture MUST be:

1. **Hash-committed to ledger** — for audit trail, OR
2. **Part of signed manifest** — for deployment verification

```toml
# Values affecting posture - MUST be in signed manifest
[security]
capability_class = "SoftwareClassA"
trust_level = "full"

# Values NOT affecting posture - may be unsigned
[logging]
level = "debug"

[ui]
theme = "dark"
```

---

## Numeric Policy

**Authority:** Founder Ruling 2025-003(a) — Determinism

### Float Handling

TOML distinguishes integers from floats. For canonical/governance data:

```toml
# FORBIDDEN for governance data
rate = 1.5          # Float - non-deterministic

# REQUIRED representations
rate_bps = 150      # Integer (basis points)
rate_str = "1.5"    # String representation
```

---

## XCLib Integration

TOML files do not directly integrate with XCLib. Configuration is:

1. **Parsed** by application code
2. **Validated** against expected schema
3. **Verified** against signed manifest (if security-affecting)

---

## Principle Mapping

| Principle | TOML Implementation |
|-----------|---------------------|
| 1. Correct Over Fast | Valid TOML, semantic correctness, cargo check |
| 2. Secure By Default | No secrets in TOML files, dependency auditing |
| 3. Fail Loud | Parse errors are fatal, cargo check fails build |
| 4. Explicit Over Implicit | Explicit sections, explicit types, explicit versions |
| 5. Automated Over Vigilant | Automated validation, cargo check, cargo audit, cargo-vet |
| 6. Composable Over Clever | Clear sections, minimal nesting, workspace inheritance |
| 7. X.I. Augments, Human Decides | Dependency changes require human review |
| 8. Future-Proof Over Trend | TOML v1.0 standard features, stable Rust editions |
| 9. Nothing Lost, Ever | Config for persistence backends (sled, rocksdb), fsync settings |

---

## Deviation Recording

For deviations from MUST requirements in TOML configurations:

```toml
# XONAIX_DEVIATION: [Reason for deviation - be specific]
# LEDGER_ACK: [User_Signature_Hash]
unsafe_config = "value"  # Deviating configuration
```

**Blade Enforcement:** Blade scans TOML files for deviation markers and records them in the Security Ledger.

---

## SECTION 1: CARGO.TOML STRUCTURE

### 1.1 Complete Example

```toml
[package]
name = "xonaix-api"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Xonaix Team <team@xonaix.com>"]
description = "Xonaix API Server"
documentation = "https://docs.xonaix.com"
homepage = "https://xonaix.com"
repository = "https://github.com/xonaix/xonaix-api"
license = "MIT OR Apache-2.0"
keywords = ["api", "server", "xonaix"]
categories = ["web-programming"]
publish = false

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

# =============================================================================
# Lints - Aligned with STANDARDS_RUST.md v5.0.0
# =============================================================================
[lints.rust]
unsafe_code = "forbid"
unused_must_use = "deny"
unused_results = "deny"
missing_docs = "deny"

[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "warn"
cognitive_complexity = "warn"
too_many_arguments = "warn"
DECIDED = "warn"
unimplemented = "warn"
dbg_macro = "warn"
print_stdout = "warn"
print_stderr = "warn"

# =============================================================================
# Features
# =============================================================================
[features]
default = []
full = ["metrics", "tracing"]
metrics = ["prometheus"]
tracing = ["opentelemetry"]

# =============================================================================
# Dependencies
# =============================================================================
[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Web framework
axum = { version = "0.7", features = ["macros"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }

# Configuration
config = "0.14"

# Cryptography (aligned with STANDARDS_RUST.md)
blake3 = "1.5"
ed25519-dalek = "2"
zeroize = { version = "1.7", features = ["derive"] }

# Optional dependencies
prometheus = { version = "0.13", optional = true }
opentelemetry = { version = "0.21", optional = true }

[dev-dependencies]
tokio-test = "0.4"
rstest = "0.18"
wiremock = "0.5"
testcontainers = "0.15"
proptest = "1"
cargo-mutants = "0.1"

[build-dependencies]
vergen = { version = "8", features = ["git", "cargo"] }

# =============================================================================
# Binary Configuration
# =============================================================================
[[bin]]
name = "xonaix-api"
path = "src/main.rs"

[[bin]]
name = "xonaix-migrate"
path = "src/bin/migrate.rs"

# =============================================================================
# Profile Configuration
# =============================================================================
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
# Security: abort on panic prevents unwinding exploits
panic = "abort"

[profile.release-debug]
inherits = "release"
debug = true
strip = false
```

### 1.2 Section Ordering

**Standard order for Cargo.toml:**

1. `[package]` — Package metadata
2. `[package.metadata.*]` — Custom metadata
3. `[lints.*]` — Lint configuration (REQUIRED)
4. `[features]` — Feature flags
5. `[dependencies]` — Runtime dependencies
6. `[dev-dependencies]` — Test/dev dependencies
7. `[build-dependencies]` — Build script dependencies
8. `[[bin]]` / `[[lib]]` / `[[example]]` — Targets
9. `[profile.*]` — Build profiles
10. `[workspace]` — Workspace configuration

---

## SECTION 2: LINTS CONFIGURATION

### 2.1 Required Lints (Aligned with STANDARDS_RUST.md)

**All Xonaix Rust projects MUST include these lints:**

```toml
[lints.rust]
# Safety: No unsafe code without explicit deviation
unsafe_code = "forbid"

# Fail Loud: No silent drops of Results
unused_must_use = "deny"

# Nothing Lost: No ignored results
unused_results = "deny"

# Documentation: All public items documented
missing_docs = "deny"

[lints.clippy]
# Error on common mistakes
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "warn"

# Composable: Warn on complexity
cognitive_complexity = "warn"
too_many_arguments = "warn"

# Development markers
DECIDED = "warn"
unimplemented = "warn"
dbg_macro = "warn"
print_stdout = "warn"
print_stderr = "warn"
```

### 2.2 Optional Stricter Lints

```toml
[lints.clippy]
# Pedantic (very strict - enable for Controlled classification)
pedantic = "warn"

# Nursery (experimental but useful)
# nursery = "warn"

# Restriction (opt-in very strict rules)
# restriction = "warn"
```

---

## SECTION 3: MSRV POLICY

### 3.1 Minimum Supported Rust Version

**MUST specify `rust-version` for reproducible builds:**

```toml
[package]
rust-version = "1.75"  # MUST specify
```

**MSRV Policy by Classification:**

| Classification | MSRV Requirement |
|----------------|------------------|
| Development | Current stable - 3 versions |
| Production | Current stable - 2 versions |
| Controlled | Current stable - 1 version |

**Rationale:** Controlled systems need recent security fixes. Production allows slightly older for stability. Development is flexible.

### 3.2 MSRV Verification in CI

```yaml
- name: Check MSRV
  run: |
    MSRV=$(grep "rust-version" Cargo.toml | cut -d'"' -f2)
    echo "MSRV: $MSRV"
    rustup install $MSRV
    cargo +$MSRV check
```

---

## SECTION 4: CARGO.LOCK POLICY

### 4.1 Commit Policy

| Project Type | Cargo.lock | Rationale |
|--------------|------------|-----------|
| **Binaries** | MUST commit | Reproducible builds |
| **Applications** | MUST commit | Reproducible deployments |
| **Libraries** | MAY commit | Recommended for CI consistency |
| **Workspaces** | MUST commit | All members use same versions |

```bash
# For binaries/applications - always commit
git add Cargo.lock

# For libraries - recommended
git add Cargo.lock
```

### 4.2 Lockfile Verification

```yaml
- name: Verify Lockfile
  run: |
    cargo check --locked
    if ! git diff --quiet Cargo.lock; then
      echo "ERROR: Cargo.lock is out of sync"
      exit 1
    fi
```

---

## SECTION 5: DEPENDENCY SPECIFICATION

### 5.1 Version Requirements

```toml
[dependencies]
# Caret (default) - compatible updates
serde = "1.0"          # Same as ^1.0, allows 1.x.y
tokio = "1"            # Same as ^1, allows 1.x.x

# Tilde - more restrictive
serde = "~1.0.100"     # Allows 1.0.x where x >= 100

# Exact version (for Controlled classification)
serde = "=1.0.193"     # Exactly this version

# Range
serde = ">=1.0, <2.0"
```

### 5.2 Dependency Sources

```toml
[dependencies]
# crates.io (default) - preferred
serde = "1.0"

# Git repository (with specific ref - REQUIRED)
my_crate = { git = "https://github.com/org/repo", tag = "v1.0.0" }
my_crate = { git = "https://github.com/org/repo", rev = "abc123def" }

# AVOID for production: branch reference (can change)
# my_crate = { git = "https://github.com/org/repo", branch = "main" }

# Local path (workspace)
my_crate = { path = "../my_crate" }
```

### 5.3 Feature Selection

```toml
[dependencies]
# With features
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# Default features disabled
serde = { version = "1.0", default-features = false, features = ["derive"] }

# Optional dependency
metrics = { version = "0.21", optional = true }

[features]
default = []
monitoring = ["metrics"]
```

---

## SECTION 6: WORKSPACE CONFIGURATION

### 6.1 Workspace Root

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
]

# Shared dependencies - DRY
[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
tracing = "0.1"
blake3 = "1.5"
ed25519-dalek = "2"

# Shared package metadata
[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Xonaix Team <team@xonaix.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/xonaix/xonaix"

# Shared lints
[workspace.lints.rust]
unsafe_code = "forbid"
unused_must_use = "deny"
unused_results = "deny"
missing_docs = "deny"

[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
cognitive_complexity = "warn"
```

### 6.2 Workspace Member

```toml
# crates/api/Cargo.toml
[package]
name = "xonaix-api"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true

[lints]
workspace = true

[dependencies]
# Use workspace dependency
tokio.workspace = true
serde.workspace = true

# Crate-specific dependency
axum = "0.7"

# Internal dependency
xonaix-core = { path = "../core" }
```

---

## SECTION 7: BUILD PROFILES

### 7.1 Standard Profiles

```toml
[profile.dev]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
lto = false
incremental = true

[profile.release]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false  # Enable for Controlled: true
lto = "thin"
codegen-units = 1
strip = true
# Security: abort on panic prevents unwinding exploits
panic = "abort"

[profile.test]
opt-level = 0
debug = true
```

### 7.2 Custom Profiles

```toml
# Release with debug info (for profiling)
[profile.release-debug]
inherits = "release"
debug = true
strip = false

# Optimized dev build (faster iteration)
[profile.dev-fast]
inherits = "dev"
opt-level = 1

# Production profile (maximum security)
[profile.production]
inherits = "release"
lto = "fat"
overflow-checks = true  # Security: catch overflows
```

### 7.3 Profile Security Settings

```toml
[profile.release]
# Security: abort on panic prevents unwinding exploits
# Unwinding can be used to bypass destructors and leak resources
panic = "abort"

# Security: strip symbols to reduce attack surface
# Symbols can reveal internal structure to attackers
strip = true

# Security: single codegen unit for better optimization
# Also makes binary analysis slightly harder
codegen-units = 1

# For Controlled classification, also enable:
# overflow-checks = true  # Catch integer overflows
```

---

## SECTION 8: DEPENDENCY VETTING

### 8.1 cargo-vet

**Controlled classification MUST vet all dependencies:**

```bash
# Install cargo-vet
cargo install cargo-vet --locked

# Initialize
cargo vet init

# Check all dependencies
cargo vet

# Import trusted audits from organizations
cargo vet trust --all Mozilla
cargo vet trust --all Google
cargo vet trust --all Rust

# Record manual audit
cargo vet certify my-dependency 1.2.3
```

### 8.2 cargo-audit

```bash
# Install
cargo install cargo-audit --locked

# Check for vulnerabilities
cargo audit

# Generate report
cargo audit --json > audit-report.json
```

### 8.3 cargo-deny

```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
copyleft = "deny"

[bans]
multiple-versions = "warn"
wildcards = "deny"
```

```bash
cargo install cargo-deny --locked
cargo deny check
```

### 8.4 cargo-geiger

**Track unsafe code in dependencies:**

```bash
# Install
cargo install cargo-geiger --locked

# Report unsafe usage
cargo geiger --all-features

# Output format for CI
cargo geiger --all-features --output-format Json > geiger-report.json
```

---

## SECTION 9: NOTHING LOST, EVER (PRINCIPLE 9)

### 9.1 Persistence Backend Configuration

```toml
# Application config.toml for persistence

[persistence]
# Backend selection
backend = "sled"  # or "rocksdb"

[persistence.sled]
path = "/var/lib/xonaix/data"
# CRITICAL: flush on every write for durability
flush_every_ms = 0  # 0 = flush immediately (fsync)
# Cache size
cache_capacity = 1073741824  # 1GB

[persistence.rocksdb]
path = "/var/lib/xonaix/rocksdb"
# WAL for durability
wal_dir = "/var/lib/xonaix/rocksdb/wal"
# Sync writes
sync = true  # CRITICAL for Principle 9

[messaging]
# NATS configuration for durable messaging
nats_url = "nats://localhost:4222"
# JetStream for persistence
jetstream = true
# Stream configuration
stream_replicas = 3
```

### 9.2 Sled Dependency Configuration

```toml
[dependencies]
# Sled for embedded persistence
sled = { version = "0.34", features = ["compression"] }

# Alternative: RocksDB
# rocksdb = { version = "0.21", features = ["snappy"] }
```

---

## SECTION 10: SECURITY

### 10.1 No Secrets in TOML

```toml
# WRONG: Secrets in config - SEVERE VIOLATION
[database]
password = "super_secret_password"

# CORRECT: Reference environment variable
# (handle in application code)
[database]
# Database URL should be set via DATABASE_URL environment variable

# CORRECT: Document that env var is needed
[database]
url_env = "DATABASE_URL"  # Application reads from env
```

### 10.2 CI Security Checks

```yaml
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
    
- name: License Check
  run: |
    cargo install cargo-deny --locked
    cargo deny check licenses
```

---

## SECTION 11: COMMENTS AND DOCUMENTATION

### 11.1 Section Comments

```toml
# =============================================================================
# Server Configuration
# =============================================================================
[server]
# The host address to bind to
# Use 0.0.0.0 to listen on all interfaces
host = "0.0.0.0"

# The port to listen on
# Requires CAP_NET_BIND_SERVICE for ports < 1024
port = 8080

# =============================================================================
# Database Configuration
# =============================================================================
[database]
# Maximum number of connections in the pool
# Higher values use more memory but handle more concurrent requests
# Formula: (cores * 2) + disk_spindles
max_connections = 20
```

### 11.2 Inline Comments

```toml
[dependencies]
# Core async runtime - full features for tokio::main and tokio::test
tokio = { version = "1", features = ["full"] }

# Web framework - using 0.7 for latest features
axum = "0.7"

# Database - postgres feature required for production
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# Cryptography - aligned with STANDARDS_RUST.md
blake3 = "1.5"       # Primary hash function
ed25519-dalek = "2"  # Classical signatures
zeroize = "1.7"      # Secure memory cleanup
```

---

## SECTION 12: COMMON PATTERNS

### 12.1 Feature-Gated Dependencies

```toml
[features]
default = []
postgres = ["sqlx/postgres"]
mysql = ["sqlx/mysql"]
sqlite = ["sqlx/sqlite"]
all-databases = ["postgres", "mysql", "sqlite"]

[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio"], optional = true }
```

### 12.2 Platform-Specific Dependencies

```toml
[target.'cfg(unix)'.dependencies]
nix = "0.27"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = ["Win32_Foundation"] }

[target.'cfg(target_os = "linux")'.dependencies]
procfs = "0.16"
```

### 12.3 Binary-Specific Dependencies

```toml
[[bin]]
name = "xonaix-cli"
path = "src/bin/cli.rs"
required-features = ["cli"]

[features]
cli = ["clap"]

[dependencies]
clap = { version = "4", features = ["derive"], optional = true }
```

---

## CI Pipeline

```yaml
name: TOML Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Check Cargo.toml syntax
        run: cargo check
        
      - name: Verify Lockfile
        run: cargo check --locked
        
      - name: Check MSRV
        run: |
          MSRV=$(grep "rust-version" Cargo.toml | cut -d'"' -f2)
          rustup install $MSRV
          cargo +$MSRV check
          
      - name: Lint Check
        run: cargo clippy --all-targets --all-features -- -D warnings
        
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
          
      - name: License Check
        run: |
          cargo install cargo-deny --locked
          cargo deny check
```

---

## X.I. Prompt Appendix

```
TOML v5.0.0 REQUIREMENTS:

NOTE: TOML is primarily for Rust configuration (Cargo.toml).
Use for Rust projects, prefer YAML for other configurations.

CARGO.TOML:
- Standard section ordering (package -> lints -> features -> deps -> profiles)
- Workspace dependencies for monorepos
- Explicit version requirements
- REQUIRED lints configuration (aligned with STANDARDS_RUST.md)

REQUIRED LINTS:
[lints.rust]
unsafe_code = "forbid"
unused_must_use = "deny"
unused_results = "deny"
missing_docs = "deny"

[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
cognitive_complexity = "warn"

MSRV POLICY:
- MUST specify rust-version
- Controlled: stable - 1
- Production: stable - 2
- Development: stable - 3

CARGO.LOCK POLICY:
- Binaries: MUST commit
- Applications: MUST commit
- Libraries: MAY commit (recommended)

DEPENDENCY VETTING (Controlled):
- cargo-vet MUST
- cargo-audit MUST
- cargo-geiger MUST
- cargo-deny SHOULD

PROFILE SECURITY:
- panic = "abort" in release (prevents unwinding exploits)
- strip = true in release (reduces attack surface)
- overflow-checks = true for Controlled

PRINCIPLE 9 (NOTHING LOST):
- sled/rocksdb with flush_every_ms = 0 (fsync)
- rocksdb sync = true
- NATS JetStream enabled

FORBIDDEN:
NO Secrets in configuration
NO Wildcard versions in production (version = "*")
NO Missing [lints] section
NO Unpinned git dependencies (use tag or rev, not branch)
NO Missing rust-version
NO Missing Cargo.lock for binaries

FLAG THESE VIOLATIONS:
NO Missing [lints] section
NO version = "*"
NO Credentials in config
NO Missing Cargo.lock for binary
NO Git dependency without tag/rev
NO Missing rust-version
NO Unvetted dependencies for Controlled
```

---

## Quick Reference

### Required Cargo.toml Sections

```toml
[package]
name = "..."
version = "..."
edition = "2021"
rust-version = "1.75"  # MUST specify

[lints.rust]
unsafe_code = "forbid"  # MUST include

[lints.clippy]
unwrap_used = "deny"    # MUST include
```

### Forbidden Patterns

```toml
# No lints section
# (missing [lints] entirely)

# Wildcard version
serde = "*"

# Unpinned git
my_crate = { git = "...", branch = "main" }

# Secrets
password = "secret123"

# Missing rust-version
# (no rust-version in [package])
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L4 Configuration)
- **MAJOR:** Added Security-Affecting Values section
- **MAJOR:** Added Numeric Policy section
- **MAJOR:** Added XCLib Integration section
- **UPDATED:** Core-Compatible to 5.7.0
- **ALIGNED:** Cross-language requirements per STANDARDS_INDEX B-5.8.5
- **Source:** Red-Blue-Black Team synthesis with Founder approval

### v5.0.0 - Core-Compatible 5.1.0 (December 2025)
- **REVIEWED:** Rainbow Team compatibility review
- **CONFIRMED:** No content changes required for Core 5.1.0 compatibility
- **NOTE:** Deviation recording syntax compatible with cryptographic enforcement model

### v5.0.0 (December 2025)
- **ALIGNED:** Core-Version updated to 5.0.0
- **ADDED:** Principle 9 mapping (persistence configs, sled, rocksdb, fsync)
- **ADDED:** Deviation Recording syntax
- **ADDED:** Complete lints section aligned with STANDARDS_RUST.md
- **ADDED:** MSRV policy by classification
- **ADDED:** Cargo.lock commit policy
- **ADDED:** cargo-vet requirements for Controlled
- **ADDED:** cargo-geiger requirements for Controlled
- **ADDED:** Profile security settings explanation
- **ADDED:** Workspace configuration patterns
- **ADDED:** Complete CI pipeline example
- **UPDATED:** X.I. Prompt Appendix with v5.0.0 requirements

### v0.1.0 (December 2025)
- Initial draft
- Based on The Xonaix Way v1.1.0

---

*TOML Standards B-5.8.5 — Part of The Xonaix Way B-5.8.5*

*"Cargo.toml is your project's contract. Security values require signed manifests."*

*Xonaix, Inc. — Intelligence, evolved.*
