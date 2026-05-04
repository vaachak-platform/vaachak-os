#!/usr/bin/env bash
set -euo pipefail

PRIMARY_CANDIDATE="${PRIMARY_CANDIDATE:-Library title layout consistency}"
SECONDARY_CANDIDATE="${SECONDARY_CANDIDATE:-Footer spacing and alignment only}"
DEFER_HIGH_RISK="${DEFER_HIGH_RISK:-Long title wrapping and typography pagination changes}"
OUT="${OUT:-/tmp/phase40e-polish-candidate-backlog.txt}"

{
  echo "# Phase 40E Reader UX Polish Candidate Backlog"
  echo "status=ACCEPTED"
  echo "plan_only=true"
  echo "primary_candidate=$PRIMARY_CANDIDATE"
  echo "secondary_candidate=$SECONDARY_CANDIDATE"
  echo "defer_high_risk=$DEFER_HIGH_RISK"
  echo "marker=phase40e=x4-reader-ux-polish-candidate-plan-ok"
  echo
  echo "## candidates"
  echo "1. Library title layout consistency | priority=first | risk=low"
  echo "2. Footer spacing/alignment only | priority=second | risk=low"
  echo "3. Reader header/status placement | priority=third | risk=medium"
  echo "4. Reader body typography/line spacing | priority=later | risk=medium"
  echo "5. Selection highlight treatment | priority=later | risk=medium"
  echo "6. Restore/status copy polish | priority=later | risk=low"
  echo "7. Empty-state copy/layout | priority=later | risk=low"
  echo "8. Long title wrapping | priority=later | risk=high"
  echo
  echo "## protected baseline"
  echo "- Footer labels remain: Back Select Open Stay"
  echo "- Input mapping remains unchanged"
  echo "- Write lane remains closed"
  echo "- Display geometry/rotation remains unchanged"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
