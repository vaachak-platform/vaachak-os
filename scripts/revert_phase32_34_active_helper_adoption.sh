#!/usr/bin/env bash
set -euo pipefail

echo "Phase 32-34 revert helper"
echo

backup_dir=".phase_backups/phase32_34/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -a "$path" "$backup_dir/$path"
  fi
}

backup_if_exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/contracts/mod.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/contracts/display_geometry.rs
backup_if_exists docs/phase32_34
backup_if_exists scripts/check_imported_reader_runtime_sync_phase32_34.sh
backup_if_exists scripts/check_phase32_34_active_helper_adoption.sh

echo "Backed up Phase 32-34 touched files to $backup_dir"
echo
echo "This helper only creates a backup. Restore the previous files manually from"
echo "$backup_dir if Phase 32-34 needs to be backed out."
