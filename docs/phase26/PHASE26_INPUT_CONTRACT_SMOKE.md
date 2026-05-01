# Phase 26 — Vaachak Input Contract Smoke

Phase 26 adds a Vaachak-owned pure input contract smoke layer without moving the working physical input path out of the imported X4/Pulp runtime.

## What this phase owns

- Physical input pin metadata contract for GPIO1, GPIO2, and GPIO3.
- Required logical button roles: Back, Select, Up, Down, Left, Right, and Power.
- Reader/library semantic action mapping documentation in code.
- Pure validation helpers for the input contract.
- Boot marker emission through the existing Vaachak runtime facade.

Expected marker:

```text
phase26=x4-input-contract-smoke-ok
```

## What this phase does not move

- ADC sampling.
- Button ladder thresholds/calibration.
- Debounce handling.
- Repeat handling.
- Physical button event routing.
- Reader menu/button dispatch inside the imported runtime.

Those behaviors remain owned by `vendor/pulp-os` for Phase 26.

## Files added

```text
target-xteink-x4/src/runtime/input_contract_smoke.rs
scripts/check_reader_runtime_sync_phase26.sh
scripts/check_phase26_input_contract_smoke.sh
scripts/revert_phase26_input_contract_smoke.sh
```

## Validation

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase26.sh
./scripts/check_phase26_input_contract_smoke.sh
```
