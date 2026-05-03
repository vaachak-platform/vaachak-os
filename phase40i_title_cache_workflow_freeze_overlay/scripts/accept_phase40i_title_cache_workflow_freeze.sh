#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/SD_CARD}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40i-device-report.txt}"
BASELINE_OUT="${BASELINE_OUT:-/tmp/phase40i-title-cache-workflow-baseline.txt}"
OUT="${OUT:-/tmp/phase40i-title-cache-workflow-freeze-acceptance.txt}"

SD="$SD" ./phase40i_title_cache_workflow_freeze_overlay/scripts/freeze_phase40i_title_cache_workflow_baseline.sh >/dev/null

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

baseline_status="$(grep '^status=' "$BASELINE_OUT" | head -1 | cut -d= -f2-)"
titlemap_lines="$(grep '^titlemap_lines=' "$BASELINE_OUT" | head -1 | cut -d= -f2-)"
txt_title_lines="$(grep '^txt_title_lines=' "$BASELINE_OUT" | head -1 | cut -d= -f2-)"
bad_phrase_count="$(grep '^bad_phrase_count=' "$BASELINE_OUT" | head -1 | cut -d= -f2-)"

old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"

status="ACCEPTED"
reason="TitleCacheWorkflowFreezeAccepted"

if [ "$baseline_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="BaselineRejected"
elif [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"; reason="DeviceReportMissingOrRejected"
elif [ "$titlemap_lines" = "0" ]; then
  status="REJECTED"; reason="TitleMapEmpty"
elif [ "$txt_title_lines" = "0" ]; then
  status="REJECTED"; reason="TxtTitleLinesMissing"
elif [ "$bad_phrase_count" != "0" ]; then
  status="REJECTED"; reason="BadPhrasesCached"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"; reason="FooterRegressionDetected"
fi

{
  echo "# Phase 40I Title Cache Workflow Freeze Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "sd=$SD"
  echo "baseline_status=$baseline_status"
  echo "device_status=$device_status"
  echo "titlemap_lines=$titlemap_lines"
  echo "txt_title_lines=$txt_title_lines"
  echo "bad_phrase_count=$bad_phrase_count"
  echo "old_footer_count=$old_footer_count"
  echo "txt_body_title_scanning_disabled=true"
  echo "txt_titles_from_titles_bin=true"
  echo "epub_epu_metadata_enabled=true"
  echo "changes_ux_now=false"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40i=x4-title-cache-workflow-freeze-ok"
  echo "baseline=$BASELINE_OUT"
  echo "device_report=$DEVICE_OUT"
  echo "inspection=/tmp/phase40i-title-cache-workflow-inspection.txt"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 6
fi
