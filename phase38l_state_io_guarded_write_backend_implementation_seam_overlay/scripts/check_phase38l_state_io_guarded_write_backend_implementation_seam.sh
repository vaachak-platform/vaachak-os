#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_implementation_seam.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38L_GUARDED_WRITE_BACKEND_IMPLEMENTATION_SEAM_MARKER' "$FILE"
grep -q 'Phase38lGuardedBackendSeam' "$FILE"
grep -q 'Phase38lDefaultGuardedBackendSeam' "$FILE"
grep -q 'PHASE_38L_DEFAULT_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'pub mod state_io_guarded_write_backend_implementation_seam;' "$RUNTIME_MOD"

echo "phase38l-check=ok"
