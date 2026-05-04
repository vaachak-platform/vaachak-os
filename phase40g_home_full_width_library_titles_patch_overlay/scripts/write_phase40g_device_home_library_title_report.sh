#!/usr/bin/env bash
set -euo pipefail
HOME_TITLE_CONFIRMED="${HOME_TITLE_CONFIRMED:-0}"
LIBRARY_TITLES_CONFIRMED="${LIBRARY_TITLES_CONFIRMED:-0}"
EPU_TITLE_SCAN_CONFIRMED="${EPU_TITLE_SCAN_CONFIRMED:-0}"
TEXT_TITLE_SCAN_CONFIRMED="${TEXT_TITLE_SCAN_CONFIRMED:-0}"
FOOTER_LABELS_STILL_CORRECT="${FOOTER_LABELS_STILL_CORRECT:-0}"
NO_INPUT_REGRESSION="${NO_INPUT_REGRESSION:-0}"
NO_WRITE_REGRESSION="${NO_WRITE_REGRESSION:-0}"
HOME_TITLE_OBSERVED="${HOME_TITLE_OBSERVED:-home current title uses full width}"
LIBRARY_TITLES_OBSERVED="${LIBRARY_TITLES_OBSERVED:-library titles readable}"
OUT="${OUT:-/tmp/phase40g-device-home-library-title-report.txt}"
status="ACCEPTED"; reason="DeviceHomeLibraryTitlesConfirmed"
[ "$HOME_TITLE_CONFIRMED" = 1 ] || { status="REJECTED"; reason="HomeTitleNotConfirmed"; }
[ "$LIBRARY_TITLES_CONFIRMED" = 1 ] || { status="REJECTED"; reason="LibraryTitlesNotConfirmed"; }
[ "$EPU_TITLE_SCAN_CONFIRMED" = 1 ] || { status="REJECTED"; reason="EpuTitleScanNotConfirmed"; }
[ "$TEXT_TITLE_SCAN_CONFIRMED" = 1 ] || { status="REJECTED"; reason="TextTitleScanNotConfirmed"; }
[ "$FOOTER_LABELS_STILL_CORRECT" = 1 ] || { status="REJECTED"; reason="FooterLabelsRegressed"; }
[ "$NO_INPUT_REGRESSION" = 1 ] || { status="REJECTED"; reason="InputRegressionObserved"; }
[ "$NO_WRITE_REGRESSION" = 1 ] || { status="REJECTED"; reason="WriteRegressionObserved"; }
{
  echo "# Phase 40G Device Home/Library Title Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "home_title_confirmed=$HOME_TITLE_CONFIRMED"
  echo "library_titles_confirmed=$LIBRARY_TITLES_CONFIRMED"
  echo "epu_title_scan_confirmed=$EPU_TITLE_SCAN_CONFIRMED"
  echo "text_title_scan_confirmed=$TEXT_TITLE_SCAN_CONFIRMED"
  echo "footer_labels_still_correct=$FOOTER_LABELS_STILL_CORRECT"
  echo "no_input_regression=$NO_INPUT_REGRESSION"
  echo "no_write_regression=$NO_WRITE_REGRESSION"
  echo "home_title_observed=$HOME_TITLE_OBSERVED"
  echo "library_titles_observed=$LIBRARY_TITLES_OBSERVED"
  echo "marker=phase40g=x4-home-full-width-library-title-patch-ok"
} | tee "$OUT"
[ "$status" = ACCEPTED ] || exit 4
