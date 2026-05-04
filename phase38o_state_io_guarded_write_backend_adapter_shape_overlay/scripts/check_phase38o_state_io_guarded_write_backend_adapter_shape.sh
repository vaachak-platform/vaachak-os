#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_adapter_shape.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$FILE"
grep -q 'PHASE_38O_GUARDED_WRITE_BACKEND_ADAPTER_SHAPE_MARKER' "$FILE"
grep -q 'Phase38oGuardedBackendAdapterShape' "$FILE"
grep -q 'Phase38oDefaultUnboundAdapter' "$FILE"
grep -q 'PHASE_38O_LIVE_MUTATION_ENABLED: bool = false' "$FILE"
grep -q 'PHASE_38O_BACKEND_BOUND: bool = false' "$FILE"
grep -q 'pub mod state_io_guarded_write_backend_adapter_shape;' "$RUNTIME_MOD"

echo "phase38o-check=ok"
