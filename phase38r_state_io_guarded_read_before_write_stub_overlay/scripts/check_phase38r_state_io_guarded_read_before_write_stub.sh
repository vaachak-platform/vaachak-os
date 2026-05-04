#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_read_before_write_stub.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38R_GUARDED_READ_BEFORE_WRITE_STUB_MARKER' "$FILE"
grep -q 'Phase38rGuardedReadBeforeWriteStub' "$FILE"
grep -q 'Phase38rDefaultReadBeforeWriteStub' "$FILE"
grep -q 'PHASE_38R_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'PHASE_38R_PERSISTENT_BACKEND_BOUND: bool = false' "$FILE"
grep -q 'PHASE_38R_PREWRITE_READ_AVAILABLE: bool = false' "$FILE"
grep -q 'phase38r_live_mutation_still_disabled' "$FILE"
grep -q 'pub mod state_io_guarded_read_before_write_stub;' "$RUNTIME_MOD"

echo "phase38r-check=ok"
