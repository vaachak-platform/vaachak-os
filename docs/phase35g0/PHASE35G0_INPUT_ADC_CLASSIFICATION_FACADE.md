# Phase 35G-0 - Input ADC Classification Facade

Phase 35G-0 adds a Vaachak-owned pure facade for Xteink X4 input ADC ladder classification.

It models:

- GPIO1 row classification for Right, Left, Confirm, and Back.
- GPIO2 row classification for VolDown and VolUp.
- GPIO3 as the power button contract.
- Oversample, debounce, long-press, and repeat timing policy values used by the working runtime.

This phase does not read ADC hardware. It does not call `read_oneshot`, own `InputDriver`, run debounce loops, emit events, or replace the imported Pulp input task.

The active runtime calls only a silent preflight that validates classification math and timing policy constants.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```
