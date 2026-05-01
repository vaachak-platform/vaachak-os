# Phase 20 Acceptance

## Build acceptance

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase20.sh
./scripts/check_phase20_boundary_scaffold.sh
```

Optional full script validation:

```bash
PHASE20_RUN_CARGO=1 ./scripts/check_phase20_boundary_scaffold.sh
```

## Device acceptance

Flash:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected serial markers:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase19=x4-vaachak-runtime-facade-ok
phase20=x4-boundary-scaffold-ok
phase18=x4-runtime-adapter-ok
```

Marker order may differ slightly, but all markers must appear.

## Reader regression acceptance

On device:

```text
Boot succeeds
Library opens
TXT opens
EPUB opens with real text, not ZIP bytes
Back returns to library
Continue works
Bookmarks work
Theme/menu/footer behavior remains unchanged
```
