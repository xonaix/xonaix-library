---
XONAIX_SPEC: true
Spec_ID: 04_CODE_CORE_TEMPLATE_LANGUAGE_STANDARD_MD
Title: The Xonaix Way
Version: B-5.8.5
Baseline: B-5.9.0
Domain: Code_Core
Authority_Tier: T4
Status: Active
Supersedes: None
Depends_On: []
Conflicts: []
Last_Amended: 2025-12-29
Owner: Xonaix Core
constitutional_conformance:
  constitution_version: "B-5.8.5"
  constitution_hash_alg: "SHA3-512"
  constitution_hash: "bde10877c0236814d920393306d45dea876f650e38124cfa78a98ef416a3c304fa4a096f13cf07b6ffddd1034e51cc17667262d424ec8bb7768aa2314ea0fe6a"
  zero_point_version: "B-5.8.5"
  zero_point_hash_alg: "SHA3-512"
  zero_point_hash: "edc7a7e5c6db077493e283df3b6e7f24b714419884a5e08684d510dbfea47457e0378569d4e66519b23325d10975a43a60947c559e9adcb2c23aa397414c25ca"
  cic_compliance:
    float_prohibition: not_applicable
    error_boundary: not_applicable
    capability_classes: not_applicable
    deterministic_accounting: not_applicable
    fail_closed: not_applicable
  deviations: []
  last_verified: "2025-12-29T00:00:00Z"
  verified_by: "Xonaix Baseline Audit B-5.8.5"
---
# The Xonaix Way
## Language Standards: [LANGUAGE_NAME]

**Version:** B-5.8.5
**Status:** [Interim/Active]
**Core-Compatible:** 5.7.0
**Trust Class:** L[1-4]
**Created:** [Month Year]
**Last Reviewed:** [Month Year]

*This document implements The Xonaix Way B-5.8.5 principles for [LANGUAGE_NAME] projects.*

---

## Document Info

| Field | Value |
|-------|-------|
| Language | [LANGUAGE_NAME] |
| Status | **[Interim/Active]** ([Primary/Secondary/Domain]) |
| Version | B-5.8.5 |
| Core-Compatible | 5.7.0 |
| Trust Class | L[1-4] |
| Created | [Month Year] |
| Last Reviewed | [Month Year] |
| Minimum Version | [Language version requirement] |
| Related Standards | [Links to related standards] |

**Prerequisites:** Read [THE_XONAIX_WAY.md](THE_XONAIX_WAY.md) first. This document assumes familiarity with the 9 Principles.

**Language Policy:** [Describe this language's role in the Xonaix ecosystem - Primary/Secondary/Domain/Deprecated]

---

## Trust Class (MANDATORY)

| Attribute | Value |
|-----------|-------|
| Trust Class | L[1-4] |
| Classification | [Constitutional/Deterministic/Orchestration/Interface] |

### Trust Class Implications

[Explain what this language MAY and MAY NOT do at this trust class]

### What L[X] May Do

- [Capability 1]
- [Capability 2]

### What L[X] May NOT Do

- [Restriction 1]
- [Restriction 2]

---

## XCLib Integration (MANDATORY for L1-L3)

**Authority:** Founder Ruling 2025-003(c) — Attested Capability

### Binding Type

[Direct / WASM Binding / Host Delegation / Not Applicable]

### Permitted Crypto Operations

| Operation | Permitted | Method |
|-----------|-----------|--------|
| Canonicalization | [Yes/No] | [How] |
| Hashing | [Yes/No] | [How] |
| Signing | [Yes/No] | [How] |
| Verification | [Yes/No] | [How] |

### Prohibited Patterns

[List patterns that violate XCLib exclusivity]

### Enforcement

[How violations are detected - linting, CI, review]

---

## Numeric Policy (MANDATORY)

**Authority:** Founder Ruling 2025-003(a) — Determinism

### Forbidden Types

| Type | Reason |
|------|--------|
| [e.g., f32, f64] | [IEEE 754 non-determinism] |

### Required Representations

| Use Case | Required Type | Example |
|----------|---------------|---------|
| Currency | [e.g., i64 cents] | `amount_cents: i64` |
| Percentages | [e.g., basis points] | `rate_bps: u32` |
| Decimals | [e.g., string] | `value: "3.14159"` |

### Enforcement

[Linting rules, CI checks]

---

## Capability & Posture Handling (MANDATORY)

**Authority:** Constitution Article I, §4 — Zero Trust

### Obtaining Posture

[How code in this language obtains current capability class and trust posture]

### Verification Pattern

```[language]
// Example code showing proper capability verification
```

### Prohibited Patterns

[Patterns that infer or assume authority]

### Posture Degradation

[What happens when posture is unavailable or downgraded]

---

## Error Handling (MANDATORY)

**Authority:** Founder Ruling 2025-003(b) — Bounded Error Surfaces

### Error Strategy

| Context | Strategy |
|---------|----------|
| L1/L2 paths | [e.g., panic=abort] |
| Public API | [e.g., Result types] |
| External boundary | [e.g., mapped to PublicErrorCode] |

### Bounded Error Codes

[Define the public error codes for this language]

### Prohibited Patterns

[Patterns that leak information through errors]

---

## Generated Code Accountability (MANDATORY)

**Authority:** Constitutional Actor Model

### Applicability

This standard applies equally to:
- Human-written code
- Forge-generated code
- Agent-generated code
- Template-generated code

### Requirements

Generated code MUST:
1. Declare target trust class
2. Declare intended authority scope
3. Pass all CI checks (same as human code)
4. Include provenance metadata

### No Exceptions

"Generated" status does not reduce requirements or bypass review.

**Language Justification:** [Quantifiable reasons for using this language]
- Performance: [Benchmark data if applicable]
- Ecosystem: [Library availability, tooling maturity]
- Team expertise: [Current capabilities]
- Use case fit: [Why this language for this domain]

---

## Principle Mapping

| Principle | [LANGUAGE_NAME] Implementation |
|-----------|-------------------------------|
| 1. Correct Over Fast | [How this language ensures correctness] |
| 2. Explicit Over Implicit | [How this language avoids magic] |
| 3. Automated Over Vigilant | [Tooling: linters, formatters, CI] |
| 4. Secure By Default | [Security patterns, input validation] |
| 5. Composable Over Clever | [Modularity patterns] |
| 6. Fail Loud | [Error handling patterns] |
| 7. Xona Augments, Human Decides | [Xona collaboration patterns] |
| 8. Future-Proof Over Trend | [Stability practices] |
| 9. Nothing Lost, Ever | [Persistence, durability, ACK patterns] |

---

## Deviation Recording (User Choice)

If a developer chooses to violate a **MUST** requirement, they **MUST** explicitly mark it for the Security Ledger.

**Syntax for [LANGUAGE_NAME]:**

```[language]
[COMMENT_SYNTAX] XONAIX_DEVIATION: [Reason for deviation - be specific]
[COMMENT_SYNTAX] LEDGER_ACK: [User_Signature_Hash]
[Deviating code]
```

**Example:**
```[language]
// XONAIX_DEVIATION: Legacy API requires deprecated pattern - migration planned Q2
// LEDGER_ACK: sha256:abc123...
[deviating_code_here]
```

**This triggers:**
1. Warning displayed to user
2. Acknowledgment required
3. Signature captured
4. Ledger record created
5. Artifact marked as "User Choice" (not Xonaix Certified)

---

## SECTION 1: PROJECT CONFIGURATION

### 1.1 [Tool/Compiler Configuration]

```[config-format]
// Example configuration file
[configuration here]
```

### 1.2 Linting Configuration

```[config-format]
// Linter configuration
[configuration here]
```

### 1.3 Formatting Configuration

```[config-format]
// Formatter configuration
[configuration here]
```

### 1.4 Build/Run Scripts

```json
{
  "scripts": {
    "build": "[build command]",
    "test": "[test command]",
    "lint": "[lint command]",
    "format": "[format command]",
    "typecheck": "[typecheck command if applicable]"
  }
}
```

---

## SECTION 2: NASA/DOD GRADE REQUIREMENTS

*Per THE_XONAIX_WAY.md Part IX, all code must meet NASA/DOD grade standards.*

### 2.1 Bounded Execution

All loops and iterations MUST have provable termination bounds.

```[language]
// CORRECT: Bounded loop
const MAX_ITERATIONS = 10_000;
let count = 0;

for (const item of items) {
    if (count++ >= MAX_ITERATIONS) {
        throw new Error(`Exceeded max iterations: ${MAX_ITERATIONS}`);
    }
    process(item);
}
```

```[language]
// VIOLATION: Unbounded loop
while (condition) {
    // No termination guarantee
}
```

### 2.2 Assertion Density

Functions MUST include assertions to verify preconditions and postconditions.

| Classification | Requirement |
|----------------|-------------|
| Development | SHOULD have 2+ assertions per function |
| Production | MUST have 2+ assertions per function |
| Controlled | MUST have 2+ assertions per function |

```[language]
function processTransaction(tx: Transaction): Receipt {
    // Precondition
    assert(tx.amount > 0, 'Amount must be positive');
    assert(tx.from !== tx.to, 'Cannot transfer to self');
    
    const receipt = executeTransfer(tx);
    
    // Postcondition
    assert(receipt.isFinalized(), 'Receipt must be finalized');
    assert(receipt.txId === tx.id, 'Receipt must match transaction');
    
    return receipt;
}
```

### 2.3 Function Size Limits

| Level | Limit | Rationale |
|-------|-------|-----------|
| MUST | ≤60 lines | Maximum complexity for reliable review |
| SHOULD | ≤30 lines | Target for optimal comprehension |
| IDEAL | ≤15 lines | Single responsibility excellence |

### 2.4 Coverage Requirements

| Classification | Line Coverage | Branch Coverage |
|----------------|---------------|-----------------|
| Development | ≥60% SHOULD | ≥50% SHOULD |
| Production | ≥80% MUST | ≥70% MUST |
| Controlled | ≥95% MUST | ≥90% MUST |

```[config-format]
// Test coverage configuration
coverage: {
    thresholds: {
        lines: 95,
        branches: 90,
        functions: 95,
        statements: 95,
    }
}
```

### 2.5 Property-Based Testing

**Controlled classification MUST use property-based tests:**

```[language]
// Example with [property-testing-framework]
test('roundtrip property', () => {
    fc.assert(
        fc.property(fc.string(), (input) => {
            const encoded = encode(input);
            const decoded = decode(encoded);
            expect(decoded).toBe(input);
        })
    );
});
```

**Recommended Framework:** [Framework name and installation]

### 2.6 Mutation Testing

**Controlled classification MUST achieve ≥95% mutation score:**

```bash
# Install mutation testing tool
[installation command]

# Run mutation testing
[run command]

# Check score meets threshold
[verification command]
```

**Surviving mutants MUST be documented and justified.**

### 2.7 Formal Verification

*If applicable to this language:*

| Scope | Tool | Requirement |
|-------|------|-------------|
| Cryptographic operations | [Tool] | MUST for Controlled |
| State machines | [Tool] | MUST for Controlled |
| Concurrent code | [Tool] | SHOULD for Controlled |

### 2.8 Chaos Testing

**Controlled classification MUST implement chaos testing:**

```[language]
// Example: Network failure test
test('handles network failure gracefully', async () => {
    // Inject fault
    mockFetch.mockRejectedValue(new Error('Network failure'));
    
    // Verify graceful handling
    await expect(fetchData()).rejects.toThrow('Network failure');
    
    // Verify no data loss (Principle 9)
    expect(pendingQueue.length).toBeGreaterThan(0);
});
```

---

## SECTION 3: ERROR HANDLING

### 3.1 MUST: Explicit Error Handling

```[language]
// CORRECT: Explicit error handling
function process(): Result<Output, Error> {
    try {
        const data = fetchData();
        return { success: true, data: transform(data) };
    } catch (e) {
        return { success: false, error: new ProcessError('Failed', e) };
    }
}
```

### 3.2 MUST NOT: Silent Swallowing

```[language]
// VIOLATION: Empty catch
try {
    mightFail();
} catch (e) {
    // Silent - FORBIDDEN
}

// VIOLATION: Ignore error and continue
const result = mightFail(); // Unchecked
```

### 3.3 Error Types

```[language]
// Define specific error types
class ProcessError extends Error {
    constructor(message: string, cause?: Error) {
        super(message);
        this.cause = cause;
        this.name = 'ProcessError';
    }
}
```

---

## SECTION 4: SECURITY

### 4.1 Input Validation

All external input MUST be validated before use:

```[language]
// CORRECT: Validated input
function processUserInput(input: unknown): ValidatedInput {
    const parsed = InputSchema.parse(input); // Throws on invalid
    return parsed;
}
```

### 4.2 Cryptography

**Approved Libraries:**

| Purpose | Library | Notes |
|---------|---------|-------|
| Hashing | [library] | [notes] |
| Signatures | [library] | [notes] |
| Encryption | [library] | [notes] |

**Post-Quantum Cryptography (PQC):**

| Phase | Timeline | Requirement |
|-------|----------|-------------|
| Phase 1: Design | v5.0.0 (Now) | SHOULD design for hybrid compatibility |
| Phase 2: Simulate | 2026 | SHOULD prototype hybrid in staging |
| Phase 3: Production | 2027 | MUST for Tier 3-4 (hybrid classical + ML-DSA) |

```[language]
// Current architecture (Phase 1) - Design for hybrid
interface Signature {
    classical: ClassicalSignature;  // Always present
    pqc?: PqcSignature;             // None until Phase 3
}
```

### 4.3 Secret Handling

```[language]
// MUST: Never log secrets
function authenticate(token: string): void {
    logger.info('Authenticating user'); // NOT the token
    // ...
}

// MUST: Clear secrets when done
function processSecret(secret: string): void {
    try {
        useSecret(secret);
    } finally {
        clearMemory(secret); // Language-specific cleanup
    }
}
```

### 4.4 FIPS 140-3 Compliance Roadmap

| Phase | Requirement | Timeline |
|-------|-------------|----------|
| Phase 1 | Use libraries wrapping FIPS-validated modules | Immediate |
| Phase 2 | Document crypto module provenance in SBOM | 2026 |
| Phase 3 | Full FIPS 140-3 deployment for Controlled | 2027 |

---

## SECTION 5: NOTHING LOST, EVER (PRINCIPLE 9)

### 5.1 Message Persistence

Governance-relevant messages MUST be persisted until acknowledged:

```[language]
// Example: Durable queue pattern
class DurableQueue {
    async enqueue(message: Message): Promise<MessageId> {
        const id = generateId();
        await this.storage.write(id, message);
        await this.storage.sync(); // fsync equivalent
        return id;
    }
    
    async acknowledge(id: MessageId): Promise<void> {
        await this.storage.delete(id);
        await this.storage.sync();
    }
    
    async recoverPending(): Promise<Message[]> {
        return this.storage.readAll(); // All unacknowledged
    }
}
```

### 5.2 Recovery Patterns

```[language]
// On startup, recover pending work
async function startup(): Promise<void> {
    const pending = await queue.recoverPending();
    for (const message of pending) {
        await processWithRetry(message);
    }
}
```

### 5.3 Persistence Technologies

| Technology | Use Case | Notes |
|------------|----------|-------|
| [Technology 1] | [Use case] | [Notes] |
| [Technology 2] | [Use case] | [Notes] |

---

## SECTION 6: DEPENDENCY MANAGEMENT

### 6.1 Dependency Policy

| Classification | Requirement |
|----------------|-------------|
| Development | SHOULD use lockfile |
| Production | MUST use exact versions |
| Controlled | MUST use exact versions + audit |

### 6.2 Security Auditing

```bash
# Audit dependencies for vulnerabilities
[audit command]

# Generate SBOM
[sbom command]
```

### 6.3 Dependency Vetting

**Controlled classification MUST vet dependencies:**

```bash
# Vetting process
[vetting commands]
```

---

## SECTION 7: TESTING

### 7.1 Test Structure

```[language]
describe('ComponentName', () => {
    describe('methodName', () => {
        beforeEach(() => {
            // Setup
        });
        
        it('handles valid input', () => {
            // Arrange
            const input = createValidInput();
            
            // Act
            const result = component.methodName(input);
            
            // Assert
            expect(result.success).toBe(true);
        });
        
        it('rejects invalid input', () => {
            // Arrange
            const input = createInvalidInput();
            
            // Act & Assert
            expect(() => component.methodName(input)).toThrow();
        });
    });
});
```

### 7.2 Mocking

```[language]
// Mock external dependencies
const mockFetch = jest.fn();
global.fetch = mockFetch;

beforeEach(() => {
    mockFetch.mockReset();
});
```

---

## SECTION 8: CI PIPELINE

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
        with:
          version: '[version]'
      
      - name: Install Dependencies
        run: [install command]
      
      - name: Type Check
        run: [typecheck command]
      
      - name: Lint
        run: [lint command]
      
      - name: Format Check
        run: [format check command]
      
      - name: Test with Coverage
        run: [test command]
      
      - name: Check Coverage Thresholds
        run: |
          # Verify coverage meets thresholds
          [coverage check command]
      
      - name: Security Audit
        run: [audit command]
      
      - name: Build
        run: [build command]

  # Controlled classification only
  controlled-checks:
    runs-on: ubuntu-latest
    if: contains(github.event.pull_request.labels.*.name, 'controlled')
    steps:
      - uses: actions/checkout@v4
      
      - name: Mutation Testing
        run: |
          [mutation testing command]
          # Fail if score < 95%
      
      - name: Property-Based Tests
        run: [property test command]
```

---

## SECTION 9: COMMON CRITERIA EAL4 COMPATIBILITY

**Controlled systems SHOULD be designed for CC EAL4 evaluation compatibility:**

- Maintain Security Target documentation
- Document all security functions with functional specifications
- Keep design documentation traceable to requirements
- Collect test evidence systematically
- Perform ongoing vulnerability analysis

---

## Xona Prompt Appendix

*Add to base prompt when working with [LANGUAGE_NAME].*

```
[LANGUAGE_NAME] v5.0.0 REQUIREMENTS:

NOTE: [Language role - Primary/Secondary/Domain/etc.]

FORBIDDEN:
- [Forbidden pattern 1]
- [Forbidden pattern 2]
- [Forbidden pattern 3]

REQUIRED:
- [Required pattern 1]
- [Required pattern 2]
- [Required pattern 3]

NASA/DOD GRADE:
- Bounded loops: ALL loops MUST have MAX_* constants
- Assertion density: 2+ assertions per function
- Function size: ≤60 lines MUST, ≤30 lines SHOULD
- Line coverage: ≥95% for Controlled
- Branch coverage: ≥90% for Controlled
- Property tests: MUST for Controlled
- Mutation testing: ≥95% score for Controlled
- Chaos testing: MUST for Controlled

CRYPTO:
- [Crypto requirements specific to language]
- Phase 3 (2027): Hybrid classical + ML-DSA for Tier 3-4

PRINCIPLE 9 (NOTHING LOST):
- [Persistence patterns for this language]
- [ACK-based messaging patterns]
- [Recovery patterns]

CONTROLLED CLASSIFICATION CHECKLIST:
[ ] All loops bounded
[ ] 2+ assertions per function
[ ] Functions ≤60 lines
[ ] Coverage ≥95%/90%
[ ] Property-based tests
[ ] Mutation score ≥95%
[ ] Chaos tests implemented
[ ] Dependencies vetted

FLAG THESE VIOLATIONS:
NO [Violation 1]
NO [Violation 2]
NO [Violation 3]
```

---

## Quick Reference

### Allowed Patterns

```[language]
// [Good pattern 1]
// [Good pattern 2]
// [Good pattern 3]
```

### Forbidden Patterns

```[language]
// [Bad pattern 1]
// [Bad pattern 2]
// [Bad pattern 3]
```

---

## Graduation Checklist

Before this standard graduates from Draft to Active:

**Core Requirements:**
- [ ] All 9 Principles mapped with concrete implementations
- [ ] Deviation Recording syntax defined
- [ ] Xona Prompt Appendix complete and tested
- [ ] Changelog started
- [ ] Core-Compatible set to 5.1.0

**NASA/DOD Grade Requirements:**
- [ ] Bounded loop patterns defined with MAX_* constants
- [ ] Assertion density requirement (2+ per function) documented
- [ ] Function size limits defined (≤60 MUST, ≤30 SHOULD)
- [ ] Coverage thresholds defined (95%/90% for Controlled)
- [ ] Property-based testing patterns with framework recommendation
- [ ] Mutation testing integration with ≥95% threshold
- [ ] Formal verification tools identified (if applicable)
- [ ] Chaos testing patterns defined

**Security & Compliance:**
- [ ] PQC hybrid crypto pattern defined (for Tier 3-4)
- [ ] FIPS 140-3 roadmap documented (if crypto involved)
- [ ] Principle 9 (Nothing Lost) patterns defined
- [ ] Dependency vetting process defined

**Quality Assurance:**
- [ ] CI pipeline example complete with all checks
- [ ] At least 3 code examples per major section
- [ ] Examples tested and verified to work
- [ ] Common Criteria EAL4 compatibility notes
- [ ] Error handling patterns complete
- [ ] Security patterns complete

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (MANDATORY)
- **MAJOR:** Added XCLib Integration section (MANDATORY for L1-L3)
- **MAJOR:** Added Numeric Policy section (MANDATORY)
- **MAJOR:** Added Capability & Posture Handling section (MANDATORY)
- **MAJOR:** Added Error Handling section (MANDATORY)
- **MAJOR:** Added Generated Code Accountability section (MANDATORY)
- **UPDATED:** Core-Compatible to 5.7.0
- **ALIGNED:** Cross-language requirements per STANDARDS_INDEX B-5.8.5
- **Source:** Red-Blue-Black Team synthesis with Founder approval

### v5.2.0 (December 2025)
- **Red-Blue Team Block 5 Synthesis:**
- **UPDATED:** Core-Compatible to 5.2.0
- **ALIGNED:** Cross-references to v5.2.0 ecosystem
- **ADDED:** XCLib requirement note for canonical operations

### v5.1.0 (December 2025)
- **PROMOTED:** From DRAFT to Active status
- **UPDATED:** Core-Compatible to 5.1.0
- **ALIGNED:** Rainbow Team ratification requirements incorporated
- **UPDATED:** Graduation checklist reflects current review requirements

### v0.1.0 DRAFT (December 2025)
- Initial draft release
- Based on The Xonaix Way v5.0.0

---

*[LANGUAGE_NAME] Standards B-5.8.5 — Part of The Xonaix Way B-5.8.5*

*"[Tagline for this language standard]"*

*Xonaix, Inc. — Intelligence, evolved.*
