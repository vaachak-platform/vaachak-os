#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase39j-sd-state-inspection.txt}"

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

{
  echo "# Phase 39J SD State Inspection"
  echo "sd=$SD"
  echo "state_dir=$STATE_DIR"
  echo
  echo "## all state files"
  find "$STATE_DIR" -maxdepth 1 -type f -printf '%f\t%s bytes\t%TY-%Tm-%Td %TH:%TM:%TS\n' | sort || true
  echo
  echo "## typed record counts"
  for ext in PRG THM MTA BKM; do
    count="$(find "$STATE_DIR" -maxdepth 1 -type f -iname "*.${ext}" | wc -l | tr -d ' ')"
    bytes="$(find "$STATE_DIR" -maxdepth 1 -type f -iname "*.${ext}" -printf '%s\n' | awk '{s+=$1} END {print s+0}')"
    echo "$ext count=$count bytes=$bytes"
  done
  if [ -f "$STATE_DIR/BMIDX.TXT" ]; then
    echo "BMIDX.TXT count=1 bytes=$(wc -c < "$STATE_DIR/BMIDX.TXT" | tr -d ' ')"
  elif [ -f "$STATE_DIR/bmidx.txt" ]; then
    echo "bmidx.txt count=1 bytes=$(wc -c < "$STATE_DIR/bmidx.txt" | tr -d ' ')"
  else
    echo "BMIDX.TXT count=0 bytes=0"
  fi
  echo
  echo "## strings preview"
  for pattern in '*.PRG' '*.THM' '*.MTA' '*.BKM' 'BMIDX.TXT' 'bmidx.txt'; do
    while IFS= read -r -d '' file; do
      echo "--- $(basename "$file") ---"
      strings -a "$file" 2>/dev/null | head -20 || true
    done < <(find "$STATE_DIR" -maxdepth 1 -type f -iname "$pattern" -print0 2>/dev/null)
  done
} | tee "$OUT"

echo
echo "Wrote: $OUT"
