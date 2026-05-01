# Phase 19 — Vaachak-Owned Runtime Facade

## Goal

Phase 19 adds a Vaachak-owned runtime facade around the imported X4/Pulp reader runtime without changing reader behavior.

The working Phase 16/17/18 path remains intact:

- TXT/MD reader behavior stays on the imported X4/Pulp runtime.
- EPUB/EPU rendering stays on `smol-epub` through the imported X4/Pulp reader.
- Progress, bookmarks, themes, footer labels, quick menu actions, and Continue behavior remain delegated to the imported runtime.
- `vendor/pulp-os` and `vendor/smol-epub` remain authoritative imported code.

## New boundary

Phase 18 extracted the runtime into:

```text
 target-xteink-x4/src/runtime/pulp_runtime.rs
```

Phase 19 adds:

```text
 target-xteink-x4/src/runtime/vaachak_runtime.rs
```

The facade owns the ESP HAL entrypoint and delegates immediately to the imported runtime:

```rust
VaachakRuntime::boot()
  -> pulp_runtime::run_pulp_runtime()
```

## Ownership rule

Vaachak-owned files:

```text
 target-xteink-x4/src/main.rs
 target-xteink-x4/src/runtime/mod.rs
 target-xteink-x4/src/runtime/vaachak_runtime.rs
 docs/phase19/*
 scripts/check_*phase19*.sh
```

Imported-but-wrapped runtime file:

```text
 target-xteink-x4/src/runtime/pulp_runtime.rs
```

Authoritative imported reader/runtime sources:

```text
 vendor/pulp-os/src/bin/main.rs
 vendor/pulp-os/src/apps/reader/*
 vendor/pulp-os/kernel/*
 vendor/smol-epub/*
```

Do not edit reader internals during Phase 19.

## Expected marker

```text
phase19=x4-vaachak-runtime-facade-ok
```
