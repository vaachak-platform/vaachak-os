#!/usr/bin/env bash
set -euo pipefail

FILES_FOOTER_CONFIRMED="${FILES_FOOTER_CONFIRMED:-0}"
READER_FOOTER_CONFIRMED="${READER_FOOTER_CONFIRMED:-0}"
PHYSICAL_BEHAVIOR_CONFIRMED="${PHYSICAL_BEHAVIOR_CONFIRMED:-0}"
NO_INPUT_REGRESSION="${NO_INPUT_REGRESSION:-0}"
NO_WRITE_REGRESSION="${NO_WRITE_REGRESSION:-0}"
FILES_FOOTER_OBSERVED="${FILES_FOOTER_OBSERVED:-Back Select Open Stay}"
READER_FOOTER_OBSERVED="${READER_FOOTER_OBSERVED:-Back Select Open Stay}"
OUT="${OUT:-/tmp/phase40d-device-footer-label-report.txt}"

status="ACCEPTED"
reason="DeviceFooterLabelsConfirmed"

if [ "$FILES_FOOTER_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="FilesFooterNotConfirmed"
elif [ "$READER_FOOTER_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="ReaderFooterNotConfirmed"
elif [ "$PHYSICAL_BEHAVIOR_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="PhysicalBehaviorNotConfirmed"
elif [ "$NO_INPUT_REGRESSION" != "1" ]; then
  status="REJECTED"
  reason="InputRegressionObserved"
elif [ "$NO_WRITE_REGRESSION" != "1" ]; then
  status="REJECTED"
  reason="WriteRegressionObserved"
fi

{
  echo "# Phase 40D Device Footer Label Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "files_footer_confirmed=$FILES_FOOTER_CONFIRMED"
  echo "reader_footer_confirmed=$READER_FOOTER_CONFIRMED"
  echo "physical_behavior_confirmed=$PHYSICAL_BEHAVIOR_CONFIRMED"
  echo "no_input_regression=$NO_INPUT_REGRESSION"
  echo "no_write_regression=$NO_WRITE_REGRESSION"
  echo "files_footer_observed=$FILES_FOOTER_OBSERVED"
  echo "reader_footer_observed=$READER_FOOTER_OBSERVED"
  echo "expected_footer=Back Select Open Stay"
  echo "marker=phase40d=x4-footer-button-label-rendering-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
