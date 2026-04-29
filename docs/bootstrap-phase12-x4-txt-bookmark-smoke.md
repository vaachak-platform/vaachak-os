# Bootstrap Phase 12 — X4 TXT Bookmark Smoke

## Goal

Add the first minimal bookmark path to the VaachakOS X4 bootstrap firmware while keeping scope TXT-only.

## Scope

Implemented:

- Library list remains from Phase 9.
- TXT reader and pagination remain from Phase 10/11.
- Reader Select toggles a bookmark at the current TXT offset.
- Bookmark state is written to an 8.3-safe flat file:

```text
state/<BOOKID>.BKM
```

- Progress remains written to:

```text
state/<BOOKID>.PRG
```

Deferred:

- EPUB reader.
- Global bookmarks screen.
- Bookmark overlay/list.
- Highlights.
- Sync.
- Full app/service split.

## Control model

In Library mode:

- Up / Down / Left / Right: move selection.
- Select: open TXT/MD.

In Reader mode:

- Down / Right: next page.
- Up: previous page.
- Select: save/remove bookmark at current offset.
- Back / Left: return to Library.

## Bookmark record shape

The Phase 12 smoke record is deliberately simple text for easy serial/debug inspection:

```text
version=phase12
file=LONG.TXT
count=2
offset=2048 page=3 total=19
offset=4096 page=5 total=19
```

This is not the final VaachakOS bookmark service format. It is an X4 smoke proof for flat SD persistence and UI state.

## Acceptance

Successful run should show:

```text
phase12=x4-txt-bookmark-smoke-ready
phase12: Bookmark saved file=LONG.TXT offset=2048 page=3/19 count=1 state/B29C8C4F.BKM
phase12=x4-txt-bookmark-smoke-ok
phase12: Bookmark removed file=LONG.TXT offset=2048 page=3/19 count=0 state/B29C8C4F.BKM
phase12=x4-txt-bookmark-smoke-ok
```
