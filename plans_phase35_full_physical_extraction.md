# Phase 35 Full Physical Behavior Extraction Plan

## Goal

Move the active Xteink X4 runtime from imported Pulp behavior to Vaachak-owned behavior for all required areas in one deliverable.

## Required active ownership areas

```text
1. Storage state IO
2. Input semantic mapping
3. Display geometry helper usage
4. Input ADC/debounce
5. SD/SPI arbitration
6. SSD1677 refresh/strip rendering
7. Reader app internals
```

## Implementation strategy

1. Preserve the current working baseline in backups.
2. Copy needed Pulp runtime/app/driver code into `target-xteink-x4/src/vaachak_x4/`.
3. Adapt copied code to use Vaachak contracts/helpers.
4. Make `main.rs` boot Vaachak-owned physical runtime.
5. Remove active black-box dependency on imported Pulp app/reader/runtime.
6. Keep `vendor/pulp-os` and `vendor/smol-epub` untouched.
7. Keep `vendor/smol-epub` as the EPUB parser dependency if needed.
8. Validate with cargo and Phase 35 Full scripts.

## Expected boot marker

```text
vaachak=x4-physical-runtime-owned
```

## Acceptance

All seven areas must be active. Partial implementation fails.
