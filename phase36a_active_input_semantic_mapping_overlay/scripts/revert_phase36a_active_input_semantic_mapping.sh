#!/usr/bin/env bash
set -euo pipefail

backup_root=".phase_backups/phase36a"

if [[ ! -d "$backup_root" ]]; then
  echo "No Phase 36A backup directory found at $backup_root"
  exit 1
fi

restore_if_present() {
  local rel="$1"
  if [[ -f "$backup_root/$rel" ]]; then
    mkdir -p "$(dirname "$rel")"
    cp "$backup_root/$rel" "$rel"
    echo "restored $rel"
  fi
}

restore_if_present target-xteink-x4/src/vaachak_x4/input/mod.rs
restore_if_present target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

rm -f target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs
rm -rf docs/phase36a
rm -f scripts/check_imported_reader_runtime_sync_phase36a.sh \
      scripts/check_phase36a_active_input_semantic_mapping.sh \
      scripts/check_phase36a_no_input_hardware_regression.sh \
      scripts/revert_phase36a_active_input_semantic_mapping.sh

echo "Phase 36A files reverted where backups were available."
