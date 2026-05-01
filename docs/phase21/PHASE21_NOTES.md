# Phase 21 Refactor Notes

Phase 21 intentionally avoids changing behavior. It creates a Vaachak-owned storage boundary that can be used in later phases to move one storage responsibility at a time.

## Do not move yet

Do not move the following in Phase 21:

- `SdCard::new`
- SPI/DMA setup
- SD chip-select behavior
- FAT volume management
- EPUB cache reads/writes
- progress writes
- bookmark writes
- theme writes
- reader file open/read/close behavior

## Likely Phase 22 candidate

The next safe step is probably a state-layout validation/reporting phase, not physical SD extraction. That would allow Vaachak to assert state file conventions before taking ownership of any IO.
