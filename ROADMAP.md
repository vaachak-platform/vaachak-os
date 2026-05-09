# Vaachak OS Roadmap

## Current baseline

The hardware migration track is accepted. The current baseline is an X4-first reader OS with Vaachak-native hardware ownership and a retained `vendor/pulp-os` non-hardware compatibility/import/reference scope.

Accepted gate:

```text
vaachak_hardware_runtime_final_acceptance=ok
```

## Roadmap principles

1. Protect the reading path.
2. Do not add platform breadth before Reader Home, resume, and library are stable.
3. Freeze reader data models before adding XTC or `.vchk` write behavior.
4. Treat XTC as compatibility/import support.
5. Treat `.vchk` as the Vaachak-native package format.
6. Align sync semantics only after local state is stable.

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
- OTA hardening.
- Broad app ecosystem.
- Palm/Tern compatibility host.
- Script/plugin runtime.
- Waveshare/S3 implementation.

## XTC compatibility

XTC compatibility remains a planned reader-format milestone after Reader Home, library/resume polish, and reader data model freeze. XTC should be treated as a compatibility/import format, not the long-term Vaachak-native state container.

