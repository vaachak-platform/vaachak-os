# Phase 18 Acceptance

## Build acceptance

Run from the repository root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase18.sh
./scripts/check_phase18_runtime_adapter.sh
```

Optional all-in-one script validation:

```bash
PHASE18_RUN_CARGO=1 ./scripts/check_phase18_runtime_adapter.sh
```

## Firmware acceptance

Flash:

```bash
cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Monitor log should include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase18=x4-runtime-adapter-ok
```

## Device acceptance

Verify on Xteink X4:

```text
1. Boot succeeds.
2. Library/Files opens.
3. TXT/MD opens.
4. EPUB/EPU opens with real book text, not ZIP/PK garbage.
5. Back returns to library.
6. Continue still opens the previous book/page.
7. TXT bookmark behavior still works.
8. EPUB bookmark behavior still works.
9. Reader footer/menu/theme behavior remains unchanged from Phase 16.
```
