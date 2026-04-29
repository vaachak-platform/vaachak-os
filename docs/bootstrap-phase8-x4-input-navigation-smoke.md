# VaachakOS Bootstrap Phase 8 — X4 Input Navigation Smoke

Status: implementation pack

## Purpose

Phase 8 proves that VaachakOS can read the Xteink X4 button hardware and use it
to drive a visible on-device UI state change.

This phase builds directly on:

- Phase 4 boot + serial smoke
- Phase 5 SSD1677 display smoke
- Phase 6 SD/FAT storage smoke
- Phase 7 minimal Home screen parity

## Hardware validated

- GPIO1 ADC ladder: row 1 buttons
- GPIO2 ADC ladder: row 2 buttons
- GPIO3 power button as active-low input
- ADC1 calibrated 11 dB sampling path
- existing DMA SPI display path remains intact
- existing shared SPI SD smoke remains intact

## UI behavior

The minimal Home screen remains intentionally small:

```text
VAACHAKOS
INPUT NAV SMOKE

■ CONTINUE
  LIBRARY
  SETTINGS
  SYSTEM

UP DOWN MOVE
SELECT LOGS ITEM

SD OK                    BAT 92
```

Button behavior:

| Button | Phase 8 behavior |
| --- | --- |
| Up | move selection up |
| Down | move selection down |
| Left | move selection up |
| Right | move selection down |
| Select | log selected item only |
| Back | log only |
| Power | log only |

## Acceptance markers

Host checks should pass:

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

X4 flash should show:

```text
phase8=x4-input-navigation-smoke-ready
phase8: input event #... button=Down kind=Press
phase8: redraw selected=1 item=Library
phase8=x4-input-navigation-smoke-ok
```

## Deferrals

Phase 8 intentionally does not implement Files, Continue, Reader, Settings, or
sleep. It only proves the input pipeline can drive Home selection state.
