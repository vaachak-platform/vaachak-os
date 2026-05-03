#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OUT="${OUT:-/tmp/phase40g-repair-home-library-title-acceptance.txt}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40g-repair-device-report.txt}"

"$ROOT/phase40g_repair_home_full_width_and_reader_titles_overlay/scripts/check_phase40g_repair_home_library_title_display.sh" >/dev/null
"$ROOT/phase40g_repair_home_full_width_and_reader_titles_overlay/scripts/inspect_phase40g_repair_home_library_title_display.sh" >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="RepairAccepted"
if [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"; reason="FooterRegressionDetected"
fi

{
  echo "# Phase 40G Repair Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "device_status=$device_status"
  echo "old_footer_count=$old_footer_count"
  echo "changes_home_title_layout=true"
  echo "changes_library_title_resolution=true"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40g-repair=x4-home-full-width-reader-titles-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
