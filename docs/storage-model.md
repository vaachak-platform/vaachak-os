# VaachakOS Reader Storage Model

Status: Bootstrap Phase 1

## Goal

VaachakOS needs a reader state model that can become device-neutral while still respecting what the Xteink X4 proving-ground taught us about SD-card path behavior.

## Two storage layouts

### Canonical Vaachak layout

This is the long-term architecture shape:

```text
/.vaachakos/
  books/
    <book_id>/
      meta.bin
      progress.bin
      bookmarks.bin
      theme.bin
      sections/
  bookmarks/
    index.bin
```

Use this for the architecture, future sync identity, and non-X4 targets once nested paths are known to be reliable.

### X4-compatible flat 8.3 layout

This preserves the known-good X4 behavior from `x4-reader-os-rs`:

```text
state/<HEX8>.MTA
state/<HEX8>.PRG
state/<HEX8>.BKM
state/<HEX8>.THM
state/BMIDX.TXT
cache/<HEX8>/
```

The X4 compatibility layout exists because the proving-ground showed that flat 8.3-safe files were reliable while longer nested state paths were not reliable during the Phase 5-6 bookmark/progress work.

## Book identity direction

Bootstrap Phase 1 defines the model, not the final cryptographic identity implementation.

Current schemes:

- `PathFnv1a32LegacyV1` — compatibility only.
- `ContentSampleFnv1a32V1` — X4-safe early content identity from file size + streamed sample bytes.
- `ContentSha256V1` — reserved future stable identity once memory and runtime cost are measured on X4.

The reader app should not invent storage paths directly. It should ask the storage layout model for paths.
