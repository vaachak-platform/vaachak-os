# Phase 30 Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync.sh
./scripts/check_vaachak_x4_runtime.sh
```

## Required Boot Marker

After flashing:

```text
vaachak=x4-runtime-ready
```

## Required Negative Checks

Active source must not contain fake EPUB smoke behavior:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

Normal boot must not emit old phase markers.

## Hardware Acceptance

After flashing:

```text
- Device boots.
- Library opens.
- TXT/MD reader still works.
- EPUB/EPU reader still renders real text through smol-epub.
- Continue still works.
- Bookmarks still work.
- Theme/menu/footer behavior unchanged.
```

## Vendor Acceptance

These directories must not be edited:

```text
vendor/pulp-os
vendor/smol-epub
```
