---
schema: "xonaix-document-header"
schema_version: "2.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/tauri/STANDARDS_TAURI.md"
unit_id: "library/standards/tauri"
title: "Tauri Framework Standard"
document_type: "standard"
language: "en"

# --- Version ---
version: "XLIB-1.0.0"
baseline: null
status: "active"

# --- Classification ---
trust_class: "L3/L4"
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

# Tauri Framework Standard

Tauri is the desktop shell for Nexus, bridging Rust backend and SvelteKit frontend for desktop deployment.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

---

## Trust Class

| Component | Trust Class | Classification |
|-----------|-------------|----------------|
| Rust Backend (IPC) | L3 | Orchestration |
| Frontend (WebView) | L4 | Interface |

### L3 Backend (IPC Commands)

The Rust IPC layer is L3 (Orchestration). It:
- Coordinates between frontend and system capabilities
- Enforces capability-based permissions
- Bridges to XCLib for crypto operations

### L4 Frontend (WebView)

The WebView frontend is L4 (Interface). It follows STANDARDS_SVELTEKIT.md and is untrusted.

### Desktop ≠ Trusted

**CRITICAL:** Desktop context does NOT imply trust. All authority verification uses XCLib proofs, same as web deployment.

---

## XCLib Integration

### Rust Backend

Tauri Rust commands MAY use XCLib directly for L1 operations:

```rust
#[tauri::command]
async fn sign_action(
    action: Action,
    state: State<'_, AppState>,
) -> Result<Signature, TauriError> {
    // Verify capability first
    state.posture.require_capability(CapabilityClass::SoftwareClassA)?;
    
    // Use XCLib
    xclib::sign(&action).map_err(TauriError::from)
}
```

### Frontend

Frontend uses XCLib WASM bindings (same as web):

```typescript
import { verify } from '@xonaix/xclib-wasm';

const result = await verify(proof);
```

---

## Capability & Posture Handling

### IPC Command Capability

All Tauri commands MUST declare required capability:

```rust
#[tauri::command]
#[capability(CapabilityClass::SoftwareClassA)]  // Custom attribute
async fn governance_action(/*...*/) -> Result<_, Error> {
    // Capability verified before handler runs
}
```

### Default Deny

Commands without explicit capability declaration are **denied by default**.

### Audit Logging

All IPC invocations MUST be logged with actor identity:

```rust
#[tauri::command]
async fn critical_action(state: State<'_, AppState>) -> Result<_, Error> {
    state.audit_log.record(AuditEvent::IpcInvocation {
        command: "critical_action",
        actor: state.current_actor()?,
        timestamp: Utc::now(),
    })?;
    
    // ... implementation
}
```

---

## Principle Mapping

| Principle | Tauri Implementation |
|-----------|----------------------|
| 1. Correct Over Fast | Type-safe commands, validated IPC, comprehensive tests |
| 2. Secure By Default | Capability-based permissions, CSP, signed builds, sandboxed WebView |
| 3. Fail Loud | IPC errors propagated, explicit error types, crash reporting |
| 4. Explicit Over Implicit | Explicit permissions, explicit IPC contracts, no ambient authority |
| 5. Automated Over Vigilant | CI builds, automated signing, update checks |
| 6. Composable Over Clever | Modular plugins, clear separation of concerns |
| 7. X.I. Augments, Human Decides | Desktop features require human review |
| 8. Future-Proof Over Trend | Tauri stable APIs, avoid experimental features |
| 9. Nothing Lost, Ever | Local persistence, offline queue, sync on reconnect |

---

## Deviation Recording

For deviations from MUST requirements in Tauri:

**Rust Backend:**
```rust
// XONAIX_DEVIATION: [Reason for deviation - be specific]
// LEDGER_ACK: [User_Signature_Hash]
#[tauri::command]
fn unsafe_command() {
    // Deviating code
}
```

**Frontend:**
```typescript
// XONAIX_DEVIATION: [Reason for deviation]
// LEDGER_ACK: [User_Signature_Hash]
// Deviating code
```

**Configuration:**
```json
{
  "_comment": "XONAIX_DEVIATION: [Reason] LEDGER_ACK: [Hash]",
  "capability": "unsafe-capability"
}
```

---

## SECTION 1: ARCHITECTURE OVERVIEW

### 1.1 Tauri in Xonaix Ecosystem

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           TAURI DESKTOP APP                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                        SVELTEKIT FRONTEND                              │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │  │
│  │  │   Routes    │  │  Components │  │   Stores    │  │   Services  │   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │  │
│  │         │                │                │                │          │  │
│  │         └────────────────┴────────────────┴────────────────┘          │  │
│  │                                   │                                    │  │
│  │                          Tauri IPC Bridge                              │  │
│  └───────────────────────────────────┼───────────────────────────────────┘  │
│                                      │                                       │
│  ┌───────────────────────────────────┼───────────────────────────────────┐  │
│  │                         RUST BACKEND                                   │  │
│  │                                   │                                    │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │  │
│  │  │  Commands   │  │   Events    │  │   State     │  │   Plugins   │   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │  │
│  │         │                │                │                │          │  │
│  │         └────────────────┴────────────────┴────────────────┘          │  │
│  │                                   │                                    │  │
│  └───────────────────────────────────┼───────────────────────────────────┘  │
│                                      │                                       │
│  ┌───────────────────────────────────┼───────────────────────────────────┐  │
│  │                        SYSTEM INTEGRATION                              │  │
│  │         ┌─────────┬─────────┬─────────┬─────────┬─────────┐           │  │
│  │         │Filesystem│ System │  HTTP   │  Tray   │ Updater │           │  │
│  │         │         │  Tray   │ Client  │ Menu    │         │           │  │
│  │         └─────────┴─────────┴─────────┴─────────┴─────────┘           │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                      │                                       │
│                              ┌───────▼───────┐                              │
│                              │  Nexus API    │                              │
│                              │   (Remote)    │                              │
│                              └───────────────┘                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Communication Flow

```
Frontend (SvelteKit)          Tauri Backend (Rust)              External
       │                              │                              │
       │  invoke('command', args)     │                              │
       ├─────────────────────────────>│                              │
       │                              │  HTTP Request                │
       │                              ├─────────────────────────────>│
       │                              │                              │
       │                              │  HTTP Response               │
       │                              │<─────────────────────────────┤
       │  Result<T, Error>            │                              │
       │<─────────────────────────────┤                              │
       │                              │                              │
       │                              │  emit('event', payload)      │
       │  on('event', handler)        │                              │
       │<─────────────────────────────┤                              │
```

### 1.3 Deployment Modes

| Mode | Network | Nexus API | Local Storage | Use Case |
|------|---------|-----------|---------------|----------|
| **Cloud** | Online | Remote | Cache only | Standard deployment |
| **Hybrid** | Mixed | Remote + Local | Sync queue | Intermittent connectivity |
| **Sovereign** | Air-gapped | None | Full local | High security |

---

## SECTION 2: PROJECT STRUCTURE

### 2.1 Directory Layout

```
src-tauri/
├── Cargo.toml                # Rust dependencies
├── tauri.conf.json           # Tauri configuration
├── capabilities/             # Permission capabilities
│   ├── default.json
│   ├── admin.json
│   └── sovereign.json
├── icons/                    # App icons
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── commands/            # IPC command handlers
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── agents.rs
│   │   └── workflows.rs
│   ├── events/              # Event emitters
│   │   ├── mod.rs
│   │   └── notifications.rs
│   ├── state/               # Application state
│   │   ├── mod.rs
│   │   └── app_state.rs
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   ├── api_client.rs
│   │   ├── local_storage.rs
│   │   └── sync.rs
│   ├── plugins/             # Custom plugins
│   │   ├── mod.rs
│   │   └── camera.rs
│   └── error.rs             # Error types
└── build.rs                  # Build script

src/                          # SvelteKit frontend (see STANDARDS_SVELTEKIT.md)
├── lib/
│   ├── tauri/               # Tauri integration
│   │   ├── commands.ts      # Command wrappers
│   │   ├── events.ts        # Event listeners
│   │   └── index.ts
│   └── ...
└── ...
```

### 2.2 Cargo.toml

```toml
[package]
name = "xonaix-desktop"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"

[lib]
name = "xonaix_desktop"
crate-type = ["lib", "cdylib", "staticlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon", "protocol-asset"] }
tauri-plugin-store = "2"
tauri-plugin-fs = "2"
tauri-plugin-http = "2"
tauri-plugin-updater = "2"
tauri-plugin-dialog = "2"
tauri-plugin-shell = "2"
tauri-plugin-os = "2"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async
tokio = { version = "1", features = ["full"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Crypto (aligned with STANDARDS_RUST.md)
blake3 = "1.5"
ed25519-dalek = "2"
zeroize = { version = "1.7", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }

# Database (local persistence)
sled = "0.34"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
devtools = ["tauri/devtools"]

# Lints aligned with STANDARDS_RUST.md
[lints.rust]
unsafe_code = "forbid"
unused_must_use = "deny"
unused_results = "deny"

[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
cognitive_complexity = "warn"
```

---

## SECTION 3: CONFIGURATION

### 3.1 tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Xonaix",
  "version": "0.1.0",
  "identifier": "com.xonaix.desktop",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../build"
  },
  "app": {
    "withGlobalTauri": false,
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https://api.xonaix.com; frame-ancestors 'none'"
    },
    "windows": [
      {
        "title": "Xonaix",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "center": true,
        "resizable": true,
        "fullscreen": false,
        "decorations": true,
        "transparent": false,
        "focus": true
      }
    ],
    "trayIcon": {
      "iconPath": "icons/tray.png",
      "iconAsTemplate": true,
      "menuOnLeftClick": false
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "macOS": {
      "minimumSystemVersion": "10.15",
      "signingIdentity": null,
      "entitlements": null
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  },
  "plugins": {
    "updater": {
      "pubkey": "YOUR_PUBLIC_KEY_HERE",
      "endpoints": [
        "https://releases.xonaix.com/{{target}}/{{arch}}/{{current_version}}"
      ]
    }
  }
}
```

### 3.2 Capability Files

```json
// capabilities/default.json
{
  "$schema": "https://schema.tauri.app/config/2/capability",
  "identifier": "default",
  "description": "Default capabilities for standard users",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-close",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "shell:allow-open",
    "dialog:allow-open",
    "dialog:allow-save",
    "http:default",
    "store:default",
    "updater:default"
  ]
}
```

```json
// capabilities/sovereign.json
{
  "$schema": "https://schema.tauri.app/config/2/capability",
  "identifier": "sovereign",
  "description": "Capabilities for sovereign/air-gapped deployment",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:allow-read",
    "fs:allow-write",
    "fs:scope-app-data",
    "dialog:allow-open",
    "dialog:allow-save",
    "store:default"
  ],
  "platforms": ["linux", "macos", "windows"]
}
```

### 3.5 Controlled Shell Access

For Controlled classification, shell access requires enhanced authorization:

**Capability Relocation:**
- `shell:allow-open` REMOVED from `default.json` (standard user profile)
- `shell:allow-open` present only in `admin.json` (privileged profile)
- Profile escalation requires explicit user action

**Signed Intent Flow:**
```
User Action -> Warning Dialog -> Biometric/FIDO2 Re-auth ->
Signed Intent (Ed25519) -> AUDIT Log -> Execute
```

**Flow Details:**
1. User triggers action requiring shell access
2. UI displays explicit warning with action details
3. User provides biometric or FIDO2 authentication
4. Backend signs intent payload (action, timestamp, user identity)
5. Intent logged to AUDIT stream before execution
6. Command executed only after successful logging

**Tier 4 Operations:**
- MAY require full QR ceremony (air-gapped signing)
- Ceremony flow per existing offline operation documentation

**Metrics:**
- `tauri.intent.signed` — successful signed intents (normal)
- `tauri.intent.unsigned_blocked` — blocked unsigned attempts (should be 0)

**Violations:**
- Unsigned shell attempts -> block + security alert
- Repeated attempts -> investigate for potential compromise

---

## SECTION 4: COMMANDS (IPC)

### 4.1 Command Pattern

```rust
// src/commands/mod.rs
pub mod agents;
pub mod auth;
pub mod workflows;
pub mod ceremony;  // Air-gapped ceremonies

use crate::error::CommandError;

/// Result type for all commands
pub type CommandResult<T> = Result<T, CommandError>;
```

```rust
// src/error.rs
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum CommandError {
    #[error("Not authenticated")]
    NotAuthenticated,
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

// Required for Tauri
impl From<CommandError> for tauri::ipc::InvokeError {
    fn from(error: CommandError) -> Self {
        tauri::ipc::InvokeError::from(serde_json::to_string(&error).unwrap_or_default())
    }
}
```

### 4.2 Command Implementation

```rust
// src/commands/agents.rs
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::error::CommandError;
use crate::services::api_client::ApiClient;
use crate::state::AppState;

/// Agent data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Inactive,
    Pending,
}

/// List agents command
#[tauri::command]
pub async fn list_agents(
    state: State<'_, AppState>,
    api_client: State<'_, ApiClient>,
) -> Result<Vec<Agent>, CommandError> {
    // Precondition: User must be authenticated
    let session = state.session.read().await;
    let session = session.as_ref().ok_or(CommandError::NotAuthenticated)?;
    
    // Bounded operation (NASA Rule 2)
    const MAX_AGENTS: usize = 1000;
    
    let agents = api_client
        .get_agents(&session.token, MAX_AGENTS)
        .await
        .map_err(|e| CommandError::Network(e.to_string()))?;
    
    // Postcondition: Valid agents returned
    debug_assert!(agents.len() <= MAX_AGENTS, "Agent count exceeded limit");
    
    Ok(agents)
}

/// Get single agent command
#[tauri::command]
pub async fn get_agent(
    id: String,
    state: State<'_, AppState>,
    api_client: State<'_, ApiClient>,
) -> Result<Agent, CommandError> {
    // Validate input
    let id = Uuid::parse_str(&id).map_err(|_| CommandError::Validation("Invalid agent ID".into()))?;
    
    let session = state.session.read().await;
    let session = session.as_ref().ok_or(CommandError::NotAuthenticated)?;
    
    let agent = api_client
        .get_agent(&session.token, &id)
        .await
        .map_err(|e| match e {
            ApiError::NotFound => CommandError::NotFound(format!("Agent {}", id)),
            e => CommandError::Network(e.to_string()),
        })?;
    
    Ok(agent)
}

/// Create agent command
#[tauri::command]
pub async fn create_agent(
    name: String,
    state: State<'_, AppState>,
    api_client: State<'_, ApiClient>,
) -> Result<Agent, CommandError> {
    // Validate input
    if name.is_empty() || name.len() > 256 {
        return Err(CommandError::Validation("Name must be 1-256 characters".into()));
    }
    
    let session = state.session.read().await;
    let session = session.as_ref().ok_or(CommandError::NotAuthenticated)?;
    
    let agent = api_client
        .create_agent(&session.token, &name)
        .await
        .map_err(|e| CommandError::Network(e.to_string()))?;
    
    Ok(agent)
}
```

### 4.3 Registering Commands

```rust
// src/lib.rs
mod commands;
mod error;
mod events;
mod services;
mod state;

use state::AppState;
use services::api_client::ApiClient;

pub fn run() {
    let app_state = AppState::new();
    let api_client = ApiClient::new("https://api.xonaix.com");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(app_state)
        .manage(api_client)
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            commands::auth::login,
            commands::auth::logout,
            commands::auth::get_session,
            // Agent commands
            commands::agents::list_agents,
            commands::agents::get_agent,
            commands::agents::create_agent,
            // Workflow commands
            commands::workflows::list_workflows,
            commands::workflows::start_workflow,
            // Ceremony commands (air-gapped)
            commands::ceremony::generate_qr_request,
            commands::ceremony::process_qr_response,
        ])
        .setup(|app| {
            // Setup system tray
            setup_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## SECTION 5: FRONTEND INTEGRATION

### 5.1 Command Wrappers

```typescript
// src/lib/tauri/commands.ts
import { invoke } from '@tauri-apps/api/core';
import { z } from 'zod';

// Error schema matching Rust CommandError
const CommandErrorSchema = z.object({
  type: z.enum([
    'NotAuthenticated',
    'PermissionDenied',
    'NotFound',
    'Validation',
    'Network',
    'Storage',
    'Internal',
  ]),
  data: z.string().optional(),
});

type CommandError = z.infer<typeof CommandErrorSchema>;

class TauriCommandError extends Error {
  constructor(
    public type: CommandError['type'],
    public data?: string
  ) {
    super(data ?? type);
    this.name = 'TauriCommandError';
  }
}

async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
  schema?: z.ZodType<T>
): Promise<T> {
  try {
    const result = await invoke<T>(command, args);
    
    if (schema) {
      return schema.parse(result);
    }
    
    return result;
  } catch (e) {
    // Parse error from Rust
    if (typeof e === 'string') {
      try {
        const parsed = JSON.parse(e);
        const error = CommandErrorSchema.parse(parsed);
        throw new TauriCommandError(error.type, error.data);
      } catch {
        throw new TauriCommandError('Internal', e);
      }
    }
    throw e;
  }
}

// Agent schemas
const AgentSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  status: z.enum(['active', 'inactive', 'pending']),
  created_at: z.string().datetime(),
});

const AgentsListSchema = z.array(AgentSchema);

export type Agent = z.infer<typeof AgentSchema>;

// Exported commands
export const commands = {
  // Auth
  login: (email: string, password: string) =>
    invokeCommand<{ token: string }>('login', { email, password }),
  
  logout: () => invokeCommand<void>('logout'),
  
  getSession: () =>
    invokeCommand<{ user: { id: string; email: string } } | null>('get_session'),
  
  // Agents
  listAgents: () =>
    invokeCommand('list_agents', undefined, AgentsListSchema),
  
  getAgent: (id: string) =>
    invokeCommand('get_agent', { id }, AgentSchema),
  
  createAgent: (name: string) =>
    invokeCommand('create_agent', { name }, AgentSchema),
  
  // Ceremonies
  generateQrRequest: (action: string, payload: unknown) =>
    invokeCommand<{ qrData: string; requestId: string }>(
      'generate_qr_request',
      { action, payload }
    ),
  
  processQrResponse: (requestId: string, qrData: string) =>
    invokeCommand<{ success: boolean; result?: unknown }>(
      'process_qr_response',
      { requestId, qrData }
    ),
};

export { TauriCommandError };
```

### 5.2 Event Listeners

```typescript
// src/lib/tauri/events.ts
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { z } from 'zod';

// Event payloads
const NotificationPayloadSchema = z.object({
  title: z.string(),
  body: z.string(),
  icon: z.string().optional(),
});

const SyncStatusPayloadSchema = z.object({
  status: z.enum(['syncing', 'synced', 'offline', 'error']),
  pendingCount: z.number().int().nonnegative(),
  lastSyncedAt: z.string().datetime().optional(),
});

export type NotificationPayload = z.infer<typeof NotificationPayloadSchema>;
export type SyncStatusPayload = z.infer<typeof SyncStatusPayloadSchema>;

// Event handlers
export const events = {
  onNotification: (handler: (payload: NotificationPayload) => void): Promise<UnlistenFn> =>
    listen<NotificationPayload>('notification', (event) => {
      const parsed = NotificationPayloadSchema.safeParse(event.payload);
      if (parsed.success) {
        handler(parsed.data);
      } else {
        console.error('Invalid notification payload:', parsed.error);
      }
    }),
  
  onSyncStatus: (handler: (payload: SyncStatusPayload) => void): Promise<UnlistenFn> =>
    listen<SyncStatusPayload>('sync-status', (event) => {
      const parsed = SyncStatusPayloadSchema.safeParse(event.payload);
      if (parsed.success) {
        handler(parsed.data);
      } else {
        console.error('Invalid sync status payload:', parsed.error);
      }
    }),
  
  onUpdateAvailable: (handler: (version: string) => void): Promise<UnlistenFn> =>
    listen<{ version: string }>('update-available', (event) => {
      handler(event.payload.version);
    }),
};
```

### 5.3 Platform Detection

```typescript
// src/lib/tauri/platform.ts
import { platform as getPlatform, arch as getArch } from '@tauri-apps/plugin-os';

export interface PlatformInfo {
  isTauri: boolean;
  platform: 'macos' | 'windows' | 'linux' | 'web';
  arch: string;
}

let cachedPlatform: PlatformInfo | null = null;

export async function getPlatformInfo(): Promise<PlatformInfo> {
  if (cachedPlatform) {
    return cachedPlatform;
  }
  
  // Check if running in Tauri
  const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;
  
  if (!isTauri) {
    cachedPlatform = {
      isTauri: false,
      platform: 'web',
      arch: 'unknown',
    };
    return cachedPlatform;
  }
  
  const [platform, arch] = await Promise.all([getPlatform(), getArch()]);
  
  cachedPlatform = {
    isTauri: true,
    platform: platform as 'macos' | 'windows' | 'linux',
    arch,
  };
  
  return cachedPlatform;
}

export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}
```

---

## SECTION 6: AIR-GAPPED CEREMONIES

### 6.1 QR Code Signing Flow

For Zero Trust and Sovereign deployments, Tauri supports air-gapped signing ceremonies using QR codes.

```
┌─────────────────┐          ┌─────────────────┐          ┌─────────────────┐
│   Tauri App     │          │   Air-Gapped    │          │   Tauri App     │
│   (Request)     │          │   Signer        │          │   (Complete)    │
└────────┬────────┘          └────────┬────────┘          └────────┬────────┘
         │                            │                            │
         │ 1. Generate Request        │                            │
         ├──────────────────────────> │                            │
         │    (Display QR Code)       │                            │
         │                            │                            │
         │                            │ 2. Scan Request QR         │
         │                            │    (Camera/Manual)         │
         │                            │                            │
         │                            │ 3. Review & Sign           │
         │                            │    (User Approval)         │
         │                            │                            │
         │                            │ 4. Display Response QR     │
         │                            ├─────────────────────────── │
         │                            │                            │
         │ 5. Scan Response QR        │                            │
         │    (Camera/Manual)         │                            │
         │<─────────────────────────────────────────────────────── │
         │                            │                            │
         │ 6. Verify Signature        │                            │
         │ 7. Apply Action            │                            │
         │                            │                            │
```

### 6.2 Ceremony Commands (Rust)

```rust
// src/commands/ceremony.rs
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::error::CommandError;
use crate::state::AppState;

/// Ceremony request data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeremonyRequest {
    pub id: Uuid,
    pub action: String,
    pub payload: serde_json::Value,
    pub timestamp: i64,
    pub nonce: [u8; 32],
}

/// Ceremony response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeremonyResponse {
    pub request_id: Uuid,
    pub approved: bool,
    pub signature: String,  // Base64 encoded
    pub signer_key: String, // Base64 encoded public key
    pub timestamp: i64,
}

/// Generate QR code data for ceremony request
#[tauri::command]
pub async fn generate_qr_request(
    action: String,
    payload: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<QrRequestResult, CommandError> {
    // Validate action
    if action.is_empty() || action.len() > 64 {
        return Err(CommandError::Validation("Invalid action".into()));
    }
    
    // Generate request
    let mut nonce = [0u8; 32];
    getrandom::getrandom(&mut nonce)
        .map_err(|e| CommandError::Internal(format!("RNG failed: {}", e)))?;
    
    let request = CeremonyRequest {
        id: Uuid::new_v4(),
        action,
        payload,
        timestamp: chrono::Utc::now().timestamp(),
        nonce,
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&request)
        .map_err(|e| CommandError::Internal(e.to_string()))?;
    
    // Encode for QR (could add compression for large payloads)
    let qr_data = BASE64.encode(json.as_bytes());
    
    // Store pending request
    state.pending_ceremonies.write().await.insert(request.id, request.clone());
    
    Ok(QrRequestResult {
        request_id: request.id,
        qr_data,
    })
}

#[derive(Debug, Serialize)]
pub struct QrRequestResult {
    pub request_id: Uuid,
    pub qr_data: String,
}

/// Process scanned QR code response
#[tauri::command]
pub async fn process_qr_response(
    request_id: String,
    qr_data: String,
    state: State<'_, AppState>,
) -> Result<CeremonyResult, CommandError> {
    // Parse request ID
    let request_id = Uuid::parse_str(&request_id)
        .map_err(|_| CommandError::Validation("Invalid request ID".into()))?;
    
    // Get pending request
    let request = state.pending_ceremonies.read().await
        .get(&request_id)
        .cloned()
        .ok_or(CommandError::NotFound("Ceremony request not found".into()))?;
    
    // Decode QR response
    let response_bytes = BASE64.decode(&qr_data)
        .map_err(|_| CommandError::Validation("Invalid QR data".into()))?;
    
    let response: CeremonyResponse = serde_json::from_slice(&response_bytes)
        .map_err(|_| CommandError::Validation("Invalid response format".into()))?;
    
    // Verify request ID matches
    if response.request_id != request_id {
        return Err(CommandError::Validation("Request ID mismatch".into()));
    }
    
    // Verify signature
    let signer_key_bytes = BASE64.decode(&response.signer_key)
        .map_err(|_| CommandError::Validation("Invalid signer key".into()))?;
    
    let signer_key = VerifyingKey::from_bytes(
        signer_key_bytes.as_slice().try_into()
            .map_err(|_| CommandError::Validation("Invalid key length".into()))?
    ).map_err(|_| CommandError::Validation("Invalid public key".into()))?;
    
    // Reconstruct signed message
    let signed_data = serde_json::to_vec(&(&request, response.approved, response.timestamp))
        .map_err(|e| CommandError::Internal(e.to_string()))?;
    
    let signature_bytes = BASE64.decode(&response.signature)
        .map_err(|_| CommandError::Validation("Invalid signature".into()))?;
    
    let signature = Signature::from_bytes(
        signature_bytes.as_slice().try_into()
            .map_err(|_| CommandError::Validation("Invalid signature length".into()))?
    );
    
    // Verify signature
    signer_key.verify_strict(&signed_data, &signature)
        .map_err(|_| CommandError::PermissionDenied("Signature verification failed".into()))?;
    
    // Verify signer is authorized (check against known keys)
    verify_authorized_signer(&signer_key, &request.action, &state).await?;
    
    // Remove from pending
    state.pending_ceremonies.write().await.remove(&request_id);
    
    // If approved, execute the action
    if response.approved {
        execute_ceremony_action(&request, &state).await?;
    }
    
    Ok(CeremonyResult {
        success: true,
        approved: response.approved,
        action: request.action,
    })
}

#[derive(Debug, Serialize)]
pub struct CeremonyResult {
    pub success: bool,
    pub approved: bool,
    pub action: String,
}

async fn verify_authorized_signer(
    _key: &VerifyingKey,
    _action: &str,
    _state: &State<'_, AppState>,
) -> Result<(), CommandError> {
    // Implementation: Check against stored authorized signers
    // For Tier 3-4 actions, require specific keys
    Ok(())
}

async fn execute_ceremony_action(
    request: &CeremonyRequest,
    _state: &State<'_, AppState>,
) -> Result<(), CommandError> {
    // Implementation: Execute the approved action
    tracing::info!(
        action = %request.action,
        request_id = %request.id,
        "Executing ceremony action"
    );
    Ok(())
}
```

### 6.3 Ceremony UI Component

```svelte
<!-- src/lib/components/domain/CeremonyFlow.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { commands } from '$lib/tauri/commands';
  import { Button, Card, QrCode, QrScanner } from '$components/ui';
  
  export let action: string;
  export let payload: unknown;
  export let onComplete: (result: { approved: boolean }) => void;
  export let onCancel: () => void;
  
  type State = 'generating' | 'display-request' | 'scanning' | 'processing' | 'complete' | 'error';
  
  let state: State = 'generating';
  let requestId: string = '';
  let qrData: string = '';
  let error: string = '';
  let scannerActive = false;
  
  onMount(async () => {
    try {
      const result = await commands.generateQrRequest(action, payload);
      requestId = result.requestId;
      qrData = result.qrData;
      state = 'display-request';
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to generate request';
      state = 'error';
    }
  });
  
  async function handleScannedQr(scannedData: string) {
    scannerActive = false;
    state = 'processing';
    
    try {
      const result = await commands.processQrResponse(requestId, scannedData);
      state = 'complete';
      onComplete({ approved: result.approved });
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to process response';
      state = 'error';
    }
  }
  
  function startScanning() {
    scannerActive = true;
    state = 'scanning';
  }
  
  function enterManually() {
    // Show manual entry dialog
  }
</script>

<Card>
  <svelte:fragment slot="header">
    <h2>Air-Gapped Signing Ceremony</h2>
    <p>Action: {action}</p>
  </svelte:fragment>
  
  {#if state === 'generating'}
    <div class="loading">
      <p>Generating request...</p>
    </div>
  {:else if state === 'display-request'}
    <div class="ceremony-step">
      <h3>Step 1: Scan this QR code with your air-gapped device</h3>
      <QrCode data={qrData} size={300} />
      <p class="hint">
        Open the Xonaix Signer app on your air-gapped device and scan this code.
      </p>
      
      <div class="actions">
        <Button on:click={startScanning}>
          I've signed it — Scan Response
        </Button>
        <Button variant="secondary" on:click={enterManually}>
          Enter Manually
        </Button>
      </div>
    </div>
  {:else if state === 'scanning'}
    <div class="ceremony-step">
      <h3>Step 2: Scan the signed response</h3>
      {#if scannerActive}
        <QrScanner onScan={handleScannedQr} onError={(e) => error = e} />
      {/if}
      <Button variant="secondary" on:click={() => state = 'display-request'}>
        Back
      </Button>
    </div>
  {:else if state === 'processing'}
    <div class="loading">
      <p>Verifying signature...</p>
    </div>
  {:else if state === 'complete'}
    <div class="success">
      <p>Ceremony completed successfully!</p>
    </div>
  {:else if state === 'error'}
    <div class="error" role="alert">
      <p>{error}</p>
      <Button on:click={() => state = 'display-request'}>
        Try Again
      </Button>
    </div>
  {/if}
  
  <svelte:fragment slot="footer">
    <Button variant="ghost" on:click={onCancel}>
      Cancel
    </Button>
  </svelte:fragment>
</Card>
```

---

## SECTION 7: LOCAL PERSISTENCE (PRINCIPLE 9)

### 7.1 Local Storage Service

```rust
// src/services/local_storage.rs
use sled::Db;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

use crate::error::CommandError;

pub struct LocalStorage {
    db: Db,
}

impl LocalStorage {
    pub fn new(path: &Path) -> Result<Self, CommandError> {
        let db = sled::open(path)
            .map_err(|e| CommandError::Storage(e.to_string()))?;
        
        Ok(Self { db })
    }
    
    /// Store value with fsync (Principle 9)
    pub fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), CommandError> {
        let bytes = bincode::serialize(value)
            .map_err(|e| CommandError::Storage(format!("Serialization failed: {}", e)))?;
        
        self.db.insert(key, bytes)
            .map_err(|e| CommandError::Storage(e.to_string()))?;
        
        // CRITICAL: Flush to disk (Principle 9 - Nothing Lost, Ever)
        self.db.flush()
            .map_err(|e| CommandError::Storage(format!("Flush failed: {}", e)))?;
        
        Ok(())
    }
    
    /// Get value
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, CommandError> {
        match self.db.get(key) {
            Ok(Some(bytes)) => {
                let value: T = bincode::deserialize(&bytes)
                    .map_err(|e| CommandError::Storage(format!("Deserialization failed: {}", e)))?;
                Ok(Some(value))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(CommandError::Storage(e.to_string())),
        }
    }
    
    /// Delete value
    pub fn delete(&self, key: &str) -> Result<(), CommandError> {
        self.db.remove(key)
            .map_err(|e| CommandError::Storage(e.to_string()))?;
        
        self.db.flush()
            .map_err(|e| CommandError::Storage(format!("Flush failed: {}", e)))?;
        
        Ok(())
    }
}
```

### 7.2 Offline Queue

```rust
// src/services/offline_queue.rs
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::error::CommandError;
use crate::services::local_storage::LocalStorage;

const QUEUE_KEY: &str = "offline_queue";
const MAX_QUEUE_SIZE: usize = 1000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedAction {
    pub id: Uuid,
    pub action: String,
    pub payload: serde_json::Value,
    pub created_at: i64,
    pub retries: u32,
}

pub struct OfflineQueue {
    storage: LocalStorage,
}

impl OfflineQueue {
    pub fn new(storage: LocalStorage) -> Self {
        Self { storage }
    }
    
    /// Add action to queue (persisted immediately)
    pub fn enqueue(&self, action: String, payload: serde_json::Value) -> Result<Uuid, CommandError> {
        let mut queue = self.load_queue()?;
        
        // Bounded queue (NASA Rule 2)
        if queue.len() >= MAX_QUEUE_SIZE {
            return Err(CommandError::Storage("Offline queue full".into()));
        }
        
        let id = Uuid::new_v4();
        let item = QueuedAction {
            id,
            action,
            payload,
            created_at: chrono::Utc::now().timestamp(),
            retries: 0,
        };
        
        queue.push_back(item);
        self.save_queue(&queue)?;
        
        Ok(id)
    }
    
    /// Get next action to process
    pub fn peek(&self) -> Result<Option<QueuedAction>, CommandError> {
        let queue = self.load_queue()?;
        Ok(queue.front().cloned())
    }
    
    /// Remove action after successful processing
    pub fn dequeue(&self, id: &Uuid) -> Result<(), CommandError> {
        let mut queue = self.load_queue()?;
        queue.retain(|item| &item.id != id);
        self.save_queue(&queue)
    }
    
    /// Mark action as failed (increment retry count)
    pub fn mark_failed(&self, id: &Uuid) -> Result<(), CommandError> {
        let mut queue = self.load_queue()?;
        
        if let Some(item) = queue.iter_mut().find(|item| &item.id == id) {
            item.retries += 1;
            
            // Move to back of queue for later retry
            if let Some(idx) = queue.iter().position(|i| &i.id == id) {
                if let Some(item) = queue.remove(idx) {
                    queue.push_back(item);
                }
            }
        }
        
        self.save_queue(&queue)
    }
    
    /// Get queue length
    pub fn len(&self) -> Result<usize, CommandError> {
        Ok(self.load_queue()?.len())
    }
    
    fn load_queue(&self) -> Result<VecDeque<QueuedAction>, CommandError> {
        self.storage
            .get::<VecDeque<QueuedAction>>(QUEUE_KEY)
            .map(|opt| opt.unwrap_or_default())
    }
    
    fn save_queue(&self, queue: &VecDeque<QueuedAction>) -> Result<(), CommandError> {
        self.storage.set(QUEUE_KEY, queue)
    }
}
```

### 7.3 Sync Service

```rust
// src/services/sync.rs
use tokio::sync::mpsc;
use std::time::Duration;

use crate::error::CommandError;
use crate::services::api_client::ApiClient;
use crate::services::offline_queue::OfflineQueue;

pub struct SyncService {
    queue: OfflineQueue,
    api_client: ApiClient,
    status_tx: mpsc::Sender<SyncStatus>,
}

#[derive(Debug, Clone)]
pub enum SyncStatus {
    Syncing,
    Synced { pending: usize },
    Offline,
    Error(String),
}

impl SyncService {
    pub fn new(
        queue: OfflineQueue,
        api_client: ApiClient,
        status_tx: mpsc::Sender<SyncStatus>,
    ) -> Self {
        Self { queue, api_client, status_tx }
    }
    
    /// Process queued actions
    pub async fn sync(&self) -> Result<usize, CommandError> {
        let mut processed = 0;
        const MAX_BATCH: usize = 10;
        
        let _ = self.status_tx.send(SyncStatus::Syncing).await;
        
        for _ in 0..MAX_BATCH {
            match self.queue.peek()? {
                Some(item) => {
                    match self.process_action(&item).await {
                        Ok(()) => {
                            self.queue.dequeue(&item.id)?;
                            processed += 1;
                        }
                        Err(e) if is_retryable(&e) => {
                            self.queue.mark_failed(&item.id)?;
                            break;  // Stop on network error
                        }
                        Err(_) => {
                            // Non-retryable error - remove from queue
                            self.queue.dequeue(&item.id)?;
                        }
                    }
                }
                None => break,
            }
        }
        
        let pending = self.queue.len()?;
        let _ = self.status_tx.send(SyncStatus::Synced { pending }).await;
        
        Ok(processed)
    }
    
    async fn process_action(&self, item: &crate::services::offline_queue::QueuedAction) -> Result<(), CommandError> {
        // Timeout for API call
        tokio::time::timeout(
            Duration::from_secs(30),
            self.api_client.execute_action(&item.action, &item.payload),
        )
        .await
        .map_err(|_| CommandError::Network("Request timed out".into()))?
    }
}

fn is_retryable(error: &CommandError) -> bool {
    matches!(error, CommandError::Network(_))
}
```

---

## SECTION 8: UPDATER

### 8.1 Update Configuration

```json
// In tauri.conf.json
{
  "plugins": {
    "updater": {
      "pubkey": "<BASE64_ED25519_UPDATE_KEY>",
      "endpoints": [
        "https://releases.xonaix.com/{{target}}/{{arch}}/{{current_version}}"
      ],
      "windows": {
        "installMode": "passive"
      }
    }
  }
}
```

Use the actual base64-encoded Ed25519 update signing key for `pubkey`; never ship the example value.

### 8.2 Update Check

```rust
// src/commands/updater.rs
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, CommandError> {
    let updater = app.updater()
        .map_err(|e| CommandError::Internal(e.to_string()))?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            Ok(Some(UpdateInfo {
                version: update.version.clone(),
                current_version: update.current_version.clone(),
                body: update.body.clone(),
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(CommandError::Network(e.to_string())),
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateInfo {
    pub version: String,
    pub current_version: String,
    pub body: Option<String>,
}

#[tauri::command]
pub async fn install_update(app: tauri::AppHandle) -> Result<(), CommandError> {
    let updater = app.updater()
        .map_err(|e| CommandError::Internal(e.to_string()))?;
    
    if let Some(update) = updater.check().await
        .map_err(|e| CommandError::Network(e.to_string()))?
    {
        update.download_and_install(|_, _| {}, || {})
            .await
            .map_err(|e| CommandError::Internal(e.to_string()))?;
    }
    
    Ok(())
}
```

---

## SECTION 9: TESTING

### 9.1 Rust Tests

```rust
// src/commands/agents_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_list_agents_requires_auth() {
        let state = AppState::new();
        let api_client = MockApiClient::new();
        
        let result = list_agents(
            tauri::State::new(state),
            tauri::State::new(api_client),
        ).await;
        
        assert!(matches!(result, Err(CommandError::NotAuthenticated)));
    }
    
    #[tokio::test]
    async fn test_create_agent_validates_name() {
        let state = AppState::with_session(mock_session());
        let api_client = MockApiClient::new();
        
        // Empty name
        let result = create_agent(
            "".to_string(),
            tauri::State::new(state.clone()),
            tauri::State::new(api_client.clone()),
        ).await;
        
        assert!(matches!(result, Err(CommandError::Validation(_))));
        
        // Name too long
        let long_name = "x".repeat(257);
        let result = create_agent(
            long_name,
            tauri::State::new(state),
            tauri::State::new(api_client),
        ).await;
        
        assert!(matches!(result, Err(CommandError::Validation(_))));
    }
}
```

### 9.2 E2E Tests

```typescript
// tests/tauri/agents.test.ts
import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { commands } from '$lib/tauri/commands';

describe('Agent Commands', () => {
  beforeAll(async () => {
    // Login for tests
    await commands.login('test@example.com', 'password');
  });
  
  afterAll(async () => {
    await commands.logout();
  });
  
  it('lists agents', async () => {
    const agents = await commands.listAgents();
    expect(Array.isArray(agents)).toBe(true);
  });
  
  it('creates an agent', async () => {
    const agent = await commands.createAgent('Test Agent');
    expect(agent.name).toBe('Test Agent');
    expect(agent.id).toBeDefined();
  });
  
  it('handles invalid agent ID', async () => {
    await expect(commands.getAgent('invalid-uuid')).rejects.toThrow();
  });
});
```

---

## SECTION 10: CI PIPELINE

```yaml
name: Tauri CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
          
      - name: Rust tests
        working-directory: src-tauri
        run: cargo test
        
      - name: Rust clippy
        working-directory: src-tauri
        run: cargo clippy --all-targets -- -D warnings
        
      - name: Rust fmt
        working-directory: src-tauri
        run: cargo fmt --check
        
  build:
    needs: test
    strategy:
      matrix:
        include:
          - platform: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - platform: macos-latest
            target: x86_64-apple-darwin
          - platform: macos-latest
            target: aarch64-apple-darwin
          - platform: windows-latest
            target: x86_64-pc-windows-msvc
            
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
          
      - name: Install Node
        uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
          
      - name: Install dependencies (Linux)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
          
      - name: Install frontend dependencies
        run: npm ci
        
      - name: Build Tauri
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          args: --target ${{ matrix.target }}
          
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: tauri-${{ matrix.target }}
          path: src-tauri/target/${{ matrix.target }}/release/bundle/
```

---

## X.I. Prompt Appendix

```
TAURI v5.1.0 REQUIREMENTS:

NOTE: Tauri is the DESKTOP SHELL for Nexus.
Bridges STANDARDS_RUST.md (backend) and STANDARDS_SVELTEKIT.md (frontend).

ARCHITECTURE:
- Tauri 2.x
- Rust backend for commands, state, events
- SvelteKit frontend for UI
- IPC bridge for communication

RUST BACKEND:
- Follow STANDARDS_RUST.md completely
- Use CommandError for all errors
- Validate all inputs
- Timeout external calls
- Bounded iterations

COMMANDS:
- MUST return Result<T, CommandError>
- MUST validate all inputs
- MUST check authentication where required
- SHOULD be async for I/O operations

SECURITY:
- MUST use capability-based permissions
- MUST configure CSP
- MUST sign builds for distribution
- MUST NOT use allowAll permissions

AIR-GAPPED CEREMONIES:
- Generate QR for request
- Scan QR for response
- Verify Ed25519 signatures
- Execute approved actions

PRINCIPLE 9 (NOTHING LOST):
- MUST use sled for local persistence
- MUST call flush() after writes
- MUST implement offline queue
- MUST sync on reconnect

PLUGINS:
- store: User preferences
- fs: File access (scoped)
- http: API calls
- updater: Auto-updates (signed)
- dialog: File dialogs
- shell: Open URLs
- os: Platform info

FORBIDDEN:
NO allowAll permissions
NO Unsigned builds for distribution
NO Commands without Result return
NO Missing input validation
NO Unscoped filesystem access
NO Fire-and-forget persistence

FLAG THESE VIOLATIONS:
NO capability: "default-all"
NO unwrap() in commands
NO Missing auth checks
NO Unvalidated command inputs
NO Missing flush() after storage writes
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L3 IPC / L4 UI)
- **MAJOR:** Added XCLib Integration section
- **MAJOR:** Added Capability & Posture Handling section
- **CRITICAL:** Desktop ≠ Trusted clarification

---

*Xonaix Library Standard*
*Canonical: `xonaix-library::specs/standards/tauri/STANDARDS_TAURI.md`*
*Authority: `xonaix-specs::THE_XONAIX_WAY.md`*
