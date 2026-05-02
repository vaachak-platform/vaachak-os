# Phase 35E-0 Acceptance

Phase 35E-0 is accepted when:

- `target-xteink-x4/src/vaachak_x4/input/input_semantics_runtime.rs` exists.
- `target-xteink-x4/src/vaachak_x4/mod.rs` exports `input`.
- The facade defines Vaachak-owned physical buttons, runtime input actions, events, and a mapper.
- The facade validates default and swapped mappings.
- The active imported runtime calls only the facade preflight.
- The active imported runtime still uses Pulp `InputDriver`, `ButtonMapper`, and `tasks::input_task`.
- No ADC, debounce, repeat, or physical button sampling code moves.
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits.
- Normal boot remains `vaachak=x4-runtime-ready`.
