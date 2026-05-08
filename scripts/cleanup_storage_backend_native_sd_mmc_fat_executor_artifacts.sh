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

remove_path "storage_backend_native_sd_mmc_fat_executor"
remove_path "storage_backend_native_sd_mmc_fat_executor.zip"
remove_path "storage_backend_native_sd_mmc_fat_executor_takeover_fix"
remove_path "storage_backend_native_sd_mmc_fat_executor_takeover_fix.zip"
remove_path "storage_backend_native_sd_mmc_fat_executor_validator_fix"
remove_path "storage_backend_native_sd_mmc_fat_executor_validator_fix.zip"
remove_path "storage_backend_native_sd_mmc_fat_executor_cleanup_validator_fix"
remove_path "storage_backend_native_sd_mmc_fat_executor_cleanup_validator_fix.zip"

find . -path '*/__pycache__' -type d -prune -exec rm -rf {} + 2>/dev/null || true

echo "storage_backend_native_sd_mmc_fat_executor_cleanup_artifacts=ok removed=${removed}"
