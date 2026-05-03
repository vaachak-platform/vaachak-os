#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40h_repair1_seed_txt_titlemap_into_titles_bin_overlay"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"

mkdir -p "$RUNTIME_DIR"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_seed_txt_titlemap_into_titles_bin_repair.rs" \
  "$RUNTIME_DIR/state_io_seed_txt_titlemap_into_titles_bin_repair.rs"
cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_seed_txt_titlemap_into_titles_bin_repair_acceptance.rs" \
  "$RUNTIME_DIR/state_io_seed_txt_titlemap_into_titles_bin_repair_acceptance.rs"

for export in \
  "pub mod state_io_seed_txt_titlemap_into_titles_bin_repair;" \
  "pub mod state_io_seed_txt_titlemap_into_titles_bin_repair_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40h_repair1_seed_titles_bin_metadata.sh"

echo "phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok"
echo "phase40h-repair1-acceptance=x4-seed-txt-titlemap-into-titles-bin-report-ok"
