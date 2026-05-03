#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40g-repair3-device-report.txt}"
OUT="${OUT:-/tmp/phase40g-repair3-disable-txt-body-title-scanning-acceptance.txt}"

"$ROOT/phase40g_repair3_disable_txt_body_title_scanning_overlay/scripts/check_phase40g_repair3_disable_txt_body_title_scanning.sh" >/dev/null
"$ROOT/phase40g_repair3_disable_txt_body_title_scanning_overlay/scripts/inspect_phase40g_repair3_disable_txt_body_title_scanning.sh" >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="TxtBodyTitleScanningDisabledAccepted"
if [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"; reason="FooterRegressionDetected"
fi

{
  echo "# Phase 40G Repair 3 Disable TXT Body-Title Scanning Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "device_status=$device_status"
  echo "old_footer_count=$old_footer_count"
  echo "scans_epub_epu=true"
  echo "scans_txt_md_body_titles=false"
  echo "requires_title_cache_rebuild=true"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40g-repair3=x4-disable-txt-body-title-scanning-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
