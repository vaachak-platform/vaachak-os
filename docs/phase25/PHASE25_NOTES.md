# Phase 25 Notes

## Why this phase exists

Phases 21–24 created the Vaachak-owned storage, input, display, and consolidated boundary metadata. Phase 25 turns the storage state naming assumptions into executable helper code while still avoiding any physical SD/FAT behavior changes.

## State records covered

```text
<BOOKID>.PRG — progress
<BOOKID>.BKM — bookmarks
<BOOKID>.THM — theme/preset state
<BOOKID>.MTA — metadata
BMIDX.TXT    — reserved bookmark index
```

`BOOKID` is treated as an uppercase 8-hex-character identifier for the current contract.

## Future extraction path

A later phase can move one storage behavior at a time:

```text
1. host-only state file name tests
2. embedded read-only state directory scan smoke
3. progress write adapter
4. bookmark write adapter
5. EPUB cache ownership split
```

The current phase deliberately stops before step 2.
