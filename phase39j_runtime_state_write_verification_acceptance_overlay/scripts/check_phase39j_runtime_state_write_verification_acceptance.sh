#!/usr/bin/env bash
set -euo pipefail

VERIFY="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$VERIFY"
test -f "$ACCEPT"
grep -q 'PHASE_39J_RUNTIME_STATE_WRITE_VERIFICATION_MARKER' "$VERIFY"
grep -q 'Phase39jVerifiedStateRecord' "$VERIFY"
grep -q 'Phase39jRuntimeStateWriteVerificationReport' "$VERIFY"
grep -q 'PHASE_39J_RUNTIME_STATE_WRITE_VERIFICATION_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39j_accept_runtime_state_write_verification' "$ACCEPT"
grep -q 'pub mod state_io_runtime_state_write_verification;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_runtime_state_write_verification_acceptance;' "$RUNTIME_MOD"

echo "phase39j-check=ok"
