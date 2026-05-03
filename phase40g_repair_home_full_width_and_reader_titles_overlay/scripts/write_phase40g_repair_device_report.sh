#!/usr/bin/env bash
set -euo pipefail

HOME_TITLE_CONFIRMED="${HOME_TITLE_CONFIRMED:-0}"
LIBRARY_TITLES_CONFIRMED="${LIBRARY_TITLES_CONFIRMED:-0}"
EPU_TITLES_CONFIRMED="${EPU_TITLES_CONFIRMED:-0}"
TEXT_TITLES_CONFIRMED="${TEXT_TITLES_CONFIRMED:-0}"
FOOTER_LABELS_STILL_CORRECT="${FOOTER_LABELS_STILL_CORRECT:-0}"
NO_INPUT_REGRESSION="${NO_INPUT_REGRESSION:-0}"
NO_WRITE_REGRESSION="${NO_WRITE_REGRESSION:-0}"
OUT="${OUT:-/tmp/phase40g-repair-device-report.txt}"

status="ACCEPTED"
reason="HomeLibraryTitlesConfirmed"

if [ "$HOME_TITLE_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="HomeTitleStillClipped"
elif [ "$LIBRARY_TITLES_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="LibraryTitlesStillWrong"
elif [ "$EPU_TITLES_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="EpuTitlesStillWrong"
elif [ "$TEXT_TITLES_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="TextTitlesStillWrong"
elif [ "$FOOTER_LABELS_STILL_CORRECT" != "1" ]; then
  status="REJECTED"; reason="FooterLabelsRegressed"
elif [ "$NO_INPUT_REGRESSION" != "1" ]; then
  status="REJECTED"; reason="InputRegression"
elif [ "$NO_WRITE_REGRESSION" != "1" ]; then
  status="REJECTED"; reason="WriteRegression"
fi

{
  echo "# Phase 40G Repair Device Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "home_title_confirmed=$HOME_TITLE_CONFIRMED"
  echo "library_titles_confirmed=$LIBRARY_TITLES_CONFIRMED"
  echo "epu_titles_confirmed=$EPU_TITLES_CONFIRMED"
  echo "text_titles_confirmed=$TEXT_TITLES_CONFIRMED"
  echo "footer_labels_still_correct=$FOOTER_LABELS_STILL_CORRECT"
  echo "no_input_regression=$NO_INPUT_REGRESSION"
  echo "no_write_regression=$NO_WRITE_REGRESSION"
  echo "marker=phase40g-repair=x4-home-full-width-reader-titles-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
