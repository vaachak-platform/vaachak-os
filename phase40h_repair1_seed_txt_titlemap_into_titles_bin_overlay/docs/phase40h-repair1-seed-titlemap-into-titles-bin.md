# Phase 40H Repair 1 — Seed TXT Title Map into TITLES.BIN

The previous Phase 40H path generated a correct `_X4/TITLEMAP.TSV`, but the
device did not apply it.

Since the device does load `_X4/TITLES.BIN`, this repair merges TXT/MD aliases
from `_X4/TITLEMAP.TSV` directly into `_X4/TITLES.BIN`.

Do this after generating `TITLEMAP.TSV` and before booting the device:

```bash
SD=/media/mindseye73/SD_CARD \
./phase40h_repair1_seed_txt_titlemap_into_titles_bin_overlay/scripts/seed_phase40h_repair1_txt_titlemap_into_titles_bin_on_sd.sh
```

Then boot the device. Do not reset `TITLES.BIN` again after seeding.
