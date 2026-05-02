#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
ARCHIVE_DIR="$ROOT/docs/archive/phase38-39-scaffolding/review-delete-later-runtime"
OUT="${OUT:-/tmp/phase39o-guarded-review-delete-later-removal-acceptance.txt}"

"$ROOT/phase39o_guarded_review_delete_later_removal_patch_overlay/scripts/guard_phase39o_accepted_write_path.sh" >/dev/null

candidate_files=(
  state_io_progress_write_backend_binding.rs
  state_io_progress_write_callback_backend.rs
  state_io_progress_write_lane_acceptance.rs
  state_io_progress_write_lane.rs
  state_io_runtime_file_api_integration_gate_acceptance.rs
  state_io_runtime_file_api_integration_gate.rs
  state_io_runtime_owned_sdfat_writer_acceptance.rs
  state_io_runtime_owned_sdfat_writer.rs
  state_io_typed_record_sdfat_adapter_acceptance.rs
  state_io_typed_record_sdfat_adapter.rs
  state_io_typed_record_write_lane_acceptance.rs
  state_io_typed_record_write_lane.rs
  state_io_typed_state_runtime_callsite_wiring_acceptance.rs
  state_io_typed_state_runtime_callsite_wiring.rs
)

archived_count=0
runtime_remaining=0
export_remaining=0

for file in "${candidate_files[@]}"; do
  mod="${file%.rs}"
  if [ -f "$ARCHIVE_DIR/$file" ]; then
    archived_count=$((archived_count + 1))
  fi
  if [ -f "$RUNTIME_DIR/$file" ]; then
    runtime_remaining=$((runtime_remaining + 1))
  fi
  if rg -n "pub mod ${mod};" "$RUNTIME_MOD" >/dev/null 2>&1; then
    export_remaining=$((export_remaining + 1))
  fi
done

status="ACCEPTED"
reason="RemovalPatchAccepted"

if [ "$archived_count" -ne "${#candidate_files[@]}" ]; then
  status="REJECTED"
  reason="ArchiveCountMismatch"
elif [ "$runtime_remaining" -ne 0 ]; then
  status="REJECTED"
  reason="RuntimeCandidatesRemain"
elif [ "$export_remaining" -ne 0 ]; then
  status="REJECTED"
  reason="CandidateExportsRemain"
fi

{
  echo "# Phase 39O Guarded Review-Delete-Later Removal Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "archived_count=$archived_count/${#candidate_files[@]}"
  echo "runtime_remaining=$runtime_remaining"
  echo "export_remaining=$export_remaining"
  echo "active_path_guard=ACCEPTED"
  echo "marker=phase39o=x4-guarded-review-delete-later-removal-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 6
fi
