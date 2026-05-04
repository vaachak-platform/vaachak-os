#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_dry_run_executor.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38M_GUARDED_WRITE_BACKEND_DRY_RUN_EXECUTOR_MARKER' "$FILE"
grep -q 'Phase38mGuardedDryRunExecutor' "$FILE"
grep -q 'Phase38mDefaultDryRunExecutor' "$FILE"
grep -q 'PHASE_38M_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'pub mod state_io_guarded_write_backend_dry_run_executor;' "$RUNTIME_MOD"

echo "phase38m-check=ok"
