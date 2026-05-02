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
