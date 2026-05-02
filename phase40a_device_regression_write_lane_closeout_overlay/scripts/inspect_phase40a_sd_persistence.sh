#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase40a-sd-persistence-inspection.txt}"

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

count_nonempty() {
  local pattern="$1"
  find "$STATE_DIR" -maxdepth 1 -type f -iname "$pattern" -size +0c | wc -l | tr -d ' '
}

progress_count="$(count_nonempty '*.PRG')"
theme_count="$(count_nonempty '*.THM')"
metadata_count="$(count_nonempty '*.MTA')"
bookmark_count="$(count_nonempty '*.BKM')"

bmidx_count=0
if [ -s "$STATE_DIR/BMIDX.TXT" ] || [ -s "$STATE_DIR/bmidx.txt" ]; then
  bmidx_count=1
fi

accepted_records=0
[ "$progress_count" -gt 0 ] && accepted_records=$((accepted_records + 1))
[ "$theme_count" -gt 0 ] && accepted_records=$((accepted_records + 1))
[ "$metadata_count" -gt 0 ] && accepted_records=$((accepted_records + 1))
[ "$bookmark_count" -gt 0 ] && accepted_records=$((accepted_records + 1))
[ "$bmidx_count" -gt 0 ] && accepted_records=$((accepted_records + 1))

status="ACCEPTED"
reason="SdPersistencePresent"
if [ "$accepted_records" -ne 5 ]; then
  status="REJECTED"
  reason="MissingTypedStateRecords"
fi

{
  echo "# Phase 40A SD Persistence Inspection"
  echo "status=$status"
  echo "reason=$reason"
  echo "sd=$SD"
  echo "state_dir=$STATE_DIR"
  echo "accepted_records=$accepted_records/5"
  echo "progress_nonempty=$progress_count"
  echo "theme_nonempty=$theme_count"
  echo "metadata_nonempty=$metadata_count"
  echo "bookmark_nonempty=$bookmark_count"
  echo "bmidx_nonempty=$bmidx_count"
  echo "marker=phase40a=x4-device-regression-write-lane-closeout-ok"
  echo
  echo "## state files"
  find "$STATE_DIR" -maxdepth 1 -type f -printf '%f\t%s bytes\t%TY-%Tm-%Td %TH:%TM:%TS\n' | sort || true
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
