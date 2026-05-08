# Hardware Native Behavior Consolidation Cleanup

Marker: `hardware_native_behavior_consolidation_cleanup=ok`

This is the final cleanup checkpoint for the accepted native behavior consolidation.

## What is finalized

The cleanup checkpoint confirms that these accepted Vaachak-native behavior migrations are consolidated and ready for commit:

- input event pipeline behavior
- display refresh command selection behavior
- storage SD/MMC/FAT command decision behavior

The canonical behavior map remains `docs/architecture/hardware-native-behavior-consolidation.md`.

## Active backend posture

- Behavior owner: `target-xteink-x4 Vaachak layer`
- Native behavior stack: `InputEventPipeline+DisplayRefreshCommandExecutor+StorageSdMmcFatExecutor`
- Low-level fallback: `PulpCompatibility`

## Guardrails preserved

This cleanup does not move or rewrite:

- physical ADC/GPIO sampling
- SSD1677 draw buffer algorithm
- SSD1677 waveform/BUSY wait behavior
- physical SD/MMC block driver
- low-level FAT algorithms
- physical SPI transfer
- chip-select GPIO toggling
- reader/file-browser UX
- app navigation screens

## Cleanup artifact policy

The cleanup script removes temporary overlay/fix artifacts for the native behavior consolidation checkpoint. It does not remove committed source, docs, or validators.
