# Prepared Reader Stabilization Cleanup

## What this stabilizes

- Prepared TXT/EPUB cache rendering remains the preferred path for mixed Latin + Devanagari books.
- Reader header shows `Prep Pg ...` when a prepared cache is active.
- Reader header shows `Read cache:<BOOKID> err:<CODE>` only when the prepared cache fails to open.
- Generic temporary `err:OPEN` debug text is removed from normal reading chrome.
- Wi-Fi Transfer keeps two tabs: `Original Transfer` and `Chunked Resume`.
- USB serial transfer scaffolding is removed when present because X4 does not provide normal USB mass-storage file transfer.

## Large FCACHE limits

The prepared reader now keeps real-book limits large enough for `YEARLY_H.TXT` style caches:

```text
MAX_META_BYTES  = 1024
MAX_INDEX_BYTES = 4 * 1024
MAX_FONT_BYTES  = 16 * 1024
MAX_PAGE_BYTES  = 24 * 1024
MAX_PAGES       = 192
MAX_GLYPHS      = 1024
```

These limits are intentionally higher than the original smoke-test values because real books can have larger `PAGES.IDX` files and larger `.VRN` pages.

## Validation checklist

```bash
cargo fmt --all --check
cargo check --workspace --target riscv32imc-unknown-none-elf
cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
git diff --check
```

After flashing:

```text
YEARLY_H.TXT:
  Expected header: Prep Pg ...
  Expected body: Sanskrit/Hindi render without ??? placeholders.

Mixed EPUB smoke:
  Expected header: Prep Pg ...
  Expected body: Hindi/Sanskrit render without ??? placeholders.
```

If a prepared cache fails, the header should show one of:

```text
Read cache:<BOOKID> err:MISSING
Read cache:<BOOKID> err:META
Read cache:<BOOKID> err:BOOK
Read cache:<BOOKID> err:INDEX
Read cache:<BOOKID> err:FONT_MISSING
Read cache:<BOOKID> err:FONT
Read cache:<BOOKID> err:PAGE
Read cache:<BOOKID> err:TOO_LARGE
```

Use the `Chunked Resume` tab for large `/FCACHE/<BOOKID>` folders.
