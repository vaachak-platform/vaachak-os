#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_owned_sdfat_writer.rs"

test -f "$FILE"

if rg -n 'self\.record_result\(\s*$' "$FILE" >/tmp/phase39f_record_result_lines.txt; then
  if rg -n 'self\.record_result\(\s*$' "$FILE" -A3 | rg 'self\.ops'; then
    echo "old direct overlapping borrow pattern may still be present" >&2
    exit 1
  fi
fi

if rg -n 'self\.record_result\(self\.ops\.write_record_atomic' "$FILE"; then
  echo "old atomic overlapping borrow pattern still present" >&2
  exit 1
fi

rg -n 'let result = self' "$FILE" >/dev/null
rg -n 'write_record_direct' "$FILE" >/dev/null
rg -n 'write_record_atomic' "$FILE" >/dev/null

echo "phase39f-borrow-repair-check=ok"
