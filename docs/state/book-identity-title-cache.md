# Book identity and title-cache ownership

This document records the Vaachak-owned pure model for book identity and title-cache data.

## Current compatibility contract

The active X4 runtime still owns SD access, file browsing, reader opening, prepared-cache opening, and title-cache I/O. This extraction only adds pure models and parse/serialize helpers in `vaachak-core`.

Current SD compatibility points:

- Title cache directory: `_x4`
- Runtime title cache file: `_x4/TITLES.BIN`
- Host-side title map file: `_x4/TITLEMAP.TSV`
- `TITLES.BIN` record format: `FILENAME<TAB>Display Title<LF>`
- Runtime title cap: 64 bytes
- Runtime record safety target: 128 bytes
- Runtime file key target: 8.3-style file name, up to 13 bytes including dot

## Non-goals

This extraction does not move SD reads/writes, file browsing, reader open behavior, prepared cache behavior, EPUB metadata extraction, or display rendering.
