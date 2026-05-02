#!/usr/bin/env bash
set -euo pipefail

file="docs/phase35_full/PHASE35_FULL_DEVICE_TEST_PLAN.md"
if [[ ! -f "$file" ]]; then
  echo "FAIL missing $file"
  exit 1
fi

for token in \
  'vaachak=x4-physical-runtime-owned' \
  'TXT/MD opens' \
  'EPUB/EPU opens' \
  'Continue restores progress' \
  'Bookmark' \
  'Display refresh' \
  'Input navigation'; do
  if rg -n "$token" "$file" >/dev/null; then
    echo "OK   device test plan covers: $token"
  else
    echo "FAIL device test plan missing: $token"
    exit 1
  fi
done
