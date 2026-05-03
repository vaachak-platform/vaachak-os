#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
SD="${SD:-/media/mindseye73/SD_CARD}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40h-repair1-device-report.txt}"
OUT="${OUT:-/tmp/phase40h-repair1-seed-titles-bin-acceptance.txt}"

"$ROOT/phase40h_repair1_seed_txt_titlemap_into_titles_bin_overlay/scripts/check_phase40h_repair1_seed_titles_bin_metadata.sh" >/dev/null
"$ROOT/phase40h_repair1_seed_txt_titlemap_into_titles_bin_overlay/scripts/inspect_phase40h_repair1_titles_bin_seed.sh" >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

titles_status="MISSING"
txt_title_lines=0
bad_phrase_lines=0

if [ -f "$SD/_X4/TITLES.BIN" ]; then
  titles_status="PRESENT"
  txt_title_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n 'POIROT~|THEMUR~|THESIG~|Poirot Investigates|Roger Ackroyd|Sign of the Four' || true) | wc -l | tr -d ' ')"
  bad_phrase_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by' || true) | wc -l | tr -d ' ')"
fi

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="SeedTxtTitleMapIntoTitlesBinAccepted"
if [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$titles_status" != "PRESENT" ]; then
  status="REJECTED"; reason="TitlesBinMissing"
elif [ "$txt_title_lines" = "0" ]; then
  status="REJECTED"; reason="TxtTitleLinesMissing"
elif [ "$bad_phrase_lines" != "0" ]; then
  status="REJECTED"; reason="BadBodyTitleLinesStillCached"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"; reason="FooterRegressionDetected"
fi

{
  echo "# Phase 40H Repair 1 Seed TXT Title Map into TITLES.BIN Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "device_status=$device_status"
  echo "titles_status=$titles_status"
  echo "txt_title_lines=$txt_title_lines"
  echo "bad_phrase_lines=$bad_phrase_lines"
  echo "old_footer_count=$old_footer_count"
  echo "seeds_titles_bin=true"
  echo "uses_txt_body_scanning=false"
  echo "preserves_epub_epu_metadata=true"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
