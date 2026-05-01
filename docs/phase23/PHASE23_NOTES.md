# Phase 23 Notes

Phase 23 is intentionally conservative.

The Vaachak-owned boundary now records the display contract, but the real display path remains in the imported X4/Pulp runtime. This avoids regressing the known-good SSD1677 path.

## Current ownership

```text
Display behavior owner: vendor/pulp-os imported runtime
EPUB render path:       vendor/pulp-os + vendor/smol-epub
Vaachak Phase 23 owns:  typed display metadata and checks only
```

## Why this matters

Earlier X4 work showed that small changes in SSD1677 geometry, rotation, strip rendering, or shared SPI ownership can break display output. Phase 23 creates guardrails before any physical display code is extracted.

## Future extraction candidates

Future phases can extract one behavior at a time:

1. Display configuration structs.
2. SSD1677 pin construction.
3. Rotation/geometry transform.
4. Strip rendering abstraction.
5. Refresh command wrapper.
6. Shared SPI arbitration boundary with SD.

Each extraction should preserve the Phase 16 reader parity baseline.
