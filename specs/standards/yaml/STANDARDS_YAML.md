---
title: "YAML Data Format Standard"
unit_id: "library/standards/yaml"
standard_type: "standard"
version: "1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# The Xonaix Way
## Standards: YAML

**Version:** B-5.8.5
**Status:** Active
**Core-Compatible:** 5.7.0
**Trust Class:** L4
**Created:** December 2025
**Last Reviewed:** December 2025

*This document implements The Xonaix Way B-5.8.5 principles for YAML configuration files.*

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
| Specification | YAML 1.2 |
| Related Standards | STANDARDS_JSON.md, STANDARDS_TOML.md |

**Prerequisites:** Read [THE_XONAIX_WAY.md](../THE_XONAIX_WAY.md) first. This document assumes familiarity with the 9 Principles.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L4 |
| Classification | Configuration |

YAML files are L4 (Configuration/Tooling). They define settings but cannot assert authority.

### Loader Security

MUST use strict/safe loaders:

```python
# CORRECT
yaml.safe_load(content)

# FORBIDDEN
yaml.load(content)  # Unsafe - allows arbitrary code execution
```

### Security-Affecting Values

Same as TOML — values affecting security posture MUST be in signed manifests.

---

## Numeric Policy

**Authority:** Founder Ruling 2025-003(a) — Determinism

### Implicit Typing Danger

YAML's implicit typing is **dangerous** for canonical data:

```yaml
# DANGEROUS: Implicit typing
version: 1.0      # Could be float or string!
rate: 1.5         # Float - non-deterministic

# REQUIRED: Explicit typing
version: "1.0"    # Explicitly string
rate: 150         # Integer (basis points)
rate_str: "1.5"   # Explicit string
```

### Disable Implicit Typing

For governance configurations, disable implicit typing:

```yaml
# Use explicit type tags
version: !!str 1.0
count: !!int 100
```

---

## XCLib Integration

YAML files do not directly integrate with XCLib. Same pattern as TOML:

1. **Parsed** with safe loader
2. **Validated** against schema
3. **Verified** against signed manifest (if security-affecting)

---

## Parse Error Handling

**Authority:** Founder Ruling 2025-003(b) — Bounded Error Surfaces

Parse failures MUST return uniform errors:

```python
def parse_config(content: str) -> Config:
    try:
        data = yaml.safe_load(content)
        return validate_schema(data)
    except yaml.YAMLError:
        # Uniform error - no content details
        raise ConfigError("YAML parse failed")
    except ValidationError:
        # Uniform error - no schema details
        raise ConfigError("Schema validation failed")
```

---

## Principle Mapping

| Principle | YAML Implementation |
|-----------|---------------------|
| 1. Correct Over Fast | Valid YAML, schema validation |
| 2. Secure By Default | No code execution, explicit types, safe_load only |
| 3. Fail Loud | Schema violations fail CI, not warn |
| 4. Explicit Over Implicit | Explicit types, no implicit conversion |
| 5. Automated Over Vigilant | Automated validation in CI, yamllint |
| 6. Composable Over Clever | Simple structures, minimal anchors |
| 7. X.I. Augments, Human Decides | Config changes require human review |
| 8. Future-Proof Over Trend | YAML 1.2 standard, stable features only |
| 9. Nothing Lost, Ever | Config for durable queues, persistence settings |

---

## Deviation Recording

For deviations from MUST requirements in YAML configurations:

```yaml
# XONAIX_DEVIATION: [Reason for deviation - be specific]
# LEDGER_ACK: [User_Signature_Hash]
unsafe_config_key: value  # Deviating configuration
```

**Blade Enforcement:** Blade scans YAML files for deviation markers and records them in the Security Ledger.

---

## SECTION 1: FORMATTING

### 1.1 Indentation

**MUST use 2 spaces for indentation:**

```yaml
# CORRECT: 2 spaces
server:
  host: localhost
  port: 8080
  database:
    host: db.local
    port: 5432

# WRONG: tabs or 4 spaces
server:
    host: localhost  # 4 spaces - VIOLATION
```

### 1.2 Quotation

**MUST quote strings that could be misinterpreted:**

```yaml
# CORRECT: Explicit strings for ambiguous values
version: "1.0"       # Without quotes becomes float 1.0
enabled: "true"      # If you want the string "true", not boolean
port: "8080"         # If you want the string "8080", not integer
country: "NO"        # Without quotes becomes boolean false (Norway!)

# CORRECT: Plain strings (unambiguous)
name: my-service
environment: production

# WRONG: Ambiguous values without quotes
version: 1.0     # Becomes float 1.0, not string "1.0"
enabled: yes     # Becomes boolean true
answer: no       # Becomes boolean false
port: 8080       # Becomes integer 8080
```

### 1.3 Character Encoding

**MUST use UTF-8 encoding without BOM:**

```yaml
# All YAML files MUST be:
# - UTF-8 encoded
# - No Byte Order Mark (BOM)
# - LF line endings (not CRLF)
```

### 1.4 Multi-Line Strings

```yaml
# Literal block (preserves newlines)
description: |
  This is a multi-line description.
  Each line break is preserved.
  
  Including blank lines.

# Folded block (folds newlines to spaces)
summary: >
  This is a long summary that will be
  folded into a single line with spaces
  replacing the newlines.

# Literal with chomping (no trailing newline)
code: |-
  function main() {
      return 0;
  }
```

### 1.5 Lists and Maps

```yaml
# Block style (preferred for readability)
servers:
  - name: server-1
    host: 10.0.0.1
  - name: server-2
    host: 10.0.0.2

# Flow style (for short lists only)
tags: [production, critical, monitored]

# Nested structures
database:
  primary:
    host: db-primary.local
    port: 5432
  replicas:
    - host: db-replica-1.local
      port: 5432
    - host: db-replica-2.local
      port: 5432
```

---

## SECTION 2: SECURITY

### 2.1 Safe Parsing Only

**MUST use safe YAML loaders. MUST NOT use unsafe loaders.**

```python
# Python - CORRECT: Always use safe_load
import yaml
config = yaml.safe_load(yaml_string)

# Python - FORBIDDEN: Allows arbitrary code execution
# config = yaml.load(yaml_string)  # NEVER DO THIS
# config = yaml.load(yaml_string, Loader=yaml.Loader)  # NEVER DO THIS
```

```rust
// Rust - serde_yaml is safe by default
use serde_yaml;
let config: Config = serde_yaml::from_str(&yaml_string)?;
// serde_yaml does not support arbitrary code execution
```

```typescript
// TypeScript/Node.js - CORRECT: Use js-yaml safeLoad
import yaml from 'js-yaml';
const config = yaml.load(yamlString);  // js-yaml v4+ is safe by default

// For older js-yaml versions:
// const config = yaml.safeLoad(yamlString);
```

### 2.2 CI Enforcement for Safe Loading

**MUST enforce safe loading in CI:**

```yaml
# .github/workflows/yaml-security.yml
- name: Check for Unsafe YAML Loading
  run: |
    # Python: Fail if yaml.load( found without safe
    if grep -rn "yaml\.load\s*(" --include="*.py" . | grep -v "safe_load" | grep -v "SafeLoader"; then
      echo "ERROR: Use yaml.safe_load() instead of yaml.load()"
      exit 1
    fi
    
    # Check for explicit unsafe loaders
    if grep -rn "Loader=yaml\.Loader\|Loader=yaml\.UnsafeLoader\|Loader=yaml\.FullLoader" --include="*.py" .; then
      echo "ERROR: Unsafe YAML loader detected"
      exit 1
    fi
```

### 2.3 No Secrets in YAML

**MUST NOT include secrets directly in YAML files:**

```yaml
# WRONG: Secrets in config - SEVERE VIOLATION
database:
  password: super_secret_password
  api_key: sk-1234567890abcdef

# CORRECT: Reference environment variables
database:
  password: ${DATABASE_PASSWORD}
  api_key: ${API_KEY}

# CORRECT: Reference secrets manager
database:
  password_ref: vault:secret/database#password
  api_key_ref: aws:secretsmanager:api-credentials#key
```

### 2.4 Secret Detection in CI

**MUST scan for secrets in CI:**

```yaml
# .github/workflows/secrets.yml
- name: Secret Detection
  run: |
    pip install detect-secrets
    detect-secrets scan --all-files --exclude-files '\.git/.*' .
    
    # Or use gitleaks
    # docker run -v $(pwd):/path zricethezav/gitleaks:latest detect --source /path
```

### 2.5 Disable Dangerous Features

**MUST NOT use:**
- Custom tags (except standard YAML types)
- Python/Ruby object tags (`!!python/object`, `!!ruby/object`)
- External entity references

```yaml
# FORBIDDEN: Custom object tags - allows code execution
dangerous: !!python/object:os.system ["rm -rf /"]

# FORBIDDEN: Ruby objects
also_dangerous: !!ruby/object:Gem::Installer

# ALLOWED: Standard YAML tags only
explicit_null: !!null
explicit_bool: !!bool true
explicit_int: !!int 42
explicit_float: !!float 3.14
explicit_str: !!str "42"
```

### 2.6 Anchor Security (Billion Laughs Mitigation)

**MUST limit anchor expansion to prevent denial-of-service:**

```yaml
# WARNING: Deeply nested anchors can cause memory exhaustion
# This is the "Billion Laughs" attack for YAML

# DANGEROUS PATTERN - DO NOT USE:
# a: &a ["lol","lol","lol","lol","lol","lol","lol","lol","lol"]
# b: &b [*a,*a,*a,*a,*a,*a,*a,*a,*a]
# c: &c [*b,*b,*b,*b,*b,*b,*b,*b,*b]
# ... exponential expansion

# MITIGATION: Configure parser limits
```

```python
# Python - Set recursion and size limits
import yaml

# Use C-based loader with limits if available
try:
    from yaml import CSafeLoader as SafeLoader
except ImportError:
    from yaml import SafeLoader

# Custom loader with depth limit
class LimitedLoader(SafeLoader):
    MAX_DEPTH = 100
    MAX_ALIASES = 1000
```

---

## SECTION 3: SCHEMA VALIDATION

### 3.1 Schema Validation is MUST for Production

**Production and Controlled YAML files MUST have schema validation:**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["server", "database"],
  "properties": {
    "server": {
      "type": "object",
      "required": ["host", "port"],
      "properties": {
        "host": {
          "type": "string",
          "pattern": "^[a-zA-Z0-9.-]+$"
        },
        "port": {
          "type": "integer",
          "minimum": 1,
          "maximum": 65535
        }
      }
    },
    "database": {
      "type": "object",
      "required": ["url"],
      "properties": {
        "url": {
          "type": "string",
          "pattern": "^postgres://"
        }
      }
    }
  }
}
```

### 3.2 Validation in CI

```yaml
# .github/workflows/validate-yaml.yml
name: Validate YAML

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install validators
        run: |
          pip install yamllint jsonschema pyyaml
          
      - name: Lint YAML files
        run: yamllint -c .yamllint.yml .
        
      - name: Validate against schemas
        run: |
          for config in config/*.yml; do
            schema_name=$(basename "$config" .yml)
            if [ -f "schemas/${schema_name}.json" ]; then
              echo "Validating $config against schemas/${schema_name}.json"
              python -c "
          import yaml, json, jsonschema, sys
          with open('$config') as cf, open('schemas/${schema_name}.json') as sf:
              config = yaml.safe_load(cf)
              schema = json.load(sf)
              try:
                  jsonschema.validate(config, schema)
                  print('Valid')
              except jsonschema.ValidationError as e:
                  print(f'Invalid: {e.message}')
                  sys.exit(1)
              "
            fi
          done
```

### 3.3 yamllint Configuration

```yaml
# .yamllint.yml
extends: default

rules:
  line-length:
    max: 120
    level: warning
  indentation:
    spaces: 2
    indent-sequences: true
  document-start: disable
  truthy:
    allowed-values: ['true', 'false']
    check-keys: true
  comments:
    min-spaces-from-content: 2
  brackets:
    min-spaces-inside: 0
    max-spaces-inside: 0
  colons:
    max-spaces-before: 0
    max-spaces-after: 1
  commas:
    max-spaces-before: 0
    min-spaces-after: 1
  hyphens:
    max-spaces-after: 1
  key-duplicates: enable
  new-line-at-end-of-file: enable
  trailing-spaces: enable
```

---

## SECTION 4: STRUCTURE

### 4.1 File Organization

```yaml
# config.yml - Well-organized structure

# =============================================================================
# Server Configuration
# =============================================================================
server:
  host: 0.0.0.0
  port: 8080
  timeout_seconds: 30

# =============================================================================
# Database Configuration
# =============================================================================
database:
  url: ${DATABASE_URL}
  pool:
    min_connections: 5
    max_connections: 20

# =============================================================================
# Logging Configuration
# =============================================================================
logging:
  level: info
  format: json
  output: stdout

# =============================================================================
# Feature Flags
# =============================================================================
features:
  new_auth: true
  beta_api: false
```

### 4.2 Comments

**MUST document non-obvious settings:**

```yaml
server:
  # Maximum concurrent connections before rejecting new ones
  # Set based on available memory: ~1KB per connection
  max_connections: 1000
  
  # Timeout for idle connections (seconds)
  # Set to 0 to disable timeout (not recommended for production)
  idle_timeout: 300
  
  # Enable HTTP/2 support
  # Requires TLS to be enabled (http2 over plaintext not supported)
  http2: true
```

### 4.3 Anchors and Aliases (Use Sparingly)

```yaml
# ACCEPTABLE: Reducing duplication for simple values
defaults: &defaults
  timeout: 30
  retries: 3

services:
  api:
    <<: *defaults
    port: 8080
  
  worker:
    <<: *defaults
    port: 8081

# AVOID: Complex anchor chains that reduce readability
# If you need more than 2 levels of anchor references,
# consider restructuring the configuration
```

---

## SECTION 5: ENVIRONMENT-SPECIFIC CONFIG

### 5.1 Environment Variables

```yaml
# Use environment variable substitution
database:
  url: ${DATABASE_URL}
  password: ${DB_PASSWORD:-default_value}  # With default

# With validation in application
server:
  host: ${SERVER_HOST:?SERVER_HOST must be set}  # Required
```

### 5.2 Config File Hierarchy

```
config/
├── default.yml      # Base configuration
├── development.yml  # Development overrides
├── staging.yml      # Staging overrides
├── production.yml   # Production overrides
└── local.yml        # Local overrides (gitignored)
```

```rust
// Rust - config-rs pattern
use config::{Config, File, Environment};

let config = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_name(&format!("config/{}", env)).required(false))
    .add_source(File::with_name("config/local").required(false))
    .add_source(Environment::with_prefix("APP").separator("__"))
    .build()?;
```

---

## SECTION 6: NOTHING LOST, EVER (PRINCIPLE 9)

### 6.1 Durable Queue Configuration

```yaml
# Message broker durability settings (RabbitMQ example)
messaging:
  broker: rabbitmq
  connection:
    host: ${RABBITMQ_HOST}
    port: 5672
  
  # PRINCIPLE 9: Messages persist until acknowledged
  durability:
    # Queues survive broker restart
    durable_queues: true
    
    # Messages survive broker restart
    persistent_messages: true
    
    # Manual acknowledgment required
    auto_ack: false
    
    # Prefetch limit for fair dispatch
    prefetch_count: 10
    
    # Dead letter queue for failed messages
    dead_letter_exchange: dlx
    dead_letter_routing_key: failed
```

### 6.2 NATS JetStream Configuration

```yaml
# NATS JetStream durability settings
nats:
  servers:
    - nats://nats-1:4222
    - nats://nats-2:4222
    - nats://nats-3:4222
  
  jetstream:
    # PRINCIPLE 9: Stream persists messages
    streams:
      governance:
        subjects:
          - "governance.>"
        retention: limits
        max_msgs: 1000000
        max_bytes: 1073741824  # 1GB
        max_age: 2592000s      # 30 days
        storage: file          # Persist to disk
        replicas: 3            # Replicate across nodes
        
    # Consumer acknowledgment settings
    consumers:
      governance_processor:
        durable_name: governance_processor
        ack_policy: explicit   # MUST explicitly ACK
        ack_wait: 30s          # Redeliver if not ACKed
        max_deliver: 5         # Retry limit before DLQ
        filter_subject: "governance.>"
```

### 6.3 Database Durability Configuration

```yaml
# PostgreSQL durability settings
database:
  postgres:
    # PRINCIPLE 9: WAL ensures durability
    wal_level: replica
    synchronous_commit: "on"
    fsync: "on"
    full_page_writes: "on"
    
    # Checkpoint settings
    checkpoint_timeout: 5min
    checkpoint_completion_target: 0.9
    
    # Archive settings for point-in-time recovery
    archive_mode: "on"
    archive_command: "cp %p /archive/%f"
```

---

## SECTION 7: GITHUB ACTIONS SPECIFIC

### 7.1 Workflow Syntax

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      
      - name: Build
        run: cargo build --release
```

### 7.2 Matrix Strategy

```yaml
jobs:
  test:
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable, beta]
        exclude:
          - os: macos-latest
            rust: beta
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - run: cargo test
```

### 7.3 Secrets and Variables

```yaml
steps:
  - name: Deploy
    env:
      # CORRECT: Use secrets for sensitive data
      API_KEY: ${{ secrets.API_KEY }}
      DATABASE_URL: ${{ secrets.DATABASE_URL }}
      
      # CORRECT: Use variables for non-sensitive config
      ENVIRONMENT: ${{ vars.ENVIRONMENT }}
      LOG_LEVEL: ${{ vars.LOG_LEVEL }}
    run: ./deploy.sh
```

---

## SECTION 8: DOCKER COMPOSE SPECIFIC

### 8.1 Service Definition

```yaml
version: "3.9"

services:
  api:
    image: xonaix-api:${VERSION:-latest}
    container_name: xonaix-api
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=${DATABASE_URL}
      - LOG_LEVEL=${LOG_LEVEL:-info}
    depends_on:
      db:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    networks:
      - backend
    
  db:
    image: postgres:16
    container_name: xonaix-db
    restart: unless-stopped
    environment:
      POSTGRES_DB: xonaix
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER} -d xonaix"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - backend

volumes:
  postgres_data:

networks:
  backend:
    driver: bridge
```

### 8.2 Resource Limits

```yaml
services:
  api:
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M
```

---

## SECTION 9: KUBERNETES SPECIFIC

### 9.1 Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: xonaix-api
  namespace: production
  labels:
    app: xonaix-api
    version: v1.0.0
spec:
  replicas: 3
  selector:
    matchLabels:
      app: xonaix-api
  template:
    metadata:
      labels:
        app: xonaix-api
    spec:
      containers:
        - name: api
          image: ghcr.io/xonaix/api:1.0.0
          ports:
            - containerPort: 8080
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: db-credentials
                  key: url
          resources:
            requests:
              memory: "256Mi"
              cpu: "500m"
            limits:
              memory: "512Mi"
              cpu: "1000m"
          livenessProbe:
            httpGet:
              path: /health
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 30
          readinessProbe:
            httpGet:
              path: /ready
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          securityContext:
            runAsNonRoot: true
            readOnlyRootFilesystem: true
            allowPrivilegeEscalation: false
```

### 9.2 ConfigMap and Secret

```yaml
# ConfigMap for non-sensitive configuration
apiVersion: v1
kind: ConfigMap
metadata:
  name: xonaix-config
data:
  LOG_LEVEL: "info"
  ENVIRONMENT: "production"

---
# Secret for sensitive data (values should be base64 encoded)
apiVersion: v1
kind: Secret
metadata:
  name: db-credentials
type: Opaque
data:
  url: cG9zdGdyZXM6Ly91c2VyOnBhc3NAaG9zdDo1NDMyL2Ri  # base64 encoded
```

---

## SECTION 10: COMMON PATTERNS

### 10.1 Boolean Values

```yaml
# CORRECT: Use true/false only
enabled: true
disabled: false

# AVOID: Other boolean representations (YAML 1.1 legacy)
# enabled: yes    # Works but not recommended
# enabled: on     # Works but not recommended
# enabled: Yes    # Ambiguous
```

### 10.2 Null Values

```yaml
# Explicit null
optional_field: null
another_null: ~

# Implicit null (empty value)
empty_field:
```

### 10.3 Numbers

```yaml
# Integers
port: 8080
max_connections: 1000

# Floats
timeout: 30.5
ratio: 0.75

# Scientific notation
large_number: 1.0e6

# Octal (careful - must quote if you want string)
file_mode: 0o755  # YAML 1.2 octal notation
```

---

## SECTION 11: ANTI-PATTERNS

### 11.1 Avoid

```yaml
# NO Tabs for indentation
server:
	host: localhost  # TAB - will cause parse errors

# NO Secrets in plain text
database:
  password: my_secret_password  # SEVERE VIOLATION

# NO Overly complex anchors
base1: &base1
  a: 1
base2: &base2
  <<: *base1
  b: 2
base3: &base3
  <<: *base2
  c: 3
# Hard to follow - restructure instead

# NO Version without quotes
version: 1.10  # Becomes 1.1 (float truncation!)

# NO Ambiguous boolean-like strings
answer: no     # Becomes boolean false
status: yes    # Becomes boolean true
country: "NO"  # Must quote - Norway!

# NO Unsafe YAML loading
# yaml.load(data)  # Python - NEVER

# NO Missing schema validation for production configs
```

---

## CI Pipeline

```yaml
name: YAML Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Tools
        run: pip install yamllint jsonschema pyyaml detect-secrets
      
      - name: Lint YAML
        run: yamllint -c .yamllint.yml .
        
      - name: Check for Unsafe Loading
        run: |
          if grep -rn "yaml\.load\s*(" --include="*.py" . | grep -v "safe_load"; then
            echo "ERROR: Unsafe YAML loading detected"
            exit 1
          fi
          
      - name: Secret Detection
        run: detect-secrets scan --all-files .
        
      - name: Schema Validation
        run: |
          for config in config/*.yml; do
            schema="schemas/$(basename $config .yml).json"
            if [ -f "$schema" ]; then
              python -c "
          import yaml, json, jsonschema
          with open('$config') as c, open('$schema') as s:
              jsonschema.validate(yaml.safe_load(c), json.load(s))
          print('$config valid')
              "
            fi
          done
```

---

## X.I. Prompt Appendix

```
YAML v5.0.0 REQUIREMENTS:

NOTE: YAML is for CONFIGURATION only.
Use for CI/CD, Docker, Kubernetes, app config.

FORMATTING:
- 2-space indentation (NEVER tabs)
- Quote strings that could be misinterpreted
- Quote version numbers: "1.0" not 1.0
- UTF-8 encoding, no BOM, LF line endings
- Use block style for complex structures

SECURITY:
- ALWAYS use safe_load (Python) or equivalent safe parser
- NEVER use yaml.load() without SafeLoader
- No secrets in YAML files - use env vars or vault refs
- No custom tags or code execution features
- Limit anchor depth to prevent billion laughs

VALIDATION:
- Schema validation MUST for Production/Controlled
- yamllint in CI
- Secret detection in CI
- Environment-specific config separation

PRINCIPLE 9 (NOTHING LOST):
- Configure durable_queues: true
- Configure persistent_messages: true
- Configure auto_ack: false (explicit ACK)
- Configure dead letter queues

FORBIDDEN:
NO Tabs for indentation
NO Plain text secrets
NO Custom/dangerous YAML tags (!!python/object, etc.)
NO yaml.load() without SafeLoader
NO Unquoted version numbers (1.10 becomes 1.1)
NO Unquoted country codes (NO becomes false)
NO Complex anchor chains (>2 levels)
NO Missing schema validation for production

FLAG THESE VIOLATIONS:
NO Secrets in config files (grep for password, api_key, secret, token)
NO Tab characters
NO Missing schema validation
NO Ambiguous boolean/numeric values
NO yaml.load( without safe
NO Non-UTF-8 encoding
```

---

## Quick Reference

### Allowed Patterns

```yaml
# Safe loading
yaml.safe_load(data)  # Python
serde_yaml::from_str  # Rust

# Quoted ambiguous values
version: "1.0"
country: "NO"

# Environment variable references
password: ${DB_PASSWORD}

# Explicit booleans
enabled: true
disabled: false

# Schema validation in CI
```

### Forbidden Patterns

```yaml
# Unsafe loading
yaml.load(data)

# Plain secrets
password: mysecret

# Unquoted ambiguous
version: 1.0

# Tabs
	key: value

# Custom tags
data: !!python/object:module.Class
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L4 Configuration)
- **MAJOR:** Added Numeric Policy section (implicit typing danger)
- **MAJOR:** Added XCLib Integration section
- **MAJOR:** Added Parse Error Handling section
- **UPDATED:** Core-Compatible to 5.7.0
- **ALIGNED:** Cross-language requirements per STANDARDS_INDEX B-5.8.5
- **Source:** Red-Blue-Black Team synthesis with Founder approval

### v5.0.0 - Core-Compatible 5.1.0 (December 2025)
- **REVIEWED:** Rainbow Team compatibility review
- **CONFIRMED:** No content changes required for Core 5.1.0 compatibility
- **NOTE:** Deviation recording syntax compatible with cryptographic enforcement model

### v5.0.0 (December 2025)
- **ALIGNED:** Core-Version updated to 5.0.0
- **ADDED:** Principle 9 mapping (durable queues, persistence configs)
- **ADDED:** Deviation Recording syntax
- **ADDED:** CI enforcement for safe_load
- **ADDED:** Secret detection requirements
- **ADDED:** UTF-8 encoding requirement
- **ADDED:** Anchor security / billion laughs mitigation
- **ADDED:** Schema validation as MUST for Production/Controlled
- **ADDED:** NATS JetStream durability configuration
- **ADDED:** Database durability configuration
- **ADDED:** Complete CI pipeline example
- **UPDATED:** X.I. Prompt Appendix with v5.0.0 requirements

### v0.1.0 (December 2025)
- Initial draft
- Based on The Xonaix Way v1.1.0

---

*YAML Standards B-5.8.5 — Part of The Xonaix Way B-5.8.5*

*"Configuration should be boring. Safe loaders are non-negotiable."*

*Xonaix, Inc. — Intelligence, evolved.*
