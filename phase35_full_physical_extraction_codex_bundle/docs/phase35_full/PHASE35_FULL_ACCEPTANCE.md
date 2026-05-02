# Phase 35 Full Acceptance

## Required commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_phase35_full_no_vendor_edits.sh
./scripts/check_phase35_full_runtime_ownership.sh
./scripts/check_phase35_full_no_scaffold_only.sh
./scripts/check_phase35_full_physical_extraction.sh
./scripts/check_phase35_full_device_acceptance_notes.sh
```

## Required boot marker

```text
vaachak=x4-physical-runtime-owned
```

## Required device behavior

After flashing:

```text
- Device boots.
- Library opens.
- TXT/MD opens.
- EPUB/EPU opens with real text through smol-epub.
- Continue works.
- Bookmarks work.
- Theme/menu/footer behavior works.
- Input navigation works.
- Display layout/rotation/refresh works.
```

## Failure conditions

Any missing behavior area fails this phase.
