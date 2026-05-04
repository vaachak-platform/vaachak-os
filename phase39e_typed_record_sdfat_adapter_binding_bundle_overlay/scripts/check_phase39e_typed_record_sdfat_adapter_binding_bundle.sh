#!/usr/bin/env bash
set -euo pipefail

ADAPTER="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_sdfat_adapter.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_sdfat_adapter_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$ADAPTER"
test -f "$ACCEPT"
grep -q 'PHASE_39E_TYPED_RECORD_SDFAT_ADAPTER_BINDING_MARKER' "$ADAPTER"
grep -q 'Phase39eSdFatLikeBackend' "$ADAPTER"
grep -q 'Phase39eTypedRecordSdFatAdapter' "$ADAPTER"
grep -q 'Phase39eRecordingSdFatBackend' "$ADAPTER"
grep -q 'AtomicTempThenReplace' "$ADAPTER"
grep -q 'PHASE_39E_TYPED_RECORD_SDFAT_ADAPTER_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39e_accept_sdfat_adapter_report' "$ACCEPT"
grep -q 'pub mod state_io_typed_record_sdfat_adapter;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_typed_record_sdfat_adapter_acceptance;' "$RUNTIME_MOD"

echo "phase39e-check=ok"
