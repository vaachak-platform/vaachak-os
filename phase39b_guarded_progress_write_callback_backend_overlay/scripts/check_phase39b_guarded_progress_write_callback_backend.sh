#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_progress_write_callback_backend.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_39B_GUARDED_PROGRESS_WRITE_CALLBACK_BACKEND_MARKER' "$FILE"
grep -q 'Phase39bProgressWriteCallback' "$FILE"
grep -q 'Phase39bCallbackProgressWriteBackend' "$FILE"
grep -q 'phase39b_execute_callback_progress_write' "$FILE"
grep -q 'PHASE_39B_PROGRESS_WRITE_ONLY: bool = true' "$FILE"
grep -q 'pub mod state_io_progress_write_callback_backend;' "$RUNTIME_MOD"

echo "phase39b-check=ok"
