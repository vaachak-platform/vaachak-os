# Phase 35B Wiring Options

## Option A — Path-Only Runtime Bridge

Preferred for Phase 35B.

```text
active imported runtime wrapper
  -> VaachakStorageStateRuntimeBridge::active_runtime_preflight()
  -> VaachakStorageStatePaths / VaachakStorageStateIoAdapter
  -> no-op/path-probe backend
```

Pros:

```text
- No vendor edits
- No physical IO changes
- Easy to validate
- Low risk to reader behavior
```

Cons:

```text
- Does not replace persistence yet
```

## Option B — Shadow Read Probe

Defer to a later phase.

Pros:

```text
- More realistic persistence validation
```

Cons:

```text
- Requires careful SD/FAT access sequencing
- Could interfere with imported runtime if done too early
```

## Option C — Replace Persistence Backend

Not for Phase 35B.

This requires deeper reader/kernel extraction and should be feature-gated in a later phase.
