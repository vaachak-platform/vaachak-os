# Phase 35D-2 Acceptance

Phase 35D-2 is accepted when:

- the active runtime calls `active_runtime_preflight()` before heap setup
- the active runtime calls `active_runtime_alloc_preflight()` only after heap setup
- the pre-heap storage preflight does not call the reader-state bridge
- the pre-heap storage preflight does not use allocation types or helpers
- direct `AppManager`, `InputDriver`, `ButtonMapper`, and `input_task` behavior remains present
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits
- release build succeeds for `riscv32imc-unknown-none-elf`

This phase does not move state IO, reader app internals, buttons, SD/SPI, or display behavior.
