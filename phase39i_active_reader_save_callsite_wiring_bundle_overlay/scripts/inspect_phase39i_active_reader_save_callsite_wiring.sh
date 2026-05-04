#!/usr/bin/env bash
set -euo pipefail

OUT="/tmp/phase39i-active-reader-save-callsite-summary.txt"

{
  echo "# Phase 39I Active Reader Save Callsite Summary"
  echo
  echo "## Phase 39I helper"
  rg -n 'phase39i|write_app_subdir|ensure_state_dir|classify_state_file_name' \
    vendor/pulp-os/src/apps/reader/typed_state_wiring.rs || true
  echo
  echo "## Active reader callsite routing"
  rg -n 'typed_state_wiring::write_app_subdir|typed_state_wiring::ensure_state_dir|persist_progress_records|persist_theme_preset|persist_meta_record|persist_bookmarks|persist_bookmarks_index|ensure_bookmark_stub' \
    vendor/pulp-os/src/apps/reader/mod.rs || true
  echo
  echo "## Direct calls that should be gone from reader/mod.rs"
  rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' \
    vendor/pulp-os/src/apps/reader/mod.rs || true
} | tee "$OUT"

echo
echo "Wrote: $OUT"
