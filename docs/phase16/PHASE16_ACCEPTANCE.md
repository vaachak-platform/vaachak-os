# Phase 16 Acceptance

## Build gate

Run from repo root:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
```

Expected: all pass.

## Guard gate

```bash
./phase16_reader_parity_overlay/scripts/check_phase16_reader_parity.sh
```

Expected: zero failures.

Warnings are allowed only when a symbol is implemented under a different name in the vendored X4/Pulp tree. If warnings appear, inspect the generated report:

```bash
./phase16_reader_parity_overlay/scripts/make_phase16_parity_report.sh
sed -n '1,240p' docs/phase16/PHASE16_PARITY_REPORT.local.md
```

## Firmware marker gate

Expected serial line after boot:

```text
phase16=x4-reader-parity-ok
```

## Anti-regression gate

These strings/functions must not appear in active target or vendored source:

```text
First readable bytes
ZIP container parsed
run_epub_reader_page_storage_smoke
EPUB raw-byte smoke
ensure_pulp_dir_async
```

## Device behavior gate

| Area | Expected result |
|---|---|
| TXT open | TXT/MD opens and renders page text. |
| TXT progress | Reopen resumes last page/offset. |
| TXT bookmarks | Bookmark toggle persists and reloads. |
| EPUB open | EPUB/EPU renders extracted book text through `smol-epub`, not ZIP bytes. |
| EPUB progress | Reopen resumes last EPUB page/chapter position. |
| EPUB bookmarks | Bookmark toggle persists and reloads for EPUB. |
| Footer labels | Reader footer labels match the X4/Pulp action mapping. |
| Reader menu | Reader menu actions are reachable and stable. |
| Theme preset/state | Theme/font/preset state persists across reader reopen. |
| Continue | Continue opens the last active book/session. |
