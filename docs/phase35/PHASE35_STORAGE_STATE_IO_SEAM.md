# Phase 35 — Storage State IO Seam

## Purpose

Create a Vaachak-owned seam for storage state IO without taking over physical SD/SPI/FAT behavior yet.

## State Kinds

The seam should represent these state kinds:

```text
Progress
Bookmark
Theme
Metadata
```

## Path Source of Truth

Path/name construction should use:

```text
target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
```

## Physical IO Ownership

Physical IO remains imported-runtime-owned in Phase 35.

Phase 35 must not introduce direct ownership of:

```text
SdCard::new
AsyncVolumeManager
open_raw_volume
open_file_in_dir
read/write/close_file
spi::master
RefCellDevice
```

## Acceptable Implementation

A trait/interface is acceptable:

```text
VaachakStorageStateIo
```

A no-op/probe adapter is acceptable.

A direct physical SD/FAT implementation is not acceptable in Phase 35.

## Implemented Seam

Phase 35 adds:

```text
target-xteink-x4/src/vaachak_x4/io/mod.rs
target-xteink-x4/src/vaachak_x4/io/storage_state.rs
target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs
```

The seam defines:

```text
VaachakStateIoKind
VaachakStorageStateIo
VaachakStorageStatePaths
VaachakStorageStateIoAdapter
VaachakStorageStatePathIo
```

`VaachakStorageStatePaths` resolves `Progress`, `Bookmark`, `Theme`, and
`Metadata` paths through `VaachakStoragePathHelpers`.

`VaachakStorageStateIoAdapter` delegates to an injected path-level backend. It
does not import or instantiate SD card, SPI, FAT volume, display, input, reader,
or EPUB cache types.

## Runtime Wiring

No imported runtime call site is changed in Phase 35.

The obvious state IO behavior is currently inside vendored reader/kernel
modules rather than a narrow call site in:

```text
target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
```

Changing those call sites would cross into reader internals and persistence
behavior, so Phase 35 stops at the seam and records that as the next manual
decision.
