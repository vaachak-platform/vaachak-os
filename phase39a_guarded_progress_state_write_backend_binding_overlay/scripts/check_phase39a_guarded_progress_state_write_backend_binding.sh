#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_progress_write_backend_binding.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_39A_GUARDED_PROGRESS_WRITE_BACKEND_BINDING_MARKER' "$FILE"
grep -q 'PHASE_39A_WRITE_LANE_STARTED: bool = true' "$FILE"
grep -q 'PHASE_39A_PROGRESS_WRITE_ONLY: bool = true' "$FILE"
grep -q 'Phase39aProgressWriteBackend' "$FILE"
grep -q 'phase39a_execute_progress_write' "$FILE"
grep -q 'pub mod state_io_progress_write_backend_binding;' "$RUNTIME_MOD"

echo "phase39a-check=ok"
