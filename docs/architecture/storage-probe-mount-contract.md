# Storage Probe/Mount Contract

This document is the canonical Vaachak-owned metadata contract for the Xteink X4 SD probe/mount lifecycle.

## Purpose

This slice defines the ownership boundary for SD card availability before any storage hardware behavior is moved.

It records:

- the SD probe/mount lifecycle steps
- the dependency on the shared SPI bus contract
- the current active owner for SD probe/mount behavior
- the current active owner for FAT behavior
- the non-goals for this checkpoint

This is a contract/metadata checkpoint only. It does not move SD, FAT, SPI, display, reader, or file browser behavior.

## Active ownership model

| Area | Current active owner | Vaachak ownership in this slice |
| --- | --- | --- |
| SD driver setup | `vendor/pulp-os` imported runtime | Metadata only |
| SD probe / identification | `vendor/pulp-os` imported runtime | Metadata only |
| SD mount / availability | `vendor/pulp-os` imported runtime | Metadata only |
| FAT volume behavior | `vendor/pulp-os` imported runtime | No behavior moved |
| FAT read/write behavior | `vendor/pulp-os` imported runtime | No behavior moved |
| SPI arbitration | `vendor/pulp-os` imported runtime | No behavior moved |
| SSD1677 display behavior | `vendor/pulp-os` imported runtime | No behavior moved |

## Dependency on shared SPI bus contract

The SD card is a storage device on the same SPI bus as the SSD1677 display.

The storage lifecycle contract depends on the shared SPI contract facts:

| Fact | Value |
| --- | ---: |
| Shared SPI contract doc | `docs/architecture/spi-bus-runtime-contract.md` |
| SD chip-select | GPIO12 |
| Display chip-select | GPIO21 |
| Shared SCLK | GPIO8 |
| Shared MOSI | GPIO10 |
| Shared MISO | GPIO7 |
| SD identification speed | 400 kHz |
| Operational speed | 20 MHz |

This dependency is documentary and static in this slice. Active SPI arbitration remains in `vendor/pulp-os`.

## Lifecycle metadata

| Step | Meaning | Active behavior owner |
| --- | --- | --- |
| Runtime boot | Pulp starts the active firmware path | `vendor/pulp-os` imported runtime |
| Shared SPI available | Pulp-owned SPI setup/arbitration is available | `vendor/pulp-os` imported runtime |
| Slow SD identification | SD identification occurs through the existing slow-clock path | `vendor/pulp-os` imported runtime |
| Card availability known | Pulp determines card presence/availability | `vendor/pulp-os` imported runtime |
| FAT volume available | Pulp owns FAT volume setup and availability | `vendor/pulp-os` imported runtime |
| Read-only facade available | Vaachak read-only storage facade may observe files after Pulp makes storage available | `vendor/pulp-os` imported runtime for physical behavior; Vaachak owns read-only contract metadata |

## Relationship to read-only storage boundary

The previously accepted read-only storage boundary remains above this lifecycle contract:

- `storage_readonly_adapter` defines Vaachak-owned file-level read-only traits.
- `storage_readonly_pulp_bridge` maps the read-only facade to the active Pulp-backed implementation.
- `storage_readonly_boundary` consolidates the read-only file boundary.

This storage probe/mount contract sits below that boundary and above the physical hardware migration plan. It does not change the reader or file browser paths.

## Explicit non-goals

This deliverable does not:

- initialize SD hardware
- probe SD hardware
- mount SD
- unmount SD
- create a FAT volume manager
- move FAT reads or writes
- add file write/delete/rename/mkdir behavior
- move SPI arbitration
- select or deselect chip-select lines at runtime
- initialize or refresh the display
- change reader behavior
- change file browser behavior

## New files

```text
 target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_contract.rs
 target-xteink-x4/src/vaachak_x4/contracts/storage_probe_mount_contract_smoke.rs
 docs/architecture/storage-probe-mount-contract.md
 scripts/validate_storage_probe_mount_contract.sh
```

## Acceptance marker

```text
storage_probe_mount_contract=ok
```
