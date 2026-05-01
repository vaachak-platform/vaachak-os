#!/usr/bin/env bash
set -euo pipefail

latest="$(find .phase_backups -maxdepth 1 -type d -name 'phase21-*' 2>/dev/null | sort | tail -n 1 || true)"
if [[ -z "$latest" ]]; then
  echo "ERROR: no .phase_backups/phase21-* backup directory found" >&2
  exit 1
fi

echo "Restoring from $latest"

restore_if_exists() {
  local path="$1"
  if [[ -e "$latest/$path" ]]; then
    mkdir -p "$(dirname "$path")"
    rm -rf "$path"
    cp -a "$latest/$path" "$path"
    echo "restored $path"
  fi
}

restore_if_exists target-xteink-x4/src/runtime/mod.rs
restore_if_exists target-xteink-x4/src/runtime/storage_boundary.rs
restore_if_exists target-xteink-x4/src/runtime/vaachak_runtime.rs
restore_if_exists scripts/check_reader_runtime_sync_phase21.sh
restore_if_exists scripts/check_phase21_storage_boundary.sh
restore_if_exists scripts/revert_phase21_storage_boundary.sh
restore_if_exists docs/phase21

echo "Phase 21 revert complete"
