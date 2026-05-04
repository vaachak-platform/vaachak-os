#!/usr/bin/env bash
set -euo pipefail

STAMP="$(date +%Y%m%d-%H%M%S)"
OUT="${OUT:-/tmp/phase41h-biscuit-ui-freeze-bundle-$STAMP.tar.gz}"
MANIFEST="${MANIFEST:-/tmp/phase41h-biscuit-ui-freeze-bundle-$STAMP.txt}"

files="
/tmp/phase41h-biscuit-ui-freeze-inspection.txt
/tmp/phase41h-device-biscuit-ui-report.txt
/tmp/phase41h-biscuit-ui-freeze-acceptance.txt
"

present_file="$(mktemp)"
for file in $files; do
  if [ -f "$file" ]; then
    echo "$file" >> "$present_file"
  fi
done

count="$(wc -l < "$present_file" | tr -d ' ')"
if [ "$count" = "0" ]; then
  echo "no Phase 41H files found under /tmp" >&2
  rm -f "$present_file"
  exit 2
fi

tar -czf "$OUT" -T "$present_file"

{
  echo "# Phase 41H Biscuit UI Freeze Bundle"
  echo "status=ACCEPTED"
  echo "bundle=$OUT"
  echo "marker=phase41h=x4-biscuit-ui-acceptance-freeze-ok"
  echo
  cat "$present_file" | sed 's/^/- /'
} | tee "$MANIFEST"

rm -f "$present_file"

echo "phase41h-bundle=$OUT"
echo "phase41h-bundle-manifest=$MANIFEST"
