# Storage Read-Only Adapter Facade

This document is retained for compatibility. The canonical storage read-only boundary document is now `docs/architecture/storage-readonly-boundary.md`.

## Purpose

This deliverable added a Vaachak-owned read-only storage adapter facade under `target-xteink-x4`.

## Active implementation remains Pulp-backed

| Capability | Vaachak facade surface | Active implementation owner today | Moved by the facade? |
| --- | --- | --- | --- |
| File existence check | `VaachakReadonlyStorage::file_exists` | `vendor/pulp-os` SD/FAT runtime | No |
| Read file from start | `VaachakReadonlyStorage::read_file_start` | `vendor/pulp-os` SD/FAT runtime | No |
| Read file chunk by offset | `VaachakReadonlyStorage::read_chunk` | `vendor/pulp-os` SD/FAT runtime | No |
| List directory metadata | `VaachakReadonlyStorage::list_directory_metadata` | `vendor/pulp-os` SD/FAT runtime | No |
| Resolve current storage paths | `VaachakReadonlyStorage::resolve_current_storage_paths` | Vaachak contract mirrors current Pulp layout | No physical behavior |

## Non-goals retained from the facade slice

The facade does not move any of the following out of `vendor/pulp-os`:

- SD card mount/probe behavior
- SD driver behavior
- FAT/filesystem behavior
- SPI arbitration
- Display behavior
- Reader behavior
- File browser behavior
- Cache creation or mutation behavior
- Any write, delete, create, rename, truncate, or append operation

## Current status

The facade is now part of the consolidated storage read-only boundary. The active call path is:

`VaachakStorageReadonlyBoundary` -> `VaachakReadonlyStorageFacade` -> `PulpReadonlyStorageBridge` -> existing Pulp read/list/size helpers.
