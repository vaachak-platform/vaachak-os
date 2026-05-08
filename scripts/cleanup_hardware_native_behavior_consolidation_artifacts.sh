#!/usr/bin/env bash
set -euo pipefail

removed=0
remove_path() {
  local path="$1"
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
}

remove_path "hardware_native_behavior_consolidation"
remove_path "hardware_native_behavior_consolidation.zip"
remove_path "hardware_native_behavior_consolidation_validator_fix"
remove_path "hardware_native_behavior_consolidation_validator_fix.zip"
remove_path "hardware_native_behavior_consolidation_cleanup_validator_fix"
remove_path "hardware_native_behavior_consolidation_cleanup_validator_fix.zip"
remove_path "hardware_native_behavior_consolidation_cleanup_validator_fix2"
remove_path "hardware_native_behavior_consolidation_cleanup_validator_fix2.zip"

find . -path '*/__pycache__' -type d -prune -exec rm -rf {} + 2>/dev/null || true

echo "hardware_native_behavior_consolidation_cleanup_artifacts=ok removed=${removed}"
