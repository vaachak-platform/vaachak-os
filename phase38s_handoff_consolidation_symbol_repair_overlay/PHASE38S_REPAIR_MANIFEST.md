# Phase 38S Repair Overlay

Repairs Phase 38S handoff consolidation after apply.

Fixes:
- `phase38c_live_writes_enabled` -> `phase38c_writes_enabled`
- Adds `#[allow(dead_code)]` to the Phase 38I helper in `dir_cache.rs` if it is unused

Expected marker:
- phase38s-repair=x4-write-lane-handoff-symbol-repair-ok
