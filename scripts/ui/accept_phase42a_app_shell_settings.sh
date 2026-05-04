#!/usr/bin/env bash
set -euo pipefail

BUILD_CONFIRMED="${BUILD_CONFIRMED:-0}"
DEVICE_CONFIRMED="${DEVICE_CONFIRMED:-0}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase42a-device-app-shell-settings-report.txt}"
OUT="${OUT:-/tmp/phase42a-app-shell-settings-acceptance.txt}"

./scripts/ui/check_phase42a_app_shell_settings.sh >/dev/null
./scripts/ui/inspect_phase42a_app_shell_settings.sh >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

custom_footer_count="$((rg -n 'Back   Select   Left   Right' vendor/pulp-os/src/apps/home.rs 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="AppShellRoutingSettingsAccepted"

if [ "$BUILD_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="BuildNotConfirmed"
elif [ "$DEVICE_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="DeviceNotConfirmed"
elif [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$custom_footer_count" != "0" ]; then
  status="REJECTED"; reason="DuplicateHomeFooterRisk"
fi

{
  echo "# Phase 42A App Shell Routing + Settings Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "build_confirmed=$BUILD_CONFIRMED"
  echo "device_confirmed=$DEVICE_CONFIRMED"
  echo "device_status=$device_status"
  echo "custom_footer_count=$custom_footer_count"
  echo "changes_home_routing=true"
  echo "adds_settings_app=true"
  echo "settings_persistence=false"
  echo "changes_files_rendering=false"
  echo "changes_reader_rendering=false"
  echo "changes_title_workflow=false"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase42a=x4-app-shell-routing-settings-implementation-ok"
  echo "device_report=$DEVICE_OUT"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
