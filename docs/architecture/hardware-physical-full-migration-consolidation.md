# Hardware Physical Full Migration Consolidation

This is the canonical Vaachak OS checkpoint for the lower-level hardware migration stack.

## Consolidated native ownership

The following accepted migrations are consolidated under one map:

- `spi_physical_native_driver_full_migration=ok`
- `display_physical_ssd1677_full_migration=ok`
- `storage_physical_sd_mmc_full_migration=ok`
- `storage_fat_algorithm_full_migration=ok`
- `input_physical_sampling_native_driver=ok`

## Active native backends

| Domain | Active backend | Pulp runtime selected? |
| --- | --- | --- |
| SPI | `VaachakNativeSpiPhysicalDriver` | No |
| Display SSD1677 | `VaachakNativeSsd1677PhysicalDriver` | No |
| Storage SD/MMC physical | `VaachakNativeSdMmcPhysicalDriver` | No |
| FAT algorithm | `VaachakNativeFatAlgorithmDriver` | No |
| Input physical sampling interpretation | `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback` | Only ADC/GPIO read fallback remains |

## What is now Vaachak-owned

- SPI bus identity, pin map, transaction lifecycle, display/storage routing, and chip-select policy
- SSD1677 command sequencing, refresh lifecycle, RAM-window state, reset/DC/BUSY policy, and native SPI request construction
- SD/MMC card lifecycle sequencing, media-state interpretation, probe/init/mount policy, storage availability, and block-device request construction
- FAT/path/list/open/read/write policy, BPB parsing, directory entry decoding, long filename assembly, cluster-chain traversal, metadata policy, and destructive operation authorization
- X4 button ADC ladder interpretation, oversample reduction, power-button low-active interpretation, and conversion into the Vaachak native input event pipeline

## Intentional remaining boundaries

The only remaining non-Vaachak execution boundaries are target HAL peripheral calls and the temporary input ADC/GPIO physical read fallback. These are not active Pulp ownership of SPI, display, SD/MMC, or FAT behavior.

## Behavior preserved

This consolidation does not change:

- reader/file-browser UX
- app navigation screens
- display layout/rendering expectations
- storage content semantics
- SD card file expectations

## Acceptance marker

```text
hardware_physical_full_migration_consolidation=ok
```
