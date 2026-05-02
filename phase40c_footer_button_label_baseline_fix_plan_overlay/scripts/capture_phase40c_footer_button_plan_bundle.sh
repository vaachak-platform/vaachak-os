#!/usr/bin/env bash
set -euo pipefail

STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase40c-footer-button-plan-bundle-$STAMP.tar.gz}"
MANIFEST="${MANIFEST:-/tmp/phase40c-footer-button-plan-bundle-$STAMP.txt}"

files=(
  /tmp/phase40c-reader-ux-baseline-guard.txt
  /tmp/phase40c-footer-button-sources.txt
  /tmp/phase40c-button-mapping-candidates.txt
  /tmp/phase40c-expected-footer-labels-baseline.txt
  /tmp/phase40c-footer-button-label-fix-plan.md
  /tmp/phase40c-footer-button-label-baseline-fix-plan-acceptance.txt
)

present=()
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    present+=("$file")
  fi
done

if [ "${#present[@]}" -eq 0 ]; then
  echo "no Phase 40C plan files found under /tmp" >&2
  exit 2
fi

tar -czf "$OUT" "${present[@]}"

{
  echo "# Phase 40C Footer/Button Plan Bundle"
  echo "status=ACCEPTED"
  echo "bundle=$OUT"
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
  echo
  for file in "${present[@]}"; do
    echo "- $file"
  done
} | tee "$MANIFEST"

echo
echo "phase40c-bundle=$OUT"
echo "phase40c-bundle-manifest=$MANIFEST"
