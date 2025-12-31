---
schema: "xonaix-document-header"
schema_version: "2.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/nats/STANDARDS_NATS.md"
unit_id: "library/standards/nats"
title: "NATS Messaging Standard"
document_type: "standard"
language: "en"

# --- Version ---
version: "XLIB-1.0.0"
baseline: null
status: "active"

# --- Classification ---
trust_class: "L2"
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
last_updated: "2025-12-31T22:00:00Z"
---

# NATS Messaging Standard

NATS is the canonical messaging system for Xonaix, providing durable, high-performance messaging with JetStream persistence.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.


---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L3 |
| Classification | Transport |

NATS is **transport only**. It cannot decide truth. Messages are envelopes — authority comes from the signatures inside them.

### What L3 May Do

- Transport messages between components
- Provide persistence via JetStream
- Deliver messages reliably
- Queue and replay messages

### What L3 May NOT Do

- Assert authority based on message content
- Verify signatures (must delegate to consumer)
- Decide governance outcomes
- Trust message headers as authoritative

---

## Authority vs Transport (CRITICAL)

**Authority:** Constitution Article I, §4 — Zero Trust

NATS messages causing state changes MUST:

1. **Carry XCLib hybrid signature** in message payload
2. **Include actor identity** (cryptographically verified)
3. **Include intent envelope** (what action is being taken)
4. **Be verified BEFORE processing** (not after, not during)

### Prohibited Pattern

```rust
// FORBIDDEN: Trust message content directly
nats.subscribe("governance.decision", |msg| {
    apply_decision(msg.data);  // NO! Must verify first
});
```

### Required Pattern

```rust
// CORRECT: Verify before apply
nats.subscribe("governance.decision", async |msg| {
    // Verify signature with XCLib
    let verified = xclib::verify(&msg.data)?;
    if !verified.valid {
        return Err(NatsError::InvalidSignature);
    }
    
    // Verify capability
    if !verified.has_capability(required_class) {
        return Err(NatsError::InsufficientCapability);
    }
    
    // NOW apply
    apply_decision(verified.payload).await?;
});
```

### Governance Subjects

Subjects for Tier ≥3 operations require:

| Requirement | Implementation |
|-------------|----------------|
| Capability attestation | Included in message envelope |
| Replay protection | Nonce or sequence number |
| Signature verification | At consumer, before processing |
| Audit logging | All governance messages logged |

---

## XCLib Integration

NATS itself does not integrate with XCLib. Instead:

1. **Producers** sign messages with XCLib before publishing
2. **Consumers** verify signatures with XCLib before processing
3. **NATS** is a dumb pipe — it just moves bytes

### Message Envelope Pattern

```rust
pub struct GovernanceMessage {
    pub payload: Vec<u8>,           // The actual data
    pub signature: XonaixSignature, // XCLib signature
    pub actor_id: ActorId,          // Who sent this
    pub intent: Intent,             // What are they trying to do
    pub nonce: u64,                 // Replay protection
    pub timestamp: i64,             // When sent
}
```

---

## Numeric Policy

NATS message content follows the same numeric policy:

- **No floats** in governance message payloads
- **Integer or string** representations for canonical data
- **Serialization** must be deterministic (use canonical JSON or CBOR)

---

## Error Handling

NATS errors should:

1. **Not leak** internal state or configuration
2. **Map to bounded codes** at API boundaries
3. **Log internally** with full details
4. **Return generic errors** to clients

---

## Principle Mapping

| Principle | NATS Implementation |
|-----------|---------------------|
| 1. Correct Over Fast | Exactly-once delivery where required, ordered streams |
| 2. Secure By Default | TLS required, authentication mandatory, authorization via accounts |
| 3. Fail Loud | Dead letter queues, explicit error subjects, alerting on failures |
| 4. Explicit Over Implicit | Explicit ACK policies, explicit retention, explicit consumer configs |
| 5. Automated Over Vigilant | Health checks, auto-reconnect, automated failover |
| 6. Composable Over Clever | Simple subject hierarchy, standard message envelopes |
| 7. X.I. Augments, Human Decides | Configuration changes require human review |
| 8. Future-Proof Over Trend | JetStream (stable), avoid experimental features |
| 9. Nothing Lost, Ever | **PRIMARY IMPLEMENTATION** — JetStream persistence, explicit ACK, dead letter handling |

---

## Deviation Recording

For deviations from MUST requirements in NATS configuration:

```yaml
# XONAIX_DEVIATION: [Reason for deviation - be specific]
# LEDGER_ACK: [User_Signature_Hash]
# Example: Using AckNone for high-volume telemetry
ack_policy: none  # Deviating from explicit ACK requirement
```

**Blade Enforcement:** Blade scans NATS configuration for deviation markers and records them in the Security Ledger.

---

## SECTION 1: ARCHITECTURE OVERVIEW

### 1.1 NATS in Xonaix Ecosystem

```
┌─────────────────────────────────────────────────────────────────────┐
│                         XONAIX ECOSYSTEM                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐       │
│  │  Forge  │     │ Cortex  │     │  Blade  │     │  Nexus  │       │
│  └────┬────┘     └────┬────┘     └────┬────┘     └────┬────┘       │
│       │               │               │               │             │
│       └───────────────┴───────────────┴───────────────┘             │
│                               │                                     │
│                    ┌──────────▼──────────┐                          │
│                    │   NATS JetStream    │                          │
│                    │  ┌───────────────┐  │                          │
│                    │  │  GOVERNANCE   │  │  ← Tier 3-4 decisions    │
│                    │  │  CORTEX       │  │  ← Agent events          │
│                    │  │  FORGE        │  │  ← Workflow status       │
│                    │  │  AUDIT        │  │  ← Immutable audit log   │
│                    │  └───────────────┘  │                          │
│                    │  ┌───────────────┐  │                          │
│                    │  │   KV Stores   │  │                          │
│                    │  │  config       │  │  ← Runtime configuration │
│                    │  │  positions    │  │  ← Consumer positions    │
│                    │  └───────────────┘  │                          │
│                    └─────────────────────┘                          │
│                               │                                     │
│              ┌────────────────┼────────────────┐                    │
│              │                │                │                    │
│         ┌────▼────┐     ┌─────▼─────┐    ┌────▼────┐               │
│         │ NATS-1  │     │  NATS-2   │    │ NATS-3  │               │
│         │ (Leader)│     │ (Follower)│    │(Follower)│               │
│         └─────────┘     └───────────┘    └─────────┘               │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.2 Communication Patterns

| Pattern | Use Case | JetStream Required |
|---------|----------|-------------------|
| **Publish-Subscribe** | Broadcasts, notifications | No (core NATS) |
| **Request-Reply** | Synchronous queries | No (core NATS) |
| **Queue Groups** | Load-balanced workers | No (core NATS) |
| **Streaming** | Durable message delivery | **Yes** |
| **Key-Value** | Configuration, caching | **Yes** |
| **Object Store** | Large artifacts | Yes (deferred) |

### 1.3 Client Selection

| Language | Client | Use Case |
|----------|--------|----------|
| **Rust** | `async-nats` | Primary — all server-side components |
| **TypeScript** | `nats.js` | Secondary — only if Nexus UI needs direct access |

**Note:** Nexus UI typically communicates via Nexus API (HTTP/WebSocket), not directly via NATS. Direct NATS access from browser is rare and requires WebSocket bridge.

---

## SECTION 2: SUBJECT HIERARCHY

### 2.1 Subject Naming Convention

**Pattern:** `xonaix.{domain}.{entity}.{action}`

```
xonaix.                    # Root namespace
├── governance.            # Governance domain
│   ├── tier1.             # Tier 1 decisions
│   │   ├── proposed       # Decision proposed
│   │   ├── approved       # Decision approved
│   │   └── rejected       # Decision rejected
│   ├── tier2.
│   ├── tier3.
│   └── tier4.
│       ├── proposed
│       ├── voting         # Active voting
│       ├── approved
│       └── rejected
├── cortex.                # Cortex domain
│   ├── agent.             # Agent events
│   │   ├── {agent_id}.    # Specific agent
│   │   │   ├── created
│   │   │   ├── updated
│   │   │   ├── context.added
│   │   │   └── context.pruned
│   │   └── *              # Wildcard for all agents
│   └── memory.
│       ├── stored
│       └── retrieved
├── forge.                 # Forge domain
│   ├── workflow.          # Workflow events
│   │   ├── {workflow_id}.
│   │   │   ├── started
│   │   │   ├── step.completed
│   │   │   ├── step.failed
│   │   │   ├── completed
│   │   │   └── failed
│   └── task.
│       ├── queued
│       ├── started
│       └── completed
├── blade.                 # Blade domain
│   ├── validation.
│   │   ├── passed
│   │   └── failed
│   └── enforcement.
│       ├── warning
│       └── blocked
├── audit.                 # Audit domain (append-only)
│   ├── security.
│   │   ├── auth.success
│   │   ├── auth.failure
│   │   └── access.denied
│   └── governance.
│       ├── decision
│       └── override
└── system.                # System domain
    ├── health.
    │   ├── heartbeat
    │   └── status
    └── config.
        └── reload
```

### 2.2 Subject Naming Rules

| Rule | Example | Rationale |
|------|---------|-----------|
| Lowercase only | `xonaix.governance` | Consistency |
| Dots for hierarchy | `xonaix.cortex.agent` | Standard NATS convention |
| No spaces or special chars | `workflow_id` not `workflow-id` | Compatibility |
| Entity IDs in path | `xonaix.cortex.agent.{uuid}` | Enables filtering |
| Actions are verbs/past-tense | `created`, `approved`, `failed` | Clarity |

### 2.3 Wildcards

```rust
// Subscribe to all governance events
"xonaix.governance.>"

// Subscribe to all Tier 4 events
"xonaix.governance.tier4.*"

// Subscribe to specific agent's events
"xonaix.cortex.agent.{agent_id}.*"

// Subscribe to all workflow completions
"xonaix.forge.workflow.*.completed"
```

**Wildcard Types:**
- `*` — Matches single token
- `>` — Matches one or more tokens (must be last)

---

## SECTION 3: JETSTREAM CONFIGURATION

### 3.1 Stream Definitions

#### GOVERNANCE Stream

```rust
use async_nats::jetstream::{self, stream};

async fn create_governance_stream(js: &jetstream::Context) -> Result<stream::Stream, Error> {
    js.get_or_create_stream(stream::Config {
        name: "GOVERNANCE".to_string(),
        description: Some("Governance decisions and voting".to_string()),
        subjects: vec![
            "xonaix.governance.>".to_string(),
        ],
        retention: stream::RetentionPolicy::Limits,
        max_messages: 10_000_000,
        max_bytes: 10 * 1024 * 1024 * 1024,  // 10 GB
        max_age: std::time::Duration::from_secs(365 * 24 * 60 * 60),  // 1 year
        max_message_size: 1024 * 1024,  // 1 MB
        storage: stream::StorageType::File,
        num_replicas: 3,  // Controlled classification
        duplicate_window: std::time::Duration::from_secs(120),
        ..Default::default()
    })
    .await
}
```

#### CORTEX Stream

```rust
async fn create_cortex_stream(js: &jetstream::Context) -> Result<stream::Stream, Error> {
    js.get_or_create_stream(stream::Config {
        name: "CORTEX".to_string(),
        description: Some("Cortex agent and memory events".to_string()),
        subjects: vec![
            "xonaix.cortex.>".to_string(),
        ],
        retention: stream::RetentionPolicy::Limits,
        max_messages: 100_000_000,
        max_bytes: 50 * 1024 * 1024 * 1024,  // 50 GB
        max_age: std::time::Duration::from_secs(90 * 24 * 60 * 60),  // 90 days
        storage: stream::StorageType::File,
        num_replicas: 3,
        ..Default::default()
    })
    .await
}
```

#### FORGE Stream

```rust
async fn create_forge_stream(js: &jetstream::Context) -> Result<stream::Stream, Error> {
    js.get_or_create_stream(stream::Config {
        name: "FORGE".to_string(),
        description: Some("Forge workflow and task events".to_string()),
        subjects: vec![
            "xonaix.forge.>".to_string(),
        ],
        retention: stream::RetentionPolicy::Limits,
        max_messages: 50_000_000,
        max_bytes: 20 * 1024 * 1024 * 1024,  // 20 GB
        max_age: std::time::Duration::from_secs(30 * 24 * 60 * 60),  // 30 days
        storage: stream::StorageType::File,
        num_replicas: 3,
        ..Default::default()
    })
    .await
}
```

#### AUDIT Stream (Immutable)

```rust
async fn create_audit_stream(js: &jetstream::Context) -> Result<stream::Stream, Error> {
    js.get_or_create_stream(stream::Config {
        name: "AUDIT".to_string(),
        description: Some("Immutable audit log - WORM storage".to_string()),
        subjects: vec![
            "xonaix.audit.>".to_string(),
        ],
        retention: stream::RetentionPolicy::Limits,
        max_messages: -1,  // Unlimited
        max_bytes: -1,     // Unlimited
        max_age: std::time::Duration::ZERO,  // Never expire
        storage: stream::StorageType::File,
        num_replicas: 3,
        // CRITICAL: Deny delete and purge for WORM compliance
        deny_delete: true,
        deny_purge: true,
        ..Default::default()
    })
    .await
}
```

### Audit Hash Policy (Principle 9)

Hash algorithms are tiered by durability requirements:

| Context | Algorithm | Security | Rationale |
|---------|-----------|----------|-----------|
| Performance paths | BLAKE3 (256-bit) | 128-bit post-quantum | Speed-critical, ephemeral data |
| AUDIT stream | SHA3-512 (512-bit) | 256-bit post-quantum | Long-term retention, regulatory |
| Tier 3-4 governance | SHA3-512 (512-bit) | 256-bit post-quantum | Constitutional significance |

**Phasing:**
- BLAKE3 for performance paths: Immediate (default)
- SHA3-512 for AUDIT/Tier 3-4: SHOULD by 2026, MUST by 2027

**Rationale:** Long-lived audit chains (7+ year retention) may still be valid when cryptographically-relevant quantum computers arrive. SHA3-512 provides adequate margin; BLAKE3 remains acceptable for ephemeral data.

### 3.2 Stream Configuration by Classification

| Stream | Classification | Replicas | Retention | Delete/Purge |
|--------|---------------|----------|-----------|--------------|
| GOVERNANCE | Controlled | 3 | 1 year | Allowed |
| CORTEX | Production | 3 | 90 days | Allowed |
| FORGE | Production | 3 | 30 days | Allowed |
| AUDIT | Controlled | 3 | **Infinite** | **Denied** |

### 3.3 Development vs Production

```rust
fn stream_replicas(classification: Classification) -> usize {
    match classification {
        Classification::Development => 1,
        Classification::Production => 3,
        Classification::Controlled => 3,
    }
}
```

---

## SECTION 4: CONSUMER CONFIGURATION

### 4.1 Consumer Types

| Type | Use Case | Durability |
|------|----------|------------|
| **Durable** | Critical processing, must survive restarts | Persisted |
| **Ephemeral** | Temporary monitoring, development | In-memory |

### 4.2 ACK Policies

| Policy | Description | Use Case |
|--------|-------------|----------|
| **Explicit** | Must ACK each message | **Default — Principle 9** |
| **None** | No ACK required | High-volume telemetry only (with deviation) |
| **All** | ACK all up to sequence | Batch processing |

**MUST use Explicit ACK for:**
- Governance messages
- Audit events
- Workflow state transitions
- Any message where loss is unacceptable

### 4.3 Durable Consumer Example

```rust
use async_nats::jetstream::consumer::{self, PullConsumer};

async fn create_governance_consumer(
    stream: &jetstream::stream::Stream,
) -> Result<PullConsumer, Error> {
    stream
        .get_or_create_consumer(
            "governance-processor",
            consumer::pull::Config {
                durable_name: Some("governance-processor".to_string()),
                description: Some("Processes governance decisions".to_string()),
                ack_policy: consumer::AckPolicy::Explicit,
                ack_wait: std::time::Duration::from_secs(30),
                max_deliver: 5,  // Retry up to 5 times
                filter_subject: "xonaix.governance.>".to_string(),
                // Dead letter after max_deliver exceeded
                // Handled via advisory messages
                ..Default::default()
            },
        )
        .await
}
```

### 4.4 Processing Pattern with ACK

```rust
use async_nats::jetstream::consumer::PullConsumer;
use async_nats::jetstream::Message;

async fn process_messages(consumer: PullConsumer) -> Result<(), ProcessError> {
    let mut messages = consumer.messages().await?;
    
    const MAX_MESSAGES: usize = 10_000;
    let mut count = 0;
    
    while let Some(message) = messages.next().await {
        // Bounded iteration (NASA Rule 2)
        if count >= MAX_MESSAGES {
            tracing::warn!("Reached max messages per batch: {}", MAX_MESSAGES);
            break;
        }
        count += 1;
        
        let msg = message.map_err(|e| ProcessError::Receive(e.to_string()))?;
        
        match process_single_message(&msg).await {
            Ok(()) => {
                // PRINCIPLE 9: Explicit ACK after successful processing
                msg.ack().await.map_err(|e| ProcessError::Ack(e.to_string()))?;
            }
            Err(e) if e.is_retryable() => {
                // NAK for retry (will be redelivered)
                msg.nak().await.map_err(|e| ProcessError::Nak(e.to_string()))?;
                tracing::warn!("Message NAK'd for retry: {}", e);
            }
            Err(e) => {
                // Terminal failure - send to dead letter
                handle_dead_letter(&msg, &e).await?;
                // ACK to prevent infinite retry
                msg.ack().await.map_err(|e| ProcessError::Ack(e.to_string()))?;
            }
        }
    }
    
    Ok(())
}

async fn process_single_message(msg: &Message) -> Result<(), ProcessError> {
    // Precondition
    debug_assert!(!msg.payload.is_empty(), "Message payload must not be empty");
    
    let envelope: MessageEnvelope = serde_json::from_slice(&msg.payload)
        .map_err(|e| ProcessError::Deserialize(e.to_string()))?;
    
    // Process based on message type...
    
    // Postcondition
    debug_assert!(envelope.processed_at.is_some(), "Message must be marked processed");
    
    Ok(())
}
```

---

## SECTION 5: MESSAGE ENVELOPE

### 5.1 Standard Envelope

All NATS messages MUST use this envelope:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Standard message envelope for all NATS messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope<T> {
    /// Unique message identifier
    pub id: Uuid,
    
    /// Message schema version (for evolution)
    pub version: String,
    
    /// ISO 8601 timestamp of creation
    pub timestamp: DateTime<Utc>,
    
    /// Source component
    pub source: String,
    
    /// Correlation ID for tracing
    pub correlation_id: Option<Uuid>,
    
    /// Causation ID (ID of message that caused this)
    pub causation_id: Option<Uuid>,
    
    /// Message payload
    pub payload: T,
    
    /// Optional metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

impl<T> MessageEnvelope<T> {
    pub fn new(source: impl Into<String>, payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            version: "1.0".to_string(),
            timestamp: Utc::now(),
            source: source.into(),
            correlation_id: None,
            causation_id: None,
            payload,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    pub fn with_causation(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
}
```

### 5.2 Governance Message Example

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceDecision {
    pub decision_id: Uuid,
    pub tier: GovernanceTier,
    pub action: GovernanceAction,
    pub subject: String,
    pub proposer: String,
    pub votes: Option<Vec<Vote>>,
    pub outcome: Option<DecisionOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceAction {
    Proposed,
    Voting,
    Approved,
    Rejected,
}

// Publishing a governance decision
async fn publish_governance_decision(
    client: &async_nats::Client,
    decision: GovernanceDecision,
) -> Result<(), PublishError> {
    let envelope = MessageEnvelope::new("forge", decision.clone());
    
    let subject = format!(
        "xonaix.governance.{}.{}",
        decision.tier.as_str(),
        decision.action.as_str()
    );
    
    let payload = serde_json::to_vec(&envelope)
        .map_err(|e| PublishError::Serialize(e.to_string()))?;
    
    client
        .publish(subject, payload.into())
        .await
        .map_err(|e| PublishError::Nats(e.to_string()))?;
    
    Ok(())
}
```

---

## SECTION 6: KEY-VALUE STORE

### 6.1 KV Buckets

| Bucket | Purpose | TTL | History |
|--------|---------|-----|---------|
| `config` | Runtime configuration | None | 5 revisions |
| `positions` | Consumer position caching | None | 1 revision |
| `sessions` | Active sessions | 24h | 1 revision |
| `locks` | Distributed locks | 5m | 1 revision |

### 6.2 KV Operations

```rust
use async_nats::jetstream::kv::{self, Store};

/// Configuration KV store
async fn create_config_bucket(js: &jetstream::Context) -> Result<Store, Error> {
    js.get_or_create_key_value(kv::Config {
        bucket: "config".to_string(),
        description: Some("Runtime configuration".to_string()),
        history: 5,
        max_value_size: 1024 * 1024,  // 1 MB
        storage: kv::StorageType::File,
        num_replicas: 3,
        ..Default::default()
    })
    .await
}

/// Get configuration value
async fn get_config<T: serde::de::DeserializeOwned>(
    kv: &Store,
    key: &str,
) -> Result<Option<T>, ConfigError> {
    match kv.get(key).await {
        Ok(entry) => {
            let value: T = serde_json::from_slice(&entry.value)
                .map_err(|e| ConfigError::Deserialize(e.to_string()))?;
            Ok(Some(value))
        }
        Err(e) if e.to_string().contains("key not found") => Ok(None),
        Err(e) => Err(ConfigError::Kv(e.to_string())),
    }
}

/// Set configuration value
async fn set_config<T: serde::Serialize>(
    kv: &Store,
    key: &str,
    value: &T,
) -> Result<u64, ConfigError> {
    let payload = serde_json::to_vec(value)
        .map_err(|e| ConfigError::Serialize(e.to_string()))?;
    
    kv.put(key, payload.into())
        .await
        .map_err(|e| ConfigError::Kv(e.to_string()))
}

/// Watch configuration changes
async fn watch_config(
    kv: &Store,
    key_pattern: &str,
) -> Result<impl futures::Stream<Item = kv::Entry>, ConfigError> {
    kv.watch(key_pattern)
        .await
        .map_err(|e| ConfigError::Kv(e.to_string()))
}
```

### 6.3 Distributed Locks

```rust
use std::time::Duration;

/// Acquire distributed lock
async fn acquire_lock(
    kv: &Store,
    lock_name: &str,
    holder: &str,
    ttl: Duration,
) -> Result<bool, LockError> {
    let lock_key = format!("lock.{}", lock_name);
    
    // Try to create (will fail if exists)
    match kv.create(&lock_key, holder.as_bytes().into()).await {
        Ok(_) => {
            // Set TTL by scheduling cleanup
            // Note: NATS KV doesn't have per-key TTL, use bucket TTL or manual cleanup
            Ok(true)
        }
        Err(e) if e.to_string().contains("key exists") => {
            Ok(false)
        }
        Err(e) => Err(LockError::Kv(e.to_string())),
    }
}

/// Release distributed lock
async fn release_lock(
    kv: &Store,
    lock_name: &str,
    holder: &str,
) -> Result<bool, LockError> {
    let lock_key = format!("lock.{}", lock_name);
    
    // Verify we hold the lock
    match kv.get(&lock_key).await {
        Ok(entry) => {
            if entry.value == holder.as_bytes() {
                kv.delete(&lock_key)
                    .await
                    .map_err(|e| LockError::Kv(e.to_string()))?;
                Ok(true)
            } else {
                Ok(false)  // Not our lock
            }
        }
        Err(_) => Ok(false),  // Lock doesn't exist
    }
}
```

---

## SECTION 7: DEAD LETTER HANDLING

### 7.1 Dead Letter Strategy

```rust
/// Dead letter subject pattern
const DEAD_LETTER_SUBJECT: &str = "xonaix.dlq";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetter<T> {
    /// Original message
    pub original: MessageEnvelope<T>,
    
    /// Original subject
    pub original_subject: String,
    
    /// Failure reason
    pub reason: String,
    
    /// Number of delivery attempts
    pub attempts: u32,
    
    /// When it was dead-lettered
    pub dead_lettered_at: DateTime<Utc>,
    
    /// Processing error details
    pub error_details: Option<String>,
}

async fn handle_dead_letter<T: Serialize>(
    client: &async_nats::Client,
    original_msg: &Message,
    original_subject: &str,
    error: &ProcessError,
) -> Result<(), DeadLetterError> {
    // Parse original envelope
    let original: MessageEnvelope<T> = serde_json::from_slice(&original_msg.payload)
        .map_err(|e| DeadLetterError::Deserialize(e.to_string()))?;
    
    let dead_letter = DeadLetter {
        original,
        original_subject: original_subject.to_string(),
        reason: error.to_string(),
        attempts: original_msg.info()
            .map(|i| i.delivered as u32)
            .unwrap_or(1),
        dead_lettered_at: Utc::now(),
        error_details: Some(format!("{:?}", error)),
    };
    
    let payload = serde_json::to_vec(&dead_letter)
        .map_err(|e| DeadLetterError::Serialize(e.to_string()))?;
    
    // Publish to dead letter queue
    let dlq_subject = format!("{}.{}", DEAD_LETTER_SUBJECT, original_subject);
    
    client
        .publish(dlq_subject, payload.into())
        .await
        .map_err(|e| DeadLetterError::Publish(e.to_string()))?;
    
    tracing::error!(
        message_id = %dead_letter.original.id,
        subject = %dead_letter.original_subject,
        reason = %dead_letter.reason,
        attempts = %dead_letter.attempts,
        "Message sent to dead letter queue"
    );
    
    Ok(())
}
```

### 7.2 Dead Letter Stream

```rust
async fn create_dlq_stream(js: &jetstream::Context) -> Result<stream::Stream, Error> {
    js.get_or_create_stream(stream::Config {
        name: "DLQ".to_string(),
        description: Some("Dead letter queue for failed messages".to_string()),
        subjects: vec!["xonaix.dlq.>".to_string()],
        retention: stream::RetentionPolicy::Limits,
        max_age: std::time::Duration::from_secs(30 * 24 * 60 * 60),  // 30 days
        storage: stream::StorageType::File,
        num_replicas: 3,
        ..Default::default()
    })
    .await
}
```

### 7.3 Dead Letter Alerting

```rust
/// Monitor dead letter queue and alert
async fn monitor_dlq(
    js: &jetstream::Context,
    alert_threshold: usize,
) -> Result<(), MonitorError> {
    let stream = js.get_stream("DLQ").await?;
    let info = stream.info().await?;
    
    if info.state.messages as usize > alert_threshold {
        tracing::error!(
            messages = info.state.messages,
            threshold = alert_threshold,
            "Dead letter queue threshold exceeded"
        );
        
        // Trigger alert (implementation depends on alerting system)
        send_alert(AlertLevel::Critical, &format!(
            "DLQ contains {} messages (threshold: {})",
            info.state.messages,
            alert_threshold
        )).await?;
    }
    
    Ok(())
}
```

---

## SECTION 8: SECURITY



### Subject Authorization Policy

NATS authorization SHALL follow least-privilege principles:

**Publish Permissions:**
- Services SHALL publish only to their designated subject prefixes
- Per-tenant/workflow scoping REQUIRED (e.g., `xonaix.forge.workflow.{tenant_id}.{workflow_id}.*`)
- Broad publishes (`>`) on governance/forge subjects are FORBIDDEN
- Exception: System health broadcasts to designated monitoring subjects

**Subscribe Permissions:**
- Services MAY subscribe to subjects required for their function
- Validators (Blade accounts) MAY subscribe broadly for monitoring and enforcement
- Wide subscriptions for audit/compliance tooling are permitted

**Account-Level Controls:**
- Each service identity bound to allowed publish prefixes
- Authorization changes require security review
- ACL changes logged to AUDIT stream

**Violations:**
- Unauthorized publish attempts -> reject + alert
- Repeated violations -> service isolation (ejection protocol)

### 8.1 TLS Configuration

**MUST use TLS for all production connections:**

```rust
use async_nats::ConnectOptions;

async fn connect_secure(
    servers: &[&str],
    cert_path: &str,
    key_path: &str,
    ca_path: &str,
) -> Result<async_nats::Client, ConnectError> {
    let client = ConnectOptions::new()
        .add_client_certificate(
            std::path::Path::new(cert_path),
            std::path::Path::new(key_path),
        )
        .add_root_certificates(std::path::Path::new(ca_path))
        .require_tls(true)
        .connect(servers)
        .await
        .map_err(|e| ConnectError::Tls(e.to_string()))?;
    
    Ok(client)
}
```

### 8.2 Authentication

```rust
/// Connect with credentials
async fn connect_with_auth(
    servers: &[&str],
    creds_path: &str,
) -> Result<async_nats::Client, ConnectError> {
    let client = ConnectOptions::new()
        .credentials_file(creds_path)
        .await
        .map_err(|e| ConnectError::Credentials(e.to_string()))?
        .connect(servers)
        .await
        .map_err(|e| ConnectError::Connect(e.to_string()))?;
    
    Ok(client)
}

/// Connect with NKey
async fn connect_with_nkey(
    servers: &[&str],
    seed: &str,
) -> Result<async_nats::Client, ConnectError> {
    let key_pair = nkeys::KeyPair::from_seed(seed)
        .map_err(|e| ConnectError::NKey(e.to_string()))?;
    
    let client = ConnectOptions::new()
        .nkey(key_pair.public_key(), move |nonce| {
            key_pair.sign(nonce).map(|sig| sig.to_vec())
        })
        .connect(servers)
        .await
        .map_err(|e| ConnectError::Connect(e.to_string()))?;
    
    Ok(client)
}
```

### 8.3 Authorization (Account-Based)

```conf
# nats-server.conf - Account-based authorization

accounts {
  FORGE {
    jetstream: enabled
    users: [
      { user: forge, password: $FORGE_PASSWORD }
    ]
    # Can publish to forge.* and governance.*
    # Can subscribe to all xonaix.*
    exports: [
      { stream: "xonaix.forge.>" }
      { stream: "xonaix.governance.>" }
    ]
    imports: [
      { stream: { account: CORTEX, subject: "xonaix.cortex.>" } }
    ]
  }
  
  CORTEX {
    jetstream: enabled
    users: [
      { user: cortex, password: $CORTEX_PASSWORD }
    ]
    exports: [
      { stream: "xonaix.cortex.>" }
    ]
  }
  
  BLADE {
    jetstream: enabled
    users: [
      { user: blade, password: $BLADE_PASSWORD }
    ]
    # Blade can read everything for validation
    imports: [
      { stream: { account: FORGE, subject: "xonaix.forge.>" } }
      { stream: { account: FORGE, subject: "xonaix.governance.>" } }
      { stream: { account: CORTEX, subject: "xonaix.cortex.>" } }
    ]
  }
}
```

---

## SECTION 9: NOTHING LOST, EVER (PRINCIPLE 9)

### 9.1 Durability Guarantees

| Guarantee | Implementation |
|-----------|----------------|
| Messages persist | JetStream file storage |
| Survive restart | Durable consumers |
| Survive node failure | Replication factor 3 |
| No silent drops | Explicit ACK required |
| Audit immutability | `deny_delete`, `deny_purge` |

### 9.2 Message Flow with Durability

```
Producer                    NATS JetStream              Consumer
    │                            │                          │
    │ 1. Publish message         │                          │
    ├───────────────────────────>│                          │
    │                            │ 2. Persist to disk       │
    │                            │ 3. Replicate to peers    │
    │ 4. PubAck                  │                          │
    │<───────────────────────────┤                          │
    │                            │                          │
    │                            │ 5. Deliver to consumer   │
    │                            ├─────────────────────────>│
    │                            │                          │
    │                            │                          │ 6. Process
    │                            │                          │
    │                            │ 7. Explicit ACK          │
    │                            │<─────────────────────────┤
    │                            │                          │
    │                            │ 8. Mark consumed         │
    │                            │                          │
```

### 9.3 Exactly-Once Semantics

```rust
/// Publish with exactly-once semantics using message deduplication
async fn publish_exactly_once(
    js: &jetstream::Context,
    subject: &str,
    message_id: &Uuid,
    payload: &[u8],
) -> Result<(), PublishError> {
    // NATS deduplicates based on Nats-Msg-Id header within duplicate_window
    let headers = async_nats::HeaderMap::from_iter([
        ("Nats-Msg-Id".parse().unwrap(), message_id.to_string().parse().unwrap()),
    ]);
    
    js.publish_with_headers(subject.to_string(), headers, payload.to_vec().into())
        .await
        .map_err(|e| PublishError::Publish(e.to_string()))?
        .await  // Wait for ack
        .map_err(|e| PublishError::Ack(e.to_string()))?;
    
    Ok(())
}
```

### 9.4 Recovery Pattern

```rust
/// Recover pending messages after restart
async fn recover_pending(
    consumer: &PullConsumer,
    handler: impl Fn(&Message) -> Result<(), ProcessError>,
) -> Result<usize, RecoveryError> {
    let info = consumer.info().await
        .map_err(|e| RecoveryError::Info(e.to_string()))?;
    
    let pending = info.num_pending as usize;
    
    if pending > 0 {
        tracing::info!(pending = pending, "Recovering pending messages");
        
        // Fetch and process all pending
        let mut messages = consumer
            .fetch()
            .max_messages(pending)
            .messages()
            .await
            .map_err(|e| RecoveryError::Fetch(e.to_string()))?;
        
        let mut recovered = 0;
        while let Some(msg) = messages.next().await {
            let msg = msg.map_err(|e| RecoveryError::Message(e.to_string()))?;
            
            match handler(&msg) {
                Ok(()) => {
                    msg.ack().await.map_err(|e| RecoveryError::Ack(e.to_string()))?;
                    recovered += 1;
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to recover message");
                    msg.nak().await.map_err(|e| RecoveryError::Nak(e.to_string()))?;
                }
            }
        }
        
        tracing::info!(recovered = recovered, "Recovery complete");
        Ok(recovered)
    } else {
        Ok(0)
    }
}
```

---

## SECTION 10: HIGH AVAILABILITY

### 10.1 Cluster Configuration

```conf
# nats-server.conf - Cluster configuration

server_name: nats-1

jetstream {
  store_dir: /data/jetstream
  max_mem: 4GB
  max_file: 100GB
}

cluster {
  name: xonaix-nats
  listen: 0.0.0.0:6222
  routes: [
    nats://nats-1:6222
    nats://nats-2:6222
    nats://nats-3:6222
  ]
}
```

### 10.2 Client Reconnection

```rust
async fn connect_with_resilience(
    servers: &[&str],
) -> Result<async_nats::Client, ConnectError> {
    let client = ConnectOptions::new()
        .retry_on_initial_connect()
        .connection_timeout(std::time::Duration::from_secs(10))
        .reconnect_delay_callback(|attempts| {
            // Exponential backoff with jitter
            let base = std::time::Duration::from_millis(100);
            let max = std::time::Duration::from_secs(30);
            let delay = base * 2u32.saturating_pow(attempts as u32);
            std::cmp::min(delay, max)
        })
        .event_callback(|event| async move {
            match event {
                async_nats::Event::Connected => {
                    tracing::info!("Connected to NATS");
                }
                async_nats::Event::Disconnected => {
                    tracing::warn!("Disconnected from NATS");
                }
                async_nats::Event::Reconnected => {
                    tracing::info!("Reconnected to NATS");
                }
                _ => {}
            }
        })
        .connect(servers)
        .await
        .map_err(|e| ConnectError::Connect(e.to_string()))?;
    
    Ok(client)
}
```

---

## SECTION 11: MONITORING

### 11.1 Health Checks

```rust
/// NATS health check
async fn health_check(client: &async_nats::Client) -> Result<HealthStatus, HealthError> {
    // Check connection
    if !client.connection_state().is_connected() {
        return Ok(HealthStatus::Unhealthy("Not connected".to_string()));
    }
    
    // Check JetStream
    let js = async_nats::jetstream::new(client.clone());
    
    match js.account_info().await {
        Ok(info) => {
            // Check stream availability
            let streams = ["GOVERNANCE", "CORTEX", "FORGE", "AUDIT"];
            for stream_name in streams {
                if js.get_stream(stream_name).await.is_err() {
                    return Ok(HealthStatus::Degraded(format!(
                        "Stream {} not available",
                        stream_name
                    )));
                }
            }
            
            Ok(HealthStatus::Healthy {
                memory: info.memory,
                storage: info.storage,
                streams: info.streams,
                consumers: info.consumers,
            })
        }
        Err(e) => Ok(HealthStatus::Unhealthy(format!("JetStream error: {}", e))),
    }
}

#[derive(Debug)]
pub enum HealthStatus {
    Healthy {
        memory: i64,
        storage: i64,
        streams: i64,
        consumers: i64,
    },
    Degraded(String),
    Unhealthy(String),
}
```

### 11.2 Metrics

```rust
/// Collect NATS metrics
async fn collect_metrics(
    js: &jetstream::Context,
) -> Result<NatsMetrics, MetricsError> {
    let account = js.account_info().await
        .map_err(|e| MetricsError::Account(e.to_string()))?;
    
    let mut stream_metrics = Vec::new();
    
    for stream_name in ["GOVERNANCE", "CORTEX", "FORGE", "AUDIT", "DLQ"] {
        if let Ok(stream) = js.get_stream(stream_name).await {
            if let Ok(info) = stream.info().await {
                stream_metrics.push(StreamMetrics {
                    name: stream_name.to_string(),
                    messages: info.state.messages,
                    bytes: info.state.bytes,
                    first_seq: info.state.first_sequence,
                    last_seq: info.state.last_sequence,
                    consumer_count: info.state.consumer_count,
                });
            }
        }
    }
    
    Ok(NatsMetrics {
        memory_used: account.memory,
        storage_used: account.storage,
        streams: stream_metrics,
        collected_at: Utc::now(),
    })
}
```

---

## SECTION 12: TESTING

### 12.1 Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use testcontainers::{clients::Cli, images::nats::Nats};
    
    #[tokio::test]
    async fn test_message_roundtrip() {
        let docker = Cli::default();
        let nats_container = docker.run(Nats::default());
        let port = nats_container.get_host_port_ipv4(4222);
        
        let client = async_nats::connect(format!("localhost:{}", port))
            .await
            .unwrap();
        
        let js = async_nats::jetstream::new(client.clone());
        
        // Create test stream
        js.create_stream(stream::Config {
            name: "TEST".to_string(),
            subjects: vec!["test.>".to_string()],
            ..Default::default()
        })
        .await
        .unwrap();
        
        // Publish
        let envelope = MessageEnvelope::new("test", "Hello, NATS!");
        let payload = serde_json::to_vec(&envelope).unwrap();
        
        js.publish("test.hello", payload.into()).await.unwrap();
        
        // Consume
        let stream = js.get_stream("TEST").await.unwrap();
        let consumer = stream
            .create_consumer(consumer::pull::Config {
                durable_name: Some("test-consumer".to_string()),
                ..Default::default()
            })
            .await
            .unwrap();
        
        let mut messages = consumer.messages().await.unwrap();
        let msg = messages.next().await.unwrap().unwrap();
        
        let received: MessageEnvelope<String> = serde_json::from_slice(&msg.payload).unwrap();
        assert_eq!(received.payload, "Hello, NATS!");
        
        msg.ack().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_dead_letter_on_failure() {
        // Setup...
        
        // Simulate processing failure
        // Verify message ends up in DLQ
        // Verify original message is ACK'd
    }
    
    #[tokio::test]
    async fn test_exactly_once_deduplication() {
        // Publish same message ID twice
        // Verify only one message in stream
    }
    
    #[tokio::test]
    async fn test_consumer_recovery() {
        // Create consumer and publish messages
        // Don't ACK
        // Reconnect consumer
        // Verify pending messages are redelivered
    }
}
```

### 12.2 Chaos Testing

```rust
#[cfg(test)]
mod chaos_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_network_partition_recovery() {
        // Start 3-node cluster
        // Partition one node
        // Verify messages still flow
        // Heal partition
        // Verify cluster reconverges
    }
    
    #[tokio::test]
    async fn test_leader_failure() {
        // Identify stream leader
        // Kill leader node
        // Verify new leader elected
        // Verify no message loss
    }
    
    #[tokio::test]
    async fn test_slow_consumer() {
        // Create consumer that processes slowly
        // Publish many messages
        // Verify backpressure works
        // Verify no message loss
    }
}
```

---

## SECTION 13: CI PIPELINE

```yaml
name: NATS Integration

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      nats:
        image: nats:2.10
        ports:
          - 4222:4222
        options: >-
          --health-cmd "nats-server --help"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Run NATS Integration Tests
        run: cargo test --features nats-integration
        env:
          NATS_URL: nats://localhost:4222
          
      - name: Run Schema Validation
        run: |
          # Validate message schemas
          cargo run --bin schema-validator
          
      - name: Verify Stream Configurations
        run: |
          # Check stream configs match expected
          cargo run --bin config-validator
```

---

## X.I. Prompt Appendix

```
NATS v5.1.0 REQUIREMENTS:

NOTE: NATS is the MESSAGING BACKBONE for Xonaix.
It is the PRIMARY implementation of Principle 9 (Nothing Lost, Ever).

CLIENT:
- Primary: async-nats (Rust)
- Reference: STANDARDS_RUST.md for all Rust patterns

JETSTREAM:
- MUST enable JetStream for all messaging
- MUST use file storage (not memory) for production
- MUST use replication factor 3 for Controlled
- MUST use explicit ACK policy
- MUST implement dead letter handling

SUBJECT HIERARCHY:
- Pattern: xonaix.{domain}.{entity}.{action}
- Domains: governance, cortex, forge, blade, audit, system
- Use wildcards: * (single), > (multi, must be last)

STREAMS:
- GOVERNANCE: Tier decisions, 1 year retention
- CORTEX: Agent events, 90 days retention
- FORGE: Workflow events, 30 days retention
- AUDIT: Immutable, infinite retention, deny_delete, deny_purge

MESSAGE ENVELOPE:
- MUST use MessageEnvelope<T> for all messages
- MUST include: id, version, timestamp, source, payload
- SHOULD include: correlation_id, causation_id

PRINCIPLE 9 (NOTHING LOST):
- MUST use durable consumers
- MUST use explicit ACK
- MUST handle dead letters
- MUST implement recovery on restart
- AUDIT stream: MUST deny_delete, MUST deny_purge

KV STORE:
- config: Runtime configuration
- positions: Consumer position caching
- sessions: Active sessions (TTL 24h)
- locks: Distributed locks (TTL 5m)

SECURITY:
- MUST use TLS in production
- MUST use authentication (credentials or NKey)
- SHOULD use account-based authorization

FORBIDDEN:
NO AckNone for governance/audit messages
NO Memory storage for production
NO Replication < 3 for Controlled
NO Missing dead letter handling
NO Fire-and-forget (no publish confirmation)

FLAG THESE VIOLATIONS:
NO No explicit ACK
NO No dead letter handling
NO AUDIT stream without deny_delete/deny_purge
NO Missing correlation_id for traces
NO Missing TLS in production config
```

---

## Quick Reference

### Stream Configuration

```rust
// Controlled classification
stream::Config {
    storage: StorageType::File,
    num_replicas: 3,
    deny_delete: true,  // For AUDIT
    deny_purge: true,   // For AUDIT
    ..
}
```

### Consumer Pattern

```rust
// Always explicit ACK
consumer::pull::Config {
    ack_policy: AckPolicy::Explicit,
    ack_wait: Duration::from_secs(30),
    max_deliver: 5,
    ..
}
```

### Message Publishing

```rust
// Always use envelope
let envelope = MessageEnvelope::new("source", payload);
js.publish(subject, serde_json::to_vec(&envelope)?).await?;
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L3 Transport)
- **MAJOR:** Added Authority vs Transport section (CRITICAL)
- **MAJOR:** Added XCLib Integration section (signature at consumer)

---

*Xonaix Library Standard*
*Canonical: `xonaix-library::specs/standards/nats/STANDARDS_NATS.md`*
*Authority: `xonaix-specs::THE_XONAIX_WAY.md`*
