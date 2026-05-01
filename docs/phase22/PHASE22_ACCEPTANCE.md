# Phase 22 Acceptance

Run from the repository root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase22.sh
./scripts/check_phase22_input_boundary.sh
```

Flash:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected boot markers include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase19=x4-vaachak-runtime-facade-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase20=x4-boundary-scaffold-ok
phase18=x4-runtime-adapter-ok
```

Device acceptance:

```text
Boot succeeds
Input still works
Library navigation still works
Reader page navigation still works
Back returns to library
TXT/EPUB progress still works
TXT/EPUB bookmarks still work
No fake EPUB raw-byte path returns
```
