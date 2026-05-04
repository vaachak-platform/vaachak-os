#!/usr/bin/env bash
set -euo pipefail
ROOT="$(pwd)"
OVERLAY="$ROOT/phase40g_home_full_width_library_titles_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
mkdir -p "$RUNTIME_DIR"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_patch.rs" "$RUNTIME_DIR/state_io_home_library_title_patch.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_patch_acceptance.rs" "$RUNTIME_DIR/state_io_home_library_title_patch_acceptance.rs"
for export in "pub mod state_io_home_library_title_patch;" "pub mod state_io_home_library_title_patch_acceptance;"; do
  grep -Fxq "$export" "$RUNTIME_MOD" || printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
done
"$OVERLAY/scripts/check_phase40g_home_library_title_patch.sh"
"$OVERLAY/scripts/guard_phase40g_patch_scope.sh"
echo "phase40g=x4-home-full-width-library-title-patch-ok"
echo "phase40g-acceptance=x4-home-library-title-patch-report-ok"
