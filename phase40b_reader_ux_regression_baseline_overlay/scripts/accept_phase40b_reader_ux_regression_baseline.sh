#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
SD="${SD:-/media/mindseye73/C0D2-109E}"
HOME_FILES_READER_CONFIRMED="${HOME_FILES_READER_CONFIRMED:-0}"
FOOTER_LABELS_CONFIRMED="${FOOTER_LABELS_CONFIRMED:-0}"
EPUB_TITLES_CONFIRMED="${EPUB_TITLES_CONFIRMED:-0}"
READER_RESTORE_CONFIRMED="${READER_RESTORE_CONFIRMED:-0}"
NO_CRASH_REBOOT="${NO_CRASH_REBOOT:-0}"
FOOTER_LABELS_OBSERVED="${FOOTER_LABELS_OBSERVED:-capture-current-device-footer-labels}"
TITLE_DISPLAY_OBSERVED="${TITLE_DISPLAY_OBSERVED:-capture-current-device-title-display}"

GUARD_OUT="${GUARD_OUT:-/tmp/phase40b-write-lane-closed-guard.txt}"
SURFACE_OUT="${SURFACE_OUT:-/tmp/phase40b-reader-ux-surface.txt}"
TITLE_OUT="${TITLE_OUT:-/tmp/phase40b-epub-title-baseline.txt}"
MANUAL_OUT="${MANUAL_OUT:-/tmp/phase40b-manual-device-ux-report.txt}"
OUT="${OUT:-/tmp/phase40b-reader-ux-regression-baseline-acceptance.txt}"

"$ROOT/phase40b_reader_ux_regression_baseline_overlay/scripts/guard_phase40b_write_lane_closed.sh" >/dev/null
"$ROOT/phase40b_reader_ux_regression_baseline_overlay/scripts/inspect_phase40b_reader_ux_surface.sh" >/dev/null
SD="$SD" "$ROOT/phase40b_reader_ux_regression_baseline_overlay/scripts/inspect_phase40b_epub_title_baseline.sh" >/dev/null
HOME_FILES_READER_CONFIRMED="$HOME_FILES_READER_CONFIRMED" \
FOOTER_LABELS_CONFIRMED="$FOOTER_LABELS_CONFIRMED" \
EPUB_TITLES_CONFIRMED="$EPUB_TITLES_CONFIRMED" \
READER_RESTORE_CONFIRMED="$READER_RESTORE_CONFIRMED" \
NO_CRASH_REBOOT="$NO_CRASH_REBOOT" \
FOOTER_LABELS_OBSERVED="$FOOTER_LABELS_OBSERVED" \
TITLE_DISPLAY_OBSERVED="$TITLE_DISPLAY_OBSERVED" \
"$ROOT/phase40b_reader_ux_regression_baseline_overlay/scripts/write_phase40b_manual_device_ux_report.sh" >/dev/null

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
manual_status="$(grep '^status=' "$MANUAL_OUT" | head -1 | cut -d= -f2-)"

title_file_count="$(grep -E '\.epub|\.EPU|\.epu|\.EPUB' "$TITLE_OUT" 2>/dev/null | wc -l | tr -d ' ')"
surface_marker="$(grep -c 'phase40b=x4-reader-ux-regression-baseline-ok' "$SURFACE_OUT" 2>/dev/null || true)"

status="ACCEPTED"
reason="ReaderUxBaselineAccepted"

if [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="WriteLaneGuardFailed"
elif [ "$manual_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="ManualDeviceUxReportRejected"
elif [ "$surface_marker" = "0" ]; then
  status="REJECTED"
  reason="ReaderUxSurfaceInspectionMissing"
fi

{
  echo "# Phase 40B Reader UX Regression Baseline Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "guard_status=$guard_status"
  echo "manual_status=$manual_status"
  echo "title_file_lines=$title_file_count"
  echo "home_files_reader_confirmed=$HOME_FILES_READER_CONFIRMED"
  echo "footer_labels_confirmed=$FOOTER_LABELS_CONFIRMED"
  echo "epub_titles_confirmed=$EPUB_TITLES_CONFIRMED"
  echo "reader_restore_confirmed=$READER_RESTORE_CONFIRMED"
  echo "no_crash_reboot=$NO_CRASH_REBOOT"
  echo "footer_labels_observed=$FOOTER_LABELS_OBSERVED"
  echo "title_display_observed=$TITLE_DISPLAY_OBSERVED"
  echo "marker=phase40b=x4-reader-ux-regression-baseline-ok"
  echo "guard=$GUARD_OUT"
  echo "surface=$SURFACE_OUT"
  echo "title_baseline=$TITLE_OUT"
  echo "manual_report=$MANUAL_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
