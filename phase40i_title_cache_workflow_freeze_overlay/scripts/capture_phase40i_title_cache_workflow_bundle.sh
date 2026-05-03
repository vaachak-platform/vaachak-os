#!/usr/bin/env bash
set -euo pipefail

STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase40i-title-cache-workflow-freeze-bundle-$STAMP.tar.gz}"
MANIFEST="${MANIFEST:-/tmp/phase40i-title-cache-workflow-freeze-bundle-$STAMP.txt}"

files=(
  /tmp/phase40i-title-cache-workflow-source-guard.txt
  /tmp/phase40i-title-cache-workflow-inspection.txt
  /tmp/phase40i-title-cache-workflow-baseline.txt
  /tmp/phase40i-device-report.txt
  /tmp/phase40i-title-cache-workflow-freeze-acceptance.txt
)

present=()
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    present+=("$file")
  fi
done

if [ "${#present[@]}" -eq 0 ]; then
  echo "no Phase 40I files found under /tmp" >&2
  exit 2
fi

tar -czf "$OUT" "${present[@]}"

{
  echo "# Phase 40I Title Cache Workflow Freeze Bundle"
  echo "status=ACCEPTED"
  echo "bundle=$OUT"
  echo "marker=phase40i=x4-title-cache-workflow-freeze-ok"
  echo
  for file in "${present[@]}"; do
    echo "- $file"
  done
} | tee "$MANIFEST"

echo "phase40i-bundle=$OUT"
echo "phase40i-bundle-manifest=$MANIFEST"
