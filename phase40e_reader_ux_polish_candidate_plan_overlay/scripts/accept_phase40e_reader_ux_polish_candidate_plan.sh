#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
GUARD_OUT="${GUARD_OUT:-/tmp/phase40e-reader-ux-polish-scope-guard.txt}"
INSPECT_OUT="${INSPECT_OUT:-/tmp/phase40e-reader-ux-source-inspection.txt}"
BACKLOG_OUT="${BACKLOG_OUT:-/tmp/phase40e-polish-candidate-backlog.txt}"
PLAN_OUT="${PLAN_OUT:-/tmp/phase40e-reader-ux-polish-candidate-plan.md}"
OUT="${OUT:-/tmp/phase40e-reader-ux-polish-candidate-plan-acceptance.txt}"

"$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/guard_phase40e_reader_ux_polish_scope.sh" >/dev/null
"$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/inspect_phase40e_reader_ux_sources.sh" >/dev/null
"$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/write_phase40e_polish_candidate_backlog.sh" >/dev/null
"$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/plan_phase40e_reader_ux_polish_candidates.sh" >/dev/null

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
backlog_status="$(grep '^status=' "$BACKLOG_OUT" | head -1 | cut -d= -f2-)"
plan_marker_count="$(grep -c 'phase40e=x4-reader-ux-polish-candidate-plan-ok' "$PLAN_OUT" 2>/dev/null || true)"
source_scan_lines="$(wc -l < "$INSPECT_OUT" | tr -d ' ')"

status="ACCEPTED"
reason="ReaderUxPolishCandidatePlanAccepted"

if [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="ScopeGuardFailed"
elif [ "$backlog_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="BacklogMissing"
elif [ "$plan_marker_count" = "0" ]; then
  status="REJECTED"
  reason="PlanMissing"
fi

{
  echo "# Phase 40E Reader UX Polish Candidate Plan Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "plan_only=true"
  echo "changes_ux_now=false"
  echo "changes_footer_labels_now=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "guard_status=$guard_status"
  echo "backlog_status=$backlog_status"
  echo "source_scan_lines=$source_scan_lines"
  echo "recommended_next=Phase 40F — Library Title Layout Polish Patch"
  echo "marker=phase40e=x4-reader-ux-polish-candidate-plan-ok"
  echo "guard=$GUARD_OUT"
  echo "source_inspection=$INSPECT_OUT"
  echo "backlog=$BACKLOG_OUT"
  echo "plan=$PLAN_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
