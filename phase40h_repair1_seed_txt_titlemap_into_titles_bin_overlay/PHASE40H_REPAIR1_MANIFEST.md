# Phase 40H Repair 1 — Seed TXT Title Map into Active TITLES.BIN

Problem:
- `_X4/TITLEMAP.TSV` exists and is correct.
- Device still displays TXT names incorrectly.
- Post-boot `_X4/TITLES.BIN` contains only EPUB/EPU mappings, which proves the firmware loads `TITLES.BIN` but not the host map path.

Repair:
- Merge TXT/MD entries from `_X4/TITLEMAP.TSV` into `_X4/TITLES.BIN`.
- Keep existing EPUB/EPU entries.
- Do not scan TXT body text.
- Do not reset/remove `TITLES.BIN` after seeding; reboot device so firmware loads it.

Expected markers:
- phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok
- phase40h-repair1-acceptance=x4-seed-txt-titlemap-into-titles-bin-report-ok
