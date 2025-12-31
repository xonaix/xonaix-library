# XONAIX SELF-GOVERNANCE CONTRACT

Status: BINDING
Authority Level: Constitutional
Scope: All Xonaix specifications, tooling, code, and operations
Effective: Upon commit to governance manifest

This contract is the authoritative reference for Xonaix governance. All specifications, tooling, code, and operational decisions are subject to this contract. Conflicts with this document indicate error in the conflicting artifact, not in this contract.

---

## 1. Purpose and Scope

### 1.1 Existence

Xonaix exists to prevent unsafe decisions before they become code.

Xonaix does not exist to:
- Accelerate development at the cost of correctness
- Provide advisory guidance that may be ignored
- Optimize for convenience over safety

### 1.2 Governance Authority

Governance within Xonaix is not advisory. Governance is enforceable.

All governance rules:
- Are machine-verifiable where possible
- Fail loudly when violated
- Cannot be bypassed without explicit, recorded authorization

### 1.3 Scope of Application

This contract applies to:
- All Xonaix specifications (current and future)
- All Xonaix tooling (xonaix-spec-tools, xonaix-library-tools, and successors)
- All Xonaix code repositories
- All Xonaix operational processes
- All agents and humans operating within the Xonaix ecosystem

No artifact, process, or actor is exempt.

---

## 2. Dogfooding Mandate

### 2.1 Self-Application Requirement

Xonaix MUST govern Xonaix.

All internal specifications, code, releases, and deployments are subject to the same governance applied to external consumers. There is no internal exception path.

### 2.2 Customer Zero

Xonaix is its own first customer. This status is permanent and non-negotiable.

Implications:
- Every governance rule is validated against Xonaix's own artifacts first
- If a rule cannot be satisfied internally, it cannot be imposed externally
- Internal compliance failures are treated with the same severity as external failures

### 2.3 No Privileged Bypass

No actor within Xonaix (human or agent) has standing to bypass governance for internal work. Expedience is not justification. Deadlines are not justification. Complexity is not justification.

---

## 3. Bootstrap Declaration

### 3.1 Current Authority Status

The current governance enforcement mechanism (Git + CI + xonaix-library-tools) is a bootstrap authority.

Bootstrap authority characteristics:
- Functionally binding within current operational scope
- Subject to migration into Xonaix Core without loss of meaning
- Not the final architectural form

### 3.2 Migration Requirement

All governance mechanisms MUST be designed for migration into Xonaix Core.

Requirements for migration-ready governance:
- Machine-parseable rule definitions
- Deterministic verification procedures
- Serializable decision records
- No dependence on human memory or informal process

### 3.3 Continuity Guarantee

Migration from bootstrap to Xonaix Core MUST preserve:
- All governance intent
- All recorded decisions
- All audit trails
- All enforcement guarantees

Loss of governance meaning during migration is a critical failure.

---

## 4. Continuous Re-Audit Requirement

### 4.1 Living Instruments

All governance specifications are living instruments. No governance document is final.

This includes:
- This contract
- All documents in `specs/_governance/`
- All enforcement rules in tooling
- All CI verification gates

### 4.2 Periodic Re-Audit

Governance documents MUST be re-audited periodically.

Re-audit triggers:
- Scheduled intervals (to be defined per document classification)
- Significant architectural changes
- Discovery of governance gaps
- External events affecting assumptions

### 4.3 Staleness as Debt

Governance that has not been re-audited within its required interval is considered debt.

Stale governance:
- MUST be flagged by tooling
- MUST block sealing of affected artifacts
- MUST be resolved before release

---

## 5. No Sacred Cows Clause

### 5.1 Longevity Does Not Imply Correctness

The age of a specification, rule, or assumption does not grant it immunity from challenge.

A governance artifact that has existed for years is not more correct than a governance artifact created yesterday. Correctness is evaluated on merit, not tenure.

### 5.2 Right to Challenge

Any assumption within Xonaix governance MAY be challenged.

Challenge requirements:
- Challenges MUST be explicit and recorded
- Challenges MUST articulate the deficiency
- Challenges MUST propose resolution or request investigation

### 5.3 Authority and Questioning

Authority approves changes. Authority does not suppress questioning.

The role of governance authority (Founder, designated reviewers) is to:
- Evaluate challenges on merit
- Approve or reject changes with recorded rationale
- Never dismiss challenges without evaluation

Dismissal without evaluation is a governance failure.

### 5.4 Rewrite and Removal

Any specification MAY be rewritten or removed if found insufficient.

Conditions for removal:
- Documented rationale
- Migration path for dependents (if applicable)
- Explicit authorization from governance authority

---

## 6. Decision Recording Principle

### 6.1 Recording Requirement

Every significant governance decision MUST be:
- Explicit (written, not implied)
- Attributable (author/approver identified)
- Auditable (retrievable for review)
- Serializable (convertible to Xonaix Core event format)

### 6.2 Completeness Test

If a decision cannot be recorded, it is incomplete.

Incomplete decisions:
- MUST NOT be treated as binding
- MUST NOT be referenced as precedent
- MUST be completed before enforcement

### 6.3 Future Serialization

All governance decisions MUST be structured for future serialization into Xonaix Core events.

This means:
- Machine-readable format where possible
- Clear decision boundaries
- Unambiguous approval/rejection status
- Timestamp and actor identification

---

## 7. Failure Philosophy

### 7.1 Fail Fast

Systems MUST fail at the earliest possible detection point.

A failure detected in CI is better than a failure detected in production.
A failure detected in specification review is better than a failure detected in CI.
A failure prevented by governance design is better than a failure detected at all.

### 7.2 Fail Loudly

Failures MUST be visible and unambiguous.

Silent failures are forbidden. A system that fails without notification has failed twice:
- Once in the original failure
- Once in the failure to report

### 7.3 Fail Before Damage

Systems MUST fail before causing downstream damage.

Acceptable failure modes:
- Build rejection
- CI gate failure
- Verification failure
- Audit failure

Unacceptable failure modes:
- Production incident
- Data corruption
- Security breach
- Customer impact

### 7.4 Silent Success Prohibition

Silent success is forbidden when risk is unresolved.

If a verification passes but known risks remain unevaluated:
- The verification MUST report the unevaluated risks
- The verification MUST NOT report clean success
- Downstream processes MUST be informed

---

## 8. Alignment with The Xonaix Way

### 8.1 Zero Debt

Technical debt is not tolerated. Governance debt is not tolerated.

Debt is:
- Any deferred decision that affects correctness
- Any incomplete implementation marked for "later"
- Any known issue not tracked to resolution
- Any warning treated as acceptable

### 8.2 Mechanical Truth

Truth within Xonaix is mechanically verifiable.

Claims that cannot be verified by tooling:
- Are treated as unverified
- Do not contribute to compliance status
- Must be converted to verifiable form or removed

### 8.3 No Shortcuts

Shortcuts that sacrifice correctness for speed are forbidden.

This applies to:
- Specification writing
- Code implementation
- Testing and verification
- Release processes
- Operational procedures

### 8.4 Correctness Over Speed

When correctness and speed conflict, correctness wins.

A correct system delivered later is acceptable.
An incorrect system delivered on time is not acceptable.

There is no deadline that justifies shipping known defects.

### 8.5 Discipline Over Convenience

Convenience that undermines discipline is rejected.

Discipline means:
- Following process even when it is tedious
- Documenting even when the answer seems obvious
- Verifying even when confidence is high
- Recording even when memory seems sufficient

---

## 9. Enforcement

### 9.1 Machine Enforcement

This contract is enforced by:
- CI verification gates
- xonaix-library-tools enforcement checks
- Governance manifest inclusion
- Future Xonaix Core integration

### 9.2 Human Enforcement

This contract is enforced by:
- Mandatory review for all governance changes
- Founder signoff for contract modifications
- Rainbow Team audit for significant changes

### 9.3 Agent Enforcement

All agents operating within Xonaix:
- MUST acknowledge this contract as binding context
- MUST evaluate their outputs against this contract
- MUST flag potential violations before execution

---

## 10. Amendment Process

### 10.1 Modification Authority

Modifications to this contract require:
- Explicit Founder authorization
- Rainbow Team review (minimum White and Green teams)
- Recorded rationale for change
- Updated governance manifest

### 10.2 Immutable Principles

The following principles are immutable and survive any amendment:
- Xonaix governs Xonaix (Section 2)
- Decisions must be recorded (Section 6)
- Failures must be visible (Section 7)
- Correctness over speed (Section 8.4)

These principles may be clarified but not weakened.

---

## 11. Ratification

This contract is ratified upon:
- Commit to `specs/_governance/`
- Inclusion in governance manifest
- CI verification pass

Upon ratification, this contract is binding on all Xonaix artifacts, processes, and actors.

---

*Constitutional Governance Document*
*Canonical location: `specs/_governance/XONAIX_SELF_GOVERNANCE_CONTRACT.md`*
