---
schema: "xonaix-document-header"
schema_version: "2.0"

# --- Identity ---
repo: "xonaix-library"
path: "specs/standards/typescript/STANDARDS_TYPESCRIPT.md"
unit_id: "library/standards/typescript"
title: "TypeScript Language Standard"
document_type: "standard"
language: "en"

# --- Version ---
version: "XLIB-1.0.0"
baseline: null
status: "active"

# --- Classification ---
trust_class: "L3"
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

# TypeScript Language Standard

TypeScript is the secondary language for Xonaix development, used for customer-facing interfaces, web dashboards, and SvelteKit frontends.

This standard assumes familiarity with the 10 Principles defined in THE_XONAIX_WAY.md.

**Language Policy:** TypeScript is permitted ONLY for:
- Customer-facing API interfaces
- Web interfaces and dashboards
- Customer SDKs
- Documentation tools
- MCP client implementations
- SvelteKit frontends
- Tauri frontend code

Core systems, governance code, and security-critical components MUST use Rust.


## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L3 |
| Classification | Orchestration |

TypeScript is an **orchestration language**. It coordinates workflows, renders UI, and interfaces with users but **cannot decide truth** or perform cryptographic operations directly.

### What L3 May Do

- Coordinate API calls and workflows
- Render UI based on verified data
- Transport data between components
- Display proof status (after verification via XCLib WASM)

### What L3 May NOT Do

- Canonicalize governance data
- Hash data for verification purposes
- Sign any payload
- Verify signatures directly (must delegate to XCLib WASM)
- Assert authority without cryptographic proof

---

## XCLib Integration

**Authority:** Founder Ruling 2025-003(c) — Attested Capability

### Binding Type

TypeScript uses **XCLib WASM bindings** for any cryptographic operations.

### Prohibited Operations

TypeScript code MUST NOT perform:

| Operation | Prohibition |
|-----------|-------------|
| Canonicalization | FORBIDDEN — delegate to Rust/WASM |
| Hashing | FORBIDDEN — delegate to XCLib WASM |
| Signing | FORBIDDEN — delegate to XCLib WASM |
| Direct Verification | FORBIDDEN — use XCLib WASM bindings |

### How to Verify Proofs

```typescript
// CORRECT: Delegate to XCLib WASM binding
import { verify } from '@xonaix/xclib-wasm';

async function displayProof(proof: Proof): Promise<VerificationResult> {
  const result = await verify(proof);
  return result;
}

// FORBIDDEN: Direct crypto
import { sha3_512 } from 'some-crypto-lib';
function verifyProof(proof: Proof): boolean {
  const hash = sha3_512(proof.data);  // NOT ALLOWED
  return hash === proof.expected;
}
```

### Enforcement

```javascript
// eslint.config.js
{
  rules: {
    'no-restricted-imports': ['error', {
      patterns: [
        'crypto',
        'sha3',
        'sha256',
        'ed25519',
        'blake3',
        '**/crypto/**'
      ],
      paths: [
        { name: 'crypto', message: 'Use @xonaix/xclib-wasm instead' }
      ]
    }]
  }
}
```

---

## Numeric Policy

**Authority:** Founder Ruling 2025-003(a) — Determinism

### Float Prohibition

JavaScript `number` is IEEE 754 double-precision float — **non-deterministic** across platforms.

**FORBIDDEN for canonical data:**
```typescript
// FORBIDDEN
const amount: number = 100.50;  // Float!
const rate: number = 0.015;     // Float!
```

**REQUIRED representations:**
```typescript
// CORRECT: Use BigInt for integers
const amountCents: bigint = 10050n;  // $100.50 in cents

// CORRECT: Use string for decimals
const rate: string = "0.015";  // String representation

// CORRECT: Use branded types for safety
type CurrencyAmount = bigint & { readonly __brand: 'currency' };
type Percentage = string & { readonly __brand: 'percentage' };

function createCurrency(cents: bigint): CurrencyAmount {
  return cents as CurrencyAmount;
}
```

### Permitted Types for Canonical Data

| Use Case | Type | Example |
|----------|------|---------|
| Currency | `bigint` | `10050n` = $100.50 |
| IDs | `string` | `"uuid-here"` |
| Percentages | `string` | `"1.5"` |
| Ratios | `{ num: bigint, den: bigint }` | `{ num: 3n, den: 2n }` |

---

## Capability & Posture Handling

**Authority:** Constitution Article I, §4 — Zero Trust

### Pattern: Display Authority, Never Assert It

```typescript
// CORRECT: Verify proof before displaying authority
import { verifyAuthorityProof } from '@xonaix/xclib-wasm';

async function renderAdminPanel(user: User): Promise<JSX.Element> {
  const proof = user.authorityProof;
  const verified = await verifyAuthorityProof(proof, 'admin');
  
  if (!verified.valid) {
    return <AccessDenied reason={verified.reason} />;
  }
  
  return <AdminPanel capabilities={verified.capabilities} />;
}

// FORBIDDEN: Trust role claim without proof
function renderAdminPanel(user: User): JSX.Element {
  if (user.role === 'admin') {  // Role claim, not proof!
    return <AdminPanel />;
  }
}
```

### Posture Degradation

When posture verification fails, UI MUST degrade gracefully:

```typescript
async function renderProofStrip(proof: Proof): Promise<JSX.Element> {
  try {
    const result = await verify(proof);
    return <ProofStrip valid={result.valid} details={result.details} />;
  } catch (error) {
    // Degrade to error state, never show false success
    return <ProofStrip valid={false} error="Verification unavailable" />;
  }
}
```

---

## Error Handling (Exception Semantics)

**Authority:** Founder Ruling 2025-003(b) — Bounded Error Surfaces

### Public Error Codes

Exceptions at API boundaries MUST be mapped to bounded public error codes:

```typescript
enum PublicErrorCode {
  VALIDATION_FAILED = 'E001',
  UNAUTHORIZED = 'E002',
  NOT_FOUND = 'E003',
  RATE_LIMITED = 'E004',
  INTERNAL_ERROR = 'E999',
}

function mapToPublicError(error: unknown): PublicErrorCode {
  if (error instanceof ValidationError) return PublicErrorCode.VALIDATION_FAILED;
  if (error instanceof AuthError) return PublicErrorCode.UNAUTHORIZED;
  // Never leak internal details
  return PublicErrorCode.INTERNAL_ERROR;
}
```

### No Information Leakage

```typescript
// FORBIDDEN: Leaks internal state
catch (error) {
  return { error: error.message };  // Might contain sensitive info
}

// CORRECT: Bounded error response
catch (error) {
  console.error('Internal error:', error);  // Log internally
  return { 
    error: PublicErrorCode.INTERNAL_ERROR,
    message: 'An error occurred'  // Generic message
  };
}
```

---

## Generated Code Accountability

**Authority:** Constitutional Actor Model

### Requirements

TypeScript code generated by Forge, agents, or templates MUST:

1. **Declare Trust Class** — Always L3 for TypeScript
2. **Declare Authority Scope** — What operations it performs
3. **Pass All CI Checks** — Same linting, testing as human code
4. **Include Provenance** — Generation metadata

### Provenance Header

```typescript
/**
 * @xonaix-generated forge:1.2.3:template:api-endpoint
 * @xonaix-trust-class L3
 * @xonaix-authority-scope read-only
 */
```

### No Exceptions

Generated code has NO reduced requirements. Same coverage, same linting, same review.

---

## Principle Mapping

| Principle | TypeScript Implementation |
|-----------|--------------------------|
| 1. Correct Over Fast | Strict mode, comprehensive types, tests |
| 2. Explicit Over Implicit | No any, explicit return types, explicit dependencies |
| 3. Automated Over Vigilant | ESLint, TypeScript strict, CI, automated formatting |
| 4. Secure By Default | Input validation, no eval, CSP, dependency scanning |
| 5. Composable Over Clever | Pure functions, small modules, standard patterns |
| 6. Fail Loud | No empty catch, explicit error handling |
| 7. X.I. Augments, Human Decides | X.I. generates; human reviews and approves |
| 8. Future-Proof Over Trend | Stable frameworks, avoid hype, LTS versions |
| 9. Nothing Lost, Ever | IndexedDB persistence, ServiceWorker queues, localStorage backup |

---

## Deviation Recording (User Choice)

If a developer chooses to violate a **MUST** requirement (e.g., using `any` for legacy compatibility), they **MUST** explicitly mark it for the Security Ledger.

**Syntax:**

```typescript
// XONAIX_DEVIATION: Legacy API requires any type - migration to Zod planned Q2
// LEDGER_ACK: sha256:abc123...
const legacyData: any = externalApi.getData();  // eslint-disable-line @typescript-eslint/no-explicit-any
```

**This triggers:**
1. Warning displayed to user
2. Acknowledgment required
3. Signature captured
4. Ledger record created
5. Artifact marked as "User Choice" (not Xonaix Certified)

---

## SECTION 1: PROJECT CONFIGURATION

### 1.1 TypeScript Configuration

```json
// tsconfig.json
{
  "compilerOptions": {
    // Strict Type Checking - ALL MUST BE ENABLED
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true,
    "strictBindCallApply": true,
    "strictPropertyInitialization": true,
    "noImplicitThis": true,
    "useUnknownInCatchVariables": true,
    "alwaysStrict": true,
    
    // Additional Safety
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noPropertyAccessFromIndexSignature": true,
    
    // Module Resolution
    "moduleResolution": "bundler",
    "module": "ESNext",
    "target": "ES2022",
    "esModuleInterop": true,
    "isolatedModules": true,
    "verbatimModuleSyntax": true,
    
    // Output
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "./dist",
    
    // Project
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}
```

### 1.2 ESLint Configuration

```javascript
// eslint.config.js
import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import prettier from 'eslint-plugin-prettier';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
  {
    languageOptions: {
      parserOptions: {
        project: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      // Type Safety - ALL MUST BE ERROR
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',
      '@typescript-eslint/no-unsafe-argument': 'error',
      
      // Best Practices
      '@typescript-eslint/explicit-function-return-type': 'error',
      '@typescript-eslint/explicit-module-boundary-types': 'error',
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-floating-promises': 'error',
      '@typescript-eslint/no-misused-promises': 'error',
      '@typescript-eslint/await-thenable': 'error',
      '@typescript-eslint/require-await': 'error',
      '@typescript-eslint/promise-function-async': 'error',
      
      // Code Quality
      '@typescript-eslint/no-non-null-assertion': 'error',
      '@typescript-eslint/prefer-nullish-coalescing': 'error',
      '@typescript-eslint/prefer-optional-chain': 'error',
      '@typescript-eslint/no-unnecessary-condition': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': 'error',
      
      // Function Size (NASA Rule 4)
      'max-lines-per-function': ['error', { max: 60, skipBlankLines: true, skipComments: true }],
      
      // Security
      'no-eval': 'error',
      'no-implied-eval': 'error',
      'no-new-func': 'error',
      'no-console': 'error',
      
      // Formatting
      'prettier/prettier': 'error',
    },
  }
);
```

### 1.3 Prettier Configuration

```json
// .prettierrc
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100,
  "bracketSpacing": true,
  "arrowParens": "always",
  "endOfLine": "lf"
}
```

### 1.4 Package.json Scripts

```json
{
  "scripts": {
    "build": "tsc",
    "lint": "eslint . --ext .ts,.tsx",
    "lint:fix": "eslint . --ext .ts,.tsx --fix",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "typecheck": "tsc --noEmit",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage",
    "test:mutation": "stryker run",
    "test:property": "vitest run --config vitest.property.config.ts",
    "ci": "npm run typecheck && npm run lint && npm run test:coverage && npm run build",
    "audit": "npm audit --audit-level=high"
  }
}
```

---

## SECTION 2: TYPE SAFETY

### 2.1 No `any` Type

```typescript
// NO VIOLATION
function process(data: any): any {
  return data.value;
}

// YES CORRECT
function process(data: UserInput): ProcessedResult {
  return { value: data.value };
}

// NO VIOLATION - implicit any
function process(data) {
  return data;
}

// YES CORRECT - unknown for external data
function process(data: unknown): ProcessedResult {
  const validated = validateInput(data);
  return { value: validated.value };
}
```

### 2.2 Use `unknown` for External Data

```typescript
// NO VIOLATION - trusts external data
async function fetchUser(id: string): Promise<User> {
  const response = await fetch(`/api/users/${id}`);
  return response.json() as User; // Dangerous!
}

// YES CORRECT - validates external data
async function fetchUser(id: string): Promise<User> {
  const response = await fetch(`/api/users/${id}`);
  const data: unknown = await response.json();
  return UserSchema.parse(data); // Validated with Zod!
}
```

### 2.3 No Unvalidated Type Assertions

```typescript
// NO VIOLATION
const user = apiResponse as User;
const element = document.getElementById('app') as HTMLDivElement;

// YES CORRECT - with validation
const user = UserSchema.parse(apiResponse);

// YES CORRECT - with type guard
const element = document.getElementById('app');
if (element instanceof HTMLDivElement) {
  // Now safely typed
}

// YES CORRECT - with assertion function
function assertDefined<T>(value: T | null | undefined, message: string): T {
  if (value === null || value === undefined) {
    throw new Error(message);
  }
  return value;
}

const element = assertDefined(
  document.getElementById('app'),
  'App element not found'
);
```

### 2.4 Explicit Return Types

```typescript
// NO VIOLATION - implicit return type
export function calculate(items: Item[]) {
  return items.reduce((sum, i) => sum + i.price, 0);
}

// YES CORRECT - explicit return type
export function calculate(items: Item[]): number {
  return items.reduce((sum, i) => sum + i.price, 0);
}

// NO VIOLATION - async implicit
export async function fetchData() {
  const response = await fetch('/api/data');
  return response.json();
}

// YES CORRECT - async explicit
export async function fetchData(): Promise<ApiResponse> {
  const response = await fetch('/api/data');
  const data: unknown = await response.json();
  return ApiResponseSchema.parse(data);
}
```

### 2.5 Discriminated Unions

```typescript
// YES CORRECT - discriminated union for results
type Result<T, E = Error> =
  | { success: true; data: T }
  | { success: false; error: E };

function parseConfig(input: string): Result<Config, ParseError> {
  try {
    const data: unknown = JSON.parse(input);
    const config = ConfigSchema.parse(data);
    return { success: true, data: config };
  } catch (e) {
    return { success: false, error: new ParseError('Invalid config', { cause: e }) };
  }
}

// Usage with exhaustive checking
function handleResult(result: Result<Config>): void {
  if (result.success) {
    useConfig(result.data);
  } else {
    logError(result.error);
  }
}
```

### 2.6 Type Guards

```typescript
// YES CORRECT - custom type guard
function isUser(value: unknown): value is User {
  return (
    typeof value === 'object' &&
    value !== null &&
    'id' in value &&
    'email' in value &&
    typeof (value as User).id === 'string' &&
    typeof (value as User).email === 'string'
  );
}

// YES CORRECT - assertion function
function assertUser(value: unknown): asserts value is User {
  if (!isUser(value)) {
    throw new TypeError('Expected User object');
  }
}

// Usage
function processUser(input: unknown): void {
  assertUser(input);
  // input is now typed as User
  console.log(input.email);
}
```

---

## SECTION 3: INPUT VALIDATION

### 3.1 Zod Schemas

```typescript
import { z } from 'zod';

// YES CORRECT - comprehensive schema
const UserSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  name: z.string().min(1).max(100),
  role: z.enum(['admin', 'user', 'guest']),
  createdAt: z.string().datetime(),
  metadata: z.record(z.string()).optional(),
});

type User = z.infer<typeof UserSchema>;

// YES CORRECT - validation with error handling
function validateUser(input: unknown): Result<User, z.ZodError> {
  const result = UserSchema.safeParse(input);
  if (result.success) {
    return { success: true, data: result.data };
  }
  return { success: false, error: result.error };
}
```

### 3.2 API Request Validation

```typescript
import { z } from 'zod';

const CreateUserRequestSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100),
  password: z.string().min(12).max(128),
});

// YES CORRECT - validate at API boundary
export async function handleCreateUser(req: Request): Promise<Response> {
  const body: unknown = await req.json();
  
  const validation = CreateUserRequestSchema.safeParse(body);
  if (!validation.success) {
    return new Response(
      JSON.stringify({ error: 'Invalid request', details: validation.error.flatten() }),
      { status: 400 }
    );
  }
  
  const user = await createUser(validation.data);
  return new Response(JSON.stringify(user), { status: 201 });
}
```

### 3.3 Environment Variable Validation

```typescript
import { z } from 'zod';

const EnvSchema = z.object({
  NODE_ENV: z.enum(['development', 'staging', 'production']),
  DATABASE_URL: z.string().url(),
  API_KEY: z.string().min(32),
  PORT: z.coerce.number().int().min(1).max(65535).default(3000),
});

// Validate on startup - fail loud
export const env = EnvSchema.parse(process.env);
```

---

## SECTION 4: ASYNC PATTERNS

### 4.1 Timeouts on All External Calls

```typescript
// YES CORRECT - timeout wrapper
async function withTimeout<T>(
  promise: Promise<T>,
  ms: number,
  message = 'Operation timed out'
): Promise<T> {
  const timeout = new Promise<never>((_, reject) => {
    setTimeout(() => reject(new TimeoutError(message)), ms);
  });
  return Promise.race([promise, timeout]);
}

// YES CORRECT - fetch with timeout
async function fetchWithTimeout(
  url: string,
  options: RequestInit = {},
  timeoutMs = 30_000
): Promise<Response> {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeoutMs);
  
  try {
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
    });
    return response;
  } finally {
    clearTimeout(timeoutId);
  }
}
```

### 4.2 Bounded Iteration

```typescript
const MAX_ITEMS = 10_000;

// YES CORRECT - bounded async iteration
async function* processStream<T>(
  stream: AsyncIterable<T>
): AsyncGenerator<T, void, unknown> {
  let count = 0;
  
  for await (const item of stream) {
    if (count >= MAX_ITEMS) {
      throw new ProcessError(`Exceeded max items: ${MAX_ITEMS}`);
    }
    yield item;
    count++;
  }
}

// YES CORRECT - bounded array processing
function processItems<T, R>(items: T[], processor: (item: T) => R): R[] {
  if (items.length > MAX_ITEMS) {
    throw new ProcessError(`Too many items: ${items.length} exceeds ${MAX_ITEMS}`);
  }
  return items.map(processor);
}
```

### 4.3 Promise Error Handling

```typescript
// NO VIOLATION - floating promise
fetchData();

// NO VIOLATION - unhandled rejection
async function process(): Promise<void> {
  const data = await fetchData(); // May throw
  useData(data);
}

// YES CORRECT - handled promise
fetchData()
  .then(handleData)
  .catch(handleError);

// YES CORRECT - try/catch in async
async function process(): Promise<Result<Data, Error>> {
  try {
    const data = await fetchData();
    return { success: true, data };
  } catch (e) {
    return { success: false, error: e instanceof Error ? e : new Error(String(e)) };
  }
}
```

---

## SECTION 5: ERROR HANDLING

### 5.1 MUST: Explicit Error Handling

```typescript
// YES CORRECT - Result type pattern
type Result<T, E = Error> =
  | { success: true; data: T }
  | { success: false; error: E };

async function fetchUser(id: string): Promise<Result<User, FetchError>> {
  try {
    const response = await fetchWithTimeout(`/api/users/${id}`, {}, 10_000);
    
    if (!response.ok) {
      return {
        success: false,
        error: new FetchError(`HTTP ${response.status}`, response.status),
      };
    }
    
    const data: unknown = await response.json();
    const user = UserSchema.parse(data);
    
    return { success: true, data: user };
  } catch (e) {
    if (e instanceof z.ZodError) {
      return { success: false, error: new FetchError('Invalid response format', 500) };
    }
    if (e instanceof TimeoutError) {
      return { success: false, error: new FetchError('Request timed out', 408) };
    }
    return { success: false, error: new FetchError('Unknown error', 500) };
  }
}
```

### 5.2 MUST NOT: Silent Swallowing

```typescript
// NO VIOLATION - empty catch
try {
  mightFail();
} catch (e) {
  // Silent - FORBIDDEN
}

// NO VIOLATION - catch and continue
try {
  mightFail();
} catch (e) {
  console.log('Something went wrong'); // Not handling, just logging
}
// Continues as if nothing happened

// YES CORRECT - proper handling
try {
  mightFail();
} catch (e) {
  throw new ProcessError('Operation failed', { cause: e });
}
```

### 5.3 Custom Error Classes

```typescript
// YES CORRECT - domain-specific errors
export class FetchError extends Error {
  constructor(
    message: string,
    public readonly statusCode: number,
    options?: ErrorOptions
  ) {
    super(message, options);
    this.name = 'FetchError';
  }
}

export class ValidationError extends Error {
  constructor(
    message: string,
    public readonly field: string,
    options?: ErrorOptions
  ) {
    super(message, options);
    this.name = 'ValidationError';
  }
}

export class TimeoutError extends Error {
  constructor(message = 'Operation timed out', options?: ErrorOptions) {
    super(message, options);
    this.name = 'TimeoutError';
  }
}
```

---

## SECTION 6: SECURITY

### 6.1 No Eval or Dynamic Code

```typescript
// NO VIOLATION - eval
eval('console.log("hello")');

// NO VIOLATION - Function constructor
new Function('return this')();

// NO VIOLATION - setTimeout with string
setTimeout('doSomething()', 1000);

// YES CORRECT - function reference
setTimeout(() => doSomething(), 1000);
```

### 6.2 Cryptography

```typescript
// YES CORRECT - use Web Crypto API
async function generateToken(): Promise<string> {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
}

// NO VIOLATION - Math.random for security
function generateToken(): string {
  return Math.random().toString(36); // NOT SECURE
}

// YES CORRECT - hash with Web Crypto
async function hashPassword(password: string): Promise<string> {
  const encoder = new TextEncoder();
  const data = encoder.encode(password);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');
}
```

**Approved Libraries:**

| Purpose | Library | Notes |
|---------|---------|-------|
| Hashing | Web Crypto API | Built-in, secure |
| Signatures | `@noble/ed25519` | Classical Ed25519 |
| Encryption | `@noble/ciphers` | ChaCha20-Poly1305 |
| PQC (Phase 3) | DECIDED | Hybrid with classical |

**Post-Quantum Cryptography (PQC):**

| Phase | Timeline | Requirement |
|-------|----------|-------------|
| Phase 1: Design | Now | SHOULD design for hybrid compatibility |
| Phase 2: Simulate | 2026 | SHOULD prototype hybrid in staging |
| Phase 3: Production | 2027 | MUST for Tier 3-4 (hybrid classical + ML-DSA) |

```typescript
// Phase 1 Architecture - Design for hybrid
interface XonaixSignature {
  classical: Uint8Array;    // Always present (Ed25519)
  pqc?: Uint8Array;         // None until Phase 3 (ML-DSA)
}

// Verification: If PQC present, BOTH must verify
async function verifySignature(
  message: Uint8Array,
  signature: XonaixSignature,
  publicKey: XonaixPublicKey
): Promise<boolean> {
  // Classical verification (always)
  const classicalValid = await verifyEd25519(message, signature.classical, publicKey.classical);
  if (!classicalValid) return false;
  
  // PQC verification (Phase 3)
  if (signature.pqc && publicKey.pqc) {
    const pqcValid = await verifyMlDsa(message, signature.pqc, publicKey.pqc);
    if (!pqcValid) return false;
  }
  
  return true;
}
```

### 6.3 URL Construction

```typescript
// NO VIOLATION - string concatenation
const url = `/api/users/${userId}?filter=${filter}`;

// YES CORRECT - URL API with validation
function buildUserUrl(userId: string, filter: string): string {
  // Validate inputs
  if (!/^[a-f0-9-]{36}$/.test(userId)) {
    throw new ValidationError('Invalid user ID', 'userId');
  }
  
  const url = new URL(`/api/users/${encodeURIComponent(userId)}`, window.location.origin);
  url.searchParams.set('filter', filter);
  return url.toString();
}
```

### 6.4 Content Security Policy

```typescript
// YES CORRECT - CSP headers
const cspDirectives = {
  'default-src': ["'self'"],
  'script-src': ["'self'"],
  'style-src': ["'self'", "'unsafe-inline'"], // Only if absolutely needed
  'img-src': ["'self'", 'data:', 'https:'],
  'font-src': ["'self'"],
  'connect-src': ["'self'", 'https://api.xonaix.com'],
  'frame-ancestors': ["'none'"],
  'form-action': ["'self'"],
  'base-uri': ["'self'"],
};

function buildCsp(directives: typeof cspDirectives): string {
  return Object.entries(directives)
    .map(([key, values]) => `${key} ${values.join(' ')}`)
    .join('; ');
}
```

### 6.5 FIPS 140-3 Compliance Roadmap

| Phase | Requirement | Timeline |
|-------|-------------|----------|
| Phase 1 | Use Web Crypto API (browser FIPS compliance varies) | Immediate |
| Phase 2 | Document crypto library provenance | 2026 |
| Phase 3 | Ensure FIPS-compliant paths for Controlled | 2027 |

---

## SECTION 7: NASA/DOD GRADE REQUIREMENTS

### 7.1 Bounded Loops

All loops MUST have provable termination bounds:

```typescript
const MAX_ITERATIONS = 10_000;

// YES CORRECT - explicit bound check
function processItems<T>(items: T[], processor: (item: T) => void): void {
  // Precondition
  console.assert(items.length <= MAX_ITERATIONS, 'Items exceed max');
  
  let count = 0;
  for (const item of items) {
    if (count >= MAX_ITERATIONS) {
      throw new ProcessError(`Exceeded max iterations: ${MAX_ITERATIONS}`);
    }
    processor(item);
    count++;
  }
  
  // Postcondition
  console.assert(count <= MAX_ITERATIONS, 'Processed count exceeds max');
}

// YES CORRECT - bounded array methods
function safeMap<T, R>(items: T[], mapper: (item: T) => R): R[] {
  if (items.length > MAX_ITERATIONS) {
    throw new ProcessError(`Array too large: ${items.length}`);
  }
  return items.map(mapper);
}
```

### 7.2 Assertion Density

MUST have 2+ assertions per function for Production/Controlled:

```typescript
// YES CORRECT - assertions for pre/postconditions
function transferFunds(from: Account, to: Account, amount: number): Receipt {
  // Precondition 1
  console.assert(amount > 0, 'Amount must be positive');
  
  // Precondition 2
  console.assert(from.balance >= amount, 'Insufficient balance');
  
  const receipt = executeTransfer(from, to, amount);
  
  // Postcondition 1
  console.assert(receipt.isFinalized, 'Transfer must be finalized');
  
  // Postcondition 2
  console.assert(
    from.balance + to.balance === from.originalBalance + to.originalBalance,
    'Conservation of funds violated'
  );
  
  return receipt;
}
```

For production builds, use a custom assert that can be tree-shaken or disabled:

```typescript
// src/utils/assert.ts
export function assert(condition: boolean, message: string): asserts condition {
  if (import.meta.env.DEV || import.meta.env.MODE === 'test') {
    if (!condition) {
      throw new AssertionError(message);
    }
  }
}

export class AssertionError extends Error {
  constructor(message: string) {
    super(`Assertion failed: ${message}`);
    this.name = 'AssertionError';
  }
}
```

### 7.3 Function Size Limits

MUST ≤60 lines; SHOULD ≤30 lines.

ESLint enforces this:
```javascript
'max-lines-per-function': ['error', { max: 60, skipBlankLines: true, skipComments: true }],
```

---

## SECTION 8: PROPERTY-BASED TESTING

**Controlled classification MUST use property-based tests:**

```bash
npm install --save-dev fast-check
```

```typescript
import * as fc from 'fast-check';

describe('Serialization', () => {
  it('roundtrip property', () => {
    fc.assert(
      fc.property(fc.string(), (input) => {
        const encoded = encode(input);
        const decoded = decode(encoded);
        expect(decoded).toBe(input);
      })
    );
  });
  
  it('never throws on valid input', () => {
    fc.assert(
      fc.property(
        fc.record({
          id: fc.uuid(),
          email: fc.emailAddress(),
          name: fc.string({ minLength: 1, maxLength: 100 }),
        }),
        (user) => {
          expect(() => processUser(user)).not.toThrow();
        }
      )
    );
  });
  
  it('balance never negative', () => {
    fc.assert(
      fc.property(
        fc.nat({ max: 1_000_000 }),
        fc.array(fc.integer({ min: -1000, max: 1000 }), { maxLength: 100 }),
        (initial, transactions) => {
          const account = new Account(initial);
          for (const tx of transactions) {
            account.applyTransaction(tx); // May fail, that's OK
          }
          expect(account.balance).toBeGreaterThanOrEqual(0);
        }
      )
    );
  });
});
```

---

## SECTION 9: MUTATION TESTING

**Controlled classification MUST achieve ≥95% mutation score:**

```bash
npm install --save-dev @stryker-mutator/core @stryker-mutator/vitest-runner
```

```javascript
// stryker.conf.js
/** @type {import('@stryker-mutator/api/core').PartialStrykerOptions} */
export default {
  packageManager: 'npm',
  reporters: ['html', 'clear-text', 'progress'],
  testRunner: 'vitest',
  coverageAnalysis: 'perTest',
  thresholds: {
    high: 95,
    low: 90,
    break: 95, // Fail if below 95%
  },
  mutate: ['src/**/*.ts', '!src/**/*.test.ts'],
};
```

```bash
npx stryker run
```

**Surviving mutants MUST be documented in `MUTATION_SURVIVORS.md`:**

```markdown
# Mutation Survivors Review

## src/utils/format.ts:23 - Changed `>=` to `>`
**Category:** Equivalent
**Reason:** Input is validated to never equal boundary on line 20.

## src/logging.ts:45 - Removed log statement
**Category:** Accepted Risk
**Reason:** Log is observability, not correctness.
```

---

## SECTION 10: CHAOS TESTING

**Controlled classification MUST implement chaos testing:**

```typescript
describe('Chaos Tests', () => {
  it('handles network failure gracefully', async () => {
    // Inject fault
    vi.spyOn(global, 'fetch').mockRejectedValue(new Error('Network failure'));
    
    // Verify graceful handling
    const result = await fetchUserSafe('user-123');
    
    expect(result.success).toBe(false);
    expect(result.error).toBeInstanceOf(FetchError);
    
    // Verify no data loss (Principle 9)
    expect(pendingQueue.hasPendingRequests()).toBe(true);
  });
  
  it('handles timeout gracefully', async () => {
    vi.useFakeTimers();
    
    const fetchPromise = fetchWithTimeout('/api/data', {}, 5000);
    
    vi.advanceTimersByTime(6000);
    
    await expect(fetchPromise).rejects.toThrow(TimeoutError);
    
    vi.useRealTimers();
  });
  
  it('handles storage full', async () => {
    // Mock IndexedDB quota exceeded
    vi.spyOn(localStorage, 'setItem').mockImplementation(() => {
      throw new DOMException('QuotaExceededError');
    });
    
    const result = await persistData({ key: 'value' });
    
    expect(result.success).toBe(false);
    expect(result.error.name).toBe('StorageError');
  });
});
```

---

## SECTION 11: NOTHING LOST, EVER (PRINCIPLE 9)

### 11.1 IndexedDB Persistence

```typescript
// Durable queue using IndexedDB
class DurableQueue<T> {
  private db: IDBDatabase | null = null;
  private readonly storeName = 'messages';
  
  async initialize(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('xonaix-queue', 1);
      
      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        this.db = request.result;
        resolve();
      };
      
      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(this.storeName)) {
          db.createObjectStore(this.storeName, { keyPath: 'id' });
        }
      };
    });
  }
  
  async enqueue(message: T): Promise<string> {
    const id = crypto.randomUUID();
    const entry = { id, message, timestamp: Date.now() };
    
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.storeName, 'readwrite');
      const store = tx.objectStore(this.storeName);
      const request = store.add(entry);
      
      request.onsuccess = () => resolve(id);
      request.onerror = () => reject(request.error);
    });
  }
  
  async acknowledge(id: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.storeName, 'readwrite');
      const store = tx.objectStore(this.storeName);
      const request = store.delete(id);
      
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }
  
  async recoverPending(): Promise<Array<{ id: string; message: T }>> {
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.storeName, 'readonly');
      const store = tx.objectStore(this.storeName);
      const request = store.getAll();
      
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }
}
```

### 11.2 Service Worker Queue

```typescript
// service-worker.ts
const QUEUE_NAME = 'governance-messages';

self.addEventListener('fetch', (event: FetchEvent) => {
  if (event.request.url.includes('/api/governance/')) {
    event.respondWith(
      fetch(event.request).catch(async () => {
        // Network failed - queue for retry
        await queueForRetry(event.request.clone());
        return new Response(
          JSON.stringify({ queued: true }),
          { status: 202, headers: { 'Content-Type': 'application/json' } }
        );
      })
    );
  }
});

async function queueForRetry(request: Request): Promise<void> {
  const cache = await caches.open(QUEUE_NAME);
  const key = `${request.url}-${Date.now()}`;
  await cache.put(key, await request.clone().blob().then(b => new Response(b)));
}

// Retry queued requests when online
self.addEventListener('sync', async (event: SyncEvent) => {
  if (event.tag === 'retry-governance') {
    const cache = await caches.open(QUEUE_NAME);
    const keys = await cache.keys();
    
    for (const key of keys) {
      const response = await cache.match(key);
      if (response) {
        try {
          await fetch(key.url, { method: 'POST', body: await response.blob() });
          await cache.delete(key);
        } catch {
          // Will retry on next sync
        }
      }
    }
  }
});
```

---

## SECTION 12: TESTING

### 12.1 Vitest Configuration

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['src/**/*.test.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: ['node_modules/', 'dist/', '**/*.test.ts'],
      thresholds: {
        lines: 95,
        branches: 90,
        functions: 95,
        statements: 95,
      },
    },
    mockReset: true,
    restoreMocks: true,
  },
});
```

### 12.2 Test Structure

```typescript
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('UserService', () => {
  describe('createUser', () => {
    beforeEach(() => {
      vi.clearAllMocks();
    });

    it('creates user with valid input', async () => {
      // Arrange
      const input: CreateUserInput = {
        email: 'test@example.com',
        name: 'Test User',
        role: 'user',
      };

      // Act
      const result = await userService.createUser(input);

      // Assert
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.email).toBe(input.email);
        expect(result.data.id).toBeDefined();
      }
    });

    it('returns validation error for invalid email', async () => {
      // Arrange
      const input = {
        email: 'not-an-email',
        name: 'Test User',
        role: 'user',
      };

      // Act
      const result = await userService.createUser(input);

      // Assert
      expect(result.success).toBe(false);
      if (!result.success) {
        expect(result.error.name).toBe('ValidationError');
      }
    });
  });
});
```

---

## SECTION 13: CI PIPELINE

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
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '22'
          cache: 'npm'
      
      - name: Install
        run: npm ci
      
      - name: Type Check
        run: npm run typecheck
      
      - name: Lint
        run: npm run lint
      
      - name: Format Check
        run: npm run format:check
      
      - name: Test with Coverage
        run: npm run test:coverage
      
      - name: Check Coverage Thresholds
        run: |
          COVERAGE=$(cat coverage/coverage-summary.json | jq '.total.lines.pct')
          if (( $(echo "$COVERAGE < 95" | bc -l) )); then
            echo "::error::Line coverage $COVERAGE% below 95%"
            exit 1
          fi
      
      - name: Build
        run: npm run build
      
      - name: Security Audit
        run: npm audit --audit-level=high

  # Controlled classification only
  controlled-checks:
    runs-on: ubuntu-latest
    if: contains(github.event.pull_request.labels.*.name, 'controlled')
    
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '22'
          cache: 'npm'
      
      - run: npm ci
      
      - name: Mutation Testing
        run: |
          npm run test:mutation
          SCORE=$(cat reports/mutation/mutation.json | jq '.mutationScore')
          if (( $(echo "$SCORE < 95" | bc -l) )); then
            echo "::error::Mutation score $SCORE% below 95%"
            exit 1
          fi
      
      - name: Property-Based Tests
        run: npm run test:property
```

---

## SECTION 14: DEPENDENCY MANAGEMENT

### 14.1 Package.json Best Practices

```json
{
  "engines": {
    "node": ">=22.0.0"
  },
  "dependencies": {
    "zod": "^3.22.0"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.3.0",
    "eslint": "^9.0.0",
    "prettier": "^3.0.0",
    "vitest": "^2.0.0",
    "fast-check": "^3.0.0",
    "@stryker-mutator/core": "^8.0.0"
  }
}
```

### 14.2 Dependency Auditing

```bash
# Check for vulnerabilities
npm audit

# Fix vulnerabilities
npm audit fix

# Generate SBOM
npx @cyclonedx/cyclonedx-npm --output-file sbom.json
```

### 14.3 Lockfile Policy

- **Development:** SHOULD commit package-lock.json
- **Production:** MUST commit package-lock.json
- **Controlled:** MUST commit package-lock.json + verify hashes

---

## X.I. Prompt Appendix

*Add to base prompt when working with TypeScript.*

```
TYPESCRIPT v5.0.0 REQUIREMENTS:

NOTE: TypeScript is SECONDARY. Use for interfaces only.
Core systems MUST use Rust.

FORBIDDEN:
- `: any` (use `unknown` and validate)
- Type assertions without validation (`as Type`)
- Empty catch blocks (`catch (e) { }`)
- Floating promises (unhandled async)
- `Math.random()` for security tokens
- `eval()` or `new Function()`
- Non-null assertion (`!`) without proof

REQUIRED:
- Strict mode enabled (all strict flags)
- Explicit return types on exported functions
- Zod or similar for runtime validation
- Timeouts on all fetch/external calls
- Bounded iteration on streams/arrays
- Web Crypto API for tokens

NASA/DOD GRADE:
- Bounded loops: ALL loops MUST have MAX_* constants
- Assertion density: 2+ assertions per function
- Function size: ≤60 lines MUST, ≤30 lines SHOULD
- Line coverage: ≥95% for Controlled
- Branch coverage: ≥90% for Controlled
- Property tests: MUST for Controlled (fast-check)
- Mutation testing: ≥95% score for Controlled (Stryker)
- Chaos testing: MUST for Controlled

CRYPTO:
- Use Web Crypto API
- Use @noble/ed25519 for signatures
- Phase 3 (2027): Hybrid classical + ML-DSA for Tier 3-4

PRINCIPLE 9 (NOTHING LOST):
- IndexedDB for persistence
- Service Worker queues for offline
- Recovery on startup

CONTROLLED CLASSIFICATION CHECKLIST:
[ ] All loops bounded with MAX_* constant
[ ] 2+ assertions per function
[ ] Functions ≤60 lines
[ ] Line coverage ≥95%
[ ] Branch coverage ≥90%
[ ] Property-based tests with fast-check
[ ] Mutation score ≥95% with Stryker
[ ] Chaos tests with fault injection
[ ] Dependencies audited

FLAG THESE VIOLATIONS:
NO `: any` without XONAIX_DEVIATION marker
NO `as Type` without prior validation
NO `catch (e) { }` empty catch
NO `fetch(url)` without timeout
NO `Math.random()` for security
NO Missing return type on exported function
NO Unbounded iteration
NO Functions > 60 lines
NO Coverage below thresholds
```

---

## Quick Reference

### Allowed Patterns

```typescript
// Explicit types
function calc(x: number): number

// Unknown for external data
function parse(data: unknown): User

// Type guards
function isUser(x: unknown): x is User

// Validated assertions
const user = UserSchema.parse(data);

// Result types
type Result<T> = { success: true; data: T } | { success: false; error: Error }
```

### Forbidden Patterns

```typescript
// Any type
function process(data: any): any

// Unvalidated assertion
const user = data as User;

// Implicit any
function process(data) { }

// Non-null assertion without proof
const value = obj.prop!;

// Empty catch
try { } catch (e) { }

// Floating promise
fetchData();

---

*Xonaix Library Standard*
*Canonical: `xonaix-library::specs/standards/typescript/STANDARDS_TYPESCRIPT.md`*
*Authority: `xonaix-specs::THE_XONAIX_WAY.md`*
