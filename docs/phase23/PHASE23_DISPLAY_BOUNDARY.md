# Phase 23 — Vaachak Display Boundary Extraction

## Goal

Phase 23 makes the Vaachak-owned display contract explicit without moving the working display implementation out of the imported X4/Pulp runtime.

This phase records:

- Xteink X4 e-paper pins.
- Shared SPI pins used by SSD1677 and SD.
- Native and logical display geometry.
- 270-degree portrait transform expectation.
- Strip-rendering assumptions.
- SSD1677 RAM and refresh command ownership notes.
- Clear flags proving physical display behavior has not moved yet.

Expected boot marker:

```text
phase23=x4-display-boundary-ok
```

## Non-goals

Phase 23 must not move or rewrite:

- SSD1677 init.
- SPI transactions.
- Display refresh/LUT behavior.
- Strip rendering.
- Framebuffer behavior.
- App/reader rendering logic.
- Any files under `vendor/pulp-os` or `vendor/smol-epub`.

## Files added or changed

```text
target-xteink-x4/src/runtime/display_boundary.rs
docs/phase23/PHASE23_DISPLAY_BOUNDARY.md
docs/phase23/PHASE23_ACCEPTANCE.md
docs/phase23/PHASE23_NOTES.md
scripts/check_reader_runtime_sync_phase23.sh
scripts/check_phase23_display_boundary.sh
scripts/revert_phase23_display_boundary.sh
```

The imported runtime remains the working source of truth. `display_boundary.rs` is a typed Vaachak-owned boundary contract, not a hardware driver.
