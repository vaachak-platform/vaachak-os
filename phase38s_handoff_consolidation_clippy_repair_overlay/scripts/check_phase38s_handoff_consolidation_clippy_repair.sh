#!/usr/bin/env bash
set -euo pipefail

PHASE38S="target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"

test -f "$PHASE38S"

if rg -n 'ProgressOnly|ThemeOnly|MetadataOnly|BookmarkOnly|BookmarkIndexOnly' "$PHASE38S"; then
  echo "old *Only enum variant remains in Phase 38S" >&2
  exit 1
fi

rg -n 'Phase38sPhase39FirstWriteScope::Progress' "$PHASE38S" >/dev/null
rg -n 'Phase38sPhase39FirstWriteScope::BookmarkIndex' "$PHASE38S" >/dev/null

echo "phase38s-clippy-repair-check=ok"
