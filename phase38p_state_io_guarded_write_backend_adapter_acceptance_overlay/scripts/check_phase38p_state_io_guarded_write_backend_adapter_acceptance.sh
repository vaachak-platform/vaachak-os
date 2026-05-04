#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_adapter_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38P_GUARDED_WRITE_BACKEND_ADAPTER_ACCEPTANCE_MARKER' "$FILE"
grep -q 'Phase38pAcceptanceReport' "$FILE"
grep -q 'PHASE_38P_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'PHASE_38P_BACKEND_BOUND: bool = false' "$FILE"
grep -q 'phase38p_live_mutation_still_disabled' "$FILE"
grep -q 'pub mod state_io_guarded_write_backend_adapter_acceptance;' "$RUNTIME_MOD"

echo "phase38p-check=ok"
