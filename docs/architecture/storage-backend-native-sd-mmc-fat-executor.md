# Storage Backend Native SD/MMC/FAT Executor

Marker: `storage_backend_native_sd_mmc_fat_executor=ok`

Cleanup checkpoint: `storage_backend_native_sd_mmc_fat_executor_cleanup=ok`

Canonical cleanup document: `docs/architecture/storage-backend-native-sd-mmc-fat-executor-cleanup.md`

## Purpose

This slice moved storage decision behavior from the Pulp-owned runtime path into the Vaachak `target-xteink-x4` layer while keeping the low-level SD/MMC block driver, FAT algorithms, SPI transfer, and chip-select behavior Pulp-compatible.

## Vaachak-owned behavior

Vaachak owns:

- SD/MMC/FAT command decision behavior
- SD card media-state interpretation
- card availability / probe / mount lifecycle intent selection
- FAT operation classification
- path-role policy
- directory listing / file open / file read / path-resolution request construction
- destructive FAT operation denial before low-level handoff

## Pulp-compatible fallback remains active

The active backend is:

```text
VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback
```

The low-level fallback remains:

```text
PulpCompatibility
```

## Explicitly not moved

This slice does not move:

- physical SD/MMC block driver
- physical card I/O
- low-level FAT implementation algorithms
- physical SPI transfer
- chip-select GPIO toggling
- display behavior
- input behavior
- reader/file-browser UX
- app navigation behavior

See the cleanup document for the final acceptance state.
