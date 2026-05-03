# Phase 40G Repair 3 — Disable TXT Body-Title Scanning

Do not accept the earlier Phase 40G Repair 2 if the device still shows body/license text as TXT titles.

This repair:
- Keeps EPUB/EPU metadata scanning.
- Disables TXT/MD body-title scanning at the directory-title candidate source.
- Requires moving `_X4/TITLES.BIN` away so bad cached titles disappear.

Expected markers:
- phase40g-repair3=x4-disable-txt-body-title-scanning-ok
- phase40g-repair3-acceptance=x4-disable-txt-body-title-scanning-report-ok
