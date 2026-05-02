# Phase 35 Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase35.sh
./scripts/check_phase35_physical_extraction_plan.sh
./scripts/check_phase35_storage_state_io_seam.sh
./scripts/check_phase35_no_hardware_regression.sh
```

## Boot Marker

Normal boot marker remains:

```text
vaachak=x4-runtime-ready
```

No old phase marker spam should be printed.

## Hardware Acceptance After User Flash

```text
- Device boots.
- Library opens.
- TXT/MD opens.
- EPUB/EPU opens with real text.
- Continue works.
- Bookmarks work.
- Theme/menu/footer behavior remains unchanged.
- Input navigation remains unchanged.
- Display geometry/refresh remains unchanged.
```

## Vendor Acceptance

No tracked edits under:

```text
vendor/pulp-os
vendor/smol-epub
```
