#!/usr/bin/env bash
set -euo pipefail

PLAN="target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_baseline_fix_plan.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_baseline_fix_plan_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PLAN"
test -f "$ACCEPT"
grep -q 'PHASE_40C_FOOTER_BUTTON_LABEL_BASELINE_FIX_PLAN_MARKER' "$PLAN"
grep -q 'PHASE_40C_PLAN_ONLY: bool = true' "$PLAN"
grep -q 'PHASE_40C_CHANGES_RENDERING_NOW: bool = false' "$PLAN"
grep -q 'Phase40cFooterButtonPlanReport' "$PLAN"
grep -q 'PHASE_40C_FOOTER_BUTTON_LABEL_BASELINE_FIX_PLAN_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase40c_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_footer_button_label_baseline_fix_plan;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_footer_button_label_baseline_fix_plan_acceptance;' "$RUNTIME_MOD"

echo "phase40c-check=ok"
