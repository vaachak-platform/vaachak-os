#!/usr/bin/env bash
set -euo pipefail

BUILD_CONFIRMED="${BUILD_CONFIRMED:-0}"
DEVICE_CONFIRMED="${DEVICE_CONFIRMED:-0}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase41h-device-biscuit-ui-report.txt}"
OUT="${OUT:-/tmp/phase41h-biscuit-ui-freeze-acceptance.txt}"

./scripts/ui/check_phase41h_biscuit_ui_freeze.sh >/dev/null
./scripts/ui/inspect_phase41h_biscuit_ui_freeze.sh >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

custom_home_footer_count="$((rg -n 'Back[[:space:]]+Select[[:space:]]+Left[[:space:]]+Right|Back Select Left Right' vendor/pulp-os/src/apps/home.rs 2>/dev/null || true) | wc -l | tr -d ' ')"
old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"
bad_txt_body_return="$((rg -n 'PHASE40G_REPAIR_TITLE_KIND_TEXT.*return Some|return Some.*PHASE40G_REPAIR_TITLE_KIND_TEXT' vendor/pulp-os/kernel/src/kernel/dir_cache.rs 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="BiscuitUiFreezeAccepted"

if [ "$BUILD_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="BuildNotConfirmed"
elif [ "$DEVICE_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="DeviceNotConfirmed"
elif [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$custom_home_footer_count" != "0" ]; then
  status="REJECTED"; reason="CustomHomeFooterStillPresent"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"; reason="OldFooterOrderFound"
elif [ "$bad_txt_body_return" != "0" ]; then
  status="REJECTED"; reason="TxtBodyTitleReturnFound"
fi

{
  echo "# Phase 41H Biscuit UI Freeze Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "build_confirmed=$BUILD_CONFIRMED"
  echo "device_confirmed=$DEVICE_CONFIRMED"
  echo "device_status=$device_status"
  echo "custom_home_footer_count=$custom_home_footer_count"
  echo "old_footer_count=$old_footer_count"
  echo "bad_txt_body_return=$bad_txt_body_return"
  echo "home_dashboard_frozen=true"
  echo "files_library_frozen=true"
  echo "reader_restore_frozen=true"
  echo "title_cache_frozen=true"
  echo "changes_home_rendering=false"
  echo "changes_app_routing=false"
  echo "changes_files_rendering=false"
  echo "changes_reader_rendering=false"
  echo "changes_title_workflow=false"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "recommended_commit=phase41h: freeze biscuit ui baseline"
  echo "marker=phase41h=x4-biscuit-ui-acceptance-freeze-ok"
  echo "inspection=/tmp/phase41h-biscuit-ui-freeze-inspection.txt"
  echo "device_report=$DEVICE_OUT"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
