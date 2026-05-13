# X4 Reader Roadmap

## Baseline

Vaachak OS is now in a cleaned X4 reader/runtime baseline. Hardware cleanup is not the main roadmap path; the next work should improve reader product behavior without reintroducing patch artifacts.

## Current accepted reader features

- TXT and EPUB reader path.
- Progress/state/cache persistence.
- Bookmarks where supported.
- Title cache and long filename work.
- Prepared cache metadata and large-cache transfer notes.
- Bionic Reading.
- Guide Dots.
- Sunlight-fading mitigation.
- Reader settings sync between Settings and Reader-visible behavior.
- SD/static font work.

## Next milestones

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
