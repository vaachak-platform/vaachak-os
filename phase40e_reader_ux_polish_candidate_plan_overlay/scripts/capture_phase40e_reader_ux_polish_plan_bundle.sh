#!/usr/bin/env bash
set -euo pipefail

STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase40e-reader-ux-polish-plan-bundle-$STAMP.tar.gz}"
MANIFEST="${MANIFEST:-/tmp/phase40e-reader-ux-polish-plan-bundle-$STAMP.txt}"

files=(
  /tmp/phase40e-reader-ux-polish-scope-guard.txt
  /tmp/phase40e-reader-ux-source-inspection.txt
  /tmp/phase40e-polish-candidate-backlog.txt
  /tmp/phase40e-reader-ux-polish-candidate-plan.md
  /tmp/phase40e-reader-ux-polish-candidate-plan-acceptance.txt
)

present=()
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    present+=("$file")
  fi
done

if [ "${#present[@]}" -eq 0 ]; then
  echo "no Phase 40E plan files found under /tmp" >&2
  exit 2
fi

tar -czf "$OUT" "${present[@]}"

{
  echo "# Phase 40E Reader UX Polish Plan Bundle"
  echo "status=ACCEPTED"
  echo "bundle=$OUT"
  echo "marker=phase40e=x4-reader-ux-polish-candidate-plan-ok"
  echo
  for file in "${present[@]}"; do
    echo "- $file"
  done
} | tee "$MANIFEST"

echo
echo "phase40e-bundle=$OUT"
echo "phase40e-bundle-manifest=$MANIFEST"
