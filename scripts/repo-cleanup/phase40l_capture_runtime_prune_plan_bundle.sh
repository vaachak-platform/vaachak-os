#!/usr/bin/env bash
set -euo pipefail

STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase40l-runtime-prune-plan-bundle-$STAMP.tar.gz}"
MANIFEST="${MANIFEST:-/tmp/phase40l-runtime-prune-plan-bundle-$STAMP.txt}"

files="
/tmp/phase40l-runtime-scaffolding-inspection.txt
/tmp/phase40l-runtime-scaffolding-classification.tsv
/tmp/phase40l-runtime-scaffolding-prune-plan.md
/tmp/phase40l-runtime-prune-no-behavior-change-guard.txt
/tmp/phase40l-runtime-phase-scaffolding-prune-plan-acceptance.txt
"

present_file="$(mktemp)"
for file in $files; do
  if [ -f "$file" ]; then
    echo "$file" >> "$present_file"
  fi
done

count="$(wc -l < "$present_file" | tr -d ' ')"
if [ "$count" = "0" ]; then
  echo "no Phase 40L files found under /tmp" >&2
  rm -f "$present_file"
  exit 2
fi

tar -czf "$OUT" -T "$present_file"

{
  echo "# Phase 40L Runtime Prune Plan Bundle"
  echo "status=ACCEPTED"
  echo "bundle=$OUT"
  echo "marker=phase40l=x4-runtime-phase-scaffolding-prune-plan-ok"
  echo
  cat "$present_file" | sed 's/^/- /'
} | tee "$MANIFEST"

rm -f "$present_file"

echo "phase40l-bundle=$OUT"
echo "phase40l-bundle-manifest=$MANIFEST"
