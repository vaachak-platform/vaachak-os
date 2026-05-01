# Phase 17 — Reader Runtime Boundary

## Purpose

Phase 17 locks down the working Phase 16 reader behavior and defines a maintenance boundary between Vaachak OS and the imported X4/Pulp reader runtime.

Phase 16 proved that the target can use the real X4/Pulp reader path, including `smol-epub`, instead of the fake raw EPUB ZIP-byte smoke reader. Phase 17 should preserve that working runtime and make future changes safer.

## Boundary rule

The authoritative imported reader runtime is:

```text
vendor/pulp-os
```

The authoritative EPUB parser path is:

```text
vendor/smol-epub
```

The active X4 target entrypoint is:

```text
target-xteink-x4/src/main.rs
```

For Phase 17, `target-xteink-x4/src/main.rs` intentionally tracks:

```text
vendor/pulp-os/src/bin/main.rs
```

with only these allowed differences:

```text
x4_os:: -> pulp_os::
phase16=x4-reader-parity-ok marker
phase17=x4-reader-refactor-ok marker
Vaachak marker/log lines
```

## Do not edit yet

Do not refactor these paths during Phase 17:

```text
vendor/pulp-os/src/apps/reader/*
vendor/pulp-os/kernel/*
vendor/smol-epub/*
```

They are the known-good imported runtime. Editing them now risks breaking TXT/EPUB parity before the Vaachak boundary is stable.

## Safe places for Vaachak-specific work

Vaachak-specific integration should start outside vendor code:

```text
target-xteink-x4/src/*
docs/phase17/*
scripts/*
hal-xteink-x4/*
core/*
```

Vendor code can be forked or wrapped later, but only after a sync-check script and parity checklist can catch regressions.

## Reader behavior that must remain unchanged

Phase 17 must preserve:

```text
TXT progress
EPUB progress
TXT bookmarks
EPUB bookmarks
Reader footer labels
Reader menu actions
Theme preset/state file support
Continue behavior
smol-epub EPUB rendering path
Back returns to library/files
```

## Why this boundary matters

The previous raw EPUB smoke function opened an EPUB as a ZIP file and rendered raw bytes. That caused broken text on screen. The X4/Pulp path already has the real reader pipeline:

```text
EPUB file
  -> ZIP central directory
  -> META-INF/container.xml
  -> OPF metadata/spine
  -> XHTML spine item
  -> smol-epub HTML strip
  -> chapter cache
  -> paging
  -> display render
```

Phase 17 keeps that imported runtime intact and makes it explicit that future Vaachak cleanup should not accidentally restore the fake EPUB path.

## Phase markers

Phase 16 marker:

```text
phase16=x4-reader-parity-ok
```

Phase 17 marker:

```text
phase17=x4-reader-refactor-ok
```

Both markers should be present after applying this phase.
