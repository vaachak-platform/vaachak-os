#!/usr/bin/env bash
set -euo pipefail

WIRE="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_state_runtime_callsite_wiring.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_state_runtime_callsite_wiring_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$WIRE"
test -f "$ACCEPT"
grep -q 'PHASE_39H_TYPED_STATE_RUNTIME_CALLSITE_WIRING_MARKER' "$WIRE"
grep -q 'phase39h_write_progress_state' "$WIRE"
grep -q 'phase39h_write_theme_state' "$WIRE"
grep -q 'phase39h_write_metadata_state' "$WIRE"
grep -q 'phase39h_write_bookmark_state' "$WIRE"
grep -q 'phase39h_append_bookmark_index' "$WIRE"
grep -q 'phase39h_replace_bookmark_index' "$WIRE"
grep -q 'phase39h_compact_bookmark_index' "$WIRE"
grep -q 'phase39h_write_all_typed_state' "$WIRE"
grep -q 'PHASE_39H_TYPED_STATE_RUNTIME_CALLSITE_WIRING_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39h_accept_typed_state_write_report' "$ACCEPT"
grep -q 'pub mod state_io_typed_state_runtime_callsite_wiring;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_typed_state_runtime_callsite_wiring_acceptance;' "$RUNTIME_MOD"

echo "phase39h-check=ok"
