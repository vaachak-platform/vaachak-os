#!/usr/bin/env bash
set -euo pipefail

TITLEMAP_TO_TITLES_WORKFLOW_CONFIRMED="${TITLEMAP_TO_TITLES_WORKFLOW_CONFIRMED:-0}"
TXT_TITLES_FROM_TITLES_BIN_CONFIRMED="${TXT_TITLES_FROM_TITLES_BIN_CONFIRMED:-0}"
TXT_BODY_SCANNING_DISABLED_CONFIRMED="${TXT_BODY_SCANNING_DISABLED_CONFIRMED:-0}"
BAD_PHRASES_ABSENT_CONFIRMED="${BAD_PHRASES_ABSENT_CONFIRMED:-0}"
EPUB_EPU_METADATA_CONFIRMED="${EPUB_EPU_METADATA_CONFIRMED:-0}"
HOME_LIBRARY_READER_REGRESSION_OK="${HOME_LIBRARY_READER_REGRESSION_OK:-0}"
NO_INPUT_WRITE_GEOMETRY_REGRESSION="${NO_INPUT_WRITE_GEOMETRY_REGRESSION:-0}"
OUT="${OUT:-/tmp/phase40i-device-report.txt}"

status="ACCEPTED"
reason="TitleCacheWorkflowDeviceBaselineConfirmed"

if [ "$TITLEMAP_TO_TITLES_WORKFLOW_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="TitleMapToTitlesWorkflowNotConfirmed"
elif [ "$TXT_TITLES_FROM_TITLES_BIN_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="TxtTitlesFromTitlesBinNotConfirmed"
elif [ "$TXT_BODY_SCANNING_DISABLED_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="TxtBodyScanningDisabledNotConfirmed"
elif [ "$BAD_PHRASES_ABSENT_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="BadPhrasesStillVisible"
elif [ "$EPUB_EPU_METADATA_CONFIRMED" != "1" ]; then
  status="REJECTED"; reason="EpubEpuMetadataNotConfirmed"
elif [ "$HOME_LIBRARY_READER_REGRESSION_OK" != "1" ]; then
  status="REJECTED"; reason="HomeLibraryReaderRegression"
elif [ "$NO_INPUT_WRITE_GEOMETRY_REGRESSION" != "1" ]; then
  status="REJECTED"; reason="InputWriteGeometryRegression"
fi

{
  echo "# Phase 40I Device Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "titlemap_to_titles_workflow_confirmed=$TITLEMAP_TO_TITLES_WORKFLOW_CONFIRMED"
  echo "txt_titles_from_titles_bin_confirmed=$TXT_TITLES_FROM_TITLES_BIN_CONFIRMED"
  echo "txt_body_scanning_disabled_confirmed=$TXT_BODY_SCANNING_DISABLED_CONFIRMED"
  echo "bad_phrases_absent_confirmed=$BAD_PHRASES_ABSENT_CONFIRMED"
  echo "epub_epu_metadata_confirmed=$EPUB_EPU_METADATA_CONFIRMED"
  echo "home_library_reader_regression_ok=$HOME_LIBRARY_READER_REGRESSION_OK"
  echo "no_input_write_geometry_regression=$NO_INPUT_WRITE_GEOMETRY_REGRESSION"
  echo "marker=phase40i=x4-title-cache-workflow-freeze-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
