# Phase 23 Acceptance

Run from the repository root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase23.sh
./scripts/check_phase23_display_boundary.sh
```

Optional full script validation:

```bash
PHASE23_RUN_CARGO=1 ./scripts/check_phase23_display_boundary.sh
```

Flash:

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected markers include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase19=x4-vaachak-runtime-facade-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase23=x4-display-boundary-ok
phase20=x4-boundary-scaffold-ok
phase18=x4-runtime-adapter-ok
```

Marker order is not important. Presence is important.

Device acceptance:

- Device boots.
- Library still opens.
- TXT still opens.
- EPUB still renders real text, not ZIP bytes.
- Back/Select/navigation behavior remains unchanged.
- Continue/progress/bookmark/theme behavior remains unchanged.
