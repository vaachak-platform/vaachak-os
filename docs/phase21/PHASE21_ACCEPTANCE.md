# Phase 21 Acceptance

Run from the `vaachak-os` repo root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase21.sh
./scripts/check_phase21_storage_boundary.sh
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
phase20=x4-boundary-scaffold-ok
phase21=x4-storage-boundary-ok
phase18=x4-runtime-adapter-ok
```

Device acceptance:

- Boots successfully.
- Library opens.
- TXT opens and progress still restores.
- EPUB opens with real text, not ZIP bytes.
- EPUB progress still restores.
- TXT/EPUB bookmarks still work.
- Theme/menu/footer behavior is unchanged.
- Continue behavior is unchanged.
