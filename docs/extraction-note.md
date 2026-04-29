# Extraction Note: First Real X4 Slice

This repo revision extracts only the first real X4 bootstrap slice from the current `x4-reader-os-rs` proving-ground.

## Source behaviors mirrored

- portrait logical UI over native 800x480 panel
- strip-based display model with 40-row strips
- shared SD/EPD SPI topology
- SD probe at low speed before runtime speed-up
- X4 button ladder thresholds
- battery ADC -> battery mV -> battery percentage mapping

## Source behaviors intentionally left behind for now

- real esp-hal peripheral ownership and DMA setup
- real `Board::init`
- real `InputDriver` debounce/long-press/repeat queue
- `Kernel`, `AppManager`, and all reader/file-browser behavior
- worker tasks and scheduler integration

## Exit criterion for this slice

This slice is considered ready when the new `vaachak-os` workspace can compile with the extracted storage/display/input/power boundaries and the embedded X4 target can boot the bootstrap path cleanly.
