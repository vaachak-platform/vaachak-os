# Phase 28 Notes

Phase 28 is intentionally a consolidation phase, not a behavior extraction phase.

Do not edit these for Phase 28:

```text
vendor/pulp-os/src/apps/reader/*
vendor/pulp-os/kernel/*
vendor/smol-epub/*
```

Do not move:

```text
physical SD init
physical SPI bus ownership
filesystem read/write calls
physical ADC sampling
input debounce/repeat logic
physical SSD1677 init
physical display refresh
strip rendering
```

A future phase can start moving one physical path at a time after this combined contract smoke remains stable on-device.
