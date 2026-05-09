#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "storage_physical_sd_mmc_full_migration validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local text="$1"
  local file="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_text() {
  local text="$1"
  local file="$2"
  if grep -Fq "$text" "$file"; then
    fail "forbidden text '$text' in $file"
  fi
}

SRC=target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs
SMOKE=target-xteink-x4/src/vaachak_x4/contracts/storage_physical_sd_mmc_native_driver_smoke.rs
DOC=docs/architecture/storage-physical-sd-mmc-full-migration.md
SPI_SRC=target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs

require_file "$SRC"
require_file "$SMOKE"
require_file "$DOC"
require_file "$SPI_SRC"
require_file scripts/validate_storage_physical_sd_mmc_full_migration.sh

require_text "pub mod storage_physical_sd_mmc_native_driver;" target-xteink-x4/src/vaachak_x4/physical/mod.rs
require_text "pub mod storage_physical_sd_mmc_native_driver_smoke;" target-xteink-x4/src/vaachak_x4/contracts/mod.rs

require_text "VaachakStoragePhysicalSdMmcNativeDriver" "$SRC"
require_text "STORAGE_PHYSICAL_SD_MMC_FULL_MIGRATION_MARKER" "$SRC"
require_text "storage_physical_sd_mmc_full_migration=ok" "$SRC"
require_text "ACTIVE_BACKEND_NAME: &'static str = \"VaachakNativeSdMmcPhysicalDriver\"" "$SRC"
require_text "TRANSPORT_BACKEND_NAME: &'static str = \"VaachakNativeSpiPhysicalDriver\"" "$SRC"
require_text "SD_MMC_CARD_LIFECYCLE_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SD_MMC_PROBE_INIT_SEQUENCE_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SD_MMC_MOUNT_LIFECYCLE_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SD_MMC_STORAGE_AVAILABILITY_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SD_MMC_USES_NATIVE_SPI_DRIVER: bool = true" "$SRC"
require_text "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED: bool = false" "$SRC"
require_text "IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE: bool = false" "$SRC"
require_text "FAT_ALGORITHM_MIGRATION_DEFERRED: bool = true" "$SRC"
require_text "VaachakSdMmcNativeTargetBackend" "$SRC"
require_text "VaachakSdMmcNativeTargetHalBoundary" "$SRC"
require_text "lifecycle_policy" "$SRC"
require_text "block_policy" "$SRC"
require_text "detect_request" "$SRC"
require_text "probe_request" "$SRC"
require_text "initialize_request" "$SRC"
require_text "mount_request" "$SRC"
require_text "read_block_request" "$SRC"
require_text "write_block_request" "$SRC"
require_text "execute_with_backend" "$SRC"
require_text "VaachakSpiPhysicalNativeDriver::storage_request" "$SRC"
require_text "VaachakSpiPhysicalNativeDriver::execute_with_backend" "$SRC"
require_text "VaachakSpiPhysicalNativeDriver::full_migration_ok" "$SRC"
require_text "migration_report" "$SRC"
require_text "full_migration_ok" "$SRC"

require_text "VaachakStoragePhysicalSdMmcNativeDriverSmoke" "$SMOKE"
require_text "STORAGE_PHYSICAL_SD_MMC_NATIVE_DRIVER_SMOKE_OK" "$SMOKE"
require_text "VaachakNativeSdMmcPhysicalDriver" "$DOC"
require_text "VaachakNativeSpiPhysicalDriver" "$DOC"
require_text "Pulp SD/MMC fallback | \`false\`" "$DOC"
require_text "FAT algorithm migration | deferred" "$DOC"

# Ensure the accepted full SPI migration is present and selected as transport.
require_text "SPI_FULLY_MIGRATED_TO_VAACHAK: bool = true" "$SPI_SRC"
require_text "PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false" "$SPI_SRC"
require_text "IMPORTED_PULP_SPI_RUNTIME_ACTIVE: bool = false" "$SPI_SRC"

# Guard against accidentally selecting old Pulp storage runtime in this boundary.
require_absent_text "VaachakHardwareRuntimePulpCompatibilityBackend" "$SRC"
require_absent_text "VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback" "$SRC"
require_absent_text "vendor/pulp-os imported runtime" "$SRC"
require_absent_text "PulpCompatibility" "$SRC"

# Guard against unrelated UX/app behavior wiring.
if [ -d target-xteink-x4/src/apps ]; then
  if grep -R "storage_physical_sd_mmc_native_driver" target-xteink-x4/src/apps >/dev/null 2>&1; then
    fail "app UX files reference SD/MMC physical driver directly"
  fi
fi
if [ -d vendor/pulp-os ]; then
  if grep -R "storage_physical_sd_mmc_native_driver" vendor/pulp-os >/dev/null 2>&1; then
    fail "vendor/pulp-os was modified to reference Vaachak SD/MMC driver"
  fi
fi

echo "storage_physical_sd_mmc_full_migration=ok"
