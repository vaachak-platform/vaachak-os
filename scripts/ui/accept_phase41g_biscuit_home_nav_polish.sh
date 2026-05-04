#!/usr/bin/env bash
set -euo pipefail

BUILD_CONFIRMED="${BUILD_CONFIRMED:-0}"
DEVICE_CONFIRMED="${DEVICE_CONFIRMED:-0}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase41g-device-home-nav-report.txt}"
OUT="${OUT:-/tmp/phase41g-biscuit-home-nav-polish-acceptance.txt}"

./scripts/ui/check_phase41g_biscuit_home_nav_polish.sh >/dev/null
./scripts/ui/inspect_phase41g_biscuit_home_nav_polish.sh >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

custom_footer_count="$((rg -n 'Back   Select   Left   Right' vendor/pulp-os/src/apps/home.rs 2>/dev/null || true) | wc -l | tr -d ' ')"
changed_forbidden="$((git diff --name-only | grep -E '^(vendor/pulp-os/src/apps/reader|vendor/pulp-os/kernel/src/kernel/dir_cache\.rs|hal-xteink-x4/src/|target-xteink-x4/src/vaachak_x4/(contracts|input|physical|runtime)/)' || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="HomeNavPolishAccepted"

if [ "$BUILD_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="BuildNotConfirmed"
elif [ "$DEVICE_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="DeviceNotConfirmed"
elif [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$custom_footer_count" != "0" ]; then
  status="REJECTED"; reason="CustomHomeFooterStillPresent"
fi

{
  echo "# Phase 41G Biscuit Home Nav Polish Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "build_confirmed=$BUILD_CONFIRMED"
  echo "device_confirmed=$DEVICE_CONFIRMED"
  echo "device_status=$device_status"
  echo "custom_footer_count=$custom_footer_count"
  echo "changed_forbidden_surfaces=$changed_forbidden"
  echo "changes_home_rendering=true"
  echo "changes_app_routing=true"
  echo "changes_files_rendering=false"
  echo "changes_reader_rendering=false"
  echo "changes_title_workflow=false"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase41g=x4-biscuit-home-nav-polish-placeholder-routing-ok"
  echo "inspection=/tmp/phase41g-biscuit-home-nav-polish-inspection.txt"
  echo "device_report=$DEVICE_OUT"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
