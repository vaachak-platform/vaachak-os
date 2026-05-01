# Phase 31 Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase31.sh
./scripts/check_phase31_storage_path_adoption.sh
```

## Required Boot Marker

After flashing:

```text
vaachak=x4-runtime-ready
```

Normal boot must not print `phase31=...` or any old phase marker.

## Required Behavior

After flashing:

```text
- Device boots.
- TXT/MD reader still works.
- EPUB/EPU reader still renders real book text.
- Continue still works.
- Bookmarks still work.
- Theme/menu/footer behavior unchanged.
```

## Negative Checks

Active source must not contain fake EPUB smoke behavior:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

Storage path helper modules must not own physical IO:

```text
SdCard::new
open_raw_volume
open_file_in_dir
read(
write(
spi::master
RefCellDevice
```

The active imported runtime may only call Vaachak storage helpers for pure
deterministic path/name adoption checks. It must not route SD, SPI, FAT,
progress, bookmark, theme, or EPUB cache IO through Vaachak code in Phase 31.

## Vendor Acceptance

These must remain untouched:

```text
vendor/pulp-os
vendor/smol-epub
```
