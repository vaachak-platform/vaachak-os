# Display Physical SSD1677 Full Migration

`display_physical_ssd1677_full_migration` moves the SSD1677 physical display driver ownership into the Vaachak `target-xteink-x4` layer.

## Active backend

- Active display backend: `VaachakNativeSsd1677PhysicalDriver`
- Transport backend: `VaachakNativeSpiPhysicalDriver`
- Pulp display fallback: `false`
- Imported Pulp SSD1677 runtime active for this boundary: `false`

## Vaachak-owned behavior

Vaachak now owns:

- SSD1677 command sequencing
- full refresh lifecycle policy
- partial refresh lifecycle policy
- clear-frame lifecycle policy
- sleep lifecycle policy
- RAM-window state tracking
- display state tracking
- reset policy
- DC/RST/BUSY pin policy
- BUSY timeout/poll policy
- native SPI display transaction request construction

## Native SPI dependency

This migration depends on the accepted `spi_physical_native_driver_full_migration` boundary. Display transport is routed through `VaachakSpiPhysicalNativeDriver`; Pulp SPI transfer/chip-select fallback is not selected for this boundary.

## What is intentionally unchanged

This migration must not change:

- reader/file-browser UX
- app navigation screens
- storage behavior
- input behavior
- SD/MMC/FAT behavior

The only remaining hardware boundary is target HAL access for pin/SPI peripherals. That is not Pulp ownership.

## Validation marker

```text

display_physical_ssd1677_full_migration=ok
```
