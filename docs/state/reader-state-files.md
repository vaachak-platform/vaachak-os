# Reader State File Ownership

This document records the Vaachak-owned pure model layer for current reader progress and bookmark state.

The active SD/file I/O path remains in the Pulp-derived runtime. This extraction only adds pure models, parse helpers, serialize helpers, constants, and compatibility tests in `vaachak-core`.

## Compatibility files

Current on-card compatibility paths remain unchanged:

- `state/<BOOKID>.PRG` for progress
- `state/<BOOKID>.BKM` for per-book bookmarks
- `state/BMIDX.TXT` for the bookmark index

`<BOOKID>` is the current 8-character uppercase hex stem derived from the active book id. A runtime book id such as `bk-8a79a61f` maps to `8A79A61F`.

## Progress record format

The compatibility record is a single UTF-8 line with pipe-delimited fields:

```text
book_id|source_path|format|chapter|page|byte_offset|font_size_idx
```

Example:

```text
bk-8a79a61f|BOOKS/YEARLY_H.TXT|txt|0|12|3456|4
```

## Bookmark record format

Per-book bookmark files contain zero or more UTF-8 lines:

```text
book_id|source_path|chapter|byte_offset|label
```

Example:

```text
bk-8a79a61f|BOOKS/YEARLY_H.TXT|2|8192|line 1 %7C note
```

## Bookmark index format

The bookmark index contains zero or more UTF-8 lines:

```text
book_id|source_path|display_title|chapter|byte_offset|label
```

Example:

```text
bk-8a79a61f|BOOKS/YEARLY_H.TXT|Yearly Hindi|3|9000|important
```

## Escaping

The current compatibility escape rules are preserved:

- `|` becomes `%7C`
- newline becomes `%0A`
- carriage return becomes `%0D`

## Ownership boundary

Vaachak-owned in this slice:

- constants for the compatibility filenames
- 8.3-safe state filename helpers
- progress record model
- bookmark entry model
- bookmark index entry model
- pure parse/serialize helpers
- tests proving current file-format compatibility

Still owned by the active runtime:

- SD directory creation
- SD read/write calls
- reader state machine
- bookmark UI
- prepared cache runtime
- display/input/storage hardware behavior
