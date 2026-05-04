#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40l-runtime-prune-no-behavior-change-guard.txt}"

status="ACCEPTED"
reason="NoBehaviorChangesDetected"

changed="$(git diff --name-only 2>/dev/null || true)"

# Phase 40L may add docs/scripts only.
forbidden="$(printf '%s\n' "$changed" | grep -E '^(vendor/pulp-os/src/apps/|vendor/pulp-os/kernel/src/kernel/|hal-xteink-x4/src/|target-xteink-x4/src/)' || true)"

if [ -n "$forbidden" ]; then
  status="REJECTED"
  reason="BehaviorOrRuntimeFilesChanged"
fi

{
  echo "# Phase 40L No Behavior Change Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo
  echo "## changed files"
  printf '%s\n' "$changed"
  echo
  echo "## forbidden changed files"
  printf '%s\n' "$forbidden"
  echo
  echo "marker=phase40l=x4-runtime-phase-scaffolding-prune-plan-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
