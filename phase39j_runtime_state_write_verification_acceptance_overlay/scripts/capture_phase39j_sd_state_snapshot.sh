#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase39j-sd-state-snapshot-$STAMP.tar.gz}"

if [ ! -d "$SD" ]; then
  echo "SD mount not found: $SD" >&2
  exit 2
fi

STATE_DIR=""
for candidate in "$SD/_X4/state" "$SD/_X4/STATE" "$SD/_PULP/state" "$SD/_PULP/STATE"; do
  if [ -d "$candidate" ]; then
    STATE_DIR="$candidate"
    break
  fi
done

if [ -z "$STATE_DIR" ]; then
  echo "No state directory found under $SD/_X4 or $SD/_PULP" >&2
  exit 3
fi

tar -czf "$OUT" -C "$(dirname "$STATE_DIR")" "$(basename "$STATE_DIR")"
echo "phase39j-snapshot=$OUT"
