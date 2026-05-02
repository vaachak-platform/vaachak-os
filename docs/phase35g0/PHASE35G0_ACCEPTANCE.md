# Phase 35G-0 Acceptance

Phase 35G-0 is accepted when:

- `target-xteink-x4/src/vaachak_x4/input/input_adc_runtime.rs` exists.
- The Vaachak input module exports `input_adc_runtime`.
- The facade validates row1 and row2 ADC ladder centers and tolerances.
- The facade records timing policy values without taking over timing loops.
- The active imported runtime calls only the pure preflight.
- Active runtime still uses Pulp `InputDriver`, `ButtonMapper`, and `tasks::input_task`.
- No ADC reads, debounce loops, repeat loops, or power GPIO reads move to Vaachak-owned active code.
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits.
- Normal boot remains `vaachak=x4-runtime-ready`.
