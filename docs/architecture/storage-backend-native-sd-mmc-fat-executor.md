# Storage Backend Native SD/MMC/FAT Executor

Status: `storage_backend_native_sd_mmc_fat_executor=ok`

This deliverable begins the actual storage behavior migration from the Pulp OS runtime into the Vaachak `target-xteink-x4` layer.

## What moved into Vaachak

Vaachak now owns storage behavior for:

- SD/MMC/FAT command decision behavior
- SD card media-state interpretation
- card availability / probe / mount lifecycle intent selection
- FAT operation classification
- path-role policy for library, reader book, state, and cache paths
- directory listing / file open / file read / path-resolution request construction
- destructive FAT operation denial before Pulp-compatible handoff

The selected native storage backend is:

```text
VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback
```

## What remains Pulp-compatible

This slice intentionally keeps the risky low-level hardware pieces active through the existing Pulp-compatible backend:

- physical SD/MMC block driver
- physical card I/O
- low-level FAT implementation algorithms
- physical SPI transfer
- chip-select GPIO toggling

The active low-level backend remains:

```text
PulpCompatibility
```

## Safety rule

Destructive FAT operations are represented in the Vaachak operation model, but they are denied before low-level handoff in this slice:

```text
CreateFileDenied
AppendFileDenied
DeleteFileDenied
RenameFileDenied
MakeDirDenied
```

This keeps the first storage behavior migration safe while still moving real decision behavior into Vaachak.

## Integration points

The storage native executor is referenced by:

- `hardware_runtime_backend_takeover.rs`
- `hardware_runtime_executor_live_handoff.rs`

The backend takeover layer now has a Vaachak-owned storage handoff path before the existing PulpCompatibility executor fallback.

## Behavior preservation

This deliverable does not change:

- reader/file-browser UX
- app navigation screens
- display behavior
- input behavior
- physical SPI transfer
- chip-select GPIO toggling
- low-level SD/MMC block I/O
- low-level FAT algorithms

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
./scripts/validate_display_backend_native_refresh_command_executor.sh
./scripts/validate_display_backend_native_refresh_command_executor_cleanup.sh
./scripts/validate_storage_backend_native_sd_mmc_fat_executor.sh
cargo build
```

Expected marker:

```text
storage_backend_native_sd_mmc_fat_executor=ok
```

## Hardware smoke expectation

After flashing:

- device boots normally
- SD card still initializes/mounts normally
- file browser still lists files
- TXT/EPUB files still open
- Back navigation still works
- display refresh remains unchanged
- buttons remain unchanged
- no storage corruption or destructive operation path is introduced
