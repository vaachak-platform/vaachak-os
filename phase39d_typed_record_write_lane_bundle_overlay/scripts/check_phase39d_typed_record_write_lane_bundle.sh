#!/usr/bin/env bash
set -euo pipefail

LANE="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_write_lane.rs"
ACCEPT="target-xteink-x4/src/vaachak_x4/runtime/state_io_typed_record_write_lane_acceptance.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$LANE"
test -f "$ACCEPT"
grep -q 'PHASE_39D_TYPED_RECORD_WRITE_LANE_BUNDLE_MARKER' "$LANE"
grep -q 'Phase39dTypedRecordKind::Progress' "$LANE"
grep -q 'Phase39dTypedRecordKind::Theme' "$LANE"
grep -q 'Phase39dTypedRecordKind::Metadata' "$LANE"
grep -q 'Phase39dTypedRecordKind::Bookmark' "$LANE"
grep -q 'Phase39dTypedRecordKind::BookmarkIndex' "$LANE"
grep -q 'Phase39dCallbackTypedWriteBackend' "$LANE"
grep -q 'Phase39dRecordingTypedWriteBackend' "$LANE"
grep -q 'PHASE_39D_TYPED_RECORD_WRITE_LANE_ACCEPTANCE_MARKER' "$ACCEPT"
grep -q 'phase39d_accept_typed_record_write_report' "$ACCEPT"
grep -q 'pub mod state_io_typed_record_write_lane;' "$RUNTIME_MOD"
grep -q 'pub mod state_io_typed_record_write_lane_acceptance;' "$RUNTIME_MOD"

echo "phase39d-check=ok"
