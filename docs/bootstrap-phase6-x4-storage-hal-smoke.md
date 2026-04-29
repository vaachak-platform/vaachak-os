# VaachakOS Bootstrap Phase 6 — X4 Storage HAL Smoke

Phase 6 proves the Xteink X4 SD-card path inside the new VaachakOS target crate without migrating Home, Files, Reader, or the full X4 runtime.

## Scope

This phase performs a storage-only smoke test:

- keeps EPD CS high so the display is not selected
- uses GPIO12 as raw SD CS
- sends 80 idle clocks with CS high
- initializes SD over SPI2 at 400 kHz using DMA buffers
- opens FAT volume 0 and root directory
- creates/opens `state/`
- writes `state/VOSMOKE.TXT`
- reads it back and validates content
- closes file, directory, and volume cleanly

## Out of scope

- Home / Files / Reader migration
- reusable file browser
- EPUB cache migration
- shared display+storage runtime arbitration beyond the single storage smoke path
- 20 MHz SD runtime speed-up

## Acceptance

Serial output reaches:

```text
phase6: sd init start
phase6: sd mounted volume=0 root=open
phase6: wrote state/VOSMOKE.TXT
phase6: readback ok state/VOSMOKE.TXT
phase6=x4-storage-hal-smoke-ok
```

The ePaper display is not expected to change in this phase.
