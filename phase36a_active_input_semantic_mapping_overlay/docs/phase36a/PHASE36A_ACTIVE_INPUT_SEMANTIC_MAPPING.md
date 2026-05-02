# Phase 36A — Active Input Semantic Mapping Takeover

## Purpose

Phase 36A moves the active runtime's semantic button-mapping construction behind a Vaachak-owned adapter while preserving the known-good imported Pulp input sampling, debounce, repeat, and input task behavior.

Expected normal boot marker remains:

```text
vaachak=x4-runtime-ready
```

No phase marker should be printed during normal boot.

## What moves in Phase 36A

Vaachak now owns the active semantic mapper adapter at:

```text
target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs
```

The active imported runtime calls:

```rust
VaachakActiveInputSemanticMapper::active_runtime_preflight()
VaachakActiveInputSemanticMapper::new_imported_button_mapper()
```

This removes direct `ButtonMapper::new()` construction from `pulp_reader_runtime.rs` and makes the mapping equivalence check Vaachak-owned.

## What does not move

Phase 36A does not move:

```text
ADC sampling
resistor ladder decoding
button debounce
button repeat timing
input_task polling
AppManager internals
reader app internals
```

Those remain in the imported Pulp runtime for now.

## Hardware acceptance

After flashing, validate:

```text
- Device boots.
- Library navigation still works.
- Select/Open still works.
- Back still works.
- TXT page navigation still works.
- EPUB page navigation still works.
- Menu/bookmark behavior remains unchanged.
```
