# Phase 18 Refactor Notes

## What changed

Phase 18 creates a runtime adapter module:

```text
target-xteink-x4/src/runtime/mod.rs
target-xteink-x4/src/runtime/pulp_runtime.rs
```

The active `target-xteink-x4/src/main.rs` becomes a small crate-root entry shell that loads the runtime module. The actual Pulp/X4 runtime code remains in `pulp_runtime.rs` and is checked against the vendored Pulp main file.

## What did not change

The imported reader app and kernel code remain untouched:

```text
vendor/pulp-os/src/apps/reader/*
vendor/pulp-os/kernel/*
vendor/smol-epub/*
```

## Follow-up phases

Recommended next steps after Phase 18:

```text
Phase 19 — Vaachak target configuration and board profile extraction
Phase 20 — Vaachak-owned UI branding layer, after reader parity remains stable
Phase 21 — Reader state adapter API, without changing Pulp reader internals
```
