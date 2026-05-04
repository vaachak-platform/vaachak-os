#!/usr/bin/env bash
set -euo pipefail

PHASE38S="target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"

test -f "$PHASE38S"

if rg -n 'phase38c_live_writes_enabled' "$PHASE38S"; then
  echo "old missing symbol still present" >&2
  exit 1
fi

rg -n 'phase38c_writes_enabled' "$PHASE38S" >/dev/null

if [ -f vendor/pulp-os/kernel/src/kernel/dir_cache.rs ]; then
  if rg -n '^fn phase38i_is_epub_or_epu_name' vendor/pulp-os/kernel/src/kernel/dir_cache.rs >/dev/null; then
    echo "phase38i helper still lacks #[allow(dead_code)]" >&2
    exit 1
  fi
fi

echo "phase38s-repair-check=ok"
