# Phase 35E-0 Notes

Phase 35E-0 is intentionally semantic-only.

The current active input path remains:

```text
InputDriver -> tasks::input_task -> ButtonMapper -> AppManager
```

This preserves the hardware-validated behavior while giving Vaachak-owned code a precise runtime vocabulary for future extraction.

The later physical input extraction must be separate and hardware-gated. It should move ADC ladder classification, debounce, long-press, and repeat policy only after semantic mapping is stable.
