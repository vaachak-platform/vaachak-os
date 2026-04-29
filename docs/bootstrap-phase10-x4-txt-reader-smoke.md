# VaachakOS Bootstrap Phase 10 — X4 TXT Reader Smoke

Phase 10 is the first reader-facing smoke test in `vaachak-os`.

It intentionally reuses the proven X4 transport shape from `x4-reader-os-rs`:

- SPI2 shared bus
- SD_CS held high during EPD operations
- EPD_CS held high during SD operations
- DMA-backed `SpiDevice` display path
- calibrated ADC ladder values passed directly into the X4 input model
- 10 ms polling cadence
- strip rendering, no full framebuffer

## In scope

- scan the SD card for files as in Phase 9
- Select on a TXT/MD file opens it
- read up to 1024 bytes
- render the first page using fixed bitmap text
- Back/Left returns to Library

## Out of scope

- EPUB reader migration
- pagination/scrolling
- progress persistence
- bookmarks
- themes
- full Home/Files/Reader app architecture

## Acceptance

Serial:

```text
phase10=x4-txt-reader-smoke-ready
phase10: reader read ok file=SHORT.TXT ...
phase10=x4-txt-reader-smoke-ok
```

Screen:

```text
VAACHAKOS
TXT READER
SHORT.TXT
<first lines of text>
BACK LIBRARY
SD OK      BAT 92
```
