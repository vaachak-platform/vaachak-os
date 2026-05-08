# Storage path helpers ownership

This document records the Vaachak-owned pure storage/path helper model.

The active X4 runtime still owns SD card probing, FAT access, file reads/writes, directory scanning, and all hardware behavior. This slice only adds pure path helpers and compatibility tests in `vaachak-core`.

## Preserved layout

- Current library root: `/`
- Optional books root: `/books`
- Reader state directory: `state`
- Reader progress: `state/<BOOKID>.PRG`
- Reader bookmarks: `state/<BOOKID>.BKM`
- Bookmark index: `state/BMIDX.TXT`
- Prepared cache root: `/FCACHE`
- Prepared cache book folder: `/FCACHE/<BOOKID>`
- Device settings: `/_x4/SETTINGS.TXT`
- Title cache: `/_x4/TITLES.BIN`
- Sleep root: `/sleep`
- Daily sleep images: `/sleep/daily`
- Sleep image mode file: `/SLPMODE.TXT`

## Compatibility rules

- Book IDs are normalized to 8 uppercase hexadecimal characters.
- Current reader state files intentionally remain relative under `state/` to match the runtime convention.
- FCACHE, settings, title-cache, and sleep paths remain absolute-style SD paths.
- Path traversal segments are rejected by pure helpers.
- 8.3-safe file-name checks accept current short-name patterns such as `YEARLY_H.TXT`, `ALICES~1.EPU`, `SETTINGS.TXT`, and `TITLES.BIN`.

## Non-goals

This extraction does not move SD mount/probe behavior, FAT I/O, directory scanning, SPI arbitration, reader open behavior, prepared-cache loading, or display rendering.
