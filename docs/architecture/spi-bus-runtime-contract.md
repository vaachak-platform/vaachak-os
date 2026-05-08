# SPI Bus Runtime Contract Consolidation

This document is the canonical Vaachak-owned SPI bus runtime contract for the Xteink X4 hardware boundary.

## Purpose

The goal of this slice is to consolidate SPI bus ownership metadata before moving any hardware-layer behavior.

The contract records:

- shared SPI pins
- shared SPI users
- chip-select ownership facts
- timing facts inherited from the working Pulp runtime
- ownership boundaries for arbitration, SD/FAT, and display behavior

This is documentation and contract metadata only. It does not move SPI arbitration or any physical behavior.

## Active ownership model

| Area | Current active owner | Vaachak ownership in this slice |
| --- | --- | --- |
| SPI peripheral setup | `vendor/pulp-os` imported runtime | Metadata only |
| SPI arbitration | `vendor/pulp-os` imported runtime | Metadata only |
| SD probe / mount | `vendor/pulp-os` imported runtime | No behavior moved |
| FAT / file I/O behavior | `vendor/pulp-os` imported runtime | No behavior moved |
| SSD1677 display driver | `vendor/pulp-os` imported runtime | No behavior moved |
| Display refresh / strip rendering | `vendor/pulp-os` imported runtime | No behavior moved |

## Shared SPI users

| User | Device role | Chip select | Shared pins | Active behavior owner |
| --- | --- | ---: | --- | --- |
| Display | SSD1677 e-paper display | GPIO21 | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 | `vendor/pulp-os` imported runtime |
| Storage | microSD over SPI | GPIO12 | SCLK GPIO8, MOSI GPIO10, MISO GPIO7 | `vendor/pulp-os` imported runtime |

The contract requires that display and storage chip-select lines are not selected at the same time. This rule is metadata in this slice; active arbitration remains imported.

## Timing metadata

| Fact | Value |
| --- | ---: |
| SD probe speed | 400 kHz |
| Operational speed | 20 MHz |
| DMA channel | 0 |
| TX DMA buffer | 4096 bytes |
| RX DMA buffer | 4096 bytes |
| SD init before display traffic | true |

## New files

```text
 target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_contract.rs
 target-xteink-x4/src/vaachak_x4/contracts/spi_bus_runtime_contract_smoke.rs
 scripts/validate_spi_bus_runtime_contract_consolidation.sh
 docs/architecture/spi-bus-runtime-contract.md
```

## Explicit non-goals

This deliverable does not:

- initialize SPI
- create a shared SPI bus object
- select or deselect display/storage chip-select lines at runtime
- probe or mount SD
- move FAT behavior
- move SD read/write behavior
- initialize SSD1677
- refresh the display
- move display strip rendering
- change reader behavior
- change file browser behavior

## Relationship to storage read-only boundary

The storage read-only boundary remains above this hardware contract. It may describe file-level read-only adapter behavior, but the physical bus, SD probe/mount, and FAT implementation remain Pulp-owned.

## Acceptance marker

```text
spi_bus_runtime_contract_consolidation=ok
```
