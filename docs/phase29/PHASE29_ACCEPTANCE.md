# Phase 29 Acceptance

Run from the `vaachak-os` repository root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase29.sh
./scripts/check_phase29_storage_path_helpers.sh
```

Flash:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected boot marker:

```text
phase29=x4-storage-path-helpers-ok
```

Previous development phase markers should no longer be printed in the active boot log.
