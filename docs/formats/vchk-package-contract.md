# Vaachak `.vchk` Package Contract

Status: Draft planning contract. Not implemented yet.

## Purpose

`.vchk` is the planned Vaachak-native full book container format for Vaachak OS.

It is intended to support:

- ESP32/Xteink-class device constraints
- deterministic local-first reading
- compact metadata access
- progress, bookmarks, highlights, and per-book state
- future sync across Vaachak devices and services

## Relationship to other formats

- TXT and EPUB remain valid reader input/open formats.
- XTC is a compatibility/import format.
- `.vchk` is the long-term Vaachak-native package.

## Logical package sections

1. Header
2. Manifest
3. Content Payload
4. Navigation Metadata
5. Reader State
6. Sync Metadata
7. Optional Assets

## First freeze depends on reader data model

Do not freeze `.vchk` before the reader state model is stable. The reader data model must settle:

- `BookIdentity`
- `ReadingProgress`
- `Bookmark`
- `Highlight`
- `PerBookSettings`
- `LibraryEntry`
- `ContentFormat`
- `ContentLocationAnchor`

## First implementation recommendation

- full embedded package
- content + manifest + reader state
- sync metadata from the beginning
- versioned and forward-compatible
- support future content payload choices without forcing one rendering strategy too early
