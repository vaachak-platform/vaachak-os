# Phase 35D-0 Acceptance

Phase 35D-0 is accepted when:

- `target-xteink-x4/src/vaachak_x4/apps/reader_state.rs` defines Vaachak-owned progress, bookmark, and bookmark-index records.
- The facade resolves `.PRG`, `.BKM`, and `BMIDX.TXT` through Vaachak-owned storage path helpers.
- Progress/bookmark encode/decode helpers round-trip in host tests.
- The active runtime remains on the direct imported Pulp `AppManager` path.
- No Vaachak app-layer wrapper is reintroduced.
- No physical SD/SPI/FAT IO is added to the facade.
- `vendor/pulp-os` and `vendor/smol-epub` have no tracked edits.
- Normal boot remains `vaachak=x4-runtime-ready`.

Phase 35D-0 is not accepted as active progress/bookmark persistence extraction. That later step must be hardware-gated and must not disturb button/input behavior.
