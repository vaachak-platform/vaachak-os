#!/usr/bin/env bash
set -euo pipefail

PATCH="target-xteink-x4/src/vaachak_x4/runtime/state_io_library_title_layout_polish_patch.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_library_title_layout_polish_patch_acceptance.rs"
HELPER="target-xteink-x4/src/vaachak_x4/ui/library_title_layout.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PATCH"
test -f "$ACCEPT"
test -f "$HELPER"
grep -q 'PHASE_40F_LIBRARY_TITLE_LAYOUT_POLISH_PATCH_MARKER' "$PATCH"
grep -q 'PHASE_40F_CHANGES_TITLE_SOURCE: bool = false' "$PATCH"
grep -q 'PHASE_40F_CHANGES_FOOTER_LABELS: bool = false' "$PATCH"
grep -q 'PHASE_40F_CHANGES_INPUT_MAPPING: bool = false' "$PATCH"
grep -q 'Phase40fLibraryTitleLayoutPatchReport' "$PATCH"
grep -q 'PHASE_40F_LIBRARY_TITLE_LAYOUT_POLISH_PATCH_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase40f_acceptance_report' "$ACCEPT"
grep -q 'PHASE_40F_LIBRARY_TITLE_LAYOUT_HELPER_MARKER' "$HELPER"
grep -q 'phase40f_polish_library_title' "$HELPER"
grep -q 'pub mod state_io_library_title_layout_polish_patch;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_library_title_layout_polish_patch_acceptance;' "$RUNTIME_MOD"

echo "phase40f-check=ok"
