#!/usr/bin/env bash
# scripts/revert_phase31_storage_path_adoption.sh

set -euo pipefail

echo "Phase 31 storage path adoption revert helper"
echo

backup_dir=".phase_backups/phase31/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -a "$path" "$backup_dir/$path"
  fi
}

backup_if_exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
backup_if_exists target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs
backup_if_exists scripts/check_imported_reader_runtime_sync_phase31.sh
backup_if_exists scripts/check_phase31_storage_path_adoption.sh
backup_if_exists docs/phase31

echo "Backed up Phase 31 touched files to $backup_dir"
echo
echo "This helper only creates a backup. Restore the previous files manually from"
echo "$backup_dir if Phase 31 needs to be backed out."
