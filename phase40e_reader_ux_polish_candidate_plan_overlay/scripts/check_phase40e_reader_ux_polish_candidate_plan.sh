#!/usr/bin/env bash
set -euo pipefail

PLAN="target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_polish_candidate_plan.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_polish_candidate_plan_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PLAN"
test -f "$ACCEPT"
grep -q 'PHASE_40E_READER_UX_POLISH_CANDIDATE_PLAN_MARKER' "$PLAN"
grep -q 'PHASE_40E_PLAN_ONLY: bool = true' "$PLAN"
grep -q 'PHASE_40E_CHANGES_UX_NOW: bool = false' "$PLAN"
grep -q 'PHASE_40E_CHANGES_FOOTER_LABELS_NOW: bool = false' "$PLAN"
grep -q 'Phase40eReaderUxPolishPlanReport' "$PLAN"
grep -q 'PHASE_40E_READER_UX_POLISH_CANDIDATE_PLAN_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase40e_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_reader_ux_polish_candidate_plan;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_reader_ux_polish_candidate_plan_acceptance;' "$RUNTIME_MOD"

echo "phase40e-check=ok"
