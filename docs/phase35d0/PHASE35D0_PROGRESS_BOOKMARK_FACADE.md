# Phase 35D-0 - Progress/Bookmark State Facade

Phase 35D-0 extends the Vaachak-owned reader state facade with progress and bookmark record contracts.

This phase models:

- progress records stored as `.PRG`
- per-book bookmark records stored as `.BKM`
- bookmark index records stored as `BMIDX.TXT`
- bookmark jump messages using the existing `BMJ` wire prefix

The facade remains pure data/path logic. It uses `VaachakStoragePathHelpers` for typed state filenames and preserves the imported Pulp reader state line formats for progress, bookmarks, bookmark indexes, and bookmark jumps.

Phase 35D-0 does not move active persistence. The active reader runtime still uses the imported Pulp `AppManager`, reader app, filesystem calls, and button/input path.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```

Vendor code remains untouched:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```
