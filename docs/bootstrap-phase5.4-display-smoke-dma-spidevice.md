# VaachakOS Bootstrap Phase 5.4 — Display Smoke DMA SpiDevice Fix

Phase 5.3 proved that the standalone display smoke driver was treating BUSY as a hard pre-write blocker. The proven `x4-reader-os-rs` SSD1677 path does not wait for BUSY after init before writing RAM; it writes RED then BW RAM using a DMA-backed `SpiDevice` and waits after the update command.

This phase ports that shape more closely:

- keep SD_CS GPIO12 high
- issue 80 idle clocks with CS high before DMA conversion
- use SPI2 with DMA buffers sized for a 40-row strip
- use `embedded_hal_bus::spi::ExclusiveDevice` for EPD CS ownership
- write RED then BW RAM before full refresh
- remove the pre-write BUSY abort

Acceptance:

- serial reaches `phase5.4=ssd1677-full-frame-smoke-dma-spidevice-ok`
- ePaper changes away from the retained old Home screen
- screen shows the Phase 5 smoke text or at least visible black/white changes

If the serial marker is printed but the display still does not change, the next step is a direct file-level port of `kernel/src/drivers/ssd1677.rs` plus the exact `StripBuffer` renderer from `x4-reader-os-rs`.
