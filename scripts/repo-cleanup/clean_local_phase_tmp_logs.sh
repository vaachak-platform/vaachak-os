#!/usr/bin/env bash
set -euo pipefail

APPLY="${APPLY:-0}"
OUT="${OUT:-/tmp/phase40k-clean-local-phase-tmp-logs.txt}"

files_file="$(mktemp)"
find /tmp -maxdepth 1 -type f \( \
  -name 'phase*.txt' -o \
  -name 'phase*.md' -o \
  -name 'phase*.tar.gz' \
\) -print 2>/dev/null | sort > "$files_file"

count="$(wc -l < "$files_file" | tr -d ' ')"

{
  echo "# Local /tmp Phase Log Cleanup"
  echo "apply=$APPLY"
  echo "count=$count"
  cat "$files_file"
} | tee "$OUT"

if [ "$APPLY" != "1" ]; then
  echo "Dry run only. Use APPLY=1 to delete local /tmp phase logs."
  rm -f "$files_file"
  exit 0
fi

while IFS= read -r file; do
  [ -f "$file" ] || continue
  rm -fv "$file"
done < "$files_file"

rm -f "$files_file"
echo "phase40k-local-log-cleanup=ok"
