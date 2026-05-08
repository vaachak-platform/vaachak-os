# Input Backend Native Executor Cleanup

Status: `input_backend_native_executor_cleanup=ok`

This checkpoint finalizes the accepted `input_backend_native_executor` work before moving to the next native hardware backend migration.

## What is accepted

Vaachak owns the first native input backend behavior layer for:

- button event normalization
- input intent mapping
- scan handoff pre-routing
- navigation handoff pre-routing

The selected input-native backend remains:

```text
VaachakInputNativeWithPulpSampling
```

The low-level physical sampling fallback remains:

```text
PulpCompatibility
```

## Active backend relationship

```text
Vaachak hardware backend takeover
  -> VaachakInputNativeWithPulpSampling
    -> PulpCompatibility physical sampling fallback
```

The cleanup verifies that the accepted backend takeover bridge and backend takeover cleanup checkpoints remain valid.

## Behavior preservation

This cleanup does not rewrite or move:

- physical ADC ladder sampling
- GPIO polling
- button debounce/repeat execution
- navigation dispatch behavior
- display behavior
- storage behavior
- SPI transfer or chip-select behavior
- reader/file-browser UX
- app navigation behavior

## Cleanup behavior

The cleanup script removes old overlay artifacts such as:

```text
input_backend_native_executor
input_backend_native_executor.zip
input_backend_native_executor_validator_fix*
```

It does not remove repository source files, docs, or validators.

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_backend_takeover_bridge.sh
./scripts/validate_hardware_runtime_backend_takeover_cleanup.sh
./scripts/validate_input_backend_native_executor.sh
./scripts/validate_input_backend_native_executor_cleanup.sh
cargo build
```

Expected markers:

```text
hardware_runtime_backend_takeover_bridge=ok
hardware_runtime_backend_takeover_cleanup=ok
input_backend_native_executor=ok
input_backend_native_executor_cleanup=ok
```

## Next recommended slice

After this checkpoint, the next native migration can be either:

```text
display_backend_native_refresh_shell
```

or a deeper input migration that owns more of the button scan/debounce path. The safer route remains display refresh shell only after hardware smoke confirms the input-native checkpoint on device.
