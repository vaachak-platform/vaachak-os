# Hardware Native Behavior Consolidation

This document is the canonical checkpoint for the accepted Vaachak-native hardware behavior migrations.

Cleanup checkpoint: `hardware_native_behavior_consolidation_cleanup=ok`

## Consolidated native behavior now owned by Vaachak

| Area | Vaachak-owned behavior | Still Pulp-compatible |
| --- | --- | --- |
| Input | Button normalization, stable-state tracking, debounce metadata, press/release/repeat classification, navigation-intent mapping | Physical ADC/GPIO sampling and final app navigation dispatch |
| Display | Refresh command selection, full/partial handoff construction, partial-to-full escalation, clear/sleep/render classification | SSD1677 draw buffer logic, waveform handling, BUSY wait, physical SPI transfer, chip-select GPIO toggling |
| Storage | SD/MMC/FAT command decisions, media-state interpretation, probe/mount lifecycle intent selection, FAT operation classification, path-role policy, destructive operation denial | Physical SD/MMC block driver, physical card I/O, low-level FAT algorithms, physical SPI transfer, chip-select GPIO toggling |

## Active stack

- Consolidation marker: `hardware_native_behavior_consolidation=ok`
- Cleanup marker: `hardware_native_behavior_consolidation_cleanup=ok`
- Behavior owner: `target-xteink-x4 Vaachak layer`
- Active native behavior stack: `InputEventPipeline+DisplayRefreshCommandExecutor+StorageSdMmcFatExecutor`
- Low-level fallback: `PulpCompatibility`

## Guardrails

This consolidation does not move:

- physical ADC/GPIO sampling
- SSD1677 draw buffer algorithm
- SSD1677 waveform/BUSY wait behavior
- physical SD/MMC block driver
- low-level FAT algorithms
- physical SPI transfer
- chip-select GPIO toggling
- reader/file-browser UX
- app navigation screens

## Relationship to previous checkpoints

This document consolidates these accepted checkpoints:

- `input-backend-native-event-pipeline.md`
- `input-backend-native-event-pipeline-cleanup.md`
- `display-backend-native-refresh-command-executor.md`
- `display-backend-native-refresh-command-executor-cleanup.md`
- `storage-backend-native-sd-mmc-fat-executor.md`
- `storage-backend-native-sd-mmc-fat-executor-cleanup.md`

## Cleanup checkpoint

The cleanup checkpoint adds:

- `hardware_native_behavior_consolidation_cleanup=ok`
- `hardware-native-behavior-consolidation-cleanup.md`
- `validate_hardware_native_behavior_consolidation_cleanup.sh`
- `cleanup_hardware_native_behavior_consolidation_artifacts.sh`

The cleanup checkpoint does not add any lower-level driver migration. Next lower-level migrations should be treated as separate physical-driver work and should not be mixed with this behavior consolidation checkpoint.

## Next lower-level driver migration checkpoint

The accepted native behavior consolidation now points to
`docs/architecture/physical-driver-migration-plan.md` for the lower-level
physical driver migration order. The first recommended native physical driver
slice is `input_physical_sampling_native_driver`; destructive FAT migration
remains last.
