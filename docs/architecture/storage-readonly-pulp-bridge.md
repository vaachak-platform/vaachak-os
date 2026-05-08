# Storage Read-Only Pulp Bridge

## Purpose

This deliverable adds a Pulp-backed read-only implementation bridge for the Vaachak storage facade under `target-xteink-x4`.

The bridge gives Vaachak-owned code a real adapter surface for read-only storage operations while keeping the active SD card, FAT/filesystem, SPI, display, reader, and file-browser behavior in `vendor/pulp-os`.

## Ownership map

Vaachak owns adapter contract definitions and bridge boundaries. Pulp remains the active implementation owner for SD/FAT runtime behavior.


| Capability | Vaachak bridge surface | Active implementation owner today | Moved in this deliverable? |
| --- | --- | --- | --- |
| File existence check | `PulpReadonlyStorageBridge::file_exists` | Existing Pulp file-size helpers | No |
| Read file from start | `PulpReadonlyStorageBridge::read_file_start` | Existing Pulp read-start helpers | No |
| Read file chunk by offset | `PulpReadonlyStorageBridge::read_chunk` | Existing Pulp read-chunk helpers | No |
| List directory metadata | `PulpReadonlyStorageBridge::list_directory_metadata` | Existing Pulp list-entry helpers | No |
| Resolve current storage paths | `PulpReadonlyStorageBridge::resolve_current_storage_paths` | Vaachak facade path map mirrors current Pulp layout | No physical behavior |

## Bridge layout

| Layer | File/type | Role |
| --- | --- | --- |
| Vaachak read-only contract | `io/storage_readonly_adapter.rs` | Trait and value types owned by Vaachak. |
| Generic bridge | `io/storage_readonly_pulp_bridge.rs` / `PulpReadonlyStorageBridge<B>` | Implements the Vaachak trait over a small backend trait; compile-safe without SD hardware. |
| Backend trait | `PulpReadonlyStorageBackend` | Narrow read-only call surface used by the bridge. |
| Embedded backend | `X4PulpReadonlyStorageBackend` | `riscv32`-only mapping to existing `x4_kernel::drivers::storage` read/list/size helpers. |
| Static smoke | `contracts/storage_readonly_pulp_bridge_smoke.rs` | Proves the bridge boundary exists and physical behavior remains imported. |

## Adapter call mapping

| Vaachak operation | Pulp-backed mapping |
| --- | --- |
| `file_exists(/BOOK.TXT)` | `storage::file_size(sd, "BOOK.TXT")` |
| `file_exists(/state/BMIDX.TXT)` | `storage::file_size_in_dir(sd, "state", "BMIDX.TXT")` |
| `file_exists(/sleep/daily/MON.BMP)` | `storage::file_size_in_subdir(sd, "sleep", "daily", "MON.BMP")` |
| `read_file_start(/BOOK.TXT)` | `storage::read_file_start(sd, "BOOK.TXT", out)` |
| `read_file_start(/state/BMIDX.TXT)` | `storage::read_file_start_in_dir(sd, "state", "BMIDX.TXT", out)` |
| `read_file_start(/sleep/daily/MON.BMP)` | `storage::read_file_start_in_subdir(sd, "sleep", "daily", "MON.BMP", out)` |
| `read_chunk(...)` | Existing Pulp chunk helper for root, one-directory, or two-directory paths. |
| `list_directory_metadata(/)` | `storage::list_root_entries(sd, scratch)` |
| `list_directory_metadata(/state)` | `storage::list_dir_entries(sd, "state", scratch)` |
| `list_directory_metadata(/sleep/daily)` | `storage::list_subdir_entries(sd, "sleep", "daily", scratch)` |
| `resolve_current_storage_paths()` | `VaachakResolvedStoragePaths::PULP_BACKED_ACTIVE_PATHS` |

The bridge normalizes `/_X4` to Pulp's active `_x4` directory name when routing through the embedded backend.

## Non-goals

This deliverable does not move or add:

- SD card mount/probe behavior
- SD driver behavior
- FAT/filesystem ownership
- SPI arbitration
- Display behavior
- Reader behavior
- File browser behavior
- Cache creation or mutation behavior
- Write, append, delete, create, rename, truncate, mkdir, mount, unmount, or format operations

## Validation intent

`scripts/validate_storage_readonly_pulp_bridge.sh` checks that:

- The bridge, smoke, docs, and module exports are present.
- The bridge uses the existing Vaachak read-only facade trait.
- The embedded backend maps only to existing Pulp read/list/size helpers.
- No write/delete/rename/create/truncate/append API is introduced in the bridge contract surface.
- No SD mount/probe, SPI arbitration, display, reader, or file-browser behavior was moved into Vaachak-owned bridge code.
- Vendored Pulp and smol-epub code remain unchanged when Git metadata is available.
