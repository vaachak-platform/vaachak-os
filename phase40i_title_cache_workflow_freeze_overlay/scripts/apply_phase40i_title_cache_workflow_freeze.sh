#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40i_title_cache_workflow_freeze_overlay"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"

mkdir -p "$RUNTIME_DIR"

cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze.rs" \
  "$RUNTIME_DIR/state_io_title_cache_workflow_freeze.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze_acceptance.rs" \
  "$RUNTIME_DIR/state_io_title_cache_workflow_freeze_acceptance.rs"

for export in \
  "pub mod state_io_title_cache_workflow_freeze;" \
  "pub mod state_io_title_cache_workflow_freeze_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40i_title_cache_workflow_freeze.sh"
"$OVERLAY/scripts/guard_phase40i_title_cache_workflow_source.sh"

echo "phase40i=x4-title-cache-workflow-freeze-ok"
echo "phase40i-acceptance=x4-title-cache-workflow-freeze-report-ok"
