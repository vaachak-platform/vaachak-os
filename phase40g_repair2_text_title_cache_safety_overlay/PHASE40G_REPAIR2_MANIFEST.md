# Phase 40G Repair 2 — Text Title Cache Safety

Root cause:
- TXT title scanner accepted arbitrary first meaningful body/license line.
- Bad result was saved into `_X4/TITLES.BIN`.

Fix:
- TXT/MD title scanner now accepts explicit `Title:` metadata only.
- It no longer writes random body/license text into `TITLES.BIN`.
- Includes SD cache reset script to move old bad title cache out of the active path.

Expected markers:
- phase40g-repair2=x4-text-title-cache-safety-ok
- phase40g-repair2-acceptance=x4-text-title-cache-safety-report-ok
