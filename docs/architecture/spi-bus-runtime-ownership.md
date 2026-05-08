# SPI Bus Runtime Ownership Bridge

This document is the canonical Vaachak-owned ownership record for the Xteink X4 shared SPI bus.

## Purpose

This deliverable moves SPI bus ownership authority into the `target-xteink-x4` Vaachak layer while keeping the known-good Pulp runtime as the active hardware executor.

This is intentionally an ownership bridge, not a full hardware behavior migration.

## What moved to Vaachak

Vaachak now owns the public SPI runtime ownership entrypoint:

```text
target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs
```

That entrypoint owns:

- shared SPI bus identity: `xteink-x4-shared-spi-bus`
- ownership authority: `target-xteink-x4 Vaachak layer`
- SPI pins: SCLK GPIO8, MOSI GPIO10, MISO GPIO7
- display chip select: GPIO21
- SD chip select: GPIO12
- display user registration: SSD1677 display
- storage user registration: microSD storage
- safe transaction ownership metadata
- active backend selection: `PulpCompatibility`

The ownership bridge exposes the fact that `SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK` is `true`.

## What remains active in Pulp

The active hardware executor remains the imported Pulp runtime through a compatibility backend:

```text
target-xteink-x4/src/vaachak_x4/physical/spi_bus_pulp_backend.rs
```

| Area | Current active executor | Vaachak status in this slice |
| --- | --- | --- |
| SPI peripheral setup | `vendor/pulp-os` imported runtime | Ownership entrypoint only |
| SPI transaction execution | `vendor/pulp-os` imported runtime | Not moved |
| SPI arbitration policy | `vendor/pulp-os` imported runtime | Not moved |
| SD probe / mount | `vendor/pulp-os` imported runtime | Not moved |
| SD FAT read/write/list behavior | `vendor/pulp-os` imported runtime | Not moved |
| SSD1677 display rendering | `vendor/pulp-os` imported runtime | Not moved |
| Display refresh behavior | `vendor/pulp-os` imported runtime | Not moved |
| Reader / file browser behavior | Existing app/runtime path | Not changed |

## Shared SPI users

| User | Role | Chip select | Shared pins | Active behavior owner |
| --- | --- | ---: | --- | --- |
| Display | SSD1677 e-paper display | GPIO21 | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 | `vendor/pulp-os` imported runtime |
| Storage | microSD over SPI | GPIO12 | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 | `vendor/pulp-os` imported runtime |

Vaachak now owns the metadata that describes these users and their transaction ownership. The actual chip-select toggling and SPI transactions remain in the Pulp compatibility backend.

## Non-goals

This deliverable does not:

- initialize SPI hardware
- transfer bytes on SPI
- toggle display or SD chip select lines
- implement a mutex or arbitration policy
- probe SD
- mount SD
- implement FAT read/write/list behavior
- initialize SSD1677
- draw pixels
- refresh the e-paper display
- change reader behavior
- change file browser behavior

## Relationship to existing contracts

This document supersedes `docs/architecture/spi-bus-runtime-contract.md` as the canonical SPI hardware ownership document.

The earlier SPI contract remains useful as metadata history. The new ownership bridge is the first SPI hardware-layer move because it establishes a Vaachak-owned entrypoint while preserving the working Pulp executor.

It also depends on the existing storage/read-only and storage probe/mount boundaries remaining above or beside this SPI ownership line:

- read-only storage boundary: file-level read-only adapter contract
- storage probe/mount contract: SD lifecycle metadata contract
- SPI runtime ownership bridge: shared physical bus ownership entrypoint

## Acceptance marker

```text
spi_bus_runtime_ownership_bridge=ok
```

## Next runtime behavior slice

The SPI bus ownership bridge is followed by the canonical arbitration runtime owner:

```text
docs/architecture/spi-bus-arbitration-runtime-ownership.md
spi_bus_arbitration_runtime_owner=ok
```

That layer moves Vaachak-owned logical arbitration request/grant metadata while keeping physical SPI transfer and chip-select execution in the Pulp compatibility backend.
