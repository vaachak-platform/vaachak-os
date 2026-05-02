#!/usr/bin/env bash
set -euo pipefail

CLOSEOUT="target-xteink-x4/src/vaachak_x4/runtime/state_io_device_regression_write_lane_closeout.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_device_regression_write_lane_closeout_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$CLOSEOUT"
test -f "$ACCEPT"
grep -q 'PHASE_40A_DEVICE_REGRESSION_WRITE_LANE_CLOSEOUT_MARKER' "$CLOSEOUT"
grep -q 'PHASE_40A_ADDS_WRITE_ABSTRACTION: bool = false' "$CLOSEOUT"
grep -q 'PHASE_40A_TOUCHES_ACTIVE_READER_PATH: bool = false' "$CLOSEOUT"
grep -q 'Phase40aCloseoutReport' "$CLOSEOUT"
grep -q 'PHASE_40A_DEVICE_REGRESSION_WRITE_LANE_CLOSEOUT_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase40a_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_device_regression_write_lane_closeout;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_device_regression_write_lane_closeout_acceptance;' "$RUNTIME_MOD"

echo "phase40a-check=ok"
