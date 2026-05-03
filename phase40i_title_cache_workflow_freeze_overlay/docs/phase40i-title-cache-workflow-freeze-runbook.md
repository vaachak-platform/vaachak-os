# Phase 40I — Title Cache Workflow Freeze Runbook

Accepted workflow:

```text
1. Put reader files on SD root.
2. Generate `_X4/TITLEMAP.TSV` from SD root TXT/MD filenames.
3. Seed TXT/MD aliases into `_X4/TITLES.BIN`.
4. Keep EPUB/EPU metadata scanning enabled.
5. Keep TXT/MD body-title scanning disabled.
6. Boot device; the device loads display titles from `_X4/TITLES.BIN`.
```

Host commands:

```bash
SD=/media/mindseye73/SD_CARD \
./phase40h_host_title_map_txt_display_names_overlay/scripts/generate_phase40h_txt_title_map_for_sd.sh

SD=/media/mindseye73/SD_CARD \
./phase40h_repair1_seed_txt_titlemap_into_titles_bin_overlay/scripts/seed_phase40h_repair1_txt_titlemap_into_titles_bin_on_sd.sh

SD=/media/mindseye73/SD_CARD \
./phase40i_title_cache_workflow_freeze_overlay/scripts/inspect_phase40i_title_cache_workflow.sh

SD=/media/mindseye73/SD_CARD \
./phase40i_title_cache_workflow_freeze_overlay/scripts/freeze_phase40i_title_cache_workflow_baseline.sh
```

Do not run a title-cache reset after seeding `_X4/TITLES.BIN`.

Regression guards:

```text
- `_X4/TITLES.BIN` must contain TXT/MD title-map lines.
- `_X4/TITLES.BIN` must not contain Project Gutenberg/body/license phrases.
- TXT body-title scanning remains disabled in firmware.
- EPUB/EPU metadata title lines remain present/allowed.
```

Preserved:

```text
- footer labels
- input mapping
- write lane
- display geometry / rotation
- reader pagination
```
