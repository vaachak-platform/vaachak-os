# X4 Title Cache Workflow

Accepted workflow:

```text
SD root filenames -> _X4/TITLEMAP.TSV -> seed TXT/MD aliases into _X4/TITLES.BIN -> device loads TITLES.BIN
```

Rules:
- EPUB/EPU metadata title caching remains enabled.
- TXT/MD body-title scanning remains disabled.
- TXT/MD display names are host-generated from filenames.
- Do not reset `_X4/TITLES.BIN` after seeding.
- Re-run `prepare_sd_title_cache.sh` after adding/removing TXT/MD files.

Useful commands:

```bash
SD=/media/mindseye73/SD_CARD ./tools/x4-title-cache/generate_title_map.py --sd /media/mindseye73/SD_CARD
SD=/media/mindseye73/SD_CARD ./tools/x4-title-cache/seed_titlemap_into_titles_bin.py --sd /media/mindseye73/SD_CARD
SD=/media/mindseye73/SD_CARD ./tools/x4-title-cache/inspect_title_cache.sh
```

Regression guard:
- `_X4/TITLES.BIN` must not contain:
  - `Project Gutenberg`
  - `most other parts`
  - `world at no cost`
  - `produced by`
  - `transcribed by`
