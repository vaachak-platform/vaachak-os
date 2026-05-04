#!/usr/bin/env bash
set -euo pipefail

GATE="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_file_api_integration_gate.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_file_api_integration_gate_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$GATE"
test -f "$ACCEPT"
grep -q 'PHASE_39G_RUNTIME_FILE_API_INTEGRATION_GATE_MARKER' "$GATE"
grep -q 'Phase39gRuntimeFileApiProbe' "$GATE"
grep -q 'phase39g_execute_runtime_file_api_gate' "$GATE"
grep -q 'PHASE_39G_RUNTIME_FILE_API_INTEGRATION_GATE_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39g_accept_integration_gate_report' "$ACCEPT"
grep -q 'pub mod state_io_runtime_file_api_integration_gate;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_runtime_file_api_integration_gate_acceptance;' "$RUNTIME_MOD"

echo "phase39g-check=ok"
