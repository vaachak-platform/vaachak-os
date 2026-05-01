# Phase 16 — Reader Parity Consolidation

Goal: consolidate the reader path after Phase 15B so `target-xteink-x4` keeps the exact X4/Pulp reader runtime and does not regress to the fake EPUB raw-byte smoke path.

Expected firmware marker:

```text
phase16=x4-reader-parity-ok
```

## Scope

- TXT + EPUB progress
- TXT + EPUB bookmarks
- Reader footer action labels
- Reader menu actions
- Theme preset/state file support
- Continue behavior
- Reader parity checklist against `x4-reader-os-rs`

## Important rule

Do not recreate the EPUB reader in `target-xteink-x4/src/main.rs`.

The Phase 16 target should continue to use the vendored X4/Pulp runtime copied from:

```text
vendor/pulp-os/src/bin/main.rs
```

with only the required crate alias and the Phase 16 marker added.

## Apply

From the `vaachak-os` repo root:

```bash
chmod +x phase16_reader_parity_overlay/scripts/*.sh
./phase16_reader_parity_overlay/scripts/apply_phase16_reader_parity.sh
```

## Validate

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./phase16_reader_parity_overlay/scripts/check_phase16_reader_parity.sh
./phase16_reader_parity_overlay/scripts/make_phase16_parity_report.sh
```

## Flash

```bash
. "$HOME/export-esp.sh"

cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

## Manual smoke path

1. Boot to the X4 reader shell.
2. Confirm the serial log includes `phase16=x4-reader-parity-ok`.
3. Open a TXT or MD file.
4. Page forward, page back, exit, reopen, and confirm progress resumes.
5. Toggle bookmark on TXT/MD, exit, reopen, and confirm bookmark state is retained.
6. Open an EPUB/EPU file.
7. Confirm it renders extracted book text, not `PK`/ZIP garbage or “First readable bytes”.
8. Page forward, exit, reopen, and confirm EPUB progress resumes.
9. Toggle bookmark on EPUB, exit, reopen, and confirm bookmark state is retained.
10. Open reader menu actions and verify the available actions are consistent with the X4/Pulp reader.
11. Cycle theme/font/preset action if available and verify state persists after exiting/reopening.
12. Use Continue from the home/library flow and confirm it opens the last active book/page.

## Acceptance

Phase 16 is accepted when:

- cargo format/check/clippy pass for `target-xteink-x4` on `riscv32imc-unknown-none-elf`.
- The fake raw-byte EPUB function and strings are absent.
- `.EPUB/.EPU` routes through the vendored X4/Pulp `smol-epub` reader path.
- TXT and EPUB both preserve progress.
- TXT and EPUB both preserve bookmarks.
- Continue opens the last reader session.
- Theme/preset state persists through the same X4/Pulp state path.
- Footer/menu labels are verified against the parity checklist.
- Serial log includes `phase16=x4-reader-parity-ok`.
