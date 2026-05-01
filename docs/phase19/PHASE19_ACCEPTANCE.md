# Phase 19 Acceptance

## Build acceptance

Run from repo root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync_phase19.sh
./scripts/check_phase19_runtime_facade.sh
```

Optional full script validation:

```bash
PHASE19_RUN_CARGO=1 ./scripts/check_phase19_runtime_facade.sh
```

## Flash acceptance

```bash
cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

Expected serial markers:

```text
phase19=x4-vaachak-runtime-facade-ok
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase18=x4-runtime-adapter-ok
```

Marker order may show Phase 19 first because the Vaachak facade prints before delegating into the imported runtime.

## Device acceptance

- Boots successfully.
- Library opens.
- TXT/MD opens.
- EPUB/EPU opens with real text, not raw ZIP bytes.
- Back returns to Library.
- Continue behavior still works.
- TXT + EPUB bookmarks still work.
- Theme/menu/footer behavior unchanged.

## Non-goals

Phase 19 must not:

- Rename Pulp UI strings.
- Reimplement `smol-epub`.
- Modify `vendor/pulp-os/src/apps/reader/*`.
- Modify `vendor/pulp-os/kernel/*`.
- Modify `vendor/smol-epub/*`.
