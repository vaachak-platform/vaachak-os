#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
OUT="${OUT:-/tmp/phase39o-external-candidate-refs.txt}"

candidates=(
  state_io_progress_write_backend_binding
  state_io_progress_write_callback_backend
  state_io_progress_write_lane_acceptance
  state_io_progress_write_lane
  state_io_runtime_file_api_integration_gate_acceptance
  state_io_runtime_file_api_integration_gate
  state_io_runtime_owned_sdfat_writer_acceptance
  state_io_runtime_owned_sdfat_writer
  state_io_typed_record_sdfat_adapter_acceptance
  state_io_typed_record_sdfat_adapter
  state_io_typed_record_write_lane_acceptance
  state_io_typed_record_write_lane
  state_io_typed_state_runtime_callsite_wiring_acceptance
  state_io_typed_state_runtime_callsite_wiring
)

allowed_metadata_re='state_io_(review_delete_later_removal_dry_run|guarded_review_delete_later_removal_patch|post_freeze_scaffolding_cleanup_plan|write_lane_cleanup_acceptance_freeze|safe_scaffolding_archive_patch|runtime_state_write_verification|active_reader_save_callsite_wiring)'

external_refs=0

{
  echo "# Phase 39O External Candidate Reference Check"
  echo
  for mod in "${candidates[@]}"; do
    echo "## $mod"

    refs="$(rg -n "$mod" "$RUNTIME_DIR"/*.rs 2>/dev/null \
      | grep -v "$RUNTIME_DIR/$mod.rs" \
      | grep -v "$RUNTIME_DIR/runtime.rs" \
      | grep -Ev "$allowed_metadata_re" || true)"

    if [ -n "$refs" ]; then
      external_refs=$((external_refs + 1))
      echo "$refs"
    else
      echo "none"
    fi
    echo
  done
  echo "external_ref_groups=$external_refs"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$external_refs" -ne 0 ]; then
  echo "External references found. Do not remove candidates until these are resolved." >&2
  exit 4
fi
