#!/usr/bin/env bash
set -euo pipefail

fail() { echo "storage_fat_algorithm_full_migration validation failed: $*" >&2; exit 1; }
require_file() { [ -f "$1" ] || fail "missing file $1"; }
require_text() { grep -Fq "$2" "$1" || fail "missing text '$2' in $1"; }
require_absent_text() { if grep -Fq "$2" "$1"; then fail "forbidden text '$2' in $1"; fi; }

MODULE=target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs
SMOKE=target-xteink-x4/src/vaachak_x4/contracts/storage_fat_algorithm_native_driver_smoke.rs
DOC=docs/architecture/storage-fat-algorithm-full-migration.md
SDMMC=target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs

require_file "$MODULE"
require_file "$SMOKE"
require_file "$DOC"
require_file "$SDMMC"

require_text target-xteink-x4/src/vaachak_x4/physical/mod.rs "pub mod storage_fat_algorithm_native_driver;"
require_text target-xteink-x4/src/vaachak_x4/contracts/mod.rs "pub mod storage_fat_algorithm_native_driver_smoke;"

for token in \
  "VaachakStorageFatAlgorithmNativeDriver" \
  "storage_fat_algorithm_full_migration=ok" \
  "VaachakNativeFatAlgorithmDriver" \
  "VaachakNativeSdMmcPhysicalDriver" \
  "FAT_ALGORITHM_FULLY_MIGRATED_TO_VAACHAK: bool = true" \
  "PATH_NORMALIZATION_MOVED_TO_VAACHAK: bool = true" \
  "BPB_BOOT_SECTOR_PARSE_MOVED_TO_VAACHAK: bool = true" \
  "DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK: bool = true" \
  "LONG_FILENAME_ALGORITHM_MOVED_TO_VAACHAK: bool = true" \
  "FAT_TABLE_TRAVERSAL_MOVED_TO_VAACHAK: bool = true" \
  "FILE_OPEN_READ_WRITE_POLICY_MOVED_TO_VAACHAK: bool = true" \
  "METADATA_UPDATE_POLICY_MOVED_TO_VAACHAK: bool = true" \
  "DESTRUCTIVE_OPERATION_POLICY_MOVED_TO_VAACHAK: bool = true" \
  "NATIVE_SD_MMC_BLOCK_DRIVER_REQUIRED: bool = true" \
  "PULP_FAT_ALGORITHM_FALLBACK_ENABLED: bool = false" \
  "IMPORTED_PULP_FAT_RUNTIME_ACTIVE: bool = false" \
  "VaachakFatNativeBlockIoBackend" \
  "VaachakFatNativeSdMmcBlockIoBoundary" \
  "mount_volume_request" \
  "list_directory_request" \
  "open_file_request" \
  "read_file_chunk_request" \
  "write_file_chunk_request" \
  "delete_file_request" \
  "rename_file_request" \
  "classify_access" \
  "execute_with_block_backend" \
  "full_migration_ok" \
  "migration_report"
do
  require_text "$MODULE" "$token"
done

require_text "$SMOKE" "storage_fat_algorithm_native_driver_smoke_ok"
require_text "$DOC" "storage_fat_algorithm_full_migration=ok"
require_text "$DOC" "VaachakNativeFatAlgorithmDriver"
require_text "$DOC" "VaachakNativeSdMmcPhysicalDriver"
require_text "$DOC" "Pulp FAT fallback | \`false\`"

require_text "$SDMMC" "VaachakStoragePhysicalSdMmcNativeDriver"
require_text "$SDMMC" "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED: bool = false"
require_text "$SDMMC" "IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE: bool = false"

require_absent_text "$MODULE" "PulpCompatibility"
require_absent_text "$MODULE" "vendor/pulp-os imported runtime"
require_absent_text "$MODULE" "VaachakHardwareRuntimePulpCompatibilityBackend"
require_absent_text "$MODULE" "VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback"

if [ -d target-xteink-x4/src/apps ]; then
  if grep -R "storage_fat_algorithm_native_driver" target-xteink-x4/src/apps >/dev/null 2>&1; then
    fail "app UX files reference FAT algorithm driver directly"
  fi
fi
if [ -d vendor/pulp-os ]; then
  if grep -R "storage_fat_algorithm_native_driver" vendor/pulp-os >/dev/null 2>&1; then
    fail "vendor/pulp-os references Vaachak FAT algorithm driver"
  fi
fi

echo "storage_fat_algorithm_full_migration=ok"
