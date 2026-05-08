#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "storage_backend_native_sd_mmc_fat_executor_cleanup validation failed: $1" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_regex() {
  local file="$1"
  local pattern="$2"
  perl -0ne "exit(!/$pattern/s)" "$file" || fail "missing pattern '$pattern' in $file"
}

require_no_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    fail "unexpected text '$text' in $file"
  fi
}

cleanup_src="target-xteink-x4/src/vaachak_x4/physical/storage_backend_native_sd_mmc_fat_executor_cleanup.rs"
cleanup_smoke="target-xteink-x4/src/vaachak_x4/contracts/storage_backend_native_sd_mmc_fat_executor_cleanup_smoke.rs"
storage_src="target-xteink-x4/src/vaachak_x4/physical/storage_backend_native_sd_mmc_fat_executor.rs"
storage_doc="docs/architecture/storage-backend-native-sd-mmc-fat-executor.md"
cleanup_doc="docs/architecture/storage-backend-native-sd-mmc-fat-executor-cleanup.md"

require_file "$cleanup_src"
require_file "$cleanup_smoke"
require_file "$storage_src"
require_file "$storage_doc"
require_file "$cleanup_doc"
require_file "scripts/cleanup_storage_backend_native_sd_mmc_fat_executor_artifacts.sh"

require_text "target-xteink-x4/src/vaachak_x4/physical/mod.rs" "pub mod storage_backend_native_sd_mmc_fat_executor_cleanup;"
require_text "target-xteink-x4/src/vaachak_x4/contracts/mod.rs" "pub mod storage_backend_native_sd_mmc_fat_executor_cleanup_smoke;"

require_text "$cleanup_src" "VaachakStorageBackendNativeSdMmcFatExecutorCleanup"
require_text "$cleanup_src" "storage_backend_native_sd_mmc_fat_executor_cleanup=ok"
require_text "$cleanup_src" "VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok()"
require_text "$cleanup_src" "VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_availability_handoff()"
require_text "$cleanup_src" "VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_fat_access_handoff()"
require_text "$cleanup_src" "VaachakStorageBackendNativeSdMmcFatExecutor::execute_destructive_operation_denial()"
require_regex "$cleanup_src" "low_level_sd_mmc_block_driver_moved_to_vaachak:[[:space:]]*VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK"
require_regex "$cleanup_src" "low_level_fat_algorithm_moved_to_vaachak:[[:space:]]*VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK"
require_regex "$cleanup_src" "reader_file_browser_ux_changed:[[:space:]]*VaachakStorageBackendNativeSdMmcFatExecutor::READER_FILE_BROWSER_UX_CHANGED"
require_regex "$cleanup_src" "app_navigation_behavior_changed:[[:space:]]*VaachakStorageBackendNativeSdMmcFatExecutor::APP_NAVIGATION_BEHAVIOR_CHANGED"
require_text "$cleanup_smoke" "storage_backend_native_sd_mmc_fat_executor_cleanup_smoke_ok"
require_text "$cleanup_smoke" "VaachakStorageBackendNativeSdMmcFatExecutorCleanup::cleanup_ok()"

require_text "$storage_src" "VaachakStorageBackendNativeSdMmcFatExecutor"
require_text "$storage_src" "SD_MMC_FAT_COMMAND_DECISION_MOVED_TO_VAACHAK: bool = true"
require_text "$storage_src" "PROBE_MOUNT_STATE_MACHINE_MOVED_TO_VAACHAK: bool = true"
require_text "$storage_src" "FAT_OPERATION_CLASSIFICATION_MOVED_TO_VAACHAK: bool = true"
require_text "$storage_src" "DESTRUCTIVE_OPERATION_GUARD_MOVED_TO_VAACHAK: bool = true"
require_text "$storage_src" "LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK: bool = false"
require_text "$storage_src" "LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK: bool = false"
require_text "$storage_src" "PulpCompatibility"
require_text "$storage_src" "execute_destructive_operation_denial"

require_text "$storage_doc" "storage_backend_native_sd_mmc_fat_executor_cleanup=ok"
require_text "$cleanup_doc" "storage_backend_native_sd_mmc_fat_executor_cleanup=ok"
require_text "$cleanup_doc" "PulpCompatibility"
require_text "$cleanup_doc" "physical SD/MMC block driver remains Pulp-compatible"
require_text "$cleanup_doc" "low-level FAT algorithms remain Pulp-compatible"

# Guard against accidental low-level/destructive implementation in the cleanup module.
require_no_text "$cleanup_src" "write_file"
require_no_text "$cleanup_src" "append_file"
require_no_text "$cleanup_src" "delete_file_impl"
require_no_text "$cleanup_src" "rename_file_impl"
require_no_text "$cleanup_src" "mk_dir_impl"
require_no_text "$cleanup_src" "block_read_impl"
require_no_text "$cleanup_src" "spi_transfer_impl"

# App/UI behavior files should not be introduced by this cleanup overlay.
if find target-xteink-x4/src/vaachak_x4 -path '*apps*' -type f 2>/dev/null | grep -q 'storage_backend_native_sd_mmc_fat_executor_cleanup'; then
  fail "cleanup unexpectedly touched app/navigation files"
fi

echo "storage_backend_native_sd_mmc_fat_executor_cleanup=ok"
