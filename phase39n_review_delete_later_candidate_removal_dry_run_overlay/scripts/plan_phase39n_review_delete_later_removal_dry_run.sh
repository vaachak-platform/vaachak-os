#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
OUT="${OUT:-/tmp/phase39n-review-delete-later-removal-dry-run.md}"

"$ROOT/phase39n_review_delete_later_candidate_removal_dry_run_overlay/scripts/guard_phase39n_accepted_write_path.sh" >/dev/null

candidates=(
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

{
  echo "# Phase 39N Review-Delete-Later Removal Dry Run"
  echo
  echo "Dry run only. No files were deleted or moved."
  echo
  echo "## Protected active path"
  echo "- vendor/pulp-os/src/apps/reader/mod.rs"
  echo "- vendor/pulp-os/src/apps/reader/typed_state_wiring.rs"
  echo "- target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs"
  echo "- Phase 39J/39K/39L/39M metadata"
  echo
  echo "## Candidate report"
  echo
  echo "| Candidate | Exists | Exported | Inbound refs outside self/runtime.rs |"
  echo "|---|---:|---:|---:|"

  for file in "${candidates[@]}"; do
    mod="${file%.rs}"
    exists="no"
    exported="no"
    refs=0

    [ -f "$RUNTIME_DIR/$file" ] && exists="yes"
    if rg -n "pub mod ${mod};" "$RUNTIME_MOD" >/dev/null 2>&1; then
      exported="yes"
    fi

    # Count references outside the file itself, runtime.rs, overlays, target build, and docs/archive.
    refs="$(rg -n "$mod" \
      target-xteink-x4/src vendor/pulp-os/src hal-xteink-x4/src core/src \
      --glob '!target/**' \
      --glob "!target-xteink-x4/src/vaachak_x4/runtime/${file}" \
      2>/dev/null \
      | grep -v 'target-xteink-x4/src/vaachak_x4/runtime.rs' \
      | grep -v 'phase39' \
      | wc -l | tr -d ' ')"

    echo "| $file | $exists | $exported | $refs |"
  done

  echo
  echo "## Suggested next real-removal sequence"
  echo "1. Run this dry run and commit the plan."
  echo "2. Build/check/clippy."
  echo "3. Flash and verify reader state writes still restore."
  echo "4. In Phase 39O, remove candidate exports from runtime.rs and move/delete candidate files."
  echo "5. Re-run accepted-path guard before and after removal."
} | tee "$OUT"

echo
echo "Wrote: $OUT"
