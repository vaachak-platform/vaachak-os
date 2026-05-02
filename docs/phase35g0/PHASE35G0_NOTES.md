# Phase 35G-0 Notes

This phase is classification-only.

The active physical input path remains:

```text
Pulp InputDriver -> ADC sample/read -> debounce/repeat -> input task -> ButtonMapper
```

Vaachak now owns a pure copy of the ladder classification policy. A later hardware-gated phase can move the sampler/debounce state machine only after this policy has been validated on repeated flashes.
