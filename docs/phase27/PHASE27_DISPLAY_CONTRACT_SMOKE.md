# Phase 27 — Display Contract Smoke

Phase 27 adds a Vaachak-owned display contract smoke layer without moving physical display behavior away from the imported Pulp/X4 runtime.

## What Phase 27 owns

- Display contract metadata validation.
- X4 panel geometry contract: native `800x480`, logical portrait `480x800`.
- Rotation contract: `270` degrees.
- Strip rendering assumption: `40` rows.
- SSD1677 command contract: current RAM `0x24`, previous RAM `0x26`, display update control `0x22`, master activate `0x20`.
- X4 display pins: CS `GPIO21`, DC `GPIO4`, RST `GPIO5`, BUSY `GPIO6`.
- Shared SPI pins: SCLK `GPIO8`, MOSI `GPIO10`, MISO `GPIO7`, SD CS `GPIO12`.
- Boot marker: `phase27=x4-display-contract-smoke-ok`.

## What Phase 27 does not move

- SSD1677 initialization.
- SPI transactions.
- Display refresh.
- Framebuffer or strip rendering behavior.
- Reader UI rendering.
- EPUB/TXT reader behavior.

Those remain owned by the imported Pulp runtime.

## Files

- `target-xteink-x4/src/runtime/display_contract_smoke.rs`
- `scripts/check_phase27_display_contract_smoke.sh`
- `scripts/check_reader_runtime_sync_phase27.sh`
