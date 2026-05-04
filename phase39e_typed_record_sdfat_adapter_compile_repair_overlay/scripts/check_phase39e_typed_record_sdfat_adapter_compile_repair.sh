#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_sdfat_adapter.rs"

test -f "$FILE"

if rg -n 'Phase39dBookId|Phase39dTypedWritePreflight|Phase39dTypedWriteReport' "$FILE"; then
  echo "unused Phase39D import still present in Phase 39E adapter" >&2
  exit 1
fi

if rg -n 'pub const fn wrote_once\(self\)' "$FILE"; then
  echo "wrote_once is still const fn" >&2
  exit 1
fi

rg -n 'pub fn wrote_once\(self\) -> bool' "$FILE" >/dev/null

echo "phase39e-compile-repair-check=ok"
