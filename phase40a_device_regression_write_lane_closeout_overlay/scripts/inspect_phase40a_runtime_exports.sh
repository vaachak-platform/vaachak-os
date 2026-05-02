#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
OUT="${OUT:-/tmp/phase40a-runtime-export-inventory.txt}"

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

violations=0

{
  echo "# Phase 40A Runtime Export Inventory"
  echo
  echo "## current state/write exports"
  rg -n 'pub mod state_io_' "$RUNTIME_MOD" || true
  echo
  echo "## archived modules exported check"
  for mod in "${archived_mods[@]}"; do
    if rg -n "pub mod ${mod};" "$RUNTIME_MOD" >/dev/null 2>&1; then
      echo "$mod exported=yes"
      violations=$((violations + 1))
    else
      echo "$mod exported=no"
    fi
  done
  echo
  echo "export_violations=$violations"
  echo "marker=phase40a=x4-device-regression-write-lane-closeout-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$violations" -ne 0 ]; then
  exit 4
fi
