---
title: "SvelteKit Framework Standard"
unit_id: "library/standards/sveltekit"
standard_type: "standard"
version: "1.0.0"
status: "active"
owner: "Founder"
last_updated: "2025-12-31"
---
# The Xonaix Way
## Standards: SvelteKit

**Version:** B-5.8.5
**Status:** Active
**Core-Compatible:** 5.7.0
**Trust Class:** L4
**Created:** December 2025
**Last Reviewed:** December 2025

*This document implements The Xonaix Way B-5.8.5 principles for SvelteKit frontend development.*

---

## Document Info

| Field | Value |
|-------|-------|
| Domain | Frontend Framework |
| Status | **Active** |
| Version | B-5.8.5 |
| Core-Compatible | 5.7.0 |
| Trust Class | L4 (Interface) |
| Created | December 2025 |
| Last Reviewed | December 2025 |
| SvelteKit Version | 2.x (current stable) |
| Svelte Version | 5.x |
| Primary Use | Nexus UI |
| Related Standards | STANDARDS_TYPESCRIPT.md, STANDARDS_TAURI.md |

**Prerequisites:** Read [THE_XONAIX_WAY.md](../THE_XONAIX_WAY.md) and [STANDARDS_TYPESCRIPT.md](STANDARDS_TYPESCRIPT.md) first. This document extends TypeScript standards for SvelteKit-specific patterns.

**Framework Role:** SvelteKit is the **primary frontend framework** for Nexus (customer-facing interface). It runs both as a web application and within Tauri for desktop deployment.

---

## Trust Class

| Attribute | Value |
|-----------|-------|
| Trust Class | L4 |
| Classification | Interface |

SvelteKit is an **interface layer**. Both client and server code are **untrusted by default**. All authority determinations MUST come from verified XCLib proofs.

### What L4 May Do

- Display information to users
- Collect user input
- Render verified proof status
- Coordinate UI workflows

### What L4 May NOT Do

- Assert authority without proof verification
- Perform cryptographic operations
- Trust server state as authoritative
- Make governance decisions

---

## XCLib Integration

**Authority:** Founder Ruling 2025-003(c) — Attested Capability

SvelteKit uses **XCLib WASM bindings** client-side for proof visualization.

### Proof Display Pattern

```svelte
<script lang="ts">
  import { verify } from '@xonaix/xclib-wasm';
  
  let { proof } = $props();
  let verificationResult = $state(null);
  
  $effect(() => {
    verify(proof).then(result => {
      verificationResult = result;
    });
  });
</script>

<!-- CORRECT: Show verified status -->
{#if verificationResult?.valid}
  <ProofStrip valid={true} details={verificationResult.details} />
{:else}
  <ProofStrip valid={false} reason="Verification failed" />
{/if}
```

### Prohibited Pattern

```svelte
<!-- FORBIDDEN: Trust server data without verification -->
<script lang="ts">
  let { data } = $props();
</script>

<!-- BAD: Trusting server claim -->
{#if data.proof.isValid}
  <ProofStrip valid={true} />
{/if}
```

---

## Capability & Posture Handling

### Server Code is Untrusted

Even server-side code (load functions, form actions) cannot assert authority:

```typescript
// +page.server.ts
export async function load({ locals }) {
  // FORBIDDEN: Trust session claim
  if (locals.user.role === 'admin') {
    return { adminData: getAdminData() };
  }
  
  // CORRECT: Verify proof server-side with XCLib
  const verified = await xclib.verifyAuthority(locals.user.authorityProof);
  if (verified.valid && verified.hasCapability('admin')) {
    return { adminData: getAdminData() };
  }
}
```

### Posture Degradation

UI MUST degrade gracefully when verification fails:

```svelte
{#if verificationResult === null}
  <LoadingSpinner />
{:else if verificationResult.valid}
  <TrustedContent />
{:else}
  <DegradedContent reason={verificationResult.reason} />
{/if}
```

---

## Principle Mapping

| Principle | SvelteKit Implementation |
|-----------|--------------------------|
| 1. Correct Over Fast | Type-safe load functions, validated form actions, comprehensive tests |
| 2. Secure By Default | CSP headers, CSRF protection, input validation, XSS prevention |
| 3. Fail Loud | Error boundaries, explicit error pages, structured logging |
| 4. Explicit Over Implicit | Explicit types, explicit routing, no magic |
| 5. Automated Over Vigilant | ESLint, Prettier, Playwright, CI/CD |
| 6. Composable Over Clever | Small components, composition over inheritance |
| 7. X.I. Augments, Human Decides | Component changes require human review |
| 8. Future-Proof Over Trend | SvelteKit stable, avoid experimental features |
| 9. Nothing Lost, Ever | Offline-first patterns, IndexedDB persistence, form state preservation |

---

## Deviation Recording

For deviations from MUST requirements in SvelteKit:

```svelte
<!-- XONAIX_DEVIATION: [Reason for deviation - be specific] -->
<!-- LEDGER_ACK: [User_Signature_Hash] -->
<script>
  // Deviating code
</script>
```

Or in TypeScript:

```typescript
// XONAIX_DEVIATION: [Reason for deviation]
// LEDGER_ACK: [User_Signature_Hash]
// Deviating code
```

---

## SECTION 1: PROJECT STRUCTURE

### 1.1 Directory Layout

```
src/
├── lib/                      # Shared library code
│   ├── components/           # Reusable UI components
│   │   ├── ui/              # Base UI components (buttons, inputs, etc.)
│   │   ├── layout/          # Layout components (header, sidebar, etc.)
│   │   └── domain/          # Domain-specific components
│   ├── stores/              # Svelte stores
│   ├── services/            # API clients and services
│   ├── utils/               # Utility functions
│   ├── types/               # TypeScript type definitions
│   └── index.ts             # Library exports
├── routes/                   # SvelteKit routes
│   ├── +layout.svelte       # Root layout
│   ├── +layout.server.ts    # Root layout server load
│   ├── +page.svelte         # Home page
│   ├── +error.svelte        # Error boundary
│   ├── api/                 # API routes
│   │   └── [...path]/
│   │       └── +server.ts
│   ├── (app)/               # Authenticated app routes
│   │   ├── +layout.svelte
│   │   ├── dashboard/
│   │   └── settings/
│   └── (auth)/              # Authentication routes
│       ├── login/
│       └── logout/
├── app.html                  # HTML template
├── app.d.ts                  # App-level type definitions
├── hooks.server.ts           # Server hooks
└── hooks.client.ts           # Client hooks
```

### 1.2 Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Components | PascalCase | `UserProfile.svelte` |
| Routes | kebab-case | `user-settings/` |
| Stores | camelCase | `userStore.ts` |
| Utils | camelCase | `formatDate.ts` |
| Types | PascalCase | `UserProfile.ts` |
| Constants | SCREAMING_SNAKE | `MAX_RETRIES` |

---

## SECTION 2: CONFIGURATION

### 2.1 svelte.config.js

```javascript
import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  
  kit: {
    adapter: adapter(),
    
    // CSP configuration
    csp: {
      directives: {
        'default-src': ['self'],
        'script-src': ['self'],
        'style-src': ['self', 'unsafe-inline'],
        'img-src': ['self', 'data:', 'https:'],
        'font-src': ['self'],
        'connect-src': ['self', 'https://api.xonaix.com'],
        'frame-ancestors': ['none'],
        'form-action': ['self'],
        'base-uri': ['self'],
      },
      reportOnly: false,
    },
    
    // CSRF protection
    csrf: {
      checkOrigin: true,
    },
    
    // TypeScript strict mode
    typescript: {
      config: (config) => ({
        ...config,
        compilerOptions: {
          ...config.compilerOptions,
          strict: true,
          noImplicitAny: true,
          strictNullChecks: true,
        },
      }),
    },
    
    alias: {
      $components: 'src/lib/components',
      $stores: 'src/lib/stores',
      $services: 'src/lib/services',
      $utils: 'src/lib/utils',
      $types: 'src/lib/types',
    },
  },
};

export default config;
```

### 2.2 vite.config.ts

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [sveltekit()],
  
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    environment: 'jsdom',
    globals: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      thresholds: {
        lines: 95,
        branches: 90,
        functions: 95,
        statements: 95,
      },
    },
  },
  
  build: {
    sourcemap: true,
  },
  
  server: {
    strictPort: true,
  },
});
```

### 2.3 TypeScript Configuration

```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true,
    "strictBindCallApply": true,
    "strictPropertyInitialization": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noPropertyAccessFromIndexSignature": true,
    "verbatimModuleSyntax": true,
    "moduleResolution": "bundler"
  }
}
```

---

## SECTION 3: LOAD FUNCTIONS

### 3.1 Server Load Functions

```typescript
// src/routes/dashboard/+page.server.ts
import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { z } from 'zod';

// Response schema validation
const DashboardDataSchema = z.object({
  user: z.object({
    id: z.string().uuid(),
    name: z.string(),
    email: z.string().email(),
  }),
  stats: z.object({
    totalWorkflows: z.number().int().nonnegative(),
    activeAgents: z.number().int().nonnegative(),
  }),
});

type DashboardData = z.infer<typeof DashboardDataSchema>;

export const load: PageServerLoad = async ({ locals, fetch, url }) => {
  // Authentication check
  if (!locals.user) {
    throw redirect(303, `/login?redirect=${encodeURIComponent(url.pathname)}`);
  }
  
  // Fetch with timeout (Principle 1: Correct Over Fast)
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 10000);
  
  try {
    const response = await fetch('/api/dashboard', {
      signal: controller.signal,
      headers: {
        'Authorization': `Bearer ${locals.session.token}`,
      },
    });
    
    if (!response.ok) {
      throw error(response.status, {
        message: 'Failed to load dashboard data',
        code: 'DASHBOARD_LOAD_ERROR',
      });
    }
    
    // Validate response (Secure By Default)
    const data: unknown = await response.json();
    const validated = DashboardDataSchema.parse(data);
    
    return {
      dashboard: validated,
    };
  } catch (e) {
    if (e instanceof z.ZodError) {
      throw error(500, {
        message: 'Invalid dashboard data received',
        code: 'VALIDATION_ERROR',
      });
    }
    if ((e as Error).name === 'AbortError') {
      throw error(504, {
        message: 'Dashboard request timed out',
        code: 'TIMEOUT_ERROR',
      });
    }
    throw e;
  } finally {
    clearTimeout(timeout);
  }
};
```

### 3.2 Universal Load Functions

```typescript
// src/routes/agents/[id]/+page.ts
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { z } from 'zod';

const AgentSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1).max(256),
  status: z.enum(['active', 'inactive', 'pending']),
  createdAt: z.string().datetime(),
});

export const load: PageLoad = async ({ params, fetch }) => {
  // Validate route parameter
  const idResult = z.string().uuid().safeParse(params.id);
  if (!idResult.success) {
    throw error(400, {
      message: 'Invalid agent ID format',
      code: 'INVALID_ID',
    });
  }
  
  const response = await fetch(`/api/agents/${params.id}`);
  
  if (response.status === 404) {
    throw error(404, {
      message: 'Agent not found',
      code: 'NOT_FOUND',
    });
  }
  
  if (!response.ok) {
    throw error(response.status, {
      message: 'Failed to load agent',
      code: 'LOAD_ERROR',
    });
  }
  
  const data: unknown = await response.json();
  const agent = AgentSchema.parse(data);
  
  return { agent };
};
```

### 3.3 Load Function Requirements

| Requirement | Level | Rationale |
|-------------|-------|-----------|
| Validate all external data | MUST | Secure By Default |
| Use Zod for validation | SHOULD | Type safety + runtime validation |
| Set request timeouts | MUST | Prevent hanging requests |
| Return typed data | MUST | TypeScript safety |
| Handle all error cases | MUST | Fail Loud |

---

## SECTION 4: FORM ACTIONS

### 4.1 Server Actions

```typescript
// src/routes/settings/profile/+page.server.ts
import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { z } from 'zod';
import { rateLimit } from '$lib/server/rateLimit';

const UpdateProfileSchema = z.object({
  name: z.string().min(1).max(256).trim(),
  email: z.string().email().max(256),
  bio: z.string().max(1000).optional(),
});

export const actions: Actions = {
  updateProfile: async ({ request, locals, cookies }) => {
    // Authentication check
    if (!locals.user) {
      throw redirect(303, '/login');
    }
    
    // Rate limiting
    const rateLimitResult = await rateLimit(locals.user.id, 'profile-update', {
      maxRequests: 10,
      windowMs: 60_000,
    });
    
    if (!rateLimitResult.allowed) {
      return fail(429, {
        error: 'Too many requests. Please try again later.',
        code: 'RATE_LIMITED',
      });
    }
    
    // Parse form data
    const formData = await request.formData();
    const rawData = Object.fromEntries(formData);
    
    // Validate input
    const validationResult = UpdateProfileSchema.safeParse(rawData);
    
    if (!validationResult.success) {
      return fail(400, {
        error: 'Invalid input',
        code: 'VALIDATION_ERROR',
        errors: validationResult.error.flatten().fieldErrors,
        values: rawData,
      });
    }
    
    const { name, email, bio } = validationResult.data;
    
    try {
      // Update profile
      await updateUserProfile(locals.user.id, { name, email, bio });
      
      // Invalidate session cache if email changed
      if (email !== locals.user.email) {
        cookies.delete('session', { path: '/' });
        throw redirect(303, '/login?message=Please+log+in+with+your+new+email');
      }
      
      return { success: true };
    } catch (e) {
      console.error('Profile update failed:', e);
      return fail(500, {
        error: 'Failed to update profile',
        code: 'UPDATE_ERROR',
        values: rawData,
      });
    }
  },
  
  deleteAccount: async ({ locals, cookies }) => {
    if (!locals.user) {
      throw redirect(303, '/login');
    }
    
    // Require re-authentication for destructive action
    // Implementation depends on auth system
    
    await deleteUserAccount(locals.user.id);
    cookies.delete('session', { path: '/' });
    
    throw redirect(303, '/?message=Account+deleted');
  },
};
```

### 4.2 Form Component Pattern

```svelte
<!-- src/routes/settings/profile/+page.svelte -->
<script lang="ts">
  import type { ActionData, PageData } from './$types';
  import { enhance } from '$app/forms';
  import { Button, Input, Textarea } from '$components/ui';
  
  export let data: PageData;
  export let form: ActionData;
  
  let loading = false;
</script>

<form
  method="POST"
  action="?/updateProfile"
  use:enhance={() => {
    loading = true;
    return async ({ update }) => {
      loading = false;
      await update();
    };
  }}
>
  {#if form?.error}
    <div role="alert" class="error">
      {form.error}
    </div>
  {/if}
  
  {#if form?.success}
    <div role="status" class="success">
      Profile updated successfully
    </div>
  {/if}
  
  <Input
    name="name"
    label="Name"
    value={form?.values?.name ?? data.user.name}
    error={form?.errors?.name?.[0]}
    required
  />
  
  <Input
    name="email"
    type="email"
    label="Email"
    value={form?.values?.email ?? data.user.email}
    error={form?.errors?.email?.[0]}
    required
  />
  
  <Textarea
    name="bio"
    label="Bio"
    value={form?.values?.bio ?? data.user.bio}
    error={form?.errors?.bio?.[0]}
    maxlength={1000}
  />
  
  <Button type="submit" disabled={loading}>
    {loading ? 'Saving...' : 'Save Changes'}
  </Button>
</form>
```

---

## SECTION 5: COMPONENTS

### 5.1 Component Structure

```svelte
<!-- src/lib/components/ui/Button.svelte -->
<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  
  // Props with TypeScript
  interface $$Props extends HTMLButtonAttributes {
    variant?: 'primary' | 'secondary' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    loading?: boolean;
  }
  
  export let variant: $$Props['variant'] = 'primary';
  export let size: $$Props['size'] = 'md';
  export let loading: $$Props['loading'] = false;
  
  // Computed classes
  $: classes = [
    'btn',
    `btn--${variant}`,
    `btn--${size}`,
    loading && 'btn--loading',
  ].filter(Boolean).join(' ');
</script>

<button
  class={classes}
  disabled={loading || $$restProps.disabled}
  {...$$restProps}
>
  {#if loading}
    <span class="spinner" aria-hidden="true"></span>
    <span class="sr-only">Loading...</span>
  {/if}
  <slot />
</button>

<style>
  .btn {
    /* Base styles */
  }
  
  .btn--primary {
    /* Primary variant */
  }
  
  /* etc. */
</style>
```

### 5.2 Component Requirements

| Requirement | Level | Rationale |
|-------------|-------|-----------|
| TypeScript for props | MUST | Type safety |
| Explicit prop types | MUST | Explicit Over Implicit |
| Accessibility attributes | MUST | Inclusive design |
| Scoped styles | SHOULD | Composable |
| JSDoc for public components | SHOULD | Documentation |

### 5.3 Compound Components

```svelte
<!-- src/lib/components/domain/AgentCard.svelte -->
<script lang="ts" context="module">
  export interface Agent {
    id: string;
    name: string;
    status: 'active' | 'inactive' | 'pending';
    lastActive: Date;
  }
</script>

<script lang="ts">
  import { formatRelativeTime } from '$utils/date';
  import { Badge, Card } from '$components/ui';
  
  export let agent: Agent;
  export let onClick: ((agent: Agent) => void) | undefined = undefined;
  
  const statusColors = {
    active: 'green',
    inactive: 'gray',
    pending: 'yellow',
  } as const;
</script>

<Card interactive={!!onClick} on:click={() => onClick?.(agent)}>
  <svelte:fragment slot="header">
    <h3>{agent.name}</h3>
    <Badge color={statusColors[agent.status]}>
      {agent.status}
    </Badge>
  </svelte:fragment>
  
  <p>Last active: {formatRelativeTime(agent.lastActive)}</p>
  
  <svelte:fragment slot="footer">
    <slot name="actions" />
  </svelte:fragment>
</Card>
```

---

## SECTION 6: STORES

### 6.1 Writable Stores

```typescript
// src/lib/stores/userStore.ts
import { writable, derived } from 'svelte/store';
import type { User } from '$types';

interface UserState {
  user: User | null;
  loading: boolean;
  error: string | null;
}

function createUserStore() {
  const { subscribe, set, update } = writable<UserState>({
    user: null,
    loading: false,
    error: null,
  });
  
  return {
    subscribe,
    
    setUser: (user: User) => {
      update((state) => ({ ...state, user, error: null }));
    },
    
    clearUser: () => {
      update((state) => ({ ...state, user: null }));
    },
    
    setLoading: (loading: boolean) => {
      update((state) => ({ ...state, loading }));
    },
    
    setError: (error: string) => {
      update((state) => ({ ...state, error, loading: false }));
    },
    
    reset: () => {
      set({ user: null, loading: false, error: null });
    },
  };
}

export const userStore = createUserStore();

// Derived stores
export const isAuthenticated = derived(
  userStore,
  ($userStore) => $userStore.user !== null
);

export const userName = derived(
  userStore,
  ($userStore) => $userStore.user?.name ?? 'Guest'
);
```

### 6.2 Persistent Stores (Principle 9)

```typescript
// src/lib/stores/persistentStore.ts
import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

interface PersistentStoreOptions<T> {
  key: string;
  initialValue: T;
  storage?: 'localStorage' | 'sessionStorage' | 'indexedDB';
  serialize?: (value: T) => string;
  deserialize?: (value: string) => T;
}

export function persistentStore<T>({
  key,
  initialValue,
  storage = 'localStorage',
  serialize = JSON.stringify,
  deserialize = JSON.parse,
}: PersistentStoreOptions<T>): Writable<T> {
  // Get initial value from storage
  let storedValue = initialValue;
  
  if (browser && storage !== 'indexedDB') {
    const storageApi = storage === 'localStorage' ? localStorage : sessionStorage;
    const stored = storageApi.getItem(key);
    if (stored !== null) {
      try {
        storedValue = deserialize(stored);
      } catch {
        console.warn(`Failed to deserialize stored value for key: ${key}`);
      }
    }
  }
  
  const store = writable<T>(storedValue);
  
  // Subscribe to changes and persist
  if (browser && storage !== 'indexedDB') {
    store.subscribe((value) => {
      const storageApi = storage === 'localStorage' ? localStorage : sessionStorage;
      try {
        storageApi.setItem(key, serialize(value));
      } catch (e) {
        console.error(`Failed to persist value for key: ${key}`, e);
      }
    });
  }
  
  return store;
}

// Usage
export const userPreferences = persistentStore({
  key: 'xonaix-user-preferences',
  initialValue: {
    theme: 'system' as 'light' | 'dark' | 'system',
    sidebarCollapsed: false,
  },
});
```

### 6.3 IndexedDB Store (Offline-First)

```typescript
// src/lib/stores/offlineStore.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const DB_NAME = 'xonaix-offline';
const DB_VERSION = 1;

interface OfflineQueueItem {
  id: string;
  action: string;
  payload: unknown;
  timestamp: number;
  retries: number;
}

function createOfflineQueue() {
  const { subscribe, update } = writable<OfflineQueueItem[]>([]);
  
  let db: IDBDatabase | null = null;
  
  async function initDB(): Promise<IDBDatabase> {
    if (db) return db;
    if (!browser) throw new Error('IndexedDB not available');
    
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(DB_NAME, DB_VERSION);
      
      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        db = request.result;
        resolve(db);
      };
      
      request.onupgradeneeded = (event) => {
        const database = (event.target as IDBOpenDBRequest).result;
        if (!database.objectStoreNames.contains('queue')) {
          database.createObjectStore('queue', { keyPath: 'id' });
        }
      };
    });
  }
  
  async function loadQueue(): Promise<void> {
    const database = await initDB();
    const transaction = database.transaction(['queue'], 'readonly');
    const store = transaction.objectStore('queue');
    const request = store.getAll();
    
    return new Promise((resolve, reject) => {
      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        update(() => request.result);
        resolve();
      };
    });
  }
  
  async function addToQueue(item: Omit<OfflineQueueItem, 'id' | 'timestamp' | 'retries'>): Promise<string> {
    const database = await initDB();
    const id = crypto.randomUUID();
    const fullItem: OfflineQueueItem = {
      ...item,
      id,
      timestamp: Date.now(),
      retries: 0,
    };
    
    return new Promise((resolve, reject) => {
      const transaction = database.transaction(['queue'], 'readwrite');
      const store = transaction.objectStore('queue');
      const request = store.add(fullItem);
      
      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        update((items) => [...items, fullItem]);
        resolve(id);
      };
    });
  }
  
  async function removeFromQueue(id: string): Promise<void> {
    const database = await initDB();
    
    return new Promise((resolve, reject) => {
      const transaction = database.transaction(['queue'], 'readwrite');
      const store = transaction.objectStore('queue');
      const request = store.delete(id);
      
      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        update((items) => items.filter((item) => item.id !== id));
        resolve();
      };
    });
  }
  
  return {
    subscribe,
    init: loadQueue,
    add: addToQueue,
    remove: removeFromQueue,
  };
}

export const offlineQueue = createOfflineQueue();
```

---

## SECTION 7: API SERVICES

### 7.1 API Client

```typescript
// src/lib/services/api.ts
import { z } from 'zod';

interface ApiOptions {
  baseUrl: string;
  timeout?: number;
  headers?: Record<string, string>;
}

interface RequestOptions<T> {
  schema?: z.ZodType<T>;
  timeout?: number;
  signal?: AbortSignal;
}

class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public code: string,
    public details?: unknown
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export function createApiClient(options: ApiOptions) {
  const { baseUrl, timeout: defaultTimeout = 30000, headers: defaultHeaders = {} } = options;
  
  async function request<T>(
    method: string,
    path: string,
    body?: unknown,
    requestOptions?: RequestOptions<T>
  ): Promise<T> {
    const { schema, timeout = defaultTimeout, signal } = requestOptions ?? {};
    
    // Create timeout controller
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);
    
    // Combine signals
    const combinedSignal = signal
      ? AbortSignal.any([controller.signal, signal])
      : controller.signal;
    
    try {
      const response = await fetch(`${baseUrl}${path}`, {
        method,
        headers: {
          'Content-Type': 'application/json',
          ...defaultHeaders,
        },
        body: body ? JSON.stringify(body) : undefined,
        signal: combinedSignal,
      });
      
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new ApiError(
          errorData.message ?? 'Request failed',
          response.status,
          errorData.code ?? 'UNKNOWN_ERROR',
          errorData.details
        );
      }
      
      const data: unknown = await response.json();
      
      // Validate response if schema provided
      if (schema) {
        return schema.parse(data);
      }
      
      return data as T;
    } catch (e) {
      if ((e as Error).name === 'AbortError') {
        throw new ApiError('Request timed out', 408, 'TIMEOUT');
      }
      if (e instanceof z.ZodError) {
        throw new ApiError('Invalid response data', 500, 'VALIDATION_ERROR', e.errors);
      }
      throw e;
    } finally {
      clearTimeout(timeoutId);
    }
  }
  
  return {
    get: <T>(path: string, options?: RequestOptions<T>) =>
      request<T>('GET', path, undefined, options),
    
    post: <T>(path: string, body: unknown, options?: RequestOptions<T>) =>
      request<T>('POST', path, body, options),
    
    put: <T>(path: string, body: unknown, options?: RequestOptions<T>) =>
      request<T>('PUT', path, body, options),
    
    patch: <T>(path: string, body: unknown, options?: RequestOptions<T>) =>
      request<T>('PATCH', path, body, options),
    
    delete: <T>(path: string, options?: RequestOptions<T>) =>
      request<T>('DELETE', path, undefined, options),
  };
}

// Create instance
export const api = createApiClient({
  baseUrl: '/api',
  timeout: 30000,
});
```

### 7.2 Domain Services

```typescript
// src/lib/services/agents.ts
import { z } from 'zod';
import { api } from './api';

export const AgentSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  status: z.enum(['active', 'inactive', 'pending']),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
});

export const AgentsListSchema = z.object({
  agents: z.array(AgentSchema),
  total: z.number().int().nonnegative(),
  page: z.number().int().positive(),
  pageSize: z.number().int().positive(),
});

export type Agent = z.infer<typeof AgentSchema>;
export type AgentsList = z.infer<typeof AgentsListSchema>;

export const agentsService = {
  list: (page = 1, pageSize = 20) =>
    api.get<AgentsList>(`/agents?page=${page}&pageSize=${pageSize}`, {
      schema: AgentsListSchema,
    }),
  
  get: (id: string) =>
    api.get<Agent>(`/agents/${id}`, {
      schema: AgentSchema,
    }),
  
  create: (data: { name: string }) =>
    api.post<Agent>('/agents', data, {
      schema: AgentSchema,
    }),
  
  update: (id: string, data: Partial<Agent>) =>
    api.patch<Agent>(`/agents/${id}`, data, {
      schema: AgentSchema,
    }),
  
  delete: (id: string) =>
    api.delete<void>(`/agents/${id}`),
};
```

---

## SECTION 8: SECURITY

### 8.1 Hooks (Server)

```typescript
// src/hooks.server.ts
import type { Handle, HandleServerError } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

// Security headers
const securityHeaders: Handle = async ({ event, resolve }) => {
  const response = await resolve(event);
  
  // Additional security headers
  response.headers.set('X-Frame-Options', 'DENY');
  response.headers.set('X-Content-Type-Options', 'nosniff');
  response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');
  response.headers.set('Permissions-Policy', 'camera=(), microphone=(), geolocation=()');
  
  return response;
};

// Authentication
const authentication: Handle = async ({ event, resolve }) => {
  const sessionToken = event.cookies.get('session');
  
  if (sessionToken) {
    try {
      const session = await validateSession(sessionToken);
      event.locals.session = session;
      event.locals.user = session.user;
    } catch {
      event.cookies.delete('session', { path: '/' });
    }
  }
  
  return resolve(event);
};

// Request logging
const requestLogging: Handle = async ({ event, resolve }) => {
  const start = Date.now();
  const response = await resolve(event);
  const duration = Date.now() - start;
  
  console.log({
    method: event.request.method,
    path: event.url.pathname,
    status: response.status,
    duration,
    userId: event.locals.user?.id,
  });
  
  return response;
};

export const handle = sequence(securityHeaders, authentication, requestLogging);

// Error handling - sanitize errors
export const handleError: HandleServerError = async ({ error, event }) => {
  const errorId = crypto.randomUUID();
  
  // Log full error server-side
  console.error({
    errorId,
    error,
    url: event.url.toString(),
    userId: event.locals.user?.id,
  });
  
  // Return sanitized error to client
  return {
    message: 'An unexpected error occurred',
    errorId,
  };
};
```

### 8.2 Input Sanitization

```typescript
// src/lib/utils/sanitize.ts
import DOMPurify from 'isomorphic-dompurify';

/**
 * Sanitize HTML to prevent XSS
 */
export function sanitizeHtml(dirty: string): string {
  return DOMPurify.sanitize(dirty, {
    ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a', 'p', 'br'],
    ALLOWED_ATTR: ['href', 'title'],
  });
}

/**
 * Escape HTML entities
 */
export function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

/**
 * Sanitize for use in URLs
 */
export function sanitizeUrl(url: string): string {
  try {
    const parsed = new URL(url);
    if (!['http:', 'https:'].includes(parsed.protocol)) {
      return '';
    }
    return parsed.toString();
  } catch {
    return '';
  }
}
```

### 8.3 CSRF Protection

SvelteKit provides CSRF protection via `checkOrigin`. Additional patterns:

```typescript
// src/lib/server/csrf.ts
import { error } from '@sveltejs/kit';

export function validateCsrfToken(
  formToken: string | undefined,
  sessionToken: string | undefined
): void {
  if (!formToken || !sessionToken) {
    throw error(403, {
      message: 'Missing CSRF token',
      code: 'CSRF_MISSING',
    });
  }
  
  if (formToken !== sessionToken) {
    throw error(403, {
      message: 'Invalid CSRF token',
      code: 'CSRF_INVALID',
    });
  }
}
```

---

## SECTION 9: TESTING

### 9.1 Component Tests

```typescript
// src/lib/components/ui/Button.test.ts
import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders with default props', () => {
    render(Button, { props: {}, slots: { default: 'Click me' } });
    expect(screen.getByRole('button')).toHaveTextContent('Click me');
  });
  
  it('applies variant class', () => {
    render(Button, { props: { variant: 'danger' }, slots: { default: 'Delete' } });
    expect(screen.getByRole('button')).toHaveClass('btn--danger');
  });
  
  it('shows loading state', () => {
    render(Button, { props: { loading: true }, slots: { default: 'Submit' } });
    expect(screen.getByRole('button')).toBeDisabled();
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });
  
  it('handles click events', async () => {
    const handleClick = vi.fn();
    render(Button, {
      props: { onclick: handleClick },
      slots: { default: 'Click' },
    });
    
    await fireEvent.click(screen.getByRole('button'));
    expect(handleClick).toHaveBeenCalledOnce();
  });
});
```

### 9.2 Integration Tests (Playwright)

```typescript
// tests/dashboard.test.ts
import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    // Login
    await page.goto('/login');
    await page.fill('[name="email"]', 'test@example.com');
    await page.fill('[name="password"]', 'password');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');
  });
  
  test('displays user stats', async ({ page }) => {
    await expect(page.getByTestId('total-workflows')).toBeVisible();
    await expect(page.getByTestId('active-agents')).toBeVisible();
  });
  
  test('navigates to agent details', async ({ page }) => {
    await page.click('[data-testid="agent-card"]:first-child');
    await expect(page).toHaveURL(/\/agents\/[a-f0-9-]+/);
  });
  
  test('handles API errors gracefully', async ({ page }) => {
    // Mock API failure
    await page.route('/api/dashboard', (route) =>
      route.fulfill({ status: 500, body: JSON.stringify({ message: 'Server error' }) })
    );
    
    await page.reload();
    await expect(page.getByRole('alert')).toContainText('Failed to load');
  });
});
```

### 9.3 Property-Based Tests

```typescript
// src/lib/utils/validation.test.ts
import { describe, it, expect } from 'vitest';
import fc from 'fast-check';
import { sanitizeHtml, escapeHtml } from './sanitize';

describe('sanitizeHtml', () => {
  it('never produces script tags', () => {
    fc.assert(
      fc.property(fc.string(), (input) => {
        const result = sanitizeHtml(input);
        expect(result).not.toContain('<script');
        expect(result).not.toContain('javascript:');
      })
    );
  });
});

describe('escapeHtml', () => {
  it('roundtrips with unescape', () => {
    fc.assert(
      fc.property(
        fc.string().filter((s) => !s.includes('&') && !s.includes('<')),
        (input) => {
          const escaped = escapeHtml(input);
          // No HTML special chars means no change
          expect(escaped).toBe(input);
        }
      )
    );
  });
});
```

---

## SECTION 10: ERROR HANDLING

### 10.1 Error Page

```svelte
<!-- src/routes/+error.svelte -->
<script lang="ts">
  import { page } from '$app/stores';
  import { Button } from '$components/ui';
  
  $: error = $page.error;
  $: status = $page.status;
</script>

<svelte:head>
  <title>Error {status} | Xonaix</title>
</svelte:head>

<main class="error-page">
  <h1>
    {#if status === 404}
      Page Not Found
    {:else if status === 403}
      Access Denied
    {:else if status >= 500}
      Server Error
    {:else}
      Something Went Wrong
    {/if}
  </h1>
  
  <p>{error?.message ?? 'An unexpected error occurred'}</p>
  
  {#if error?.errorId}
    <p class="error-id">Error ID: {error.errorId}</p>
  {/if}
  
  <div class="actions">
    <Button href="/">Go Home</Button>
    <Button variant="secondary" on:click={() => window.location.reload()}>
      Try Again
    </Button>
  </div>
</main>
```

### 10.2 Error Boundary

```svelte
<!-- src/lib/components/ErrorBoundary.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  
  export let fallback: typeof SvelteComponent | undefined = undefined;
  
  let error: Error | null = null;
  
  function handleError(e: ErrorEvent) {
    error = e.error;
    console.error('Caught error:', e.error);
  }
  
  onMount(() => {
    window.addEventListener('error', handleError);
    return () => window.removeEventListener('error', handleError);
  });
</script>

{#if error}
  {#if fallback}
    <svelte:component this={fallback} {error} />
  {:else}
    <div role="alert" class="error-boundary">
      <h2>Something went wrong</h2>
      <p>{error.message}</p>
      <button on:click={() => error = null}>Try again</button>
    </div>
  {/if}
{:else}
  <slot />
{/if}
```

---

## SECTION 11: ACCESSIBILITY

### 11.1 Requirements

| Requirement | Level | Standard |
|-------------|-------|----------|
| Semantic HTML | MUST | WCAG 2.1 |
| Keyboard navigation | MUST | WCAG 2.1 |
| ARIA labels | MUST | WCAG 2.1 |
| Color contrast | MUST | WCAG 2.1 AA |
| Focus management | MUST | WCAG 2.1 |
| Screen reader support | MUST | WCAG 2.1 |

### 11.2 Patterns

```svelte
<!-- Skip link -->
<a href="#main-content" class="skip-link">
  Skip to main content
</a>

<!-- Landmark regions -->
<header role="banner">...</header>
<nav role="navigation" aria-label="Main">...</nav>
<main id="main-content" role="main">...</main>
<footer role="contentinfo">...</footer>

<!-- Loading states -->
{#if loading}
  <div role="status" aria-live="polite">
    Loading...
  </div>
{/if}

<!-- Form errors -->
<input
  id="email"
  type="email"
  aria-invalid={!!error}
  aria-describedby={error ? 'email-error' : undefined}
/>
{#if error}
  <span id="email-error" role="alert">
    {error}
  </span>
{/if}
```

---

## SECTION 12: CI PIPELINE

```yaml
name: SvelteKit CI

on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
      
      - run: npm ci
      - run: npm run lint
      - run: npm run format:check
      - run: npm run check  # Svelte check
  
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
      
      - run: npm ci
      - run: npm run test:coverage
      
      - name: Check Coverage Thresholds
        run: |
          # Coverage thresholds enforced in vite.config.ts
          npm run test:coverage
  
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
      
      - run: npm ci
      - run: npx playwright install --with-deps
      - run: npm run build
      - run: npm run test:e2e
      
      - uses: actions/upload-artifact@v4
        if: failure()
        with:
          name: playwright-report
          path: playwright-report/
  
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
      
      - run: npm ci
      - run: npm run build
      
      - uses: actions/upload-artifact@v4
        with:
          name: build
          path: build/
  
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
      
      - run: npm ci
      - run: npm audit --audit-level=high
```

---

## X.I. Prompt Appendix

```
SVELTEKIT v5.0.0 REQUIREMENTS:

NOTE: SvelteKit is the PRIMARY FRONTEND for Nexus.
Extends STANDARDS_TYPESCRIPT.md — all TypeScript rules apply.

FRAMEWORK:
- SvelteKit 2.x, Svelte 5.x
- TypeScript strict mode
- Vite for build

LOAD FUNCTIONS:
- MUST validate all external data with Zod
- MUST set request timeouts
- MUST return typed data
- MUST handle all error cases

FORMS:
- MUST use progressive enhancement
- MUST validate input server-side
- MUST implement rate limiting for mutations
- MUST preserve form state on error

COMPONENTS:
- MUST use TypeScript for props
- MUST include accessibility attributes
- SHOULD use scoped styles
- MUST be documented (public components)

STORES:
- MUST type all store values
- SHOULD use persistent stores for user preferences
- MUST use IndexedDB for offline queue (Principle 9)

SECURITY:
- MUST configure CSP in svelte.config.js
- MUST enable CSRF protection
- MUST sanitize user-generated content
- MUST validate all inputs

TESTING:
- MUST test components with Testing Library
- MUST test routes with Playwright
- SHOULD use property-based tests for utilities
- Coverage: 95% line, 90% branch for Controlled

PRINCIPLE 9 (NOTHING LOST):
- MUST persist form drafts
- MUST queue offline actions
- MUST sync on reconnect

FORBIDDEN:
NO Untyped props
NO Unvalidated external data
NO Missing error handling in load functions
NO Empty catch blocks
NO innerHTML without sanitization
NO Missing accessibility attributes

FLAG THESE VIOLATIONS:
NO fetch() without timeout
NO Missing Zod validation on API responses
NO Form actions without rate limiting
NO Components without TypeScript
NO Missing error boundaries
```

---

## Changelog

### B-5.8.5 (December 2025)
- **MAJOR:** Added Trust Class section (L4 Interface)
- **MAJOR:** Added XCLib Integration section (WASM bindings for proofs)
- **MAJOR:** Added Capability & Posture Handling section
- **UPDATED:** Core-Compatible to 5.7.0
- **ALIGNED:** Cross-language requirements per STANDARDS_INDEX B-5.8.5
- **Source:** Red-Blue-Black Team synthesis with Founder approval

### v5.0.0 - Core-Compatible 5.1.0 (December 2025)
- **REVIEWED:** Rainbow Team compatibility review
- **CONFIRMED:** No content changes required for Core 5.1.0 compatibility
- **NOTE:** Deviation recording syntax compatible with cryptographic enforcement model

### v5.0.0 (December 2025)
- **CREATED:** Initial SvelteKit standard for Nexus frontend
- **ADDED:** Complete project structure and conventions
- **ADDED:** Configuration for svelte.config.js, vite.config.ts
- **ADDED:** Load function patterns with validation
- **ADDED:** Form action patterns with rate limiting
- **ADDED:** Component patterns with TypeScript
- **ADDED:** Store patterns including persistent and offline
- **ADDED:** API client patterns
- **ADDED:** Security patterns (CSP, CSRF, XSS)
- **ADDED:** Testing patterns (unit, integration, property-based)
- **ADDED:** Error handling patterns
- **ADDED:** Accessibility requirements
- **ADDED:** Complete CI pipeline
- **ADDED:** X.I. Prompt Appendix

---

*SvelteKit Standards B-5.8.5 — Part of The Xonaix Way B-5.8.5*

*"The best UI is invisible. Trust comes from XCLib verification."*

*Xonaix, Inc. — Intelligence, evolved.*
