# Phase 25 — Storage State Contract Smoke

## Goal

Make the Vaachak-owned storage boundary testable with a non-invasive state contract smoke.

This phase does **not** move physical SD/SPI/filesystem ownership away from the imported Pulp runtime. It only records and validates the state-file contract that Vaachak expects to own over time.

## Added module

```text
target-xteink-x4/src/runtime/storage_state_contract.rs
```

The module defines `VaachakStorageStateContract` and helper methods for:

```text
state/ directory ownership expectations
8.3-safe per-book state file names
book ID validation
known state extensions: PRG, BKM, THM, MTA
reserved state file: BMIDX.TXT
EPUB cache ownership boundary
non-movement of physical SD/FAT/EPUB-cache IO in Phase 25
```

## Boot marker

```text
phase25=x4-storage-contract-smoke-ok
```

The marker is emitted only when the in-firmware smoke validator passes.

## Non-goals

Phase 25 does not move:

```text
SD card initialization
SPI bus ownership
FAT directory/file calls
reader progress writes
bookmark writes
EPUB cache reads/writes
```

Those behaviors remain in the imported Pulp runtime until a later extraction phase.
