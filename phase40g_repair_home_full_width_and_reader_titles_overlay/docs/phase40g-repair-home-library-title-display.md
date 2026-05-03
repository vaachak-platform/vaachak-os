# Phase 40G Repair — Home Full Width and Reader Titles

This repair directly patches the observed failures:

- Home current/continue title is rendered in a full-width content region instead of the narrow menu-item region.
- `.EPU` entries are included in the EPUB title scanner.
- `.TXT` and `.MD` entries are title-scanned from the first meaningful text heading and written to `TITLES.BIN`.

Preserved:
- Footer labels
- Input mapping
- Write lane
- Display geometry / rotation
- Reader pagination
