#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase39l-scaffolding-cleanup-plan.md}"

runtime_dir="target-xteink-x4/src/vaachak_x4/runtime"

{
  echo "# Phase 39L Scaffolding Cleanup Plan"
  echo
  echo "Review-only: this plan does not delete files."
  echo
  echo "## KEEP ACTIVE"
  echo "- vendor/pulp-os/src/apps/reader/mod.rs"
  echo "- vendor/pulp-os/src/apps/reader/typed_state_wiring.rs"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs"
  echo
  echo "## KEEP VERIFICATION"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification.rs"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification_acceptance.rs"
  echo "- phase39j_runtime_state_write_verification_acceptance_overlay/scripts/*.sh"
  echo
  echo "## KEEP FREEZE METADATA"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze.rs"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze_report.rs"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_post_freeze_scaffolding_cleanup_plan.rs"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_post_freeze_scaffolding_cleanup_plan_acceptance.rs"
  echo
  echo "## REVIEW DELETE LATER CANDIDATES"
  find "$runtime_dir" -maxdepth 1 -type f \
    \( -name 'state_io_progress_write_*.rs' \
       -o -name 'state_io_typed_record_*.rs' \
       -o -name 'state_io_runtime_owned_sdfat_writer*.rs' \
       -o -name 'state_io_runtime_file_api_integration_gate*.rs' \
       -o -name 'state_io_typed_state_runtime_callsite_wiring*.rs' \) \
    -printf '- %p\n' | sort || true
  echo
  echo "## REVIEW ARCHIVE CANDIDATES"
  find "$runtime_dir" -maxdepth 1 -type f \
    \( -name 'state_io_guarded_*.rs' \
       -o -name 'state_io_write_*design*.rs' \
       -o -name 'state_io_write_plan_design.rs' \
       -o -name 'state_io_write_lane_entry_contract.rs' \
       -o -name 'state_io_write_lane_handoff_consolidation.rs' \
       -o -name 'state_io_shadow_write*.rs' \
       -o -name 'state_io_pre_behavior_write_enablement_consolidation.rs' \) \
    -printf '- %p\n' | sort || true
  echo
  echo "## PRESERVE HISTORICAL / NON-WRITE-LANE"
  find "$runtime_dir" -maxdepth 1 -type f \
    \( -name 'file_explorer_*display*.rs' \) \
    -printf '- %p\n' | sort || true
  echo
  echo "## Guard before future deletion"
  echo '```bash'
  echo './phase39l_post_freeze_scaffolding_cleanup_plan_overlay/scripts/guard_phase39l_accepted_write_path.sh'
  echo '```'
} | tee "$OUT"

echo
echo "Wrote: $OUT"
