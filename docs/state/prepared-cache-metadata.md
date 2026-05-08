# Prepared cache metadata ownership

This document records the Vaachak-owned pure model for prepared cache metadata.

## Compatibility contract

The active X4 runtime still owns SD reads, FCACHE binary page loading, prepared glyph rendering, reader open behavior, and progress/bookmark behavior. This extraction only adds pure models and helpers in `vaachak-core`.

Current compatibility points:

- Prepared cache root: `/FCACHE`
- Runtime-relative prepared cache root: `FCACHE`
- Per-book cache directory: `/FCACHE/<BOOKID>`
- `BOOKID` is treated as an 8-character hex identifier for cache compatibility.
- Metadata file: `META.TXT`
- Font index file: `FONTS.IDX`
- Page index file: `PAGES.IDX`
- Metadata keys currently understood: `book_id`, `source`, `page_count`, optional `kind`
- Font index keys currently understood: `Latin`, `Devanagari`
- Page index entries are cache-safe page file names such as `00000000.VRN`.
- Prepared page binary records use the `VRUN` page header currently consumed by the Pulp-derived runtime.

## Error classification

Prepared-cache error handling is intentionally split:

- Missing cache: safe fallback for regular TXT/EPUB. Do not show this as a reader header error.
- Malformed metadata/index/page/font data: diagnostic-worthy failure.
- Mismatched book ID: diagnostic-worthy failure.
- Too-large cache data: diagnostic-worthy failure.

## Non-goals

This slice does not move SD/file I/O, prepared glyph bitmap rendering, reader state-machine behavior, progress/bookmark behavior, or display refresh behavior.
