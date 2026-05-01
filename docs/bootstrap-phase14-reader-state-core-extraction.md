# VaachakOS Bootstrap Phase 14 — Reader State/Core Extraction Cleanup

## Purpose

Phase 14 stops the reader smoke work from turning `target-xteink-x4/src/main.rs` into the long-term reader model owner.

The X4 target remains the hardware smoke runtime, but stable reader concepts now have a core home:

- reader file kind
- recursive library entry
- reader UI mode
- reader navigation action
- reader page state
- reader session state

This is intentionally low-risk. It does not migrate the Phase 13 runtime loop yet.

## What changed

Added core models:

```text
core/src/models/reader_file.rs
core/src/models/reader_runtime.rs
```

Updated exports:

```text
core/src/models/mod.rs
core/src/apps/reader/mod.rs
```

## New model ownership

| Concept | New owner |
|---|---|
| TXT/MD/EPU/EPUB file kind | `core::models::ReaderFileKind` |
| Path-aware library row | `core::models::LibraryEntry` |
| Recursive scan policy | `core::models::LibraryScanPolicy` |
| Library/Reader/BookmarkList/Pending mode | `core::models::ReaderUiMode` |
| Navigation action vocabulary | `core::models::ReaderNavAction` |
| Offset/page/bookmark page status | `core::models::ReaderPageState` |
| Open reader session descriptor | `core::models::ReaderSessionState` |

## What did not change

- No X4 display driver changes
- No SD/FAT changes
- No input changes
- No runtime loop rewrite
- No EPUB opening yet
- No replacement of the working Phase 13 target path yet

## Acceptance

Host checks should pass:

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Embedded behavior should remain the same as Phase 13.

## Next phase

Phase 15 should start wiring the target code to these core models or begin EPUB reader smoke, depending on whether you want another cleanup phase before EPUB.
