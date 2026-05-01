# Phase 31 — Active Storage Path Helper Adoption

## Purpose

Phase 31 starts using Vaachak-owned pure storage path/name helpers in the active target runtime path.

This is the first real adoption step after Phase 30, but it deliberately avoids physical storage IO.
The active imported runtime calls a Vaachak-owned pure adoption probe so these
helpers are compiled into the active path without changing filesystem behavior.

## Accepted Baseline

Phase 30 is accepted.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB reader behavior works.

## Ownership Model

Vaachak owns:

```text
target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
```

for pure path/name helper logic.

The storage state contract now delegates its pure state filename validation to
that helper module, so the helper is the source of truth for state directory
names, state extensions, bookmark index naming, reserved-file checks, and
book-id validation.

Imported Pulp still owns:

```text
SD initialization
SPI bus setup
filesystem IO
progress/bookmark/theme reads and writes
EPUB cache IO
reader runtime behavior
```

## In Scope

Pure deterministic helpers for:

```text
state directory name
progress file path/name
bookmark file path/name
theme file path/name
metadata file path/name
bookmark index filename
reserved state file detection
book id validation
state extension validation
```

## Out of Scope

Do not move:

```text
SD/SPI initialization
filesystem open/read/write/close
EPUB cache IO
progress/bookmark/theme IO
reader app construction
EPUB parsing/rendering
TXT reader behavior
```

## Boot Marker

Normal boot remains:

```text
vaachak=x4-runtime-ready
```

No old phase markers should be emitted during normal boot.
Phase 31 does not add a normal boot marker.
