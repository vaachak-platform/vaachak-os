#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "storage_backend_native_sd_mmc_fat_executor validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  file="$1"
  text="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_text() {
  file="$1"
  text="$2"
  if grep -Fq "$text" "$file"; then
    fail "forbidden text '$text' in $file"
  fi
}

MODULE="target-xteink-x4/src/vaachak_x4/physical/storage_backend_native_sd_mmc_fat_executor.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_backend_native_sd_mmc_fat_executor_smoke.rs"
DOC="docs/architecture/storage-backend-native-sd-mmc-fat-executor.md"
TAKEOVER="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs"
LIVE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff.rs"

require_file "$MODULE"
require_file "$SMOKE"
require_file "$DOC"
require_file "$TAKEOVER"
require_file "$LIVE"

require_text "$MODULE" "pub struct VaachakStorageBackendNativeSdMmcFatExecutor"
require_text "$MODULE" "SD_MMC_FAT_COMMAND_DECISION_MOVED_TO_VAACHAK: bool = true"
require_text "$MODULE" "PROBE_MOUNT_STATE_MACHINE_MOVED_TO_VAACHAK: bool = true"
require_text "$MODULE" "FAT_OPERATION_CLASSIFICATION_MOVED_TO_VAACHAK: bool = true"
require_text "$MODULE" "PATH_ROLE_POLICY_MOVED_TO_VAACHAK: bool = true"
require_text "$MODULE" "DESTRUCTIVE_OPERATION_GUARD_MOVED_TO_VAACHAK: bool = true"
require_text "$MODULE" "LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK: bool = false"
require_text "$MODULE" "LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK: bool = false"
require_text "$MODULE" "pub fn execute_storage_operation"
require_text "$MODULE" "pub fn decide_storage_operation"
require_text "$MODULE" "pub fn execute_destructive_operation_denial"
require_text "$MODULE" "VaachakStorageNativeFatOperation::DeleteFileDenied"
require_text "$MODULE" "VaachakStorageNativeBackend::VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback"
require_text "$MODULE" "PulpCompatibility"

require_text "$SMOKE" "storage_backend_native_sd_mmc_fat_executor_smoke_ok"
require_text "$DOC" "storage_backend_native_sd_mmc_fat_executor=ok"
require_text "$DOC" "VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback"
require_text "$DOC" "PulpCompatibility"

require_text "target-xteink-x4/src/vaachak_x4/physical/mod.rs" "pub mod storage_backend_native_sd_mmc_fat_executor;"
require_text "target-xteink-x4/src/vaachak_x4/contracts/mod.rs" "pub mod storage_backend_native_sd_mmc_fat_executor_smoke;"
require_text "$TAKEOVER" "VaachakStorageBackendNativeSdMmcFatExecutor"
require_text "$TAKEOVER" "execute_storage_native_sd_mmc_fat_handoff"
require_text "$LIVE" "VaachakStorageBackendNativeSdMmcFatExecutor"
require_text "$LIVE" "adopt_storage_fat_access_handoff"

# Guard against accidental risky native low-level movement in this slice.
require_absent_text "$MODULE" "format_card("
require_absent_text "$MODULE" "erase_card("
require_absent_text "$MODULE" "delete_file("
require_absent_text "$MODULE" "rename_file("
require_absent_text "$MODULE" "mkdir("
require_absent_text "$MODULE" "write_sector("
require_absent_text "$MODULE" "raw_block_write"

# Keep UI/app paths out of this hardware migration.
if find src target-xteink-x4/src -path '*apps*' -type f 2>/dev/null | grep -q .; then
  true
fi

printf '%s\n' "storage_backend_native_sd_mmc_fat_executor=ok"
