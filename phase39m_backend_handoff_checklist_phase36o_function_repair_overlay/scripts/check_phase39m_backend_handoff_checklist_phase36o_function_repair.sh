#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_backend_handoff_checklist.rs"
ARCHIVE="docs/archive/phase38-39-scaffolding/runtime/state_io_shadow_write_acceptance.rs"

test -f "$FILE"
test -f "$ARCHIVE"

if rg -n 'use super::state_io_shadow_write_acceptance' "$FILE"; then
  echo "backend handoff checklist still imports archived module" >&2
  exit 1
fi

rg -n 'Phase39mArchivedShadowWriteAcceptanceReport' "$FILE" >/dev/null
rg -n 'STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT' "$FILE" >/dev/null
rg -n 'pub const fn phase36o_marker\(\)' "$FILE" >/dev/null
rg -n 'pub const fn phase36o_acceptance_report\(\)' "$FILE" >/dev/null
rg -n 'backend_bound' "$FILE" >/dev/null
rg -n 'storage_behavior_moved' "$FILE" >/dev/null
rg -n 'display_behavior_moved' "$FILE" >/dev/null
rg -n 'input_behavior_moved' "$FILE" >/dev/null
rg -n 'power_behavior_moved' "$FILE" >/dev/null

# Accepted active path must still be intact.
test -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
rg -n 'typed_state_wiring::write_app_subdir' vendor/pulp-os/src/apps/reader/mod.rs >/dev/null
rg -n 'typed_state_wiring::ensure_state_dir' vendor/pulp-os/src/apps/reader/mod.rs >/dev/null

echo "phase39m-phase36o-function-repair-check=ok"
