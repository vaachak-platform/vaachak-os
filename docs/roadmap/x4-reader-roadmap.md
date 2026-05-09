# X4 Reader Roadmap

## Baseline

Vaachak-native hardware runtime is accepted. The next roadmap is product/runtime work, not hardware migration.

## Next deliverables

### 1. Reader Home + Resume Foundation

- Continue Reading model.
- Last-opened book state.
- Library list with saved progress.
- Reader return path to Reader Home.

### 2. Reader Data Model Freeze

- `BookIdentity`
- `ReadingProgress`
- `Bookmark`
- `Highlight`
- `PerBookSettings`
- `LibraryEntry`
- `ContentFormat`
- `ContentLocationAnchor`

### 3. Library Index Polish

- SD scan.
- recent/last-opened ordering.
- broken content handling.
- title/long filename stability.

### 4. XTC Compatibility

- detect XTC
- open XTC
- page reliably
- persist progress

### 5. `.vchk` Spec Freeze

- package header
- manifest
- payload sections
- navigation metadata
- reader-state section
- sync metadata section

### 6. `.vchk` Read/Open

- validate package
- parse manifest
- load content
- load existing state
- render through reader

### 7. `.vchk` Mutable State

- update progress
- add/remove bookmarks
- add/remove highlights
- deterministic state revisions

### 8. Sync Alignment

- object IDs
- revision semantics
- offline mutation queue
- serialization for progress/bookmarks/highlights
