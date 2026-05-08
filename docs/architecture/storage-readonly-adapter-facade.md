# Storage Read-Only Adapter Facade

## Purpose

This deliverable adds a Vaachak-owned read-only storage adapter facade under `target-xteink-x4`.

The facade defines the contract shape Vaachak code can depend on later without taking over hardware behavior in this slice.

## Active implementation remains Pulp-backed

| Capability | Vaachak facade surface | Active implementation owner today | Moved in this deliverable? |
| --- | --- | --- | --- |
| File existence check | `VaachakReadonlyStorage::file_exists` | `vendor/pulp-os` SD/FAT runtime | No |
| Read file from start | `VaachakReadonlyStorage::read_file_start` | `vendor/pulp-os` SD/FAT runtime | No |
| Read file chunk by offset | `VaachakReadonlyStorage::read_chunk` | `vendor/pulp-os` SD/FAT runtime | No |
| List directory metadata | `VaachakReadonlyStorage::list_directory_metadata` | `vendor/pulp-os` SD/FAT runtime | No |
| Resolve current storage paths | `VaachakReadonlyStorage::resolve_current_storage_paths` | Vaachak contract mirrors current Pulp layout | No physical behavior |

## Current path map

| Vaachak path role | Current Pulp-backed path |
| --- | --- |
| Root | `/` |
| Library root | `/` |
| State root | `/state` |
| EPUB/prepared cache root | `/FCACHE` |
| Settings file | `/_X4/SETTINGS.TXT` |
| Title cache file | `/_X4/TITLES.BIN` |
| Sleep assets root | `/sleep` |
| Daily sleep assets root | `/sleep/daily` |

## Non-goals

This deliverable does not move any of the following out of `vendor/pulp-os`:

- SD card mount/probe behavior
- SD driver behavior
- FAT/filesystem behavior
- SPI arbitration
- Display behavior
- Reader behavior
- File browser behavior
- Cache creation or mutation behavior
- Any write, delete, create, rename, truncate, or append operation

## Files added or changed

| File | Role |
| --- | --- |
| `target-xteink-x4/src/vaachak_x4/io/storage_readonly_adapter.rs` | Read-only facade traits, path structs, directory metadata structs, read chunk structs, and Pulp-backed active path constants. |
| `target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_adapter_facade_smoke.rs` | Static smoke contract proving the facade exists and physical behavior remains imported. |
| `target-xteink-x4/src/vaachak_x4/io/mod.rs` | Exports the new facade module. |
| `target-xteink-x4/src/vaachak_x4/contracts/mod.rs` | Exports the static smoke contract. |
| `docs/architecture/storage-readonly-adapter-facade.md` | Ownership and mapping documentation. |
| `scripts/validate_storage_readonly_adapter_facade.sh` | Static validation gate for this deliverable. |

## Validation intent

The validation gate checks that:

- The facade and smoke files are present.
- The five required read-only contract methods exist.
- The current Pulp-backed path map is documented and present in code.
- New Rust facade files do not import or call active Pulp hardware/runtime APIs.
- The facade does not define write/delete/create/rename/truncate/append contracts.
- `vendor/pulp-os` and `vendor/smol-epub` remain untouched when Git metadata is available.
