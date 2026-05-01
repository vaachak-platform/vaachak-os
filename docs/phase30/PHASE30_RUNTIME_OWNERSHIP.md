# Phase 30 — Vaachak Runtime Ownership Consolidation

## Purpose

Phase 30 turns the target structure into a Vaachak-owned runtime namespace while preserving the working imported Pulp reader runtime.

Expected marker:

```text
vaachak=x4-runtime-ready
```

## Ownership Model

Vaachak owns:

```text
target-xteink-x4/src/main.rs
target-xteink-x4/src/vaachak_x4/**
```

Imported Pulp still owns:

```text
reader app construction
TXT/EPUB reader behavior
smol-epub integration
progress/bookmark/theme IO behavior
SD/SPI runtime behavior
input runtime behavior
display runtime behavior
```

Vendored source remains read-only for this phase:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

## Target Structure

```text
target-xteink-x4/src/
  main.rs
  vaachak_x4/
    mod.rs
    boot.rs
    runtime.rs
    contracts/
      mod.rs
      boundary_contract.rs
      boundary_contract_smoke.rs
      storage.rs
      input.rs
      display.rs
      storage_state_contract.rs
      storage_path_helpers.rs
      input_contract_smoke.rs
      display_contract_smoke.rs
    imported/
      mod.rs
      pulp_reader_runtime.rs
```

## Non-Goals

Do not move hardware behavior in Phase 30.

Do not move:

```text
SD/SPI initialization
filesystem reads/writes
ADC sampling
button debounce/repeat
SSD1677 init
display refresh
strip rendering
reader app construction
EPUB parsing/rendering
bookmark/progress/theme IO
```

## Boot Marker Policy

Phase 30 normal boot should print:

```text
vaachak=x4-runtime-ready
```

Old phase markers should not print during normal boot.

They may remain in docs, tests, or compatibility constants.
