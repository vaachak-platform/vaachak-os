#!/usr/bin/env bash
set -euo pipefail

BASELINE="target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_regression_baseline.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_regression_baseline_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$BASELINE"
test -f "$ACCEPT"
grep -q 'PHASE_40B_READER_UX_REGRESSION_BASELINE_MARKER' "$BASELINE"
grep -q 'PHASE_40B_ADDS_FEATURES: bool = false' "$BASELINE"
grep -q 'PHASE_40B_TOUCHES_ACTIVE_READER_PATH: bool = false' "$BASELINE"
grep -q 'Phase40bReaderUxBaselineReport' "$BASELINE"
grep -q 'PHASE_40B_READER_UX_REGRESSION_BASELINE_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase40b_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_reader_ux_regression_baseline;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_reader_ux_regression_baseline_acceptance;' "$RUNTIME_MOD"

echo "phase40b-check=ok"
