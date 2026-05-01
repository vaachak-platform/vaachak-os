# VaachakOS Bootstrap Phase 15 — EPUB Reader Smoke

## Scope

- Use the working `x4-reader-os-rs` reader path as the behavioral reference.
- Keep recursive Library from Phase 13.
- Keep TXT/MD pagination, progress, and bookmark behavior from Phase 12.
- Add `.EPU` / `.EPUB` open path.
- Persist EPUB progress using the same flat 8.3-safe `state/<BOOKID>.PRG` pattern.
- Back/Left returns to Library.
- EPUB bookmarks remain deferred.

## Implementation note

This phase deliberately keeps EPUB support at smoke level. It validates that VaachakOS can select an EPUB/EPU file from the all-files library, open it from SD, recognize the ZIP container signature, render a first smoke page, and write/read the progress record. Full OPF/spine/chapter extraction should follow in the next EPUB parity phase.

## Acceptance markers

```text
phase15=x4-epub-reader-smoke-ready
phase15: reader open start file=... kind=Epub
phase15: epub read ok file=... zip_ok=true progress=XXXXXXXX.PRG
phase15=x4-epub-reader-smoke-ok
```
