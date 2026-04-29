# VaachakOS Bootstrap Phase 3 — X4 Input + Power HAL Extraction

Status: implementation slice

This phase extracts the safest real X4 HAL behavior first: input semantics and
battery/power semantics. It intentionally does **not** move the SSD1677 display
or SD/FAT implementation yet.

## What moved from proving-ground truth into HAL code

- X4 ADC ladder threshold constants
- power-button priority over ADC ladders
- debounce / long-press / repeat timing policy
- hold-state suppression after consumed navigation
- GPIO0 battery divider model: 100K/100K, multiply ADC-domain mV by 2
- Li-ion discharge curve used by the current X4 runtime
- charge-state reporting seam
- light-sleep request recording seam for future target glue

## Still deferred

- real esp-hal ADC pin ownership
- real GPIO3 interrupt binding
- real GPIO20 charge-detect pin binding
- real deep-sleep entry
- display driver migration
- storage driver migration

## Validation commands

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Next recommended phase

Bootstrap Phase 4 should extract the Storage HAL adapter plan or target runtime
adapter scaffolding. Keep SSD1677 display migration until input/power/storage
seams are boring and tested.
