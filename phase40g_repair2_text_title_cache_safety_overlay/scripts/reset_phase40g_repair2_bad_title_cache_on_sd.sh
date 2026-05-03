#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"

if [ ! -d "$SD" ]; then
  echo "SD mount not found: $SD" >&2
  exit 2
fi

STAMP="$(date +%Y%m%d-%H%M%S)"
BACKUP="$SD/_X4_BACKUP_BEFORE_PHASE40G_REPAIR2_TITLE_REBUILD_$STAMP"
mkdir -p "$BACKUP"

moved=0
for file in \
  "$SD/_X4/TITLES.BIN" \
  "$SD/_X4/RECENT" \
  "$SD/_PULP/TITLES.BIN" \
  "$SD/_PULP/RECENT"
do
  if [ -f "$file" ]; then
    mv -v "$file" "$BACKUP/"
    moved=$((moved + 1))
  fi
done

sync

cat <<EOF
# Phase 40G Repair 2 SD Title Cache Reset
status=ACCEPTED
moved=$moved
backup=$BACKUP
marker=phase40g-repair2=x4-text-title-cache-safety-ok
EOF
