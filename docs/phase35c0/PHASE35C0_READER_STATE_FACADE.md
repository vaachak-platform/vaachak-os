# Phase 35C-0 — Vaachak Reader State Facade Extraction

## Purpose

Phase 35C-0 extracts the portable reader state facade needed before active
theme and metadata IO can move out of imported Pulp reader internals.

This phase is not active persistence adoption.

## Vaachak-Owned Facade

```text
target-xteink-x4/src/vaachak_x4/apps/reader_state.rs
```

The facade owns:

```text
VaachakBookId
VaachakBookIdentity
VaachakBookMetaRecord
VaachakReaderThemePreset
VaachakReaderThemeRecord
VaachakReaderStateLayout
```

It preserves the existing line-oriented record formats for metadata and theme
records, including percent-escaped `|`, `\n`, and `\r` fields.

## Path Ownership

Theme and metadata filenames are produced through existing Vaachak storage path
helpers:

```text
8A79A61F.THM
8A79A61F.MTA
```

## Active Runtime Status

Active reader `.THM` and `.MTA` read/write behavior still lives in imported
Pulp reader internals after Phase 35C-0.

The next phase may wire this facade into active theme and metadata persistence
only after the relevant reader slice is Vaachak-owned.
