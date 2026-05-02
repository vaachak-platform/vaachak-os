#!/usr/bin/env bash
set -euo pipefail

ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_post_cleanup_runtime_surface_acceptance.rs"
REPORT="target-xteink-x4/src/vaachak_x4/runtime/state_io_post_cleanup_runtime_surface_acceptance_report.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$ACCEPT"
test -f "$REPORT"
grep -q 'PHASE_39P_POST_CLEANUP_RUNTIME_SURFACE_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'PHASE_39P_ADDS_WRITE_ABSTRACTION: bool = false' "$ACCEPT"
grep -q 'PHASE_39P_DELETES_CODE_NOW: bool = false' "$ACCEPT"
grep -q 'Phase39pRuntimeSurfaceReport' "$ACCEPT"
grep -q 'PHASE_39P_POST_CLEANUP_RUNTIME_SURFACE_ACCEPTANCE_REPORT_MARKER' "$REPORT"
grep -q 'phase39p_acceptance_report' "$REPORT"
grep -q 'pub mod state_io_post_cleanup_runtime_surface_acceptance;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_post_cleanup_runtime_surface_acceptance_report;' "$RUNTIME_MOD"

echo "phase39p-check=ok"
