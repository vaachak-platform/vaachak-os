# VaachakOS Bootstrap Phase 13 — Recursive All-Files Library

## Goal

Replace the `/BOOKS`-only smoke library with an all-files library scan suitable for the next reader parity work.

## Scope

Implemented:

- Scan SD root recursively.
- Include supported reader files from root and nested folders.
- Supported extensions: `.TXT`, `.MD`, `.EPU`, `.EPUB`.
- Store both short display name and path-aware library entry.
- Render file kind plus path on the Library smoke screen.
- Keep TXT/MD reader open path working.
- List EPUB/EPU entries but leave opening deferred.

Deferred:

- EPUB reader rendering.
- Directory pagination beyond the fixed smoke-list capacity.
- Long filename support.
- Global Home bookmarks screen.
- Full reader menu consolidation.

## Expected serial markers

```text
phase13: recursive scan start root + nested folders
phase13: recursive scan visible=... total=...
phase13: file[0]=BOOKS/LONG.TXT kind=Txt size=...
phase13=x4-recursive-library-smoke-ready
phase13=x4-recursive-library-smoke-ok
```

## Implementation notes

This phase remains faithful to the working `x4-reader-os-rs` hardware lessons:

- calibrated ADC ladder input values are used directly
- 10 ms input polling cadence
- shared SPI with explicit SD/EPD chip-select separation
- DMA-backed SSD1677 display path
- flat 8.3-safe state files for progress/bookmarks

The main new behavior is recursive path discovery. Progress and bookmark file IDs now hash the path-aware library entry rather than only the short file name.
