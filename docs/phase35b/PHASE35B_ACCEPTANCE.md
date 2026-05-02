# Phase 35B Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase35b.sh
./scripts/check_phase35b_storage_state_io_wiring.sh
./scripts/check_phase35b_no_vendor_or_hardware_regression.sh
```

## Required Boot Marker

After flashing:

```text
vaachak=x4-runtime-ready
```

Normal boot remains `vaachak=x4-runtime-ready` only.

No normal boot path prints:

```text
phase35=
phase35b=
```

## Required Behavior

```text
TXT opens
EPUB/EPU opens with real text
Continue works
Bookmarks work
Theme/menu/footer behavior unchanged
```

## Negative Checks

Phase 35B wires a Vaachak-owned storage state IO seam into active runtime as a
path-only/no-op preflight.

Phase 35B does not replace progress/bookmark/theme persistence.

Physical SD/SPI/FAT IO remains owned by the imported Pulp runtime.

`vendor/pulp-os` and `vendor/smol-epub` are untouched.

Active source must not contain fake EPUB smoke behavior:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

The runtime bridge must not own physical IO:

```text
SdCard::new
open_raw_volume
open_file_in_dir
read(
write(
spi::master
RefCellDevice
BlockDevice
AsyncVolumeManager
```
