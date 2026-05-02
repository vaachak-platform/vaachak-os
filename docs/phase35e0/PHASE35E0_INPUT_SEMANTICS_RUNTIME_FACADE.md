# Phase 35E-0 - Input Semantics Runtime Facade

Phase 35E-0 adds a Vaachak-owned runtime facade for input semantic mapping.

It models the same physical-button to action mapping used by the active imported runtime:

- `VolDown` to `Next`
- `VolUp` to `Previous`
- `Right` to `NextJump`
- `Left` to `PreviousJump`
- `Confirm` to `Select`
- `Back` to `Back`
- `Power` to `Menu`

It also models the swapped-button layout used for left-handed mode:

- `Right` to `Select`
- `Left` to `Back`
- `Confirm` to `NextJump`
- `Back` to `PreviousJump`

The active runtime calls a silent, allocation-free preflight for this facade during early boot. This proves the Vaachak-owned semantic mapper can be reached without changing button behavior.

Phase 35E-0 does not move ADC sampling, debounce, repeat, long-press, the Pulp `ButtonMapper`, or `tasks::input_task`.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```
