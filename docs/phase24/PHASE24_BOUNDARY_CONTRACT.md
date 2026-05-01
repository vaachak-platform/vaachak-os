# Phase 24 — Boundary Contract Consolidation

## Goal

Phase 24 consolidates Vaachak-owned display, input, and storage boundary metadata into a single contract layer:

```text
target-xteink-x4/src/runtime/boundary_contract.rs
```

This phase does **not** move physical hardware behavior.

## Current ownership

```text
Vaachak-owned:
- display boundary metadata
- input boundary metadata
- storage boundary metadata
- consolidated boundary contract
- boot marker coordination

Imported X4/Pulp-owned:
- SSD1677 initialization
- display refresh and strip rendering
- SPI bus transactions
- ADC input reads and debounce/repeat handling
- SD card initialization and filesystem IO
- reader app behavior
- EPUB parsing through smol-epub
```

## Contract rule

Phase 24 adds:

```text
phase24=x4-boundary-contract-ok
```

while keeping these prior markers:

```text
phase20=x4-boundary-scaffold-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase23=x4-display-boundary-ok
```

## Non-goals

Do not move:

```text
- physical display init
- physical input polling
- physical storage IO
- reader app construction
- EPUB parsing
- bookmark/progress/theme behavior
```
