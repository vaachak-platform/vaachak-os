# Phase 35C-0 Acceptance

## Required Commands

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_phase35c0_reader_state_facade.sh
./scripts/check_phase35c0_no_active_io_takeover.sh
```

## Acceptance Criteria

```text
- Vaachak reader state facade exists.
- Metadata and theme record encode/decode helpers exist.
- Theme and metadata state filenames use Vaachak storage path helpers.
- No vendor files are edited.
- Active reader persistence is not claimed as moved in Phase 35C-0.
- Normal boot marker remains vaachak=x4-runtime-ready.
```

## Hardware Scope

No flashing is required for Phase 35C-0.

There is no intended user-visible behavior change.
