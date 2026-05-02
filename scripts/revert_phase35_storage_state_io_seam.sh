#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
cd "$repo_root"

backup_dir=".phase_backups/phase35/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -a "$path" "$backup_dir/$path"
    echo "Backed up $path to $backup_dir/$path"
  fi
}

backup_if_exists target-xteink-x4/src/vaachak_x4/io
backup_if_exists docs/phase35
backup_if_exists scripts/check_phase35_physical_extraction_plan.sh
backup_if_exists scripts/check_phase35_storage_state_io_seam.sh
backup_if_exists scripts/check_phase35_no_hardware_regression.sh
backup_if_exists scripts/check_imported_reader_runtime_sync_phase35.sh
backup_if_exists scripts/revert_phase35_storage_state_io_seam.sh

rm -rf target-xteink-x4/src/vaachak_x4/io docs/phase35
rm -f scripts/check_phase35_physical_extraction_plan.sh
rm -f scripts/check_phase35_storage_state_io_seam.sh
rm -f scripts/check_phase35_no_hardware_regression.sh
rm -f scripts/check_imported_reader_runtime_sync_phase35.sh
rm -f scripts/revert_phase35_storage_state_io_seam.sh

echo "Removed Phase 35 scaffold docs/scripts/io directory after backup."
echo "Use git restore for any manual source edits outside the Phase 35 scaffold."
