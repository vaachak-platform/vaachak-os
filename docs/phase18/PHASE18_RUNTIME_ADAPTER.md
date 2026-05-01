# Phase 18 — Vaachak Runtime Adapter Extraction

## Purpose

Phase 18 extracts the working X4/Pulp runtime entry code from the active `target-xteink-x4/src/main.rs` file into a Vaachak-owned runtime adapter boundary:

```text
target-xteink-x4/src/main.rs
  -> target-xteink-x4/src/runtime/mod.rs
  -> target-xteink-x4/src/runtime/pulp_runtime.rs
```

The goal is repo maintainability, not behavior change.

## Non-goals

Phase 18 does **not** rewrite or refactor:

```text
vendor/pulp-os/src/apps/reader/*
vendor/pulp-os/kernel/*
vendor/smol-epub/*
```

Those remain the known-good imported reader path.

## Expected behavior retained

The following Phase 16/17 behavior must remain unchanged:

```text
TXT progress
EPUB progress
TXT bookmarks
EPUB bookmarks
Reader footer action labels
Reader menu actions
Theme preset/state file support
Continue behavior
smol-epub EPUB rendering path
```

## Expected markers

The firmware should continue to print:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
```

Phase 18 adds:

```text
phase18=x4-runtime-adapter-ok
```

## Boundary rule

`target-xteink-x4/src/runtime/pulp_runtime.rs` should still track `vendor/pulp-os/src/bin/main.rs` after:

```text
x4_os:: -> pulp_os::
removing crate-level inner attributes from the runtime module
allowing phase marker lines
allowing rustfmt import ordering
```

Use:

```bash
./scripts/check_reader_runtime_sync_phase18.sh
```

## Why this phase matters

Phase 15B and Phase 16 proved the correct approach: use the existing X4/Pulp reader runtime and `smol-epub` instead of recreating EPUB parsing. Phase 17 cleaned up the repository and documented the imported-runtime boundary. Phase 18 now creates a cleaner Vaachak-owned adapter layer so future work can gradually introduce Vaachak abstractions without destabilizing the reader.
