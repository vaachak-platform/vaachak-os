#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
cd "$repo_root"

overlay_dir="${PHASE36A_OVERLAY_DIR:-phase36a_active_input_semantic_mapping_overlay/overlays}"
backup_root=".phase_backups/phase36a/$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_root"

backup_file() {
  local rel="$1"
  if [[ -f "$rel" ]]; then
    mkdir -p "$backup_root/$(dirname "$rel")"
    cp "$rel" "$backup_root/$rel"
  fi
}

copy_file() {
  local rel="$1"
  mkdir -p "$(dirname "$rel")"
  cp "$overlay_dir/$rel" "$rel"
  echo "installed $rel"
}

backup_file target-xteink-x4/src/vaachak_x4/input/mod.rs
backup_file target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

copy_file target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs
copy_file target-xteink-x4/src/vaachak_x4/input/mod.rs
copy_file target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

mkdir -p docs/phase36a scripts
cp -R phase36a_active_input_semantic_mapping_overlay/docs/phase36a/. docs/phase36a/
cp phase36a_active_input_semantic_mapping_overlay/scripts/check_imported_reader_runtime_sync_phase36a.sh scripts/
cp phase36a_active_input_semantic_mapping_overlay/scripts/check_phase36a_active_input_semantic_mapping.sh scripts/
cp phase36a_active_input_semantic_mapping_overlay/scripts/check_phase36a_no_input_hardware_regression.sh scripts/
cp phase36a_active_input_semantic_mapping_overlay/scripts/revert_phase36a_active_input_semantic_mapping.sh scripts/
chmod +x scripts/check_imported_reader_runtime_sync_phase36a.sh \
         scripts/check_phase36a_active_input_semantic_mapping.sh \
         scripts/check_phase36a_no_input_hardware_regression.sh \
         scripts/revert_phase36a_active_input_semantic_mapping.sh

echo "Phase 36A active input semantic mapping overlay applied. Backup: $backup_root"
