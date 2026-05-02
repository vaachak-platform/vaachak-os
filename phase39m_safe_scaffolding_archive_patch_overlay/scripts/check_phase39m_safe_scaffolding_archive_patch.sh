#!/usr/bin/env bash
set -euo pipefail

PATCH="target-xteink-x4/src/vaachak_x4/runtime/state_io_safe_scaffolding_archive_patch.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_safe_scaffolding_archive_patch_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PATCH"
test -f "$ACCEPT"
grep -q 'PHASE_39M_SAFE_SCAFFOLDING_ARCHIVE_PATCH_MARKER' "$PATCH"
grep -q 'PHASE_39M_DELETES_ACTIVE_PATH: bool = false' "$PATCH"
grep -q 'PHASE_39M_ARCHIVES_REVIEW_DELETE_LATER: bool = false' "$PATCH"
grep -q 'Phase39mArchivePlanEntry' "$PATCH"
grep -q 'PHASE_39M_SAFE_SCAFFOLDING_ARCHIVE_PATCH_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39m_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_safe_scaffolding_archive_patch;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_safe_scaffolding_archive_patch_acceptance;' "$RUNTIME_MOD"

echo "phase39m-check=ok"
