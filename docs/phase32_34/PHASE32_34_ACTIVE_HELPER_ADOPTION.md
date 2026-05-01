# Phase 32–34 — Active Helper Adoption Consolidation

## Purpose

Phase 32–34 adopts Vaachak-owned pure helper logic in the active runtime for storage path names, input semantics, and display geometry.

The active imported runtime calls pure Vaachak probes for:

```text
storage path helper adoption
input semantic helper adoption
display geometry helper adoption
```

Those probes do not print and do not perform IO.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```

## Accepted Baseline

Phase 31 is accepted.

TXT and EPUB reader behavior works.

## Ownership Model

Vaachak owns pure helper contracts under:

```text
target-xteink-x4/src/vaachak_x4/contracts/
```

Primary helper modules:

```text
storage_path_helpers.rs
storage_state_contract.rs
input_semantics.rs
display_geometry.rs
```

Imported Pulp still owns physical behavior:

```text
SD/SPI initialization
filesystem IO
EPUB cache IO
ADC sampling and debounce
SSD1677 refresh and strip rendering
reader app construction
TXT/EPUB rendering
progress/bookmark/theme IO
```

This phase does not move physical hardware behavior. Physical SD/SPI,
filesystem, input, and display behavior still belongs to the imported Pulp
runtime.

## In Scope

```text
storage path/name helpers
input semantic helpers
display geometry helpers
active runtime pure adoption probe(s)
host-side tests for pure helpers
```

## Out of Scope

```text
physical SD/SPI behavior
filesystem IO
ADC sampling/debounce
SSD1677 SPI/refresh behavior
reader app internals
EPUB parsing/rendering
```
