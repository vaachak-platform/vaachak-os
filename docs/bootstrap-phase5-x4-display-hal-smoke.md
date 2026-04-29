# VaachakOS Bootstrap Phase 5 — X4 Display HAL Smoke

## Goal

Prove that the new `vaachak-os` target can initialize the Xteink X4 SSD1677 ePaper display and push a single full-frame strip-rendered smoke screen.

This phase intentionally does **not** migrate Home, Files, Reader, SD/FAT, or the full runtime from `x4-reader-os-rs`.

## What changed

- Adds `hal-xteink-x4::display_smoke::X4Ssd1677Smoke`.
- Uses the proven X4 display pins:
  - GPIO8 SCLK
  - GPIO10 MOSI
  - GPIO7 MISO, reserved on shared bus
  - GPIO21 EPD CS
  - GPIO4 DC
  - GPIO5 RST
  - GPIO6 BUSY
- Preserves the proven logical geometry:
  - native 800x480
  - logical portrait 480x800
  - Deg270 mapping
  - 40-row strips
- Uses no framebuffer.
- Draws one full-screen smoke pattern using a 4 KB strip buffer.

## Expected serial markers

```text
VaachakOS X4 display smoke starting
phase=bootstrap-phase5-x4-display-hal-smoke
phase5: configuring X4 SPI2 + SSD1677 pins
phase5: display init start
phase5: display init complete busy=...
phase5: full-frame smoke draw start
phase5: full-frame smoke refresh complete busy=...
VaachakOS X4 display smoke complete
phase5=ssd1677-full-frame-smoke-ok
```

## Expected screen

The ePaper display should refresh away from the retained old Home image and show a portrait smoke screen containing:

```text
VAACHAKOS
X4 DISPLAY SMOKE
PHASE 5
480X800 PORTRAIT
BOOT OK
```

## Important notes

- The refresh can take several seconds.
- This phase does not mount SD.
- This phase does not run Home/Files/Reader.
- If serial succeeds but the screen does not update, the next fix should stay inside `display_smoke.rs` / pin or SSD1677 sequencing.
