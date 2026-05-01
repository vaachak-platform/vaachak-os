# Phase 24 Acceptance

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase24.sh
./scripts/check_phase24_boundary_contract.sh
```

Flash:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected serial markers include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase19=x4-vaachak-runtime-facade-ok
phase24=x4-boundary-contract-ok
phase23=x4-display-boundary-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase20=x4-boundary-scaffold-ok
phase18=x4-runtime-adapter-ok
```

The exact order may vary based on facade marker order, but all markers must appear.
