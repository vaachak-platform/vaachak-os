#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PLAN_OUT="/tmp/phase39n-review-delete-later-removal-dry-run.md"
GUARD_OUT="/tmp/phase39n-accepted-write-path-guard.txt"
OUT="${OUT:-/tmp/phase39n-review-delete-later-removal-dry-run-acceptance.txt}"

"$ROOT/phase39n_review_delete_later_candidate_removal_dry_run_overlay/scripts/guard_phase39n_accepted_write_path.sh" >/dev/null
"$ROOT/phase39n_review_delete_later_candidate_removal_dry_run_overlay/scripts/plan_phase39n_review_delete_later_removal_dry_run.sh" >/dev/null

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
candidate_rows="$(grep -c '^| state_io_' "$PLAN_OUT" || true)"
files_deleted="0"

status="ACCEPTED"
reason="DryRunPlanAccepted"

if [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="GuardBlocked"
elif [ "$candidate_rows" -ne 14 ]; then
  status="REJECTED"
  reason="CandidateCountMismatch"
fi

{
  echo "# Phase 39N Review-Delete-Later Removal Dry Run Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "dry_run_only=true"
  echo "deletes_code_now=false"
  echo "moves_code_now=false"
  echo "guard_status=$guard_status"
  echo "candidate_rows=$candidate_rows/14"
  echo "files_deleted=$files_deleted"
  echo "marker=phase39n=x4-review-delete-later-candidate-removal-dry-run-ok"
  echo "plan=$PLAN_OUT"
  echo "guard=$GUARD_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
