#!/usr/bin/env bash
set -euo pipefail

PATCH="target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_rendering_patch.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_rendering_patch_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$PATCH"
test -f "$ACCEPT"
grep -q 'PHASE_40D_FOOTER_BUTTON_LABEL_RENDERING_PATCH_MARKER' "$PATCH"
grep -q 'PHASE_40D_CHANGES_RENDERING_LABELS: bool = true' "$PATCH"
grep -q 'PHASE_40D_CHANGES_INPUT_MAPPING: bool = false' "$PATCH"
grep -q 'PHASE_40D_TOUCHES_WRITE_LANE: bool = false' "$PATCH"
grep -q 'Back", "Select", "Open", "Stay' "$PATCH"
grep -q 'PHASE_40D_FOOTER_BUTTON_LABEL_RENDERING_PATCH_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase40d_acceptance_report' "$ACCEPT"
grep -q 'pub mod state_io_footer_button_label_rendering_patch;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_footer_button_label_rendering_patch_acceptance;' "$RUNTIME_MOD"

echo "phase40d-check=ok"
