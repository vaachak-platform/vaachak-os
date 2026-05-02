#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase39k-write-lane-scaffolding-inventory.txt}"

{
  echo "# Phase 39K Write Lane Scaffolding Inventory"
  echo
  echo "## accepted active path"
  rg -n 'phase39i|typed_state_wiring::write_app_subdir|typed_state_wiring::ensure_state_dir|persist_progress_records|persist_theme_preset|persist_meta_record|persist_bookmarks|persist_bookmarks_index|ensure_bookmark_stub' \
    vendor/pulp-os/src/apps/reader/mod.rs vendor/pulp-os/src/apps/reader/typed_state_wiring.rs 2>/dev/null || true
  echo
  echo "## accepted verification path"
  rg -n 'phase39j|inspect_phase39j|accept_phase39j|state_io_runtime_state_write_verification' \
    target-xteink-x4/src/vaachak_x4/runtime.rs target-xteink-x4/src/vaachak_x4/runtime scripts phase39j_runtime_state_write_verification_acceptance_overlay 2>/dev/null || true
  echo
  echo "## older Phase 38/39 state/write scaffolding candidates"
  find target-xteink-x4/src/vaachak_x4/runtime -maxdepth 1 -type f \
    \( -name 'state_io_*write*.rs' -o -name 'state_io_*state*.rs' -o -name 'file_explorer_*display*.rs' \) \
    -printf '%f\n' | sort || true
  echo
  echo "## phase exports"
  rg -n 'state_io_.*(write|state|sdfat|runtime_file|typed_state|cleanup|verification)' \
    target-xteink-x4/src/vaachak_x4/runtime.rs 2>/dev/null || true
} | tee "$OUT"

echo
echo "Wrote: $OUT"
