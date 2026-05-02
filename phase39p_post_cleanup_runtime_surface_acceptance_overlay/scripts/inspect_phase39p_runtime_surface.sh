#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
ARCHIVE_A="$ROOT/docs/archive/phase38-39-scaffolding/runtime"
ARCHIVE_B="$ROOT/docs/archive/phase38-39-scaffolding/review-delete-later-runtime"
OUT="${OUT:-/tmp/phase39p-runtime-surface-inspection.txt}"

"$ROOT/phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/guard_phase39p_accepted_write_path.sh" >/dev/null

archived_mods=(
  state_io_guarded_persistent_backend_stub
  state_io_guarded_read_before_write_stub
  state_io_guarded_write_backend_adapter_acceptance
  state_io_guarded_write_backend_adapter_shape
  state_io_guarded_write_backend_binding
  state_io_guarded_write_backend_dry_run_executor
  state_io_guarded_write_backend_implementation_seam
  state_io_guarded_write_dry_run_acceptance
  state_io_pre_behavior_write_enablement_consolidation
  state_io_shadow_write_acceptance
  state_io_shadow_write_plan
  state_io_write_design_consolidation
  state_io_write_lane_entry_contract
  state_io_write_lane_handoff_consolidation
  state_io_write_plan_design
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

export_violations=0
runtime_file_violations=0
archive_present=0

{
  echo "# Phase 39P Runtime Surface Inspection"
  echo
  echo "## active kept modules"
  for file in \
    "$RUNTIME_DIR/state_io_active_reader_save_callsite_wiring.rs" \
    "$RUNTIME_DIR/state_io_runtime_state_write_verification.rs" \
    "$RUNTIME_DIR/state_io_write_lane_cleanup_acceptance_freeze.rs" \
    "$RUNTIME_DIR/state_io_post_freeze_scaffolding_cleanup_plan.rs" \
    "$RUNTIME_DIR/state_io_safe_scaffolding_archive_patch.rs" \
    "$RUNTIME_DIR/state_io_review_delete_later_removal_dry_run.rs" \
    "$RUNTIME_DIR/state_io_guarded_review_delete_later_removal_patch.rs" \
    "$RUNTIME_DIR/state_io_post_cleanup_runtime_surface_acceptance.rs"
  do
    if [ -f "$file" ]; then
      echo "present $(basename "$file")"
    else
      echo "missing $(basename "$file")"
    fi
  done

  echo
  echo "## archived module export/runtime-file violations"
  for mod in "${archived_mods[@]}"; do
    file="$mod.rs"
    exported="no"
    runtime_file="no"
    archived="no"

    if rg -n "pub mod ${mod};" "$RUNTIME_MOD" >/dev/null 2>&1; then
      exported="yes"
      export_violations=$((export_violations + 1))
    fi

    if [ -f "$RUNTIME_DIR/$file" ]; then
      runtime_file="yes"
      runtime_file_violations=$((runtime_file_violations + 1))
    fi

    if [ -f "$ARCHIVE_A/$file" ] || [ -f "$ARCHIVE_B/$file" ]; then
      archived="yes"
      archive_present=$((archive_present + 1))
    fi

    echo "$file exported=$exported runtime_file=$runtime_file archived=$archived"
  done

  echo
  echo "export_violations=$export_violations"
  echo "runtime_file_violations=$runtime_file_violations"
  echo "archived_present=$archive_present/${#archived_mods[@]}"
  echo "marker=phase39p=x4-post-cleanup-runtime-surface-acceptance-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$export_violations" -ne 0 ] || [ "$runtime_file_violations" -ne 0 ]; then
  exit 4
fi
