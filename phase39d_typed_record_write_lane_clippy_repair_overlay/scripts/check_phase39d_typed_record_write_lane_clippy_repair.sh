#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_write_lane.rs"

test -f "$FILE"

if rg -n 'if let Some\(book_id\) = request\.book_id \{' "$FILE"; then
  echo "old nested book_id validation block may still be present" >&2
  exit 1
fi

rg -n 'if let Some\(book_id\) = request\.book_id' "$FILE" >/dev/null
rg -n '&& !book_id\.is_hex8\(\)' "$FILE" >/dev/null

echo "phase39d-clippy-repair-check=ok"
