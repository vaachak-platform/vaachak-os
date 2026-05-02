#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
cd "$repo_root"

backup_dir=".phase_backups/phase35c0/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$backup_dir/$(dirname "$path")"
    cp -a "$path" "$backup_dir/$path"
    echo "Backed up $path to $backup_dir/$path"
  fi
}

backup_if_exists target-xteink-x4/src/vaachak_x4/apps
backup_if_exists target-xteink-x4/src/vaachak_x4/mod.rs
backup_if_exists docs/phase35c0
backup_if_exists scripts/check_phase35c0_reader_state_facade.sh
backup_if_exists scripts/check_phase35c0_no_active_io_takeover.sh
backup_if_exists scripts/revert_phase35c0_reader_state_facade.sh

rm -rf target-xteink-x4/src/vaachak_x4/apps docs/phase35c0
rm -f scripts/check_phase35c0_reader_state_facade.sh
rm -f scripts/check_phase35c0_no_active_io_takeover.sh
rm -f scripts/revert_phase35c0_reader_state_facade.sh

echo "Removed Phase 35C-0 reader state facade after backup."
