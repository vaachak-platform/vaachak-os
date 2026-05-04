#!/usr/bin/env bash
set -euo pipefail

DRY="target-xteink-x4/src/vaachak_x4/runtime/state_io_review_delete_later_removal_dry_run.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_review_delete_later_removal_dry_run_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$DRY"
test -f "$ACCEPT"
grep -q 'PHASE_39N_REVIEW_DELETE_LATER_REMOVAL_DRY_RUN_MARKER' "$DRY"
grep -q 'PHASE_39N_DELETES_CODE_NOW: bool = false' "$DRY"
grep -q 'PHASE_39N_DRY_RUN_ONLY: bool = true' "$DRY"
grep -q 'Phase39nRemovalCandidate' "$DRY"
grep -q 'PHASE_39N_REVIEW_DELETE_LATER_REMOVAL_DRY_RUN_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39n_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_review_delete_later_removal_dry_run;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_review_delete_later_removal_dry_run_acceptance;' "$RUNTIME_MOD"

echo "phase39n-check=ok"
