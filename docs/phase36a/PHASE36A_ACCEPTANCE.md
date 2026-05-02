# Phase 36A Acceptance

## Required validation

Run from the `vaachak-os` repo root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase36a.sh
./scripts/check_phase36a_active_input_semantic_mapping.sh
./scripts/check_phase36a_no_input_hardware_regression.sh
```

## Required active source checks

```text
- target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs exists.
- input/mod.rs exports active_semantic_mapper.
- pulp_reader_runtime.rs calls VaachakActiveInputSemanticMapper::active_runtime_preflight().
- pulp_reader_runtime.rs uses VaachakActiveInputSemanticMapper::new_imported_button_mapper().
- pulp_reader_runtime.rs does not directly import ButtonMapper.
- pulp_reader_runtime.rs does not directly call ButtonMapper::new().
- vendor/pulp-os and vendor/smol-epub have no tracked edits.
- normal boot marker remains vaachak=x4-runtime-ready.
- old phase markers are not printed during normal boot.
```

## Required negative checks

The Vaachak active semantic mapper must not own physical input behavior:

```text
Adc::new
read_oneshot
decode_ladder
ROW1_THRESHOLDS
ROW2_THRESHOLDS
DEBOUNCE_MS
REPEAT_MS
Instant::now
input_task
InputDriver::new
```

## Device acceptance

After flashing:

```text
- Device boots.
- TXT and EPUB still open.
- Navigation, select/open, back, page turn, menu/bookmark behavior still work.
```
