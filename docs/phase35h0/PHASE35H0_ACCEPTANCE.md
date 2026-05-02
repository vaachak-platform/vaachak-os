# Phase 35H-0 Acceptance

Phase 35H-0 is accepted when:

- `target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs` exists.
- `target-xteink-x4/src/vaachak_x4/mod.rs` exports `physical`.
- The facade validates shared SPI pins and display/SD chip-select pins.
- The facade validates slow SD probe timing, fast operational timing, and DMA buffer contracts.
- The facade rejects simultaneous display and SD chip-select ownership.
- The active imported runtime calls only the pure preflight.
- Active runtime still owns physical SPI setup, bus device construction, SD init, and display init through imported Pulp code.
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits.
- Normal boot remains `vaachak=x4-runtime-ready`.
