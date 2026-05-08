# SD/FAT Runtime Read-Only Ownership

`sd_fat_runtime_readonly_owner` is the first SD/FAT runtime ownership move above the accepted storage probe/mount owner.

Expected validator marker:

```text
sd_fat_runtime_readonly_owner=ok
```

## Ownership statement

Vaachak now owns the SD/FAT read-only runtime authority boundary in `target-xteink-x4`.

```text
SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true
```

The active FAT executor remains the existing Pulp-compatible runtime.

```text
ACTIVE_BACKEND = PulpCompatibility
ACTIVE_EXECUTOR_OWNER = vendor/pulp-os imported runtime
```

## What moved

The following authority moved into the Vaachak X4 layer:

- SD/FAT read-only runtime ownership identity
- allowed read-only FAT operation metadata
- read-only operation ownership metadata for:
  - file exists
  - read file start
  - read chunk
  - list directory metadata
  - resolve current storage paths
- dependency checks for:
  - SPI bus runtime ownership
  - SD probe/mount runtime ownership
  - read-only storage boundary

## What did not move

The following behavior remains outside this slice:

- FAT read executor implementation
- FAT write/append/delete/rename/mkdir behavior
- SD card initialization
- SD probe/mount executor implementation
- SPI arbitration implementation
- SSD1677 display rendering or refresh
- reader behavior
- file browser behavior

This slice intentionally keeps `vendor/pulp-os` as the active FAT executor while Vaachak owns the read-only authority boundary.

## Dependency chain

This owner sits above the already accepted ownership layers:

1. `docs/architecture/spi-bus-runtime-ownership.md`
2. `docs/architecture/storage-probe-mount-runtime-ownership.md`
3. `docs/architecture/storage-readonly-boundary.md`
4. `docs/architecture/sd-fat-runtime-readonly-ownership.md`

The dependency is intentional:

```text
SPI ownership -> SD probe/mount ownership -> SD/FAT read-only ownership -> read-only storage boundary
```

## Active backend

The active backend descriptor is:

```text
target-xteink-x4/src/vaachak_x4/physical/sd_fat_readonly_pulp_backend.rs
```

It documents the current executor owner:

```text
PulpCompatibility
vendor/pulp-os imported runtime
```

The backend descriptor is metadata-only. It does not import Pulp storage modules, call FAT APIs, mount SD, transfer SPI data, or draw to the display.

## Public owner entrypoint

The public owner entrypoint is:

```text
target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs
```

It exposes:

- `VaachakSdFatRuntimeReadonlyOwner`
- `VaachakSdFatReadonlyOperation`
- `VaachakSdFatReadonlyLifecycleStep`
- `VaachakSdFatReadonlyOwnershipReport`

## Contract smoke

The contract smoke is:

```text
target-xteink-x4/src/vaachak_x4/contracts/sd_fat_runtime_readonly_ownership_smoke.rs
```

It verifies:

- Vaachak ownership authority is active
- active backend is `PulpCompatibility`
- SPI ownership dependency is valid
- SD probe/mount ownership dependency is valid
- read-only boundary dependency is valid
- registered operations are read-only
- writable operations remain denied
- no SD probe/mount, SPI, display, reader, or file browser behavior moved

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_sd_fat_runtime_readonly_owner.sh
cargo build
```

Expected result:

```text
sd_fat_runtime_readonly_owner=ok
```
