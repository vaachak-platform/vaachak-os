# Phase 17 Acceptance Criteria

## Build acceptance

Run from the `vaachak-os` repo root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync.sh
./scripts/check_phase17_reader_refactor.sh
```

Optional full script validation:

```bash
PHASE17_RUN_CARGO=1 ./scripts/check_phase17_reader_refactor.sh
```

## Flash acceptance

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

## Device acceptance

The device should:

```text
Boot successfully.
Open the library/files screen.
Open TXT/MD files.
Open EPUB/EPU files using real text from the smol-epub path.
Never show raw ZIP bytes for EPUB files.
Return to library/files with Back.
Preserve Continue behavior.
Preserve TXT and EPUB progress.
Preserve TXT and EPUB bookmarks.
Preserve reader footer/menu/theme behavior from Phase 16.
```

## Source acceptance

The active source must not contain the fake EPUB smoke reader:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

The target main should track the vendored X4/Pulp main with only approved Vaachak differences.

## Expected markers

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
```
