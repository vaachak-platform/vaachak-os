# Display Runtime Ownership

This document is the canonical Vaachak-owned ownership record for the Xteink X4 SSD1677 e-paper display runtime.

## Purpose

This deliverable moves display runtime ownership authority into the `target-xteink-x4` Vaachak layer while keeping the known-good Pulp runtime as the active SSD1677/e-paper rendering executor.

This is an ownership bridge, not a draw/refresh migration.

## What moved to Vaachak

Vaachak now owns the public display runtime ownership entrypoint:

```text
target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs
```

That entrypoint owns metadata for:

- display runtime identity: `xteink-x4-ssd1677-display-runtime`
- ownership authority: `target-xteink-x4 Vaachak layer`
- active backend selection: `PulpCompatibility`
- panel identity: SSD1677 e-paper display
- display pins: CS GPIO21, DC GPIO4, RST GPIO5, BUSY GPIO6
- shared SPI pins: SCLK GPIO8, MOSI GPIO10, MISO GPIO7
- storage chip select on the same bus: GPIO12
- native panel geometry: 800x480
- logical geometry: 480x800
- rotation: 270 degrees
- strip rows: 40
- operation metadata for full-refresh, partial-refresh, surface-render, and busy-wait ownership

The ownership bridge exposes:

```text
DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true
```

## Dependency on SPI bus runtime ownership

The display runtime owner depends on the accepted shared SPI bus ownership line:

```text
target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs
```

The display owner checks that:

- `VaachakSpiBusRuntimeOwner::ownership_bridge_ok()` is true
- the display user is registered on the shared SPI bus
- the display chip select remains GPIO21
- storage remains a separate shared-SPI user on GPIO12

This keeps display and SD sharing explicit while avoiding a premature arbitration move.

## What remains active in Pulp

The active display hardware executor remains the imported Pulp runtime through:

```text
target-xteink-x4/src/vaachak_x4/physical/display_pulp_backend.rs
```

| Area | Current active executor | Vaachak status in this slice |
| --- | --- | --- |
| SSD1677 init | `vendor/pulp-os` imported runtime | Ownership entrypoint only |
| Draw/pixel rendering | `vendor/pulp-os` imported runtime | Not moved |
| Full e-paper refresh | `vendor/pulp-os` imported runtime | Not moved |
| Partial e-paper refresh | `vendor/pulp-os` imported runtime | Not moved |
| BUSY wait | `vendor/pulp-os` imported runtime | Not moved |
| Display SPI transactions | `vendor/pulp-os` imported runtime | Not moved |
| SPI arbitration | Existing Pulp-compatible path | Not moved |
| SD probe/mount | Existing storage runtime path | Not changed |
| SD/FAT read-only behavior | Existing storage runtime path | Not changed |
| Reader / file browser behavior | Existing app/runtime path | Not changed |

## Non-goals

This deliverable does not:

- initialize SSD1677
- send SSD1677 commands
- draw pixels
- render packed pixels
- allocate or mutate a framebuffer
- execute full refresh
- execute partial refresh
- wait on BUSY
- transfer bytes on SPI
- toggle display chip select
- move SPI arbitration
- change SD probe/mount behavior
- change SD/FAT read/write/list behavior
- change reader behavior
- change file browser behavior

## Static validation

Run:

```bash
cargo fmt --all
./scripts/validate_display_runtime_owner.sh
cargo build
```

Expected marker:

```text
display_runtime_owner=ok
```

## Hardware smoke

Expected device behavior is unchanged:

- boot normally
- Home/category dashboard appears
- buttons navigate normally
- display refresh looks unchanged
- file browser still opens
- SD file listing still works
- TXT/EPUB open path still works
- Back navigation works
- no display freeze
- no new SD/FAT error

## Acceptance marker

```text
display_runtime_owner=ok
```
