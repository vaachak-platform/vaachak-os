#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/SD_CARD}"
TITLEMAP="$SD/_X4/TITLEMAP.TSV"
TITLES="$SD/_X4/TITLES.BIN"

if [ ! -d "$SD" ]; then
  echo "SD mount not found: $SD" >&2
  exit 2
fi

if [ ! -f "$TITLEMAP" ]; then
  echo "missing TITLEMAP: $TITLEMAP" >&2
  echo "Run generate_phase40h_txt_title_map_for_sd.sh first." >&2
  exit 3
fi

mkdir -p "$SD/_X4"

STAMP="$(date +%Y%m%d-%H%M%S)"
BACKUP="$SD/_X4_BACKUP_BEFORE_PHASE40H_REPAIR1_TITLES_SEED_$STAMP"
mkdir -p "$BACKUP"

if [ -f "$TITLES" ]; then
  cp -v "$TITLES" "$BACKUP/TITLES.BIN.before-phase40h-repair1"
fi

python3 - "$TITLEMAP" "$TITLES" <<'PY'
from pathlib import Path
import sys

titlemap = Path(sys.argv[1])
titles = Path(sys.argv[2])

existing_lines = []
existing_keys = set()

if titles.exists():
    for raw in titles.read_text(encoding="utf-8", errors="ignore").splitlines():
        if not raw.strip():
            continue
        existing_lines.append(raw)
        key = raw.split("\t", 1)[0].strip().upper()
        if key:
            existing_keys.add(key)

added = 0
for raw in titlemap.read_text(encoding="utf-8", errors="ignore").splitlines():
    if not raw.strip() or "\t" not in raw:
        continue
    key, title = raw.split("\t", 1)
    key = key.strip()
    title = title.strip()
    if not key or not title:
        continue

    # Keep this repair focused on TXT/MD display names.
    upper_key = key.upper()
    if not (upper_key.endswith(".TXT") or upper_key.endswith(".MD")):
        continue

    if upper_key in existing_keys:
        continue

    existing_lines.append(f"{key}\t{title}")
    existing_keys.add(upper_key)
    added += 1

titles.write_text("\n".join(existing_lines) + "\n", encoding="utf-8")

print("# Phase 40H Repair 1 TITLES.BIN Seed")
print("status=ACCEPTED")
print(f"titlemap={titlemap}")
print(f"titles={titles}")
print(f"added={added}")
print(f"total={len(existing_lines)}")
print("marker=phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok")
PY

sync
