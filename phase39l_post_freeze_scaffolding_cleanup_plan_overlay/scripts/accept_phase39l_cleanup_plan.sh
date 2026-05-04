#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase39l-cleanup-plan-acceptance.txt}"
GUARD_OUT="/tmp/phase39l-accepted-write-path-guard.txt"
PLAN_OUT="/tmp/phase39l-scaffolding-cleanup-plan.md"

./phase39l_post_freeze_scaffolding_cleanup_plan_overlay/scripts/guard_phase39l_accepted_write_path.sh >/dev/null
./phase39l_post_freeze_scaffolding_cleanup_plan_overlay/scripts/plan_phase39l_scaffolding_cleanup.sh >/dev/null

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
delete_candidates="$(grep -c '^- target-xteink-x4/src/vaachak_x4/runtime/state_io_' "$PLAN_OUT" || true)"

status="ACCEPTED"
reason="ReviewPlanAccepted"
if [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="GuardBlocked"
fi

{
  echo "# Phase 39L Cleanup Plan Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "review_only=true"
  echo "deletes_code_now=false"
  echo "guard_status=$guard_status"
  echo "candidate_lines=$delete_candidates"
  echo "marker=phase39l=x4-post-freeze-scaffolding-cleanup-plan-ok"
  echo "plan=$PLAN_OUT"
  echo "guard=$GUARD_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
