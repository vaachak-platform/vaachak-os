# Vaachak OS Roadmap

## Current baseline

The current baseline is an X4-first reader firmware with Vaachak-owned runtime code, a cleaned repository root, production helper scripts, and current-state documentation.

Accepted baseline:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
reader-bionic=x4-reader-bionic-reading-ok
reader-guide-dots=x4-reader-guide-dots-ok
reader-sunlight=x4-reader-sunlight-fading-fix-ok
```

## Roadmap principles

1. Protect the reading path.
2. Keep the accepted X4/CrossPoint partition table unchanged.
3. Do not add broad platform features before Reader Home, resume, and library are stable.
4. Keep optional Lua apps bounded to `/VAACHAK/APPS`; do not move native features into Lua unless that is explicitly chosen.
5. Freeze reader data models before XTC or `.vchk` write behavior.
6. Treat XTC as compatibility/import support.
7. Treat `.vchk` as the Vaachak-native package format.
8. Align sync semantics only after local state is stable.

## Milestones

### M1 — Reader Home + Resume

- Continue Reading surface at top of Reader Home.
- Last-opened book model.
- Saved progress display in the library list.
- Return from reader to Reader Home.

### M2 — Reader Data Model Freeze

Freeze these models:

- `BookIdentity`
- `ReadingProgress`
- `Bookmark`
- `Highlight`
- `PerBookSettings`
- `LibraryEntry`
- `ContentFormat`
- `ContentLocationAnchor`

Use a hybrid anchor model: fast local page index for reopen, plus logical anchors where available for sync-safe state.

### M3 — Library Index Polish

- Local SD scan.
- Recent/last-opened ordering.
- Broken package/file handling.
- Title and long filename stability.

### M4 — XTC Compatibility

- Detect XTC.
- Open XTC packages.
- Page reliably.
- Save progress against XTC entries.

### M5 — `.vchk` Spec Freeze

- Header.
- Manifest.
- Content payload rules.
- Navigation metadata.
- Reader-state section.
- Sync metadata section.

### M6 — `.vchk` Read/Open

- Validate package.
- Parse manifest.
- Load content and navigation metadata.
- Load existing reader state.
- Render through the reader path.

### M7 — `.vchk` Mutable State

- Update progress.
- Add/remove bookmarks.
- Add/remove highlights.
- Persist revisions safely.

### M8 — Vaachak Sync Alignment

- Sync object IDs.
- Revision semantics.
- Offline-first local mutation queue.
- Progress/bookmark/highlight serialization.

## Deferred until after the reader path is stable

- OPDS.
- OTA hardening beyond the current app0/partition helpers.
- Broad app ecosystem.
- Palm/Tern compatibility host.
- Plugin/runtime expansion beyond the bounded Lua app path.
- Waveshare/S3 implementation.
