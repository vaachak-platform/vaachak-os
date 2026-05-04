#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_persistent_backend_stub.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38Q_GUARDED_PERSISTENT_BACKEND_STUB_MARKER' "$FILE"
grep -q 'Phase38qGuardedPersistentBackendStub' "$FILE"
grep -q 'Phase38qDefaultPersistentBackendStub' "$FILE"
grep -q 'PHASE_38Q_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'PHASE_38Q_PERSISTENT_BACKEND_BOUND: bool = false' "$FILE"
grep -q 'phase38q_live_mutation_still_disabled' "$FILE"
grep -q 'pub mod state_io_guarded_persistent_backend_stub;' "$RUNTIME_MOD"

echo "phase38q-check=ok"
