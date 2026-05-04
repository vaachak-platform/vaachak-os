#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40l-runtime-phase-scaffolding-prune-plan-acceptance.txt}"
PLAN="${PLAN:-/tmp/phase40l-runtime-scaffolding-prune-plan.md}"
CLASSIFICATION="${CLASSIFICATION:-/tmp/phase40l-runtime-scaffolding-classification.tsv}"

./scripts/repo-cleanup/phase40l_guard_no_behavior_changes.sh >/dev/null
./scripts/repo-cleanup/phase40l_generate_prune_plan.sh >/dev/null

prune_count="$(awk -F '\t' 'NR>1 && $1=="PRUNE-CANDIDATE" {c++} END {print c+0}' "$CLASSIFICATION")"
review_count="$(awk -F '\t' 'NR>1 && $1=="REVIEW" {c++} END {print c+0}' "$CLASSIFICATION")"
do_not_touch_count="$(awk -F '\t' 'NR>1 && $1=="DO-NOT-TOUCH" {c++} END {print c+0}' "$CLASSIFICATION")"

status="ACCEPTED"
reason="RuntimeScaffoldingPrunePlanAccepted"

if [ ! -f "$PLAN" ]; then
  status="REJECTED"
  reason="PrunePlanMissing"
elif [ "$prune_count" = "0" ] && [ "$review_count" = "0" ]; then
  status="REJECTED"
  reason="NoRuntimeScaffoldingFound"
fi

{
  echo "# Phase 40L Runtime Phase Scaffolding Prune Plan Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "plan_only=true"
  echo "prune_candidate_count=$prune_count"
  echo "review_count=$review_count"
  echo "do_not_touch_count=$do_not_touch_count"
  echo "changes_behavior=false"
  echo "deletes_files=false"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "recommended_next=Phase 40M — Guarded Runtime Scaffolding Prune Patch"
  echo "marker=phase40l=x4-runtime-phase-scaffolding-prune-plan-ok"
  echo "plan=$PLAN"
  echo "classification=$CLASSIFICATION"
  echo "inspection=/tmp/phase40l-runtime-scaffolding-inspection.txt"
  echo "guard=/tmp/phase40l-runtime-prune-no-behavior-change-guard.txt"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
