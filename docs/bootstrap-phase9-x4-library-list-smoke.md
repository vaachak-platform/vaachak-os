# VaachakOS Bootstrap Phase 9 — X4 Minimal Library List Smoke

Phase 9 combines the proven X4 boot, shared SPI, SD/FAT, display, and input paths into a minimal Library list smoke test.

## Scope

In scope:

- mount SD card
- write/read the existing flat 8.3-safe smoke file
- scan `/BOOKS` first, then root as fallback
- accept `.TXT`, `.MD`, `.EPU`, and `.EPUB` entries
- render up to five supported files on the X4 ePaper display
- use X4 button input to move the selected row
- Select logs the selected file only

Out of scope:

- opening Reader
- parsing EPUB/TXT contents
- full Files app migration
- paging beyond the first visible entries
- title extraction

## Acceptance

Expected serial markers:

```text
phase9: library scan ok dir=BOOKS count=... total=...
phase9=x4-library-list-smoke-ready
phase9: input event #... button=Down kind=Press
phase9: redraw selected=1 file=...
phase9=x4-library-list-smoke-ok
phase9: select file=... idx=... size=...
```

Expected screen:

```text
VAACHAKOS
LIBRARY SMOKE

BOOKS          FILES N
■ BOOK1.TXT
  BOOK2.EPU
  BOOK3.EPUB

SELECT LOGS FILE
SD OK          BAT 92
```

## Notes

This is still a smoke test. It proves the library listing and navigation seam before moving any real Home/Files/Reader application code.
