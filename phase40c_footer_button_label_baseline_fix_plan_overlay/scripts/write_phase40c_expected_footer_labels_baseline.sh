#!/usr/bin/env bash
set -euo pipefail

EXPECTED_HOME_FOOTER="${EXPECTED_HOME_FOOTER:-Back Select Open Stay}"
EXPECTED_FILES_FOOTER="${EXPECTED_FILES_FOOTER:-Back Select Open Stay}"
EXPECTED_READER_FOOTER="${EXPECTED_READER_FOOTER:-Back Select Open Stay}"
EXPECTED_DIALOG_FOOTER="${EXPECTED_DIALOG_FOOTER:-Back Select Open Stay}"
PHYSICAL_ORDER_NOTE="${PHYSICAL_ORDER_NOTE:-left-to-right labels must match actual physical button actions}"
OUT="${OUT:-/tmp/phase40c-expected-footer-labels-baseline.txt}"

{
  echo "# Phase 40C Expected Footer Labels Baseline"
  echo "status=ACCEPTED"
  echo "expected_home_footer=$EXPECTED_HOME_FOOTER"
  echo "expected_files_footer=$EXPECTED_FILES_FOOTER"
  echo "expected_reader_footer=$EXPECTED_READER_FOOTER"
  echo "expected_dialog_footer=$EXPECTED_DIALOG_FOOTER"
  echo "physical_order_note=$PHYSICAL_ORDER_NOTE"
  echo "known_priority_fix=Files/Library and Reader footer labels must not show Select/Open/Back/Stay if physical behavior is Back/Select/Open/Stay"
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
  echo
  echo "## expected slot order"
  echo "slot1=Back"
  echo "slot2=Select"
  echo "slot3=Open"
  echo "slot4=Stay"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
