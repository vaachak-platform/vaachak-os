#!/usr/bin/env bash
set -euo pipefail

LIBRARY_LAYOUT_CONFIRMED="${LIBRARY_LAYOUT_CONFIRMED:-0}"
EPUB_TITLES_STILL_CORRECT="${EPUB_TITLES_STILL_CORRECT:-0}"
FOOTER_LABELS_STILL_CORRECT="${FOOTER_LABELS_STILL_CORRECT:-0}"
READER_OPEN_BACK_RESTORE_CONFIRMED="${READER_OPEN_BACK_RESTORE_CONFIRMED:-0}"
NO_INPUT_REGRESSION="${NO_INPUT_REGRESSION:-0}"
NO_WRITE_REGRESSION="${NO_WRITE_REGRESSION:-0}"
LIBRARY_LAYOUT_OBSERVED="${LIBRARY_LAYOUT_OBSERVED:-library title rows consistent and readable}"
OUT="${OUT:-/tmp/phase40f-device-library-title-layout-report.txt}"

status="ACCEPTED"
reason="DeviceLibraryTitleLayoutConfirmed"

if [ "$LIBRARY_LAYOUT_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="LibraryLayoutNotConfirmed"
elif [ "$EPUB_TITLES_STILL_CORRECT" != "1" ]; then
  status="REJECTED"
  reason="EpubTitlesRegressed"
elif [ "$FOOTER_LABELS_STILL_CORRECT" != "1" ]; then
  status="REJECTED"
  reason="FooterLabelsRegressed"
elif [ "$READER_OPEN_BACK_RESTORE_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="ReaderOpenBackRestoreRegressed"
elif [ "$NO_INPUT_REGRESSION" != "1" ]; then
  status="REJECTED"
  reason="InputRegressionObserved"
elif [ "$NO_WRITE_REGRESSION" != "1" ]; then
  status="REJECTED"
  reason="WriteRegressionObserved"
fi

{
  echo "# Phase 40F Device Library Title Layout Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "library_layout_confirmed=$LIBRARY_LAYOUT_CONFIRMED"
  echo "epub_titles_still_correct=$EPUB_TITLES_STILL_CORRECT"
  echo "footer_labels_still_correct=$FOOTER_LABELS_STILL_CORRECT"
  echo "reader_open_back_restore_confirmed=$READER_OPEN_BACK_RESTORE_CONFIRMED"
  echo "no_input_regression=$NO_INPUT_REGRESSION"
  echo "no_write_regression=$NO_WRITE_REGRESSION"
  echo "library_layout_observed=$LIBRARY_LAYOUT_OBSERVED"
  echo "marker=phase40f=x4-library-title-layout-polish-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
