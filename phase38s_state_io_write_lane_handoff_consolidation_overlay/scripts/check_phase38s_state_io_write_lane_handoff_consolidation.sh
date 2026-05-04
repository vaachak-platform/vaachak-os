#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38S_WRITE_LANE_HANDOFF_CONSOLIDATION_MARKER' "$FILE"
grep -q 'PHASE_38S_IS_FINAL_PHASE_38: bool = true' "$FILE"
grep -q 'PHASE_38S_PHASE39_WRITE_LANE_ALLOWED_NEXT: bool = true' "$FILE"
grep -q 'PHASE_38S_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'Phase38sWriteLaneHandoffReport' "$FILE"
grep -q 'Phase 39A — Guarded Progress State Write Backend Binding' "$FILE"
grep -q 'pub mod state_io_write_lane_handoff_consolidation;' "$RUNTIME_MOD"

echo "phase38s-check=ok"
