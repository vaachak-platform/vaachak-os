# VaachakOS Bootstrap Phase 12 — X4 TXT Bookmark Smoke

This replacement-file pack adds a narrow TXT-only bookmark smoke on top of Phase 11 pagination/progress.

It intentionally does not add EPUB, global Home bookmarks, highlights, or sync.

## Apply

```bash
cd /home/mindseye73/Documents/projects/vaachak-os
unzip -o /path/to/vaachak-os-bootstrap-phase12-x4-txt-bookmark-smoke.zip
```

## Validate host checks

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Flash X4

```bash
. "$HOME/export-esp.sh"

cargo run -p target-xteink-x4 \
  --release \
  --target riscv32imc-unknown-none-elf
```

## Expected behavior

- Library scans `/BOOKS` first and shows TXT/MD files.
- Select opens a TXT/MD file.
- Down / Right advances reader page.
- Up goes to previous reader page.
- Select in Reader toggles bookmark at the current TXT offset.
- Back / Left returns to Library.
- Progress remains stored in `state/<BOOKID>.PRG`.
- TXT bookmarks are stored in `state/<BOOKID>.BKM`.

## Expected serial markers

```text
phase12=x4-txt-bookmark-smoke-ready
phase12: bookmarks loaded state/<BOOKID>.BKM count=... current=...
phase12: Bookmark saved file=... offset=... page=X/Y count=... state/<BOOKID>.BKM
phase12: Bookmark removed file=... offset=... page=X/Y count=... state/<BOOKID>.BKM
phase12=x4-txt-bookmark-smoke-ok
```
