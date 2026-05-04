#!/usr/bin/env bash
set -euo pipefail

PLAN="target-xteink-x4/src/vaachak_x4/runtime/state_io_post_freeze_scaffolding_cleanup_plan.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_post_freeze_scaffolding_cleanup_plan_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PLAN"
test -f "$ACCEPT"
grep -q 'PHASE_39L_POST_FREEZE_SCAFFOLDING_CLEANUP_PLAN_MARKER' "$PLAN"
grep -q 'PHASE_39L_DELETES_CODE_NOW: bool = false' "$PLAN"
grep -q 'PHASE_39L_REVIEW_ONLY: bool = true' "$PLAN"
grep -q 'Phase39lCleanupPlanEntry' "$PLAN"
grep -q 'PHASE_39L_POST_FREEZE_SCAFFOLDING_CLEANUP_PLAN_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39l_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_post_freeze_scaffolding_cleanup_plan;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_post_freeze_scaffolding_cleanup_plan_acceptance;' "$RUNTIME_MOD"

echo "phase39l-check=ok"
