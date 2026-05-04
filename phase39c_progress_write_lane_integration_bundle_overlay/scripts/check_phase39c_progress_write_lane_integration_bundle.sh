#!/usr/bin/env bash
set -euo pipefail

LANE="target-xteink-x4/src/vaachak_x4/runtime/state_io_progress_write_lane.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_progress_write_lane_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$LANE"
test -f "$ACCEPT"
grep -q 'PHASE_39C_PROGRESS_WRITE_LANE_INTEGRATION_MARKER' "$LANE"
grep -q 'Phase39cRecordingProgressBackend' "$LANE"
grep -q 'phase39c_execute_progress_write_lane' "$LANE"
grep -q 'PHASE_39C_PROGRESS_WRITE_LANE_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39c_accept_progress_write_report' "$ACCEPT"
grep -q 'pub mod state_io_progress_write_lane;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_progress_write_lane_acceptance;' "$RUNTIME_MOD"

echo "phase39c-check=ok"
