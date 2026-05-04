#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40l-runtime-scaffolding-prune-plan.md}"
CLASSIFICATION="${CLASSIFICATION:-/tmp/phase40l-runtime-scaffolding-classification.tsv}"

./scripts/repo-cleanup/phase40l_inspect_runtime_scaffolding.sh >/dev/null
python3 ./scripts/repo-cleanup/phase40l_classify_runtime_scaffolding.py --out "$CLASSIFICATION" >/dev/null

prune_count="$(awk -F '\t' 'NR>1 && $1=="PRUNE-CANDIDATE" {c++} END {print c+0}' "$CLASSIFICATION")"
review_count="$(awk -F '\t' 'NR>1 && $1=="REVIEW" {c++} END {print c+0}' "$CLASSIFICATION")"
keep_count="$(awk -F '\t' 'NR>1 && $1=="KEEP" {c++} END {print c+0}' "$CLASSIFICATION")"
do_not_touch_count="$(awk -F '\t' 'NR>1 && $1=="DO-NOT-TOUCH" {c++} END {print c+0}' "$CLASSIFICATION")"

{
  echo "# Phase 40L Runtime Phase Scaffolding Prune Plan"
  echo
  echo "Status: PLAN ONLY"
  echo
  echo "## Summary"
  echo
  echo "- prune_candidate_count=$prune_count"
  echo "- review_count=$review_count"
  echo "- keep_count=$keep_count"
  echo "- do_not_touch_count=$do_not_touch_count"
  echo
  echo "## PRUNE-CANDIDATE"
  echo
  awk -F '\t' 'NR>1 && $1=="PRUNE-CANDIDATE" {print "- `" $3 "` — " $2}' "$CLASSIFICATION"
  echo
  echo "## REVIEW"
  echo
  awk -F '\t' 'NR>1 && $1=="REVIEW" {print "- `" $3 "` — " $2}' "$CLASSIFICATION"
  echo
  echo "## DO-NOT-TOUCH"
  echo
  awk -F '\t' 'NR>1 && $1=="DO-NOT-TOUCH" {print "- `" $3 "` — " $2}' "$CLASSIFICATION"
  echo
  echo "## KEEP"
  echo
  awk -F '\t' 'NR>1 && $1=="KEEP" {print "- `" $3 "` — " $2}' "$CLASSIFICATION"
  echo
  echo "## Next phase rule"
  echo
  echo "Do not delete anything in Phase 40L. Use this plan to create:"
  echo
  echo '```text'
  echo "Phase 40M — Guarded Runtime Scaffolding Prune Patch"
  echo '```'
  echo
  echo "Phase 40M must remove files one small group at a time and run full build/device checks."
  echo
  echo "marker=phase40l=x4-runtime-phase-scaffolding-prune-plan-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
