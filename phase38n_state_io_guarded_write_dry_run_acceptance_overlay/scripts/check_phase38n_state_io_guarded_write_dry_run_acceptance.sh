#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_dry_run_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38N_GUARDED_WRITE_DRY_RUN_ACCEPTANCE_MARKER' "$FILE"
grep -q 'Phase38nAcceptanceReport' "$FILE"
grep -q 'PHASE_38N_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'phase38n_live_mutation_still_disabled' "$FILE"
grep -q 'pub mod state_io_guarded_write_dry_run_acceptance;' "$RUNTIME_MOD"

echo "phase38n-check=ok"
