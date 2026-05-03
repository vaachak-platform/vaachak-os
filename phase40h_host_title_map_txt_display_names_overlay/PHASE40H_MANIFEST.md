# Phase 40H — FAT Long Filename / Host Title Map for TXT Display Names

Purpose:
- TXT display names should come from SD root filenames or a host-generated title map.
- TXT body text must not be scanned or guessed as a title.

Firmware patch:
- `vendor/pulp-os/kernel/src/kernel/dir_cache.rs` loads `_X4/TITLEMAP.TSV` before `_X4/TITLES.BIN`.

Host tool:
- `scripts/generate_phase40h_txt_title_map_for_sd.sh`
- Writes `_X4/TITLEMAP.TSV`.
- Maps likely FAT 8.3 aliases such as `POIROT~1.TXT` to friendly titles.

Expected markers:
- `phase40h=x4-host-title-map-txt-display-names-ok`
- `phase40h-acceptance=x4-host-title-map-txt-display-names-report-ok`
