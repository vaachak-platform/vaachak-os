# Reader Runtime Stability Gate

This gate validates the current reader-first X4 runtime without adding a new architecture layer.

## Scope

- Validate `YEARLY_H.TXT` prepared-cache open path on device.
- Validate mixed EPUB prepared-cache open path on device.
- Keep successful prepared-cache chrome clean: `Prep Pg ...`.
- Keep cache-failure diagnostics only on failure: `Read cache:<BOOKID> err:<CODE>`.
- Ensure successful reads do not show temporary `err:OPEN` or debug-only reader chrome.
- Confirm progress restore for prepared TXT and prepared EPUB.
- Confirm Back returns to Library/Home without state corruption.
- Confirm Reader settings still apply after repository cleanup.
- Confirm Wi-Fi Transfer can still upload large `/FCACHE/<BOOKID>` folders through Chunked Resume.

## Build gate

Run from the repository root:

```bash
./scripts/validate_reader_runtime_stability.sh
```

This script checks repository hygiene, host builds, embedded checks, clippy, the flashable `target-xteink-x4` binary, and the active Pulp-derived runtime build.

## Why `target-xteink-x4/build.rs` exists

The root Cargo config intentionally does not add `-Tlinkall.x`, because parent Cargo config also affects `vendor/pulp-os` and can make the active runtime receive the linker script twice.

`target-xteink-x4/build.rs` adds `-Tlinkall.x` only when building the embedded `riscv32imc-unknown-none-elf` target. This keeps host workspace checks clean while restoring the linker script required by the flashable Vaachak target.

## Device checklist

### Prepared TXT

1. Upload or keep the prepared cache for `YEARLY_H.TXT` under `/FCACHE/<BOOKID>`.
2. Open `YEARLY_H.TXT` from the Library.
3. Expected successful chrome: `Prep Pg ...`.
4. Expected body: prepared glyph output renders the intended Devanagari/Hindi text.
5. Not expected on success: `err:OPEN`, `Read cache:<BOOKID> err:<CODE>`, or debug-only cache text.

### Prepared EPUB smoke

1. Upload or keep the mixed EPUB smoke book and its prepared cache.
2. Open the EPUB from the Library.
3. Expected successful chrome: `Prep Pg ...`.
4. Page forward and backward.
5. Press Back and return to Library/Home.
6. Reopen the EPUB and confirm progress restore.

### Failure diagnostic check

1. Temporarily rename one prepared cache directory, for example `/FCACHE/<BOOKID>` to `/FCACHE/<BOOKID>.OFF`.
2. Open the matching book.
3. Expected failure chrome: `Read cache:<BOOKID> err:<CODE>`.
4. The reader must not freeze.
5. Restore the cache directory name and reopen the book.
6. Expected successful chrome returns to `Prep Pg ...`.

### Settings and transfer checks

1. Change Reader font/theme/progress settings.
2. Reopen prepared TXT and prepared EPUB books.
3. Confirm settings still apply.
4. Open Wi-Fi Transfer.
5. Confirm both `Original Transfer` and `Chunked Resume` tabs are present.
6. Use `Chunked Resume` to upload a large `/FCACHE/<BOOKID>` folder.
