# Storage Probe/Mount Runtime Ownership

This document is the canonical ownership record for the Xteink X4 SD card probe/mount lifecycle after the SPI bus runtime ownership bridge.

## Status

`storage_probe_mount_runtime_owner=ok`

## Ownership change

Vaachak now owns the SD probe/mount runtime ownership boundary in `target-xteink-x4`:

- public owner entrypoint: `target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs`
- Pulp compatibility backend descriptor: `target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_pulp_backend.rs`
- contract smoke: `target-xteink-x4/src/vaachak_x4/contracts/storage_probe_mount_runtime_ownership_smoke.rs`

The ownership marker is:

```text
x4-storage-probe-mount-runtime-owner-ok
```

The explicit authority flag is:

```text
STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true
```

## Active backend

The active backend remains:

```text
PulpCompatibility
```

That means the existing imported Pulp runtime remains the active hardware executor for:

- SD card detection behavior
- slow SD identification behavior at `400 kHz`
- SD mount availability behavior
- FAT volume availability behavior
- existing file I/O behavior

## SPI dependency

This ownership layer depends on the accepted shared SPI runtime owner:

- `docs/architecture/spi-bus-runtime-ownership.md`
- SCLK `GPIO8`
- MOSI `GPIO10`
- MISO `GPIO7`
- SD chip select `GPIO12`
- display chip select `GPIO21`

The storage owner records SD lifecycle authority on top of the SPI ownership bridge. It does not move SPI arbitration behavior.

## What moved

Moved into Vaachak:

- SD probe/mount runtime ownership authority
- SD lifecycle identity
- SD card-detection lifecycle authority metadata
- slow SD identification lifecycle authority metadata
- card availability lifecycle authority metadata
- dependency mapping to shared SPI ownership metadata
- Pulp compatibility backend selection

## What did not move

Not moved in this deliverable:

- SD driver implementation
- SD card initialization execution
- SD mount execution
- FAT read/write/list behavior
- file open/read/write/rename/delete/mkdir behavior
- SPI arbitration execution
- SSD1677 display rendering or refresh behavior
- reader behavior
- file browser behavior

## Lifecycle model

| Step | Vaachak role | Active executor |
|---|---|---|
| Runtime boot | owns lifecycle authority boundary | Pulp compatibility backend |
| Shared SPI ready | depends on Vaachak SPI ownership bridge | Pulp compatibility backend |
| Card detection authority | Vaachak-owned metadata | Pulp compatibility backend |
| Slow identification authority | Vaachak-owned metadata | Pulp compatibility backend |
| Card availability authority | Vaachak-owned metadata | Pulp compatibility backend |
| FAT volume availability observed | observation only | Pulp compatibility backend |
| Read-only boundary observed | observation only | Pulp compatibility backend |

## Static validation

Run:

```bash
./scripts/validate_storage_probe_mount_runtime_owner.sh
```

Expected output:

```text
storage_probe_mount_runtime_owner=ok
```

The validator proves:

- Vaachak has the storage probe/mount runtime owner entrypoint.
- Pulp compatibility backend remains active.
- The accepted SPI ownership bridge is referenced.
- FAT read/write/list behavior did not move.
- SPI arbitration did not move.
- display behavior did not move.
- reader and file browser behavior did not change.
- no direct SD/FAT/SPI/display implementation crates were introduced in the new owner files.
