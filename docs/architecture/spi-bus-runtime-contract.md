# SPI Bus Runtime Contract Consolidation

This metadata contract has been superseded by the canonical SPI hardware ownership document:

```text
docs/architecture/spi-bus-runtime-ownership.md
```

The earlier SPI bus runtime contract consolidated metadata for the shared Xteink X4 SPI bus. The ownership bridge now moves the SPI ownership authority into the Vaachak target layer while keeping the active hardware executor in the imported Pulp runtime.

## Shared SPI users

This section preserves the original shared SPI users metadata while pointing to the new ownership bridge.

| User | Role | Chip select | Shared pins | Current active behavior owner |
| --- | --- | ---: | --- | --- |
| Display | SSD1677 e-paper display | GPIO21 | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 | `vendor/pulp-os` imported runtime |
| Storage | microSD over SPI | GPIO12 | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 | `vendor/pulp-os` imported runtime |

## Current ownership summary

| Area | Current status |
| --- | --- |
| Vaachak SPI ownership entrypoint | `target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs` |
| Active backend | `PulpCompatibility` |
| Active hardware executor | `vendor/pulp-os` imported runtime |
| Display chip select | GPIO21 |
| SD chip select | GPIO12 |
| Shared pins | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 |
| SPI arbitration | Not moved |
| SD probe / mount | Not moved |
| SD FAT behavior | Not moved |
| Display refresh | Not moved |

## Compatibility marker

Older validators may still check the original consolidation marker:

```text
spi_bus_runtime_contract_consolidation=ok
```

The canonical ownership marker is now:

```text
spi_bus_runtime_ownership_bridge=ok
```
