# Phase 19 Refactor Notes

Phase 19 is a structural ownership step only.

## What changed

`target-xteink-x4/src/runtime/pulp_runtime.rs` no longer owns the ESP HAL entrypoint directly. Its `main()` function is renamed to:

```rust
pub fn run_pulp_runtime() -> !
```

A new Vaachak-owned facade file owns the target entrypoint:

```text
target-xteink-x4/src/runtime/vaachak_runtime.rs
```

The facade prints:

```text
phase19=x4-vaachak-runtime-facade-ok
```

then delegates to:

```rust
pulp_runtime::run_pulp_runtime()
```

## Why

This gives Vaachak OS a stable ownership seam before deeper extraction:

- Vaachak can own boot policy, diagnostics, and future feature gates.
- Imported X4/Pulp reader behavior remains unchanged.
- Future phases can add Vaachak-specific adapters without changing EPUB/bookmark/progress code.

## Guardrail

`./scripts/check_reader_runtime_sync_phase19.sh` compares the imported runtime against `vendor/pulp-os/src/bin/main.rs` after normalizing only these allowed differences:

- `x4_os::` crate path becomes `pulp_os::`.
- crate-root attrs move out of the runtime module.
- `#[esp_hal::main] fn main()` moves into the Vaachak facade.
- imported runtime exposes `pub fn run_pulp_runtime() -> !`.
- phase marker log lines are allowed.

Any other drift is flagged.
