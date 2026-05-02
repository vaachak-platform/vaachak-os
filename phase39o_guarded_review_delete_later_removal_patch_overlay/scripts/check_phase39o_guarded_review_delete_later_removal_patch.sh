#!/usr/bin/env bash
set -euo pipefail

PATCH="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_review_delete_later_removal_patch.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_review_delete_later_removal_patch_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PATCH"
test -f "$ACCEPT"
grep -q 'PHASE_39O_GUARDED_REVIEW_DELETE_LATER_REMOVAL_PATCH_MARKER' "$PATCH"
grep -q 'PHASE_39O_EXPECTED_CANDIDATE_COUNT: usize = 14' "$PATCH"
grep -q 'PHASE_39O_DELETES_ACTIVE_PATH: bool = false' "$PATCH"
grep -q 'PHASE_39O_MOVES_CANDIDATES_TO_ARCHIVE: bool = true' "$PATCH"
grep -q 'Phase39oRemovalCandidate' "$PATCH"
grep -q 'PHASE_39O_GUARDED_REVIEW_DELETE_LATER_REMOVAL_PATCH_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39o_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_guarded_review_delete_later_removal_patch;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_guarded_review_delete_later_removal_patch_acceptance;' "$RUNTIME_MOD"

echo "phase39o-check=ok"
