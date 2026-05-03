#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
SD="${SD:-/media/mindseye73/C0D2-109E}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40h-device-report.txt}"
OUT="${OUT:-/tmp/phase40h-host-title-map-txt-display-names-acceptance.txt}"

"$ROOT/phase40h_host_title_map_txt_display_names_overlay/scripts/check_phase40h_host_title_map_txt_display_names.sh" >/dev/null
"$ROOT/phase40h_host_title_map_txt_display_names_overlay/scripts/inspect_phase40h_host_title_map_txt_display_names.sh" >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

titlemap_status="MISSING"
titlemap_lines=0
if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
  titlemap_status="PRESENT"
  titlemap_lines="$(wc -l < "$SD/_X4/TITLEMAP.TSV" | tr -d ' ')"
fi

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="HostTitleMapTxtDisplayNamesAccepted"
if [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$titlemap_status" != "PRESENT" ]; then
  status="REJECTED"; reason="TitleMapMissing"
elif [ "$titlemap_lines" = "0" ]; then
  status="REJECTED"; reason="TitleMapEmpty"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"; reason="FooterRegressionDetected"
fi

{
  echo "# Phase 40H Host Title Map TXT Display Names Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "device_status=$device_status"
  echo "titlemap_status=$titlemap_status"
  echo "titlemap_lines=$titlemap_lines"
  echo "old_footer_count=$old_footer_count"
  echo "loads_host_title_map=true"
  echo "scans_txt_body_titles=false"
  echo "scans_epub_epu_metadata=true"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40h=x4-host-title-map-txt-display-names-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
