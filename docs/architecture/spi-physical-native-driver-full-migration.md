# SPI Physical Native Driver Full Migration

`spi_physical_native_driver_full_migration` is the full SPI migration checkpoint for Vaachak OS on Xteink X4.

## Ownership state

Vaachak now owns the SPI physical-driver behavior surface in `target-xteink-x4`:

- SPI bus identity and X4 pin map
- SCLK GPIO8, MOSI GPIO10, MISO GPIO7
- display chip-select GPIO21
- SD chip-select GPIO12
- storage probe clock: `400 kHz`
- operational clock: `20 MHz`
- display transaction routing
- storage transaction routing
- chip-select policy
- transaction lifecycle
- transfer request/result construction
- backend selection

The active SPI backend is:

```text
VaachakNativeSpiPhysicalDriver
```

The imported Pulp SPI runtime is no longer the active SPI owner for this boundary.

## Remaining target boundary

The only remaining non-Vaachak piece is the unavoidable ESP32-C3 target HAL peripheral call that clocks bytes on hardware.
That is a target HAL boundary, not a Pulp ownership boundary.

## Explicitly not changed

This deliverable does not change:

- SSD1677 draw buffer algorithm
- SSD1677 waveform handling
- SD/MMC block-driver algorithm
- low-level FAT algorithm
- reader/file-browser UX
- app navigation screens
- input behavior

## Runtime expectation

Hardware behavior should remain visually unchanged after this migration. The expected validation result is:

```text
spi_physical_native_driver_full_migration=ok
```

## Hardware smoke

After flashing, validate:

- boot succeeds
- Home/category dashboard appears
- full and partial display refresh still work
- SD file listing works
- TXT/EPUB still open
- buttons still work
- no SD mount/probe regression
- no display stuck/blank refresh
