# Phase 28 — Boundary Contract Test Consolidation

## Goal

Phase 28 consolidates the Vaachak-owned storage, input, and display contract smokes into one boundary test layer.

Expected boot marker:

```text
phase28=x4-boundary-contract-smoke-ok
```

## What changes

Adds:

```text
target-xteink-x4/src/runtime/boundary_contract_smoke.rs
```

Updates:

```text
target-xteink-x4/src/runtime/mod.rs
target-xteink-x4/src/runtime/vaachak_runtime.rs
```

The Vaachak facade emits the Phase 28 marker through the combined boundary contract smoke layer.

## What does not change

Phase 28 does not move physical behavior.

Still owned by imported Pulp runtime:

```text
SD/SPI setup
filesystem IO
ADC button ladder sampling
debounce/repeat handling
SSD1677 init
SPI display transactions
strip rendering
e-paper refresh
ReaderApp / FilesApp / AppManager behavior
```

Still vendored and unchanged:

```text
vendor/pulp-os
vendor/smol-epub
```

## Contract sources consolidated

Phase 28 consolidates these Vaachak-owned contract layers:

```text
Phase 25: target-xteink-x4/src/runtime/storage_state_contract.rs
Phase 26: target-xteink-x4/src/runtime/input_contract_smoke.rs
Phase 27: target-xteink-x4/src/runtime/display_contract_smoke.rs
```

## Why this phase matters

After Phase 28, Vaachak has a single testable boundary contract surface before moving any real hardware behavior out of the imported Pulp runtime.
