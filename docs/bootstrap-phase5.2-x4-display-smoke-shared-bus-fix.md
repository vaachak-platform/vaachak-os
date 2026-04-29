# VaachakOS Bootstrap Phase 5.2 — X4 Display Smoke Shared-Bus Fix

Phase 5 booted and reported SSD1677 init/refresh success, but the X4 ePaper
panel retained the old `x4-reader-os-rs` Home image.

The most likely cause is shared SPI bus ownership: on X4, ePaper and SD share
SPI2, and SD_CS is GPIO12. The proven X4 runtime drives GPIO12 high before EPD
traffic. Phase 5 did not configure GPIO12, so the SD card could remain selected
or floating and interfere with EPD transfers while serial logs still appeared
successful.

Phase 5.2 therefore:

- forces SD_CS GPIO12 high before any SPI traffic
- uses 400 kHz safe SPI for the display smoke path
- chunks blocking SPI writes to avoid relying on large non-DMA transfers
- keeps Home/Files/Reader/SD migration out of scope

Expected serial marker:

```text
phase5.2=ssd1677-full-frame-smoke-sd-cs-high-ok
```

Expected screen:

```text
VAACHAKOS
X4 DISPLAY SMOKE
PHASE 5
480X800 PORTRAIT
BOOT OK
```
