# Physical Driver Migration Plan

## Status

`physical_driver_migration_plan=ok`

This document is the canonical plan for moving the next lower-level physical driver functionality from the imported Pulp-compatible runtime into the Vaachak `target-xteink-x4` layer.

This is a planning/checkpoint deliverable. It does **not** move another physical driver by itself.

## Accepted prerequisite stack

The plan assumes the accepted native behavior stack is already present:

- input event pipeline behavior is Vaachak-owned
- display refresh command selection behavior is Vaachak-owned
- SD/MMC/FAT command decision behavior is Vaachak-owned
- `PulpCompatibility` remains the active low-level fallback

## Migration order

| Order | Deliverable | Risk | What moves into Vaachak | What remains Pulp-compatible during the slice |
|---:|---|---|---|---|
| 1 | `input_physical_sampling_native_driver` | Low | ADC ladder sample interpretation and physical button sampling shell | known-good Pulp sampling fallback and current navigation dispatch |
| 2 | `spi_physical_transaction_native_driver` | High | shared SPI transaction execution shell and chip-select ownership gate | existing Pulp SPI transfer implementation until display and SD smoke pass |
| 3 | `display_ssd1677_physical_refresh_native_driver` | Medium | SSD1677 refresh command execution wrapper and BUSY/wait ownership shell | existing draw buffer and waveform behavior until refresh smoke passes |
| 4 | `storage_sd_mmc_block_native_driver` | Critical | SD card block-driver probe/read shell below the Vaachak storage decision layer | existing low-level SD/MMC block I/O until repeated mount/list/read smoke passes |
| 5 | `storage_fat_algorithm_native_driver` | Critical | FAT directory traversal, open, read, and cache path access algorithms | existing FAT implementation until non-destructive read smoke is stable |

## Recommended first driver

Start with:

```text
input_physical_sampling_native_driver
```

Reason:

- it is visible immediately through button behavior
- it does not risk SD corruption
- it does not risk e-paper refresh lockups
- it builds directly on the accepted Vaachak-native input event pipeline

## Gates for every lower-level driver migration

Each physical driver migration must include:

- a Vaachak-owned native driver module
- a Pulp-compatible fallback path
- an explicit active backend selector
- a static validator
- a hardware smoke checklist
- a rollback gate
- no reader/file-browser UX changes unless the deliverable explicitly requests them
- no app navigation screen changes unless the deliverable explicitly requests them

## Rollback rules

Rollback must be possible by switching active backend selection back to `PulpCompatibility` without deleting the Vaachak-owned native module.

## Hardware smoke requirements

Every physical driver slice must validate:

- normal boot
- Home/category dashboard appears
- button navigation works
- file browser opens
- SD file listing works
- TXT/EPUB open paths still work
- Back navigation works
- display refresh remains stable
- no new SD/FAT/path errors

## Storage safety rule

The FAT native driver remains last. Destructive FAT operations stay denied until read-only, non-destructive access is stable through repeated hardware smoke.

## What this plan does not move

This plan does not move:

- physical ADC/GPIO sampling
- SSD1677 draw buffer algorithm
- SSD1677 waveform or BUSY wait behavior
- physical SD/MMC block driver
- low-level FAT algorithms
- physical SPI transfer
- chip-select GPIO toggling
- reader/file-browser UX
- app navigation screens

## SPI full migration checkpoint

- `spi_physical_native_driver_full_migration`: SPI physical-driver ownership moved fully to Vaachak.
