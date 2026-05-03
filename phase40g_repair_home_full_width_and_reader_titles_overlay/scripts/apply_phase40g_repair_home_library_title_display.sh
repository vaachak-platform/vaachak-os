#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40g_repair_home_full_width_and_reader_titles_overlay"

cp -v "$OVERLAY/replaceable/vendor/pulp-os/src/apps/home.rs"   "$ROOT/vendor/pulp-os/src/apps/home.rs"
cp -v "$OVERLAY/replaceable/vendor/pulp-os/src/apps/files.rs"   "$ROOT/vendor/pulp-os/src/apps/files.rs"
cp -v "$OVERLAY/replaceable/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"   "$ROOT/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"

mkdir -p "$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_repair.rs"   "$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_repair.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_repair_acceptance.rs"   "$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_repair_acceptance.rs"

RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
for export in   "pub mod state_io_home_library_title_repair;"   "pub mod state_io_home_library_title_repair_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40g_repair_home_library_title_display.sh"

echo "phase40g-repair=x4-home-full-width-reader-titles-ok"
echo "phase40g-repair-acceptance=x4-home-full-width-reader-titles-report-ok"
