# HAL Traits Refined Against Real X4 Code

This note captures the first HAL refinement pass against the current `x4-reader-os-rs` proving-ground.

## Key decisions

### Display
- keep display as strip-rendered, no framebuffer
- expose **logical** geometry, not just native panel geometry
- include rotation and strip-row count in the display seam
- keep panel init as a display-HAL responsibility, but keep SPI-speed choreography in the target bootstrap path

### Input
- keep non-blocking `poll()`
- keep an explicit `reset_hold_state()` seam because the current X4 runtime already relies on a hold-consumption behavior
- keep button IDs abstracted to `Up/Down/Left/Right/Select/Back/Power`

### Power
- keep battery under the power boundary for the long-term architecture
- acknowledge that the current X4 implementation samples battery from the same ADC block used by button ladders
- allow the X4 HAL to solve that internally without leaking ADC coupling into `core`

### Storage
- refine storage around the real X4 lifecycle:
  1. low-speed card probe
  2. filesystem mount
  3. normal open/read/write operations
  4. flush/close before sleep or halt
- do not hide this lifecycle behind a single monolithic `sd_present()` method

## Immediate implication

The first extraction slice should focus on:
- display geometry + strip contract
- input event contract + hold reset seam
- battery/power seam
- two-stage storage lifecycle

Not on:
- reader rendering
- app orchestration
- sync or network features
