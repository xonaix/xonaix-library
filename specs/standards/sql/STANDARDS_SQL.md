---
title: "SQL Database Standard"
unit_id: "library/standards/sql"
standard_type: "standard"
version: "1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# The Xonaix Way
## Standards: SQL

**Version:** B-5.8.5
**Status:** Active
**Core-Compatible:** 5.7.0
**Trust Class:** L3
**Created:** December 2025
**Last Reviewed:** December 2025

*This document implements The Xonaix Way B-5.8.5 principles for SQL queries across all database systems.*

---

## Document Info

| Field | Value |
|-------|-------|
| Domain | Database Queries |
| Status | **Active** |
| Version | B-5.8.5 |
| Core-Compatible | 5.7.0 |
| Trust Class | L3 (Orchestration / Data Access) |
| Created | December 2025 |
| Last Reviewed | December 2025 |
| Applies To | PostgreSQL (primary), all SQL databases |
| Related Standards | STANDARDS_RUST.md (for sqlx usage) |

**Prerequisites:** Read [THE_XONAIX_WAY.md](../THE_XONAIX_WAY.md) first.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L3 |
| Classification | Orchestration / Data Access |

SQL provides **data access** but **cannot assert authority**. All authority determinations MUST be verified by L1 code before SQL results are trusted for governance.

### What L3 May Do

- Query and retrieve data
- Store data with proper constraints
- Enforce row-level security policies
- Execute bounded queries

### What L3 May NOT Do

- Assert authority based on data alone
- Perform cryptographic operations
- Decide governance outcomes

---

## XCLib Integration

**Authority:** Founder Ruling 2025-003(c) — Attested Capability

SQL does not directly integrate with XCLib. Instead:

1. **Authority verification** happens in L1/L2 Rust code BEFORE SQL queries
2. **Results** are verified against signed authority proofs in application code
3. **Signatures** stored in database are verified by XCLib when read

### Pattern: Authority Before Query

```rust
// In Rust (L1/L2) code
async fn get_governance_data(actor: &Actor, db: &Pool) -> Result<Data, Error> {
    // FIRST: Verify authority with XCLib
    let proof = actor.authority_proof()?;
    xclib::verify_authority(&proof, Scope::GovernanceRead)?;
    
    // THEN: Query database
    let data = sqlx::query_as!(...)
        .fetch_one(db)
        .await?;
    
    Ok(data)
}
```

---

## Numeric Policy

**Authority:** Founder Ruling 2025-003(a) — Determinism

### Float Prohibition in Governance Tables

| Type | Status | Alternative |
|------|--------|-------------|
| `FLOAT` | FORBIDDEN | `DECIMAL(38,18)` |
| `DOUBLE PRECISION` | FORBIDDEN | `DECIMAL(38,18)` |
| `REAL` | FORBIDDEN | `INTEGER` or `DECIMAL` |

### Governance Table Schema Pattern

```sql
-- CORRECT: Use precise types
CREATE TABLE governance_decisions (
    id UUID PRIMARY KEY,
    amount DECIMAL(38,18) NOT NULL,  -- Precise decimal
    percentage INTEGER NOT NULL,      -- Basis points (150 = 1.5%)
    timestamp TIMESTAMPTZ NOT NULL,
    -- FORBIDDEN: amount FLOAT, rate DOUBLE PRECISION
);
```

### Enforcement

Add CHECK constraints to prevent float usage:

```sql
-- Migration to add governance table constraints
ALTER TABLE governance_decisions
ADD CONSTRAINT check_numeric_types CHECK (
    pg_typeof(amount)::text = 'numeric'
);
```

---

## Capability & Posture Handling

**Authority:** Constitution Article I, §4 — Zero Trust

### Prohibited Pattern: Authority from Data Alone

```sql
-- FORBIDDEN: Role claim without proof verification
SELECT * FROM actions 
WHERE user_role = 'admin';  -- Role is a claim, not proof!

-- FORBIDDEN: Trust stored role
SELECT a.* FROM actions a
JOIN users u ON a.user_id = u.id
WHERE u.is_admin = true;  -- Admin flag can be spoofed
```

### Required Pattern: Join with Verified Authority

```sql
-- CORRECT: Join with verified authority table
SELECT a.* FROM actions a
INNER JOIN verified_authorities v 
  ON a.actor_id = v.actor_id 
  AND v.verified_at > NOW() - INTERVAL '1 hour'
  AND v.capability_class >= 2
  AND v.signature_valid = true;
```

The `verified_authorities` table is populated by L1 code that has verified XCLib proofs.

---

## Determinism Requirements

### ORDER BY Mandate

All queries affecting governance logic MUST include explicit ORDER BY:

```sql
-- CORRECT: Explicit ordering
SELECT * FROM decisions 
WHERE tier >= 3 
ORDER BY created_at ASC, id ASC;

-- FORBIDDEN: Non-deterministic order
SELECT * FROM decisions 
WHERE tier >= 3;  -- Order not guaranteed!
```

### Query Bounds

All queries MUST have:

```sql
-- Timeout limits
SET statement_timeout = '30s';

-- Row count limits for unbounded queries
SELECT * FROM large_table
ORDER BY id
LIMIT 10000;
```

---

## Error Handling

**Authority:** Founder Ruling 2025-003(b) — Bounded Error Surfaces

### Error Messages Must Not Leak

```sql
-- Application code must catch and sanitize errors
-- Never expose raw PostgreSQL errors to clients

-- Error responses should be:
-- E001: Query failed
-- E002: Permission denied
-- E003: Not found
-- Never: "ERROR: relation 'secret_table' does not exist"
```

---

## Generated Code Accountability

**Authority:** Constitutional Actor Model

SQL generated by Forge, agents, or query builders MUST:

1. Pass same security review as human-written SQL
2. Include provenance comments
3. Meet all determinism requirements

```sql
-- @xonaix-generated: forge:1.2.3:template:governance-query
-- @xonaix-trust-class: L3
SELECT * FROM decisions 
WHERE tier >= 3 
ORDER BY created_at ASC;
```

---

## Principle Mapping

| Principle | SQL Implementation |
|-----------|-------------------|
| 1. Correct Over Fast | Correct results over query speed. Optimize only proven bottlenecks. |
| 2. Explicit Over Implicit | Explicit column lists, explicit JOINs, explicit aliases |
| 3. Automated Over Vigilant | Migration-managed schema, automated index suggestions |
| 4. Secure By Default | Parameterized queries always. No string concatenation. RLS policies. |
| 5. Composable Over Clever | Simple queries over complex CTEs when possible |
| 6. Fail Loud | Queries fail on errors, not return partial results silently |
| 7. X.I. Augments, Human Decides | Query optimization suggestions reviewed by humans |
| 8. Future-Proof Over Trend | Standard SQL, database-specific features documented |
| 9. Nothing Lost, Ever | WAL=full, fsync=on, transaction durability, audit logging |

---

## Deviation Recording (User Choice)

For deviations from MUST requirements (e.g., non-parameterized query in legacy):

```sql
-- XONAIX_DEVIATION: Legacy reporting query requires dynamic SQL - migration planned Q2
-- LEDGER_ACK: sha256:abc123...
SELECT * FROM users WHERE id = 1; -- Static ID, not parameterized
```

**This triggers ledger recording and marks output as "User Choice".**

---

## SECTION 1: SQL INJECTION PREVENTION

### 1.1 Parameterized Queries (MANDATORY)

**MUST use parameterized queries for ALL user input:**

```rust
// CORRECT: Parameterized query
let users = sqlx::query_as!(
    User,
    "SELECT id, email FROM users WHERE email = $1",
    user_email
)
.fetch_all(&pool)
.await?;

// CORRECT: Multiple parameters
let users = sqlx::query_as!(
    User,
    "SELECT id, email FROM users WHERE status = $1 AND created_at > $2",
    status,
    since_date
)
.fetch_all(&pool)
.await?;
```

**FORBIDDEN - String concatenation:**

```rust
// NO NEVER DO THIS - SQL INJECTION VULNERABILITY
let query = format!("SELECT * FROM users WHERE email = '{}'", user_email);

// NO NEVER DO THIS - Even with "escaping"
let query = format!("SELECT * FROM users WHERE id = {}", user_id.replace("'", "''"));
```

### 1.2 Dynamic Queries

When query structure must vary, use query builders or whitelisted values:

```rust
// CORRECT: Whitelist for column names
fn get_order_column(requested: &str) -> &'static str {
    match requested {
        "email" => "email",
        "created_at" => "created_at",
        "name" => "name",
        _ => "created_at", // Safe default
    }
}

// CORRECT: Query builder pattern
let mut query = QueryBuilder::new("SELECT id, email FROM users WHERE 1=1");

if let Some(status) = filter.status {
    query.push(" AND status = ");
    query.push_bind(status);
}

if let Some(org_id) = filter.organization_id {
    query.push(" AND organization_id = ");
    query.push_bind(org_id);
}

let users = query.build_query_as::<User>().fetch_all(&pool).await?;
```

### 1.3 LIKE Queries

```rust
// CORRECT: Escape LIKE wildcards in user input
fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

let search_pattern = format!("%{}%", escape_like(&user_search));
let users = sqlx::query_as!(
    User,
    "SELECT id, email FROM users WHERE email LIKE $1 ESCAPE '\\'",
    search_pattern
)
.fetch_all(&pool)
.await?;
```

### 1.4 Dynamic Identifiers (MUST)

**MUST use whitelists or query builders for dynamic identifiers:**

Dynamic identifiers (column names, table names, sort orders) derived from user input MUST be validated against explicit allowlists or constructed via query builder APIs.

**MUST:**
```rust
// CORRECT: Whitelist for ORDER BY
fn get_order_column(requested: &str) -> &'static str {
    match requested {
        "email" => "email",
        "created_at" => "created_at",
        "name" => "name",
        _ => "created_at", // Safe default
    }
}

// CORRECT: Query builder with validated identifier
let order_col = get_order_column(&user_input);
let query = format!("SELECT id, email FROM users ORDER BY {}", order_col);
```

**MUST NOT:**
```rust
// VIOLATION: Direct interpolation of user input
let query = format!("SELECT * FROM users ORDER BY {}", user_input);

// VIOLATION: String replacement without validation
let query = base_query.replace("{sort}", &user_input);
```

**Scope:**
- Column names in ORDER BY, GROUP BY, SELECT
- Table names in dynamic queries
- Schema names in multi-tenant contexts
- Any identifier position accepting external input

**CI Enforcement:**
- Linters SHALL flag raw string interpolation in identifier positions
- Query builders with type-safe identifier APIs are RECOMMENDED
- Violations block merge for Production/Controlled classification

---

## SECTION 2: QUERY FORMATTING

### 2.1 Capitalization

**Keywords:** UPPERCASE
**Identifiers:** lowercase

```sql
-- CORRECT
SELECT id, email, created_at
FROM users
WHERE status = 'active'
ORDER BY created_at DESC;

-- NO AVOID: inconsistent capitalization
select ID, Email, CREATED_AT from USERS where Status = 'active';
```

### 2.2 Indentation

- Use 4 spaces (not tabs)
- Align major clauses
- Indent continued lines

```sql
-- CORRECT: Readable multi-line query
SELECT 
    u.id,
    u.email,
    u.status,
    o.name AS organization_name,
    COUNT(d.id) AS document_count
FROM users u
INNER JOIN organizations o 
    ON u.organization_id = o.id
LEFT JOIN documents d 
    ON d.user_id = u.id
WHERE u.status = 'active'
    AND u.created_at > '2025-01-01'
GROUP BY u.id, u.email, u.status, o.name
HAVING COUNT(d.id) > 0
ORDER BY u.created_at DESC
LIMIT 100;
```

### 2.3 Column Lists

**MUST:**
- List columns explicitly (no `SELECT *` in application code)
- One column per line for queries with many columns
- Always alias calculated columns

```sql
-- CORRECT: Explicit columns
SELECT 
    id,
    email,
    created_at
FROM users;

-- NO FORBIDDEN in application code
SELECT * FROM users;

-- CORRECT: Aliased calculations
SELECT 
    id,
    email,
    EXTRACT(YEAR FROM created_at) AS signup_year,
    COUNT(*) OVER () AS total_count
FROM users;
```

**Exception:** `SELECT *` is acceptable in:
- Ad-hoc queries during development
- Migrations checking data
- `EXISTS` subqueries (but prefer `SELECT 1`)

---

## SECTION 3: JOINS

### 3.1 Explicit JOIN Syntax

**MUST use explicit JOIN syntax:**

```sql
-- CORRECT: Explicit JOIN
SELECT u.id, u.email, o.name
FROM users u
INNER JOIN organizations o ON u.organization_id = o.id;

-- NO FORBIDDEN: Implicit join (comma syntax)
SELECT u.id, u.email, o.name
FROM users u, organizations o
WHERE u.organization_id = o.id;
```

### 3.2 JOIN Types

| Type | Use Case |
|------|----------|
| `INNER JOIN` | Both tables must have matching rows |
| `LEFT JOIN` | Include all rows from left table |
| `RIGHT JOIN` | Avoid - rewrite as LEFT JOIN |
| `FULL OUTER JOIN` | Rare - document why needed |
| `CROSS JOIN` | Rare - document why needed |

### 3.3 Table Aliases

**MUST:**
- Use meaningful aliases (not `a`, `b`, `c`)
- Prefix all columns with alias when joining

```sql
-- CORRECT: Meaningful aliases, all columns prefixed
SELECT 
    u.id,
    u.email,
    org.name AS organization_name,
    doc.title AS document_title
FROM users u
INNER JOIN organizations org ON u.organization_id = org.id
LEFT JOIN documents doc ON doc.user_id = u.id;

-- NO AVOID: Single letter aliases
SELECT a.id, b.name
FROM users a
JOIN organizations b ON a.organization_id = b.id;
```

---

## SECTION 4: FILTERING

### 4.1 WHERE Clauses

```sql
-- CORRECT: Multiple conditions
SELECT id, email
FROM users
WHERE status = 'active'
    AND organization_id = $1
    AND created_at > $2;

-- CORRECT: OR with parentheses
SELECT id, email
FROM users
WHERE organization_id = $1
    AND (status = 'active' OR status = 'pending');
```

### 4.2 NULL Handling

**MUST use IS NULL / IS NOT NULL:**

```sql
-- CORRECT
SELECT id FROM users WHERE deleted_at IS NULL;
SELECT id FROM users WHERE deleted_at IS NOT NULL;

-- NO WRONG: NULL comparisons with = don't work
SELECT id FROM users WHERE deleted_at = NULL;  -- Always returns no rows!
```

### 4.3 IN Clauses

```sql
-- CORRECT: Small lists
SELECT id FROM users WHERE status IN ('active', 'pending', 'review');

-- For large lists, use parameterized arrays (PostgreSQL)
SELECT id FROM users WHERE id = ANY($1::uuid[]);
```

---

## SECTION 5: AGGREGATION

### 5.1 GROUP BY

**MUST include all non-aggregated columns in GROUP BY:**

```sql
-- CORRECT
SELECT 
    organization_id,
    status,
    COUNT(*) AS user_count
FROM users
GROUP BY organization_id, status;

-- PostgreSQL allows grouping by primary key (includes all columns)
SELECT 
    u.id,
    u.email,
    u.organization_id,
    COUNT(d.id) AS document_count
FROM users u
LEFT JOIN documents d ON d.user_id = u.id
GROUP BY u.id;  -- PK includes all user columns
```

### 5.2 HAVING vs WHERE

```sql
-- WHERE: Filter rows before aggregation
-- HAVING: Filter groups after aggregation

SELECT 
    organization_id,
    COUNT(*) AS user_count
FROM users
WHERE status = 'active'          -- Filter: only active users
GROUP BY organization_id
HAVING COUNT(*) > 10;            -- Filter: only orgs with >10 users
```

---

## SECTION 6: SUBQUERIES AND CTEs

### 6.1 Subquery Placement

```sql
-- Scalar subquery in SELECT
SELECT 
    id,
    email,
    (SELECT COUNT(*) FROM documents WHERE user_id = users.id) AS doc_count
FROM users;

-- Subquery in WHERE
SELECT id, email
FROM users
WHERE organization_id IN (
    SELECT id FROM organizations WHERE status = 'active'
);

-- Prefer EXISTS for existence checks
SELECT id, email
FROM users u
WHERE EXISTS (
    SELECT 1 FROM documents d WHERE d.user_id = u.id
);
```

### 6.2 Common Table Expressions (CTEs)

```sql
-- CORRECT: CTE for readability
WITH active_orgs AS (
    SELECT id, name
    FROM organizations
    WHERE status = 'active'
),
user_counts AS (
    SELECT 
        organization_id,
        COUNT(*) AS user_count
    FROM users
    WHERE status = 'active'
    GROUP BY organization_id
)
SELECT 
    ao.name,
    COALESCE(uc.user_count, 0) AS user_count
FROM active_orgs ao
LEFT JOIN user_counts uc ON uc.organization_id = ao.id
ORDER BY user_count DESC;
```

**Guidelines:**
- Use CTEs for complex queries with multiple logical steps
- Don't over-use CTEs for simple queries
- Consider performance (CTEs may not optimize as well as subqueries in some cases)

---

## SECTION 7: QUERY TIMEOUTS

### 7.1 Statement Timeouts (MUST)

**All queries MUST have timeouts to prevent DoS:**

```sql
-- PostgreSQL: Set session-level timeout
SET statement_timeout = '30s';

-- Or per-query
SET LOCAL statement_timeout = '10s';
SELECT * FROM large_table WHERE ...;
```

```rust
// Application level with sqlx
use tokio::time::{timeout, Duration};

async fn fetch_users(pool: &PgPool) -> Result<Vec<User>, QueryError> {
    timeout(
        Duration::from_secs(30),
        sqlx::query_as!(User, "SELECT id, email FROM users")
            .fetch_all(pool)
    )
    .await
    .map_err(|_| QueryError::Timeout)?
    .map_err(QueryError::Database)
}
```

### 7.2 Timeout Guidelines

| Query Type | Timeout | Rationale |
|------------|---------|-----------|
| Simple lookup | 5s | Should be instant with index |
| Complex join | 30s | May require table scans |
| Report/aggregate | 60s | Expected to be slow |
| Batch operation | 300s | Large data processing |

---

## SECTION 8: ROW-LEVEL SECURITY (RLS)

### 8.1 Enabling RLS

**Multi-tenant systems MUST use RLS for data isolation:**

```sql
-- Enable RLS on table
ALTER TABLE documents ENABLE ROW LEVEL SECURITY;

-- Force RLS even for table owners
ALTER TABLE documents FORCE ROW LEVEL SECURITY;
```

### 8.2 RLS Policies

```sql
-- Policy: Users can only see their own documents
CREATE POLICY user_documents ON documents
    FOR ALL
    USING (user_id = current_setting('app.current_user_id')::uuid);

-- Policy: Organization members see org documents
CREATE POLICY org_documents ON documents
    FOR SELECT
    USING (
        organization_id IN (
            SELECT organization_id FROM users 
            WHERE id = current_setting('app.current_user_id')::uuid
        )
    );

-- Policy: Admins see all
CREATE POLICY admin_documents ON documents
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM users 
            WHERE id = current_setting('app.current_user_id')::uuid 
            AND role = 'admin'
        )
    );
```

### 8.3 Setting User Context

```rust
// Set user context for RLS
async fn with_user_context<T>(
    pool: &PgPool,
    user_id: Uuid,
    f: impl FnOnce(&PgPool) -> T,
) -> Result<T, sqlx::Error> {
    sqlx::query("SELECT set_config('app.current_user_id', $1, true)")
        .bind(user_id.to_string())
        .execute(pool)
        .await?;
    
    Ok(f(pool))
}
```

---

## SECTION 9: AUDIT LOGGING

### 9.1 Audit Trigger

**Controlled systems MUST have audit logging:**

```sql
-- Audit log table
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name TEXT NOT NULL,
    operation TEXT NOT NULL,
    old_data JSONB,
    new_data JSONB,
    changed_by UUID,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Generic audit trigger function
CREATE OR REPLACE FUNCTION audit_trigger_func() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO audit_log (
        table_name,
        operation,
        old_data,
        new_data,
        changed_by
    ) VALUES (
        TG_TABLE_NAME,
        TG_OP,
        CASE WHEN TG_OP = 'DELETE' OR TG_OP = 'UPDATE' 
             THEN row_to_json(OLD) ELSE NULL END,
        CASE WHEN TG_OP = 'INSERT' OR TG_OP = 'UPDATE' 
             THEN row_to_json(NEW) ELSE NULL END,
        NULLIF(current_setting('app.current_user_id', true), '')::UUID
    );
    
    IF TG_OP = 'DELETE' THEN
        RETURN OLD;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Apply to table
CREATE TRIGGER users_audit
    AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_func();
```

### 9.2 Audit Log Retention

Per classification requirements:
- **Production:** 1 year minimum
- **Controlled:** 7 years or regulatory requirement, WORM storage

---

## SECTION 10: CONNECTION SECURITY

### 10.1 SSL/TLS Required

**Production and Controlled MUST use encrypted connections:**

```rust
// CORRECT: Require SSL
let pool = PgPoolOptions::new()
    .max_connections(20)
    .connect("postgres://user:pass@host/db?sslmode=require")
    .await?;

// CORRECT: Verify certificate
let pool = PgPoolOptions::new()
    .max_connections(20)
    .connect("postgres://user:pass@host/db?sslmode=verify-full&sslrootcert=/path/to/ca.pem")
    .await?;
```

### 10.2 Connection Pool Settings

```rust
let pool = PgPoolOptions::new()
    .max_connections(20)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(1800))
    .connect(&database_url)
    .await?;
```

### 10.3 PQC for Connections (Phase 3)

By 2027, Controlled connections SHOULD use TLS with hybrid key exchange supporting post-quantum algorithms where database servers support it.

---

## SECTION 11: TRANSACTIONS

### 11.1 Transaction Boundaries

```rust
// CORRECT: Explicit transaction
let mut tx = pool.begin().await?;

sqlx::query!("INSERT INTO users (email) VALUES ($1)", email)
    .execute(&mut *tx)
    .await?;

sqlx::query!(
    "INSERT INTO audit_logs (action, user_email) VALUES ($1, $2)", 
    "user_created", 
    email
)
    .execute(&mut *tx)
    .await?;

tx.commit().await?;

// On error, transaction automatically rolls back when dropped
```

### 11.2 Isolation Levels

| Level | Use Case |
|-------|----------|
| Read Committed | Default, suitable for most operations |
| Repeatable Read | Reports requiring consistent snapshot |
| Serializable | Financial transactions, inventory |

```rust
let mut tx = pool.begin().await?;
sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
    .execute(&mut *tx)
    .await?;
// ... critical operations ...
tx.commit().await?;
```

### 11.3 Deadlock Prevention

**MUST:**
- Access tables in consistent order
- Keep transactions short
- Use `SELECT ... FOR UPDATE` carefully

```sql
-- CORRECT: Lock rows in consistent order
SELECT id FROM accounts WHERE id IN ($1, $2) ORDER BY id FOR UPDATE;

-- Then perform updates
UPDATE accounts SET balance = balance - $3 WHERE id = $1;
UPDATE accounts SET balance = balance + $3 WHERE id = $2;
```

---

## SECTION 12: PERFORMANCE

### 12.1 Query Analysis

**MUST analyze slow queries:**

```sql
EXPLAIN ANALYZE
SELECT u.id, u.email, COUNT(d.id)
FROM users u
LEFT JOIN documents d ON d.user_id = u.id
WHERE u.status = 'active'
GROUP BY u.id;
```

### 12.2 Pagination

**MUST use keyset pagination for large datasets:**

```sql
-- CORRECT: Keyset pagination (efficient)
SELECT id, email, created_at
FROM users
WHERE created_at < $1  -- Last seen timestamp
    OR (created_at = $1 AND id < $2)  -- Tiebreaker
ORDER BY created_at DESC, id DESC
LIMIT 20;

-- NO AVOID: OFFSET pagination (inefficient for large offsets)
SELECT id, email FROM users ORDER BY created_at LIMIT 20 OFFSET 10000;
```

### 12.3 LIMIT

**MUST:**
- Always use LIMIT for user-facing queries
- Set reasonable maximums server-side

```sql
-- Application code with server-side cap
SELECT id, email, created_at
FROM users
WHERE organization_id = $1
ORDER BY created_at DESC
LIMIT LEAST($2, 100);  -- User requested limit, capped at 100
```

### 12.4 Counting

```sql
-- For exact counts (use when needed)
SELECT COUNT(*) FROM users WHERE status = 'active';

-- For existence checks (faster)
SELECT EXISTS(SELECT 1 FROM users WHERE email = $1);

-- For estimates on large tables (faster, approximate)
SELECT reltuples::bigint AS estimate
FROM pg_class
WHERE relname = 'users';
```

---

## SECTION 13: DATA MODIFICATION

### 13.1 INSERT

```sql
-- CORRECT: Explicit column list
INSERT INTO users (email, status, organization_id)
VALUES ($1, $2, $3)
RETURNING id, created_at;

-- CORRECT: Bulk insert
INSERT INTO users (email, status, organization_id)
VALUES 
    ($1, $2, $3),
    ($4, $5, $6),
    ($7, $8, $9)
RETURNING id;

-- CORRECT: Upsert (PostgreSQL)
INSERT INTO users (email, status)
VALUES ($1, $2)
ON CONFLICT (email) DO UPDATE SET
    status = EXCLUDED.status,
    updated_at = NOW()
RETURNING id;
```

### 13.2 UPDATE

```sql
-- CORRECT: Always have WHERE clause
UPDATE users
SET status = $1, updated_at = NOW()
WHERE id = $2
RETURNING id, status, updated_at;

-- CORRECT: Conditional update
UPDATE users
SET 
    status = CASE 
        WHEN last_login < NOW() - INTERVAL '90 days' THEN 'inactive'
        ELSE status
    END,
    updated_at = NOW()
WHERE organization_id = $1;
```

### 13.3 DELETE

```sql
-- CORRECT: Always have WHERE clause
DELETE FROM sessions
WHERE user_id = $1
    AND created_at < NOW() - INTERVAL '7 days'
RETURNING id;

-- Prefer soft delete for audit trail
UPDATE users
SET deleted_at = NOW()
WHERE id = $1
RETURNING id;
```

---

## SECTION 14: NOTHING LOST, EVER (PRINCIPLE 9)

### 14.1 WAL Configuration

**PostgreSQL MUST be configured for durability:**

```ini
# postgresql.conf
wal_level = replica
fsync = on
synchronous_commit = on
full_page_writes = on
```

### 14.2 Transaction Durability

```sql
-- Ensure transaction is synced to disk
COMMIT;
-- Or explicitly
SET synchronous_commit = on;
```

### 14.3 Backup and Recovery

```bash
# WAL archiving for point-in-time recovery
archive_mode = on
archive_command = 'cp %p /archive/%f'
```

---

## SECTION 15: NASA/DOD GRADE REQUIREMENTS

### 15.1 Bounded Queries

All queries MUST have result limits:

```sql
-- CORRECT: Always bounded
SELECT * FROM large_table LIMIT 10000;

-- CORRECT: With server-side cap
SELECT * FROM users LIMIT LEAST($1, 1000);
```

### 15.2 Query Coverage

**Controlled classification MUST have ≥95% query coverage in tests.**

```rust
#[sqlx::test]
async fn test_user_query(pool: PgPool) -> sqlx::Result<()> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email FROM users WHERE id = $1",
        test_user_id
    )
    .fetch_one(&pool)
    .await?;
    
    assert_eq!(user.email, "test@example.com");
    Ok(())
}
```

### 15.3 Mutation Testing

Test query logic by mutating:
- WHERE conditions
- JOIN types
- ORDER BY columns
- LIMIT values

---

## SECTION 16: COMMENTS

### 16.1 Query Comments

```sql
-- Single line comment for context
SELECT id, email FROM users WHERE status = 'active';

/*
 * Multi-line comment for complex queries
 * explaining business logic
 */
SELECT 
    u.id,
    u.email
FROM users u
WHERE u.status = 'active';
```

### 16.2 Migration Comments

```sql
-- Migration: 20251214_001_add_user_phone
-- Purpose: Add phone number field for MFA recovery
-- Author: system
-- Reversible: Yes

ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- Down migration
-- ALTER TABLE users DROP COLUMN phone;
```

---

## SECTION 17: ANTI-PATTERNS

### 17.1 FORBIDDEN Patterns

```sql
-- NO SELECT * in application code
SELECT * FROM users;

-- NO String concatenation for values
"SELECT * FROM users WHERE email = '" + email + "'"

-- NO Implicit joins
SELECT * FROM users, organizations WHERE users.org_id = organizations.id;

-- NO UPDATE/DELETE without WHERE
UPDATE users SET status = 'inactive';  -- Updates ALL rows!
DELETE FROM logs;  -- Deletes ALL rows!

-- NO LIKE without escaping user input
WHERE email LIKE '%' || user_input || '%'

-- NO ORDER BY column number
ORDER BY 1, 2  -- Use column names

-- NO NOT IN with NULLs (returns no rows)
WHERE id NOT IN (SELECT user_id FROM banned WHERE user_id IS NOT NULL)
```

### 17.2 Code Smells

```sql
-- WARNING N+1 queries (fix with JOIN or batch)
-- Instead of looping and querying each user's org

-- WARNING Large OFFSET values
OFFSET 100000  -- Use keyset pagination

-- WARNING Functions on indexed columns in WHERE
WHERE LOWER(email) = 'test@test.com'  -- Can't use index
-- Fix: Create functional index or store normalized

-- WARNING OR conditions that prevent index use
WHERE status = 'active' OR organization_id = $1
-- Consider: UNION of two indexed queries
```

---

## SECTION 18: CI PIPELINE

```yaml
name: SQL Checks

on: [push, pull_request]

jobs:
  sql-checks:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Run Migrations
        run: |
          psql $DATABASE_URL -f migrations/*.sql
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost/postgres
      
      - name: SQL Lint
        run: |
          pip install sqlfluff
          sqlfluff lint sql/
      
      - name: Test Queries
        run: cargo test --features sqlx-test
```

---

## X.I. Prompt Appendix

```
SQL v5.1.0 REQUIREMENTS:

SECURITY:
- ALWAYS use parameterized queries
- NEVER concatenate user input into SQL
- Escape LIKE wildcards in user input
- Whitelist dynamic column/table names
- Use RLS for multi-tenant data isolation
- Require SSL for Production/Controlled connections

FORMATTING:
- Keywords UPPERCASE
- Identifiers lowercase
- Explicit column lists (no SELECT *)
- Explicit JOIN syntax

QUERY PATTERNS:
- WHERE clause on UPDATE/DELETE (always)
- LIMIT on user-facing queries (always)
- Query timeouts (always)
- Keyset pagination for large datasets
- EXISTS for existence checks

TRANSACTIONS:
- Explicit transaction boundaries
- Appropriate isolation level
- Short transaction duration
- Consistent table access order

NASA/DOD GRADE:
- All queries bounded with LIMIT
- Query timeouts enforced
- ≥95% query coverage for Controlled
- Audit logging for Controlled

PRINCIPLE 9 (NOTHING LOST):
- WAL = full
- fsync = on
- synchronous_commit = on
- Audit triggers on critical tables

FORBIDDEN:
NO String concatenation for query building
NO SELECT * in application code
NO Implicit joins (comma syntax)
NO UPDATE/DELETE without WHERE
NO ORDER BY column numbers
NO Queries without timeout
NO Large OFFSET pagination

FLAG THESE VIOLATIONS:
NO User input in query string
NO Missing parameterization
NO SELECT * in production code
NO Missing RLS on multi-tenant tables
NO Missing audit logging on Controlled tables
NO Unencrypted connections
```

---

## Quick Reference

### Allowed Patterns

```sql
-- Parameterized query
SELECT * FROM users WHERE id = $1;

-- Explicit columns
SELECT id, email, status FROM users;

-- Explicit JOIN
SELECT u.*, o.name FROM users u JOIN organizations o ON u.org_id = o.id;

-- Bounded query
SELECT * FROM users LIMIT 100;

-- Keyset pagination
SELECT * FROM users WHERE id > $1 ORDER BY id LIMIT 20;
```

### Forbidden Patterns

```sql
-- String concatenation
"SELECT * FROM users WHERE email = '" + email + "'"

-- SELECT * in production
SELECT * FROM users;

-- Implicit join
SELECT * FROM users, orgs WHERE users.org_id = orgs.id;

-- Unbounded query
SELECT * FROM users;

-- Large offset
SELECT * FROM users OFFSET 100000;
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L3 Data Access)
- **MAJOR:** Added XCLib Integration section (authority before query)
- **MAJOR:** Added Numeric Policy section (FLOAT/DOUBLE forbidden)
- **MAJOR:** Added Capability & Posture Handling section
- **MAJOR:** Added Determinism Requirements section (ORDER BY mandate)
- **MAJOR:** Added Error Handling section
- **MAJOR:** Added Generated Code Accountability section
- **UPDATED:** Core-Compatible to 5.7.0
- **ALIGNED:** Cross-language requirements per STANDARDS_INDEX B-5.8.5
- **Source:** Red-Blue-Black Team synthesis with Founder approval

### v5.1.0 (December 2025)
- **RATIFIED:** Rainbow Team consensus (Red-Blue-White-Green)
- **ADDED:** Section 1.4 Dynamic Identifiers (MUST use whitelists/builders)
- **ADDED:** CI lint requirement for raw identifier detection
- **ALIGNED:** Synchronized with The Xonaix Way v5.1.0
- **ADDED:** Principle 9 (Nothing Lost) - WAL, fsync, audit logging
- **ADDED:** Deviation Recording syntax
- **ADDED:** Query timeout requirements (MUST)
- **ADDED:** Row-Level Security (RLS) section
- **ADDED:** Audit logging section with trigger example
- **ADDED:** Connection security (SSL/TLS, PQC roadmap)
- **ADDED:** NASA/DOD grade requirements section
- **ADDED:** Bounded query requirements
- **ADDED:** CI pipeline example
- **UPDATED:** X.I. Prompt Appendix

### v0.1.0 (December 2025)
- Initial draft
- SQL injection prevention, formatting, patterns

---

*SQL Standards B-5.8.5 — Part of The Xonaix Way B-5.8.5*

*"Parameterized queries are non-negotiable. Authority comes from XCLib, not data."*

*Xonaix, Inc. — Intelligence, evolved.*
