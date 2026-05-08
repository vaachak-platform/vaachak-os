# Input Backend Native Event Pipeline Cleanup

Status: `input_backend_native_event_pipeline_cleanup=ok`

This checkpoint finalizes the accepted `input_backend_native_event_pipeline` behavior move before the next native hardware backend migration.

## What is accepted

Vaachak now owns actual input event behavior in `target-xteink-x4` for:

- raw sampled button state normalization
- stable button state tracking
- debounce window metadata
- press event generation
- release event generation
- repeat event generation
- button-to-navigation intent mapping
- scan handoff pre-routing
- navigation handoff pre-routing

The active native event pipeline remains:

```text
VaachakNativeEventPipelineWithPulpSampling
```

The low-level physical sampling fallback remains:

```text
PulpCompatibility
```

## Active backend relationship

```text
Vaachak hardware backend takeover
  -> VaachakInputNativeWithPulpSampling
    -> VaachakNativeEventPipelineWithPulpSampling
      -> PulpCompatibility physical ADC/GPIO sampling fallback
```

## Behavior preservation

This cleanup does not rewrite or move:

- physical ADC ladder sampling
- GPIO polling
- physical hardware access
- final app navigation dispatch
- display behavior
- storage behavior
- SPI transfer or chip-select behavior
- reader/file-browser UX
- app navigation screens
- SD/FAT behavior

## Cleanup behavior

The cleanup script removes old overlay artifacts such as:

```text
input_backend_native_event_pipeline
input_backend_native_event_pipeline.zip
input_backend_native_event_pipeline_takeover_fix
input_backend_native_event_pipeline_takeover_fix.zip
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
./scripts/validate_display_backend_native_refresh_shell.sh
./scripts/validate_display_backend_native_refresh_shell_cleanup.sh
./scripts/validate_input_backend_native_event_pipeline.sh
./scripts/validate_input_backend_native_event_pipeline_cleanup.sh
cargo build
```

Expected markers:

```text
hardware_runtime_backend_takeover_bridge=ok
hardware_runtime_backend_takeover_cleanup=ok
input_backend_native_executor=ok
input_backend_native_executor_cleanup=ok
display_backend_native_refresh_shell=ok
display_backend_native_refresh_shell_cleanup=ok
input_backend_native_event_pipeline=ok
input_backend_native_event_pipeline_cleanup=ok
```

## Next recommended slice

After this checkpoint, the next actual behavior move should be selected based on hardware smoke results. The safest next candidates are:

```text
display_backend_native_refresh_command_executor
storage_backend_native_probe_mount_shell
```

Do not move SD/FAT algorithms until input and display smoke remain stable on hardware.
