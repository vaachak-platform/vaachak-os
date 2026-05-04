#!/usr/bin/env bash
set -euo pipefail

WRITER="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_owned_sdfat_writer.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_owned_sdfat_writer_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$WRITER"
test -f "$ACCEPT"
grep -q 'PHASE_39F_RUNTIME_OWNED_SDFAT_TYPED_RECORD_WRITER_BINDING_MARKER' "$WRITER"
grep -q 'Phase39fRuntimeOwnedFileOps' "$WRITER"
grep -q 'Phase39fRuntimeOwnedSdFatBackend' "$WRITER"
grep -q 'Phase39fRecordingRuntimeFileOps' "$WRITER"
grep -q 'phase39f_execute_with_runtime_owned_file_ops' "$WRITER"
grep -q 'PHASE_39F_RUNTIME_OWNED_SDFAT_WRITER_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39f_accept_runtime_writer_report' "$ACCEPT"
grep -q 'pub mod state_io_runtime_owned_sdfat_writer;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_runtime_owned_sdfat_writer_acceptance;' "$RUNTIME_MOD"

echo "phase39f-check=ok"
