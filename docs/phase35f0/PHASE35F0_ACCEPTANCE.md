# Phase 35F-0 Acceptance

Phase 35F-0 is accepted when:

- `target-xteink-x4/src/vaachak_x4/display/display_geometry_runtime.rs` exists.
- `target-xteink-x4/src/vaachak_x4/mod.rs` exports `display`.
- The facade validates native `800x480`, logical `480x800`, rotation, strip, and reader bounds.
- The active imported runtime calls only the facade preflight.
- Active SSD1677 init, refresh, strip rendering, and display SPI behavior remain imported Pulp-owned.
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits.
- Normal boot remains `vaachak=x4-runtime-ready`.
