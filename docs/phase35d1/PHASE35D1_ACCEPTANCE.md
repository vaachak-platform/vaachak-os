# Phase 35D-1 Acceptance

Phase 35D-1 is accepted when:

- `target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs` exists.
- The new bridge exercises Vaachak progress, bookmark, bookmark index, theme, and metadata record formats.
- The storage state runtime exposes an allocation-aware preflight that calls the reader state runtime bridge only after heap setup.
- The active imported runtime wrapper remains on the direct Pulp `AppManager` path.
- No Vaachak app-layer wrapper is present.
- No active progress/bookmark/theme/metadata persistence is replaced.
- No physical SD/SPI/FAT IO is introduced into the bridge.
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits.
- Normal boot remains `vaachak=x4-runtime-ready`.
