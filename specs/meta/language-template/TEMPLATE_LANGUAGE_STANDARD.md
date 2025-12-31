---
title: "Language Standard Template"
unit_id: "library/meta/language-template"
standard_type: "standard"
version: "1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# Language Standard Template

## [LANGUAGE_NAME] Language Standard

**For principles:** See THE_XONAIX_WAY.md in xonaix-specs/core.

**For cross-language requirements:** See STANDARDS_INDEX.md.

---

## Document Info

| Field | Value |
|-------|-------|
| Language | [LANGUAGE_NAME] |
| Version | 1.0.0 |
| Trust Class | L[1-4] |
| Status | [Active/Deprecated] |
| Owner | Founder |
| Last Updated | [YYYY-MM-DD] |

---

## SECTION 1: TRUST CLASS

| Attribute | Value |
|-----------|-------|
| Trust Class | L[1-4] |
| Classification | [Constitutional/Deterministic/Orchestration/Interface] |

### What This Trust Class May Do

- [Capability 1]
- [Capability 2]

### What This Trust Class May NOT Do

- [Restriction 1]
- [Restriction 2]

---

## SECTION 2: CROSS-LANGUAGE REQUIREMENTS

*Implementation of requirements from STANDARDS_INDEX.md.*

### 2.1 XCLib Integration

| Operation | Permitted | Method |
|-----------|-----------|--------|
| Canonicalization | [Yes/No] | [How] |
| Hashing | [Yes/No] | [How] |
| Signing | [Yes/No] | [How] |
| Verification | [Yes/No] | [How] |

**Prohibited Patterns:**
```[language]
// Patterns that violate XCLib exclusivity
```

### 2.2 Numeric Policy

| Type | Status | Alternative |
|------|--------|-------------|
| [e.g., f32] | Forbidden | [e.g., i64] |

**Required Representations:**
```[language]
// Correct numeric handling
amount_cents: i64  // NOT amount: f64
```

### 2.3 Authority Verification

Authority must never be inferred. Always verify cryptographic proofs.

```[language]
// CORRECT: Verify proof
fn check_authority(proof: &Proof) -> Result<Authority> {
    xclib::verify(proof)?
}

// FORBIDDEN: Infer from context
fn check_authority(role: &str) -> bool {
    role == "admin"  // NO!
}
```

### 2.4 Error Handling

| Context | Strategy |
|---------|----------|
| L1/L2 paths | [e.g., panic=abort] |
| Public API | [e.g., Result types] |
| Boundary | [e.g., mapped to PublicErrorCode] |

---

## SECTION 3: PRINCIPLE IMPLEMENTATION

*How this language implements principles from THE_XONAIX_WAY.md.*

| Principle | Implementation |
|-----------|----------------|
| Correct Over Fast | [How this language ensures correctness] |
| Explicit Over Implicit | [How this language avoids magic] |
| Automated Over Vigilant | [Tooling: linters, formatters, CI] |
| Secure By Default | [Security patterns] |
| Composable Over Clever | [Modularity patterns] |
| Fail Loud | [Error handling patterns] |
| Xona Augments, Human Decides | [Xona collaboration patterns] |
| Future-Proof Over Trend | [Stability practices] |
| Nothing Lost, Ever | [Persistence, durability patterns] |

---

## SECTION 4: PROJECT CONFIGURATION

### 4.1 Compiler/Runtime Configuration

```[config-format]
// Configuration here
```

### 4.2 Linting Configuration

```[config-format]
// Linter configuration
```

### 4.3 Formatting Configuration

```[config-format]
// Formatter configuration
```

---

## SECTION 5: TESTING REQUIREMENTS

### 5.1 Coverage Requirements

| Classification | Line Coverage | Branch Coverage |
|----------------|---------------|-----------------|
| Development | ≥60% | ≥50% |
| Production | ≥80% | ≥70% |
| Controlled | ≥95% | ≥90% |

### 5.2 Bounded Execution

All loops MUST have provable termination bounds.

```[language]
// CORRECT: Bounded loop
const MAX_ITERATIONS = 10_000;
for (i, item) in items.iter().enumerate() {
    if i >= MAX_ITERATIONS {
        return Err("Exceeded max iterations");
    }
    process(item);
}
```

### 5.3 Assertion Density

Functions MUST include 2+ assertions for preconditions and postconditions.

```[language]
fn process(input: Input) -> Output {
    assert!(input.is_valid());      // Precondition

    let result = compute(input);

    assert!(result.is_finalized()); // Postcondition
    result
}
```

### 5.4 Function Size

| Level | Limit |
|-------|-------|
| MUST | ≤60 lines |
| SHOULD | ≤30 lines |

### 5.5 Property-Based Testing

Controlled classification MUST use property-based tests.

```[language]
// Example property test
```

### 5.6 Mutation Testing

Controlled classification MUST achieve ≥95% mutation score.

---

## SECTION 6: SECURITY

### 6.1 Input Validation

All external input MUST be validated before use.

```[language]
fn process_input(input: RawInput) -> Result<ValidatedInput> {
    let validated = schema.validate(input)?;
    Ok(validated)
}
```

### 6.2 Secret Handling

```[language]
// MUST: Never log secrets
fn authenticate(token: &str) {
    log::info!("Authenticating user");  // NOT the token
}
```

### 6.3 Approved Crypto Libraries

| Purpose | Library |
|---------|---------|
| All crypto | XCLib (required) |

---

## SECTION 7: NOTHING LOST, EVER

*Implementation of Principle 9.*

### 7.1 Message Persistence

Governance-relevant messages MUST be persisted until acknowledged.

```[language]
// Durable queue pattern
async fn enqueue(message: Message) -> MessageId {
    let id = generate_id();
    storage.write(id, message).await;
    storage.sync().await;
    id
}

async fn acknowledge(id: MessageId) {
    storage.delete(id).await;
    storage.sync().await;
}
```

### 7.2 Recovery Pattern

```[language]
// On startup, recover pending work
async fn startup() {
    let pending = queue.recover_pending().await;
    for message in pending {
        process_with_retry(message).await;
    }
}
```

---

## SECTION 8: DEPENDENCY MANAGEMENT

### 8.1 Dependency Policy

| Classification | Requirement |
|----------------|-------------|
| Development | SHOULD use lockfile |
| Production | MUST use exact versions |
| Controlled | MUST use exact versions + audit |

### 8.2 Security Auditing

```bash
# Audit command for this language
[audit command]
```

---

## SECTION 9: CI PIPELINE

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup [Language]
        uses: [setup-action]

      - name: Lint
        run: [lint command]

      - name: Format Check
        run: [format command]

      - name: Test
        run: [test command]

      - name: Security Audit
        run: [audit command]

      - name: Build
        run: [build command]
```

---

## SECTION 10: DEVIATION RECORDING

If a developer must violate a MUST requirement, they MUST mark it explicitly.

```[language]
// XONAIX_DEVIATION: [Reason for deviation]
// LEDGER_ACK: [signature_hash]
[deviating code]
```

This triggers:
1. Warning displayed
2. Acknowledgment required
3. Ledger record created
4. Artifact marked as "User Choice"

---

## SECTION 11: XONA PROMPT APPENDIX

*Add to base prompt when working with [LANGUAGE_NAME].*

```
[LANGUAGE_NAME] REQUIREMENTS:

TRUST CLASS: L[X]

FORBIDDEN:
- [Pattern 1]
- [Pattern 2]

REQUIRED:
- [Pattern 1]
- [Pattern 2]

TESTING (Controlled):
- Bounded loops: ALL loops MUST have MAX_* constants
- Assertion density: 2+ per function
- Function size: ≤60 lines MUST, ≤30 lines SHOULD
- Line coverage: ≥95%
- Branch coverage: ≥90%
- Mutation score: ≥95%

FLAG THESE VIOLATIONS:
NO [violation 1]
NO [violation 2]
```

---

## SECTION 12: GRADUATION CHECKLIST

Before this standard graduates from Draft to Active:

**Core Requirements:**
- [ ] Trust Class declared
- [ ] All Cross-Language Requirements implemented
- [ ] All Principles mapped with implementations
- [ ] Deviation Recording syntax defined
- [ ] Xona Prompt Appendix complete

**Testing Requirements:**
- [ ] Coverage thresholds defined
- [ ] Bounded loop patterns defined
- [ ] Assertion density patterns defined
- [ ] Property-based testing patterns defined
- [ ] Mutation testing integration defined

**Security Requirements:**
- [ ] XCLib integration documented
- [ ] Numeric policy implementation documented
- [ ] Authority verification patterns documented
- [ ] Dependency vetting process defined

**Documentation:**
- [ ] CI pipeline example complete
- [ ] At least 3 code examples per section
- [ ] All examples tested and verified

---

## References

- **Principles:** THE_XONAIX_WAY.md (xonaix-specs/core)
- **Cross-Language Requirements:** STANDARDS_INDEX.md
- **Header Format:** LIBRARY_STANDARD_HEADER_CONTRACT.md

---

*[LANGUAGE_NAME] Language Standard v1.0.0*

*Xonaix Library*
