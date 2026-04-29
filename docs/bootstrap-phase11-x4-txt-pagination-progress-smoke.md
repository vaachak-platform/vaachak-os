# VaachakOS Bootstrap Phase 11 — X4 TXT Reader Pagination + Progress Smoke

## Goal

Phase 11 extends the Phase 10 TXT reader smoke test with minimal pagination and flat progress persistence.

This is still a smoke milestone, not the full reader app migration.

## Included behavior

- Library scan stays from Phase 9/10.
- Select opens a TXT/MD file.
- Reader renders a fixed-size 1024-byte page chunk.
- Down / Right / Select advance to the next chunk.
- Up returns to the previous chunk.
- Back / Left returns to the Library list.
- Progress is written as an 8.3-safe flat file under `state/`.

Example:

```text
state/5A1B2C3D.PRG
```

The file contains simple text fields:

```text
version=phase11
file=LONG.TXT
size=18984
offset=2048
page=3
total=19
```

## Why this shape

The X4 proving-ground showed that flat 8.3-safe state files are reliable on this SD path, while nested/long paths need more care. Phase 11 keeps that proven approach while moving VaachakOS closer to real reader behavior.

## Acceptance markers

```text
phase11=x4-txt-pagination-progress-smoke-ready
phase11: reader read ok file=... offset=... page=X/Y restored=...
phase11: progress wrote state/<BOOKID>.PRG offset=... page=X/Y file=...
phase11: reader page ok file=... offset=... page=X/Y progress=<BOOKID>.PRG
phase11=x4-txt-pagination-progress-smoke-ok
```

## Deferred

- EPUB.
- Bookmarks.
- Full progress service.
- Content-based BookId finalization.
- Semantic page layout.
