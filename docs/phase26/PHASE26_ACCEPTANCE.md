# Phase 26 Acceptance

Phase 26 is accepted when all of the following pass:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase26.sh
./scripts/check_phase26_input_contract_smoke.sh

cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected boot markers include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase19=x4-vaachak-runtime-facade-ok
phase25=x4-storage-contract-smoke-ok
phase26=x4-input-contract-smoke-ok
phase24=x4-boundary-contract-ok
phase23=x4-display-boundary-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase20=x4-boundary-scaffold-ok
phase18=x4-runtime-adapter-ok
```

Marker order may differ slightly as long as all markers are present and the device boots.
