#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
GUARD_OUT="${GUARD_OUT:-/tmp/phase40c-reader-ux-baseline-guard.txt}"
FOOTER_OUT="${FOOTER_OUT:-/tmp/phase40c-footer-button-sources.txt}"
MAPPING_OUT="${MAPPING_OUT:-/tmp/phase40c-button-mapping-candidates.txt}"
EXPECTED_OUT="${EXPECTED_OUT:-/tmp/phase40c-expected-footer-labels-baseline.txt}"
PLAN_OUT="${PLAN_OUT:-/tmp/phase40c-footer-button-label-fix-plan.md}"
OUT="${OUT:-/tmp/phase40c-footer-button-label-baseline-fix-plan-acceptance.txt}"

"$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/guard_phase40c_reader_ux_baseline.sh" >/dev/null
"$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_footer_button_sources.sh" >/dev/null
"$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_button_mapping_candidates.sh" >/dev/null

if [ ! -f "$EXPECTED_OUT" ]; then
  "$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/write_phase40c_expected_footer_labels_baseline.sh" >/dev/null
fi

"$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/plan_phase40c_footer_button_label_fix.sh" >/dev/null

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
expected_status="$(grep '^status=' "$EXPECTED_OUT" | head -1 | cut -d= -f2-)"
plan_marker_count="$(grep -c 'phase40c=x4-footer-button-label-baseline-fix-plan-ok' "$PLAN_OUT" 2>/dev/null || true)"

footer_scan_lines="$(wc -l < "$FOOTER_OUT" | tr -d ' ')"
mapping_scan_lines="$(wc -l < "$MAPPING_OUT" | tr -d ' ')"

status="ACCEPTED"
reason="FooterButtonFixPlanAccepted"

if [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="ReaderUxBaselineGuardFailed"
elif [ "$expected_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="ExpectedFooterLabelsMissing"
elif [ "$plan_marker_count" = "0" ]; then
  status="REJECTED"
  reason="FixPlanMissing"
fi

{
  echo "# Phase 40C Footer/Button Label Baseline and Fix Plan Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "plan_only=true"
  echo "changes_rendering_now=false"
  echo "changes_input_now=false"
  echo "touches_write_lane=false"
  echo "guard_status=$guard_status"
  echo "expected_status=$expected_status"
  echo "footer_scan_lines=$footer_scan_lines"
  echo "mapping_scan_lines=$mapping_scan_lines"
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
  echo "guard=$GUARD_OUT"
  echo "footer_sources=$FOOTER_OUT"
  echo "button_mapping=$MAPPING_OUT"
  echo "expected_labels=$EXPECTED_OUT"
  echo "fix_plan=$PLAN_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
