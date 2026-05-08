# Storage Backend Native SD/MMC/FAT Executor Cleanup

Marker: `storage_backend_native_sd_mmc_fat_executor_cleanup=ok`

This checkpoint finalizes the accepted `storage_backend_native_sd_mmc_fat_executor` migration for commit and GitHub upload.

## Accepted storage behavior move

Vaachak now owns the storage behavior decision layer for:

- SD/MMC/FAT command decision behavior
- SD card media-state interpretation
- card availability / probe / mount lifecycle intent selection
- FAT operation classification
- path-role policy
- directory listing / file open / file read / path-resolution request construction
- destructive FAT operation denial before low-level handoff

## Active backend selection

| Layer | Owner / selection |
| --- | --- |
| Native storage behavior owner | `target-xteink-x4` Vaachak layer |
| Active backend | `VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback` |
| Low-level executor fallback | `PulpCompatibility` |
| Low-level executor owner | `vendor/pulp-os` imported runtime |

## Explicitly not moved in this checkpoint

The cleanup checkpoint preserves the same low-level behavior split as the accepted storage executor slice:

- physical SD/MMC block driver remains Pulp-compatible
- physical card I/O remains Pulp-compatible
- low-level FAT algorithms remain Pulp-compatible
- physical SPI transfer remains unchanged
- chip-select GPIO toggling remains unchanged
- display behavior remains unchanged
- input behavior remains unchanged
- reader/file-browser UX remains unchanged
- app navigation behavior remains unchanged

## Cleanup scope

This package adds:

- `target-xteink-x4/src/vaachak_x4/physical/storage_backend_native_sd_mmc_fat_executor_cleanup.rs`
- `target-xteink-x4/src/vaachak_x4/contracts/storage_backend_native_sd_mmc_fat_executor_cleanup_smoke.rs`
- `scripts/validate_storage_backend_native_sd_mmc_fat_executor_cleanup.sh`
- `scripts/cleanup_storage_backend_native_sd_mmc_fat_executor_artifacts.sh`

It also updates the storage executor documentation to reference this cleanup checkpoint.

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
./scripts/validate_storage_backend_native_sd_mmc_fat_executor_cleanup.sh
cargo build
```

Expected:

```text
storage_backend_native_sd_mmc_fat_executor=ok
storage_backend_native_sd_mmc_fat_executor_cleanup=ok
```

## Hardware smoke

After flashing, validate:

- device boots normally
- no SD mount/probe regression
- file browser opens
- SD file listing works
- long filename/title mapping behaves as before
- TXT files open
- EPUB files open
- Back navigation works
- display refresh unchanged
- buttons unchanged
- no new FAT/path errors
