#!/usr/bin/env bash
set -euo pipefail

FREEZE="target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze.rs"
REPORT="target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze_report.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FREEZE"
test -f "$REPORT"
grep -q 'PHASE_39K_WRITE_LANE_CLEANUP_ACCEPTANCE_FREEZE_MARKER' "$FREEZE"
grep -q 'PHASE_39K_DELETES_CODE_NOW: bool = false' "$FREEZE"
grep -q 'PHASE_39K_ADDS_NEW_WRITE_ABSTRACTION: bool = false' "$FREEZE"
grep -q 'Phase39kWriteLaneFreezeReport' "$FREEZE"
grep -q 'PHASE_39K_WRITE_LANE_CLEANUP_ACCEPTANCE_FREEZE_REPORT_MARKER' "$REPORT"
grep -q 'phase39k_acceptance_report' "$REPORT"
grep -q 'pub mod state_io_write_lane_cleanup_acceptance_freeze;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_write_lane_cleanup_acceptance_freeze_report;' "$RUNTIME_MOD"

echo "phase39k-check=ok"
