# Phase 32–34 Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase32_34.sh
./scripts/check_phase32_34_active_helper_adoption.sh
```

## Required Boot Marker

After flashing:

```text
vaachak=x4-runtime-ready
```

Normal boot must emit only this Vaachak runtime-ready marker. It must not print
`phase32=...`, `phase33=...`, `phase34=...`, or old phase markers.

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

Pure helper modules must not own physical behavior.

The active imported runtime must call Vaachak-owned pure helper probes for
storage path names, input semantics, and display geometry. Those calls must not
change SD/SPI/filesystem/input/display IO or reader behavior.

## Vendor Acceptance

These must remain untouched:

```text
vendor/pulp-os
vendor/smol-epub
```
