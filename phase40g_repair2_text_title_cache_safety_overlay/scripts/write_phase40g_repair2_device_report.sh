#!/usr/bin/env bash
set -euo pipefail

TEXT_TITLES_NOT_BODY_CONFIRMED="${TEXT_TITLES_NOT_BODY_CONFIRMED:-0}"
NO_BAD_GUTENBERG_LINES_CONFIRMED="${NO_BAD_GUTENBERG_LINES_CONFIRMED:-0}"
EPUB_EPU_TITLES_STILL_OK="${EPUB_EPU_TITLES_STILL_OK:-0}"
HOME_TITLE_STILL_OK="${HOME_TITLE_STILL_OK:-0}"
FOOTER_LABELS_STILL_CORRECT="${FOOTER_LABELS_STILL_CORRECT:-0}"
NO_INPUT_REGRESSION="${NO_INPUT_REGRESSION:-0}"
NO_WRITE_REGRESSION="${NO_WRITE_REGRESSION:-0}"
OUT="${OUT:-/tmp/phase40g-repair2-device-report.txt}"

status="ACCEPTED"
reason="TextTitleCacheSafetyConfirmed"

if [ "$TEXT_TITLES_NOT_BODY_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="TextTitlesStillUseBodyLines"
elif [ "$NO_BAD_GUTENBERG_LINES_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="BadGutenbergLinesStillVisible"
elif [ "$EPUB_EPU_TITLES_STILL_OK" != "1" ]; then
  status="REJECTED"; reason="EpubEpuTitlesRegressed"
elif [ "$HOME_TITLE_STILL_OK" != "1" ]; then
  status="REJECTED"; reason="HomeTitleRegressed"
elif [ "$FOOTER_LABELS_STILL_CORRECT" != "1" ]; then
  status="REJECTED"; reason="FooterLabelsRegressed"
elif [ "$NO_INPUT_REGRESSION" != "1" ]; then
  status="REJECTED"; reason="InputRegression"
elif [ "$NO_WRITE_REGRESSION" != "1" ]; then
  status="REJECTED"; reason="WriteRegression"
fi

{
  echo "# Phase 40G Repair 2 Device Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "text_titles_not_body_confirmed=$TEXT_TITLES_NOT_BODY_CONFIRMED"
  echo "no_bad_gutenberg_lines_confirmed=$NO_BAD_GUTENBERG_LINES_CONFIRMED"
  echo "epub_epu_titles_still_ok=$EPUB_EPU_TITLES_STILL_OK"
  echo "home_title_still_ok=$HOME_TITLE_STILL_OK"
  echo "footer_labels_still_correct=$FOOTER_LABELS_STILL_CORRECT"
  echo "no_input_regression=$NO_INPUT_REGRESSION"
  echo "no_write_regression=$NO_WRITE_REGRESSION"
  echo "marker=phase40g-repair2=x4-text-title-cache-safety-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
