#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
ARCHIVE_DIR="$ROOT/docs/archive/phase38-39-scaffolding/runtime"
OUT="${OUT:-/tmp/phase39m-archive-patch-acceptance.txt}"

"$ROOT/phase39m_safe_scaffolding_archive_patch_overlay/scripts/guard_phase39m_archive_patch.sh" >/dev/null

archive_files=(
  state_io_guarded_persistent_backend_stub.rs
  state_io_guarded_read_before_write_stub.rs
  state_io_guarded_write_backend_adapter_acceptance.rs
  state_io_guarded_write_backend_adapter_shape.rs
  state_io_guarded_write_backend_binding.rs
  state_io_guarded_write_backend_dry_run_executor.rs
  state_io_guarded_write_backend_implementation_seam.rs
  state_io_guarded_write_dry_run_acceptance.rs
  state_io_pre_behavior_write_enablement_consolidation.rs
  state_io_shadow_write_acceptance.rs
  state_io_shadow_write_plan.rs
  state_io_write_design_consolidation.rs
  state_io_write_lane_entry_contract.rs
  state_io_write_lane_handoff_consolidation.rs
  state_io_write_plan_design.rs
)

archived_count=0
runtime_remaining=0
export_remaining=0

for file in "${archive_files[@]}"; do
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
reason="ArchivePatchAccepted"
if [ "$archived_count" -ne "${#archive_files[@]}" ]; then
  status="REJECTED"
  reason="ArchiveCountMismatch"
elif [ "$runtime_remaining" -ne 0 ]; then
  status="REJECTED"
  reason="RuntimeArchiveCandidatesRemain"
elif [ "$export_remaining" -ne 0 ]; then
  status="REJECTED"
  reason="ArchivedExportsRemain"
fi

{
  echo "# Phase 39M Archive Patch Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "archived_count=$archived_count/${#archive_files[@]}"
  echo "runtime_remaining=$runtime_remaining"
  echo "export_remaining=$export_remaining"
  echo "active_path_guard=ACCEPTED"
  echo "review_delete_later_touched=false"
  echo "marker=phase39m=x4-safe-scaffolding-archive-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
