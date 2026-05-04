#!/usr/bin/env bash
set -euo pipefail
PATCH="target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_patch.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_patch_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"
test -f "$PATCH"; test -f "$ACCEPT"
grep -q 'PHASE_40G_HOME_LIBRARY_TITLE_PATCH_MARKER' "$PATCH"
grep -q 'PHASE_40G_CHANGES_HOME_TITLE_LAYOUT: bool = true' "$PATCH"
grep -q 'PHASE_40G_CHANGES_LIBRARY_TITLE_RESOLUTION: bool = true' "$PATCH"
grep -q 'PHASE_40G_CHANGES_INPUT_MAPPING: bool = false' "$PATCH"
grep -q 'PHASE_40G_HOME_LIBRARY_TITLE_PATCH_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'pub mod state_io_home_library_title_patch;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_home_library_title_patch_acceptance;' "$RUNTIME_MOD"
echo "phase40g-check=ok"
