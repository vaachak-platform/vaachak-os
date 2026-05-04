# Phase 38K/38I Repair Overlay

Purpose:
- Repair the Phase 38I `.EPU` / `.EPUB` helper recursion warnings left in:
  - vendor/pulp-os/kernel/src/kernel/dir_cache.rs
  - vendor/pulp-os/src/apps/files.rs
- Keep Phase 38K guarded write-backend scaffold unchanged.
- No SD/FAT/SPI/display/input/power behavior is moved.
- No writes are enabled by this repair.
