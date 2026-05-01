# Phase 28 Acceptance

Run from the `vaachak-os` repo root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase28.sh
./scripts/check_phase28_boundary_contract_smoke.sh
```

Flash:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected markers include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase19=x4-vaachak-runtime-facade-ok
phase28=x4-boundary-contract-smoke-ok
phase27=x4-display-contract-smoke-ok
phase25=x4-storage-contract-smoke-ok
phase26=x4-input-contract-smoke-ok
phase24=x4-boundary-contract-ok
phase23=x4-display-boundary-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase20=x4-boundary-scaffold-ok
phase18=x4-runtime-adapter-ok
```

Marker ordering may differ slightly. Presence matters more than order.
