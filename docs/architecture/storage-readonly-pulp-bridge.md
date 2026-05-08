# Storage Read-Only Pulp Bridge

This document is retained for compatibility. The canonical storage read-only boundary document is now `docs/architecture/storage-readonly-boundary.md`.

## Purpose

This deliverable added a Pulp-backed read-only implementation bridge for the Vaachak storage facade under `target-xteink-x4`.

The bridge gives Vaachak-owned code a real adapter surface for read-only storage operations while keeping the active SD card, FAT/filesystem, SPI, display, reader, and file-browser behavior in `vendor/pulp-os`.

## Ownership map

Vaachak owns adapter contract definitions and bridge boundaries. Pulp remains the active implementation owner for SD/FAT runtime behavior.

## Bridge layout

| Layer | File/type | Role |
| --- | --- | --- |
| Vaachak read-only contract | `io/storage_readonly_adapter.rs` | Trait and value types owned by Vaachak. |
| Consolidated boundary | `io/storage_readonly_boundary.rs` / `VaachakStorageReadonlyBoundary<B>` | Canonical entrypoint that composes the facade and bridge. |
| Generic bridge | `io/storage_readonly_pulp_bridge.rs` / `PulpReadonlyStorageBridge<B>` | Implements the Vaachak trait over a small backend trait; compile-safe without SD hardware. |
| Backend trait | `PulpReadonlyStorageBackend` | Narrow read-only call surface used by the bridge. |
| Embedded backend | `X4PulpReadonlyStorageBackend` | `riscv32`-only mapping to existing `x4_kernel::drivers::storage` read/list/size helpers. |

## Adapter call mapping

| Vaachak operation | Pulp-backed mapping |
| --- | --- |
| `file_exists(/BOOK.TXT)` | `storage::file_size(sd, "BOOK.TXT")` |
| `read_file_start(/BOOK.TXT)` | `storage::read_file_start(sd, "BOOK.TXT", out)` |
| `read_chunk(...)` | Existing Pulp chunk helper for root, one-directory, or two-directory paths. |
| `list_directory_metadata(/)` | `storage::list_root_entries(sd, scratch)` |
| `resolve_current_storage_paths()` | `VaachakResolvedStoragePaths::PULP_BACKED_ACTIVE_PATHS` |

## Non-goals retained from the bridge slice

This bridge does not move or add:

- SD card mount/probe behavior
- SD driver behavior
- FAT/filesystem ownership
- SPI arbitration
- Display behavior
- Reader behavior
- File browser behavior
- Cache creation or mutation behavior
- Write, append, delete, create, rename, truncate, mkdir, mount, unmount, or format operations
