# First Real X4 Slice into vaachak-os

This bundle extracts only the X4 bootstrap seam needed to start the future `vaachak-os` repo without disturbing `x4-reader-os-rs`.

## Included

- storage bootstrap lifecycle (`init_card -> mount -> flush_and_close`)
- display bootstrap lifecycle (`init -> begin_frame/draw_strip/end_frame -> sleep`)
- input ladder threshold model for the currently proven X4 button rows
- power model for battery ADC -> battery mV -> percentage conversion
- host-friendly boot sequence in `target-xteink-x4`

## Not included

- `Board::init`
- `InputDriver`
- `Kernel`
- `AppManager`
- reader/file browser behavior
- worker tasks
- actual esp-hal peripherals

## Extraction rule

`x4-reader-os-rs` remains the hardware truth source until this slice compiles cleanly and the embedded target boot path is proven.
