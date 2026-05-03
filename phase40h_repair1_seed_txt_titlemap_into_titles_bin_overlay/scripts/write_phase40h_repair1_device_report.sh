#!/usr/bin/env bash
set -euo pipefail

TXT_TITLES_FROM_TITLES_BIN_CONFIRMED="${TXT_TITLES_FROM_TITLES_BIN_CONFIRMED:-0}"
TXT_BODY_TITLES_GONE="${TXT_BODY_TITLES_GONE:-0}"
EPUB_EPU_TITLES_STILL_OK="${EPUB_EPU_TITLES_STILL_OK:-0}"
HOME_TITLE_STILL_OK="${HOME_TITLE_STILL_OK:-0}"
FOOTER_LABELS_STILL_CORRECT="${FOOTER_LABELS_STILL_CORRECT:-0}"
NO_INPUT_REGRESSION="${NO_INPUT_REGRESSION:-0}"
NO_WRITE_REGRESSION="${NO_WRITE_REGRESSION:-0}"
OUT="${OUT:-/tmp/phase40h-repair1-device-report.txt}"

status="ACCEPTED"
reason="TxtTitlesLoadedFromTitlesBinConfirmed"

if [ "$TXT_TITLES_FROM_TITLES_BIN_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="TxtTitlesNotLoadedFromTitlesBin"
elif [ "$TXT_BODY_TITLES_GONE" != "1" ]; then
  status="REJECTED"; reason="TxtBodyTitlesStillVisible"
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
  echo "# Phase 40H Repair 1 Device Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "txt_titles_from_titles_bin_confirmed=$TXT_TITLES_FROM_TITLES_BIN_CONFIRMED"
  echo "txt_body_titles_gone=$TXT_BODY_TITLES_GONE"
  echo "epub_epu_titles_still_ok=$EPUB_EPU_TITLES_STILL_OK"
  echo "home_title_still_ok=$HOME_TITLE_STILL_OK"
  echo "footer_labels_still_correct=$FOOTER_LABELS_STILL_CORRECT"
  echo "no_input_regression=$NO_INPUT_REGRESSION"
  echo "no_write_regression=$NO_WRITE_REGRESSION"
  echo "marker=phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
