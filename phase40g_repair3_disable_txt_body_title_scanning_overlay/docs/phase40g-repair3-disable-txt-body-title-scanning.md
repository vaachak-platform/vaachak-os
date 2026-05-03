# Phase 40G Repair 3 — Disable TXT Body-Title Scanning

TXT title guessing from file contents is unsafe. Project Gutenberg body/license
text can be incorrectly cached as a title.

This repair disables TXT/MD body-title scanning entirely.

Expected behavior after reset:
- EPUB/EPU titles remain metadata-driven.
- TXT entries no longer show body/license lines.
- TXT entries may fall back to 8.3 names until a proper FAT LFN/title-map lane is implemented.

Reset active title cache after applying:

```bash
SD=/media/mindseye73/C0D2-109E \
./phase40g_repair3_disable_txt_body_title_scanning_overlay/scripts/reset_phase40g_repair3_bad_title_cache_on_sd.sh
```
