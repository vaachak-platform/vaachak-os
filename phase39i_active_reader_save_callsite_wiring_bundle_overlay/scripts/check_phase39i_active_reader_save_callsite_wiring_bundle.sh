#!/usr/bin/env bash
set -euo pipefail

READER_MOD="vendor/pulp-os/src/apps/reader/mod.rs"
HELPER="vendor/pulp-os/src/apps/reader/typed_state_wiring.rs"
TARGET="target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs"
RUNTIME_MOD="target-xteink-x4/src/vaachak_x4/runtime.rs"

test -f "$READER_MOD"
test -f "$HELPER"
test -f "$TARGET"

grep -q 'PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER' "$HELPER"
grep -q 'mod typed_state_wiring;' "$READER_MOD"
grep -q 'typed_state_wiring::write_app_subdir' "$READER_MOD"
grep -q 'typed_state_wiring::ensure_state_dir' "$READER_MOD"
grep -q 'PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER' "$TARGET"
grep -q 'pub mod state_io_active_reader_save_callsite_wiring;' "$RUNTIME_MOD"

if rg -n '\bk\s*\.\s*write_app_subdir\s*\(' "$READER_MOD"; then
  echo "direct active reader k.write_app_subdir call remains in reader/mod.rs" >&2
  exit 1
fi

if rg -n '\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' "$READER_MOD"; then
  echo "direct active reader state-dir ensure call remains in reader/mod.rs" >&2
  exit 1
fi

for needle in \
  'persist_progress_records' \
  'persist_theme_preset' \
  'persist_meta_record' \
  'persist_bookmarks' \
  'persist_bookmarks_index' \
  'ensure_bookmark_stub'
do
  grep -q "$needle" "$READER_MOD"
done

echo "phase39i-check=ok"
