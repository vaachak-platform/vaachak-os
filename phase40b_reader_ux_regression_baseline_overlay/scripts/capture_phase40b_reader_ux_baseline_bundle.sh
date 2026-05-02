#!/usr/bin/env bash
set -euo pipefail

STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase40b-reader-ux-baseline-bundle-$STAMP.tar.gz}"
MANIFEST="${MANIFEST:-/tmp/phase40b-reader-ux-baseline-bundle-$STAMP.txt}"

files=(
  /tmp/phase40b-write-lane-closed-guard.txt
  /tmp/phase40b-reader-ux-surface.txt
  /tmp/phase40b-epub-title-baseline.txt
  /tmp/phase40b-manual-device-ux-report.txt
  /tmp/phase40b-reader-ux-regression-baseline-acceptance.txt
)

present=()
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    present+=("$file")
  fi
done

if [ "${#present[@]}" -eq 0 ]; then
  echo "no Phase 40B baseline files found under /tmp" >&2
  exit 2
fi

tar -czf "$OUT" "${present[@]}"

{
  echo "# Phase 40B Reader UX Baseline Bundle"
  echo "status=ACCEPTED"
  echo "bundle=$OUT"
  echo "marker=phase40b=x4-reader-ux-regression-baseline-ok"
  echo
  for file in "${present[@]}"; do
    echo "- $file"
  done
} | tee "$MANIFEST"

echo
echo "phase40b-bundle=$OUT"
echo "phase40b-bundle-manifest=$MANIFEST"
