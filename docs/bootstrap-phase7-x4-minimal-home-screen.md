# VaachakOS Bootstrap Phase 7 — Minimal X4 Home Screen Parity

## Purpose

Phase 7 combines the first three hardware foundations proven in VaachakOS:

- Phase 4: ESP32-C3 boot + serial smoke
- Phase 5: SSD1677 display smoke using the DMA `SpiDevice` path
- Phase 6: SD/FAT storage smoke using the shared SPI bus

This phase renders a minimal static Home screen on the Xteink X4 after proving SD write/readback. It is intentionally **not** the full Home/Files/Reader app migration yet.

## Scope

In scope:

- boot the real X4 target
- initialise the shared SPI bus once
- prove SD/FAT read/write with `state/VOSMOKE.TXT`
- switch the shared SPI bus to the SSD1677 display
- render a minimal VaachakOS Home screen
- show SD status and battery percentage from the current model path

Out of scope:

- input navigation
- Files/Library browser
- Reader migration
- Continue/resume wiring
- settings persistence
- full app shell/runtime migration

## Expected serial markers

```text
VaachakOS X4 minimal home starting
phase=bootstrap-phase7-x4-minimal-home-screen
phase7: storage smoke ok
phase7: minimal home refresh complete busy=false
phase7=x4-minimal-home-screen-ok
```

## Expected screen

```text
VAACHAKOS
BOOTSTRAP HOME

■ CONTINUE
  LIBRARY
  SETTINGS
  SYSTEM

READER MIGRATION NEXT

SD OK                         BAT 92
```

The display is rendered with the same 480x800 logical portrait mapping and full-frame strip writes used in Phase 5.4.

## Why this phase matters

After this checkpoint, VaachakOS has a real X4 firmware path that can:

1. boot,
2. talk to SD/FAT,
3. switch the shared SPI bus safely,
4. refresh the ePaper display, and
5. present a product-shaped Home surface.

The next recommended phase is `Bootstrap Phase 8 — X4 Input Navigation Smoke`.
