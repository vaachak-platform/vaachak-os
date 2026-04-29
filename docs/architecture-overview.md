# VaachakOS Bootstrap Architecture Overview

This skeleton now reflects the first HAL refinement pass against the real X4 code:

- `x4-reader-os-rs` remains the X4 proving-ground
- `vaachak-os` becomes the clean architecture repo
- structure follows `core + hal + target`
- product scope remains reading-first
- HAL traits now mirror the current X4 realities more closely:
  - logical portrait display over a rotated native panel
  - strip rendering with a concrete strip height seam
  - input hold/reset behavior
  - battery under the power boundary even though X4 currently samples via the same ADC block as buttons
  - two-stage SD lifecycle: probe -> mount -> flush/close

## Crate intent

### `core/`
Shared traits, services, UI stack, and reader-facing contracts.

### `hal-xteink-x4/`
Concrete X4 implementation of display/input/power/storage traits.

### `target-xteink-x4/`
The boot/runtime entrypoint for the X4 build once real embedded startup is wired in.

## First extraction target

The first real code slice to extract from `x4-reader-os-rs` should be:
- input abstraction
- display abstraction
- storage abstraction
- power abstraction

Not:
- current runtime orchestration
- reader rendering internals
- feature-heavy UX
