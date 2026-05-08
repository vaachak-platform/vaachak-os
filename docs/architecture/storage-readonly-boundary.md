# Storage Read-Only Boundary

This is the canonical architecture document for the Vaachak OS read-only storage boundary on the Xteink X4 target.

The boundary consolidates the earlier read-only storage adapter facade and Pulp bridge work into one clean layer before moving to hardware-layer ownership work.

## Ownership summary

Vaachak owns the read-only storage boundary contract and boundary entrypoint. Pulp remains the active implementation owner for SD/FAT runtime behavior.

| Layer | Current owner | Role |
| --- | --- | --- |
| Public read-only contract | Vaachak `target-xteink-x4` | `VaachakReadonlyStorage` facade trait and value types. |
| Boundary entrypoint | Vaachak `target-xteink-x4` | `VaachakStorageReadonlyBoundary` composes facade plus Pulp bridge. |
| Active read-only bridge | Vaachak `target-xteink-x4` | `PulpReadonlyStorageBridge` maps Vaachak calls to a narrow backend trait. |
| Active SD/FAT implementation | `vendor/pulp-os` | Existing SD card, FAT/filesystem, read/list/size helpers, and runtime state. |
| SD mount/probe | `vendor/pulp-os` | Not moved. |
| SPI arbitration | `vendor/pulp-os` | Not moved. |
| Display behavior | `vendor/pulp-os` | Not moved. |
| Reader/File browser behavior | `vendor/pulp-os` | Not changed. |

## Consolidated boundary files

| File | Role |
| --- | --- |
| `target-xteink-x4/src/vaachak_x4/io/storage_readonly_adapter.rs` | Vaachak-owned public read-only facade contract. |
| `target-xteink-x4/src/vaachak_x4/io/storage_readonly_pulp_bridge.rs` | Pulp-backed read-only bridge implementation. |
| `target-xteink-x4/src/vaachak_x4/io/storage_readonly_boundary.rs` | Canonical boundary entrypoint that composes facade plus bridge. |
| `target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_adapter_facade_smoke.rs` | Static smoke for the facade contract. |
| `target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_pulp_bridge_smoke.rs` | Static smoke for the bridge contract. |
| `target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_boundary_smoke.rs` | Static smoke for the consolidated boundary. |
| `scripts/validate_storage_readonly_boundary.sh` | Canonical validator for the consolidated boundary. |

## Public read-only operations

| Operation | Public facade method | Active implementation path |
| --- | --- | --- |
| File exists | `file_exists` | Boundary -> facade -> Pulp bridge -> existing Pulp file-size helpers. |
| Read file start | `read_file_start` | Boundary -> facade -> Pulp bridge -> existing Pulp read-start helpers. |
| Read chunk | `read_chunk` | Boundary -> facade -> Pulp bridge -> existing Pulp read-chunk helpers. |
| List directory metadata | `list_directory_metadata` | Boundary -> facade -> Pulp bridge -> existing Pulp list-entry helpers. |
| Resolve storage paths | `resolve_current_storage_paths` | Vaachak path map mirrors the current active Pulp layout. |

## Active Pulp-backed implementation remains active

The Pulp-backed bridge remains the active implementation path for the read-only boundary. The bridge only delegates to existing Pulp read/list/size helpers through a narrow backend trait.

The embedded backend remains behind `cfg(target_arch = "riscv32")`, so host/static checks do not need SD hardware.

## Path map

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

This consolidation does not move or add:

- SD card mount/probe behavior
- SD driver behavior
- FAT/filesystem ownership
- SPI arbitration
- Display behavior
- Reader behavior
- File browser behavior
- Cache creation or mutation behavior
- Write, append, delete, create, rename, truncate, mkdir, mount, unmount, or format operations

## Why this is the final read-only storage boundary slice

The storage boundary now has a Vaachak-owned facade contract, a Pulp-backed read-only bridge, and a canonical boundary entrypoint with one validation gate. That is enough to start hardware-layer movement next without mixing hardware ownership changes into reader/library behavior.

The next hardware slice should start by clarifying shared SPI ownership before SD mount/probe behavior is moved.

## Validation

```bash
cargo fmt --all
./scripts/validate_storage_readonly_adapter_facade.sh
./scripts/validate_storage_readonly_pulp_bridge.sh
./scripts/validate_storage_readonly_boundary.sh
cargo build
```

Expected marker:

```text
storage_readonly_boundary=ok
```
