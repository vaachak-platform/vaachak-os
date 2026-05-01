# Phase 21 — Vaachak Storage Boundary Extraction

## Purpose

Phase 21 expands the Vaachak-owned storage boundary from a simple scaffold into typed metadata and pure helper functions while preserving the working imported Pulp reader runtime.

This phase is intentionally conservative. It does **not** move SD card initialization, SPI bus arbitration, FAT volume handling, EPUB cache IO, progress persistence, bookmark persistence, theme persistence, or reader file IO out of the imported runtime.

## Ownership boundary after Phase 21

Vaachak owns:

- X4 storage metadata constants.
- State directory naming expectations.
- State artifact naming expectations.
- Pure helper functions for future validation.
- Documentation for storage ownership and future extraction.
- The `phase21=x4-storage-boundary-ok` marker.

Imported Pulp/smol-epub still owns:

- SD card init.
- Shared SPI bus handling.
- FAT volume open/close behavior.
- TXT/MD read behavior.
- EPUB ZIP/OPF/HTML/chapter-cache behavior.
- Progress persistence behavior.
- Bookmark persistence behavior.
- Theme preset/state behavior.
- Continue behavior.

## Key file

```text
 target-xteink-x4/src/runtime/storage_boundary.rs
```

The storage boundary defines the following core facts:

```text
state/
state/<BOOKID>.PRG
state/<BOOKID>.BKM
state/<BOOKID>.THM
state/<BOOKID>.MTA
state/BMIDX.TXT
```

It also records that the X4 microSD uses GPIO12 for chip-select and shares the display SPI bus. This is metadata only in Phase 21.

## Phase marker

```text
phase21=x4-storage-boundary-ok
```
