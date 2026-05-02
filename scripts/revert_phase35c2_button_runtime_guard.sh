#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
cd "$repo_root"

backup_dir=".phase_backups/phase35c2/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -a "$path" "$backup_dir/$path"
    echo "Backed up $path to $backup_dir/$path"
  fi
}

backup_if_exists docs/phase35c2
backup_if_exists scripts/check_phase35c2_button_runtime_guard.sh
backup_if_exists scripts/check_phase35c2_direct_app_manager_runtime.sh
backup_if_exists scripts/revert_phase35c2_button_runtime_guard.sh

rm -rf docs/phase35c2
rm -f scripts/check_phase35c2_button_runtime_guard.sh
rm -f scripts/check_phase35c2_direct_app_manager_runtime.sh
rm -f scripts/revert_phase35c2_button_runtime_guard.sh

echo "Removed Phase 35C-2 guard docs/scripts after backup."
