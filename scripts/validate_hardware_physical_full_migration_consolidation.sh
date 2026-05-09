#!/usr/bin/env bash
set -euo pipefail

PHYSICAL_DIR="target-xteink-x4/src/vaachak_x4/physical"
CONTRACT_DIR="target-xteink-x4/src/vaachak_x4/contracts"
DOC="docs/architecture/hardware-physical-full-migration-consolidation.md"
MAIN="$PHYSICAL_DIR/hardware_physical_full_migration_consolidation.rs"
SMOKE="$CONTRACT_DIR/hardware_physical_full_migration_consolidation_smoke.rs"

missing=0
require_file() {
  if [ ! -f "$1" ]; then
    echo "hardware_physical_full_migration_consolidation validation failed: missing file $1" >&2
    missing=1
  fi
}
require_text() {
  local needle="$1"
  local file="$2"
  if ! grep -Fq "$needle" "$file"; then
    echo "hardware_physical_full_migration_consolidation validation failed: missing text '$needle' in $file" >&2
    missing=1
  fi
}
reject_text() {
  local needle="$1"
  local file="$2"
  if grep -Fq "$needle" "$file"; then
    echo "hardware_physical_full_migration_consolidation validation failed: forbidden text '$needle' in $file" >&2
    missing=1
  fi
}

require_file "$MAIN"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYSICAL_DIR/spi_physical_native_driver.rs"
require_file "$PHYSICAL_DIR/display_physical_ssd1677_native_driver.rs"
require_file "$PHYSICAL_DIR/storage_physical_sd_mmc_native_driver.rs"
require_file "$PHYSICAL_DIR/storage_fat_algorithm_native_driver.rs"
require_file "$PHYSICAL_DIR/input_physical_sampling_native_driver.rs"
require_file "$PHYSICAL_DIR/mod.rs"
require_file "$CONTRACT_DIR/mod.rs"

if [ "$missing" -ne 0 ]; then
  exit 1
fi

require_text "pub mod hardware_physical_full_migration_consolidation;" "$PHYSICAL_DIR/mod.rs"
require_text "pub mod hardware_physical_full_migration_consolidation_smoke;" "$CONTRACT_DIR/mod.rs"

require_text "hardware_physical_full_migration_consolidation=ok" "$MAIN"
require_text "VaachakHardwarePhysicalFullMigrationConsolidation" "$MAIN"
require_text "VaachakSpiPhysicalNativeDriver::full_migration_ok()" "$MAIN"
require_text "VaachakSsd1677PhysicalNativeDriver::full_migration_ok()" "$MAIN"
require_text "VaachakStoragePhysicalSdMmcNativeDriver::full_migration_ok()" "$MAIN"
require_text "VaachakStorageFatAlgorithmNativeDriver::full_migration_ok()" "$MAIN"
require_text "VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok()" "$MAIN"
require_text "VaachakNativeSpiPhysicalDriver" "$MAIN"
require_text "VaachakNativeSsd1677PhysicalDriver" "$MAIN"
require_text "VaachakNativeSdMmcPhysicalDriver" "$MAIN"
require_text "VaachakNativeFatAlgorithmDriver" "$MAIN"
require_text "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback" "$MAIN"
require_text "PULP_SPI_TRANSFER_FALLBACK_ENABLED" "$MAIN"
require_text "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED" "$MAIN"
require_text "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED" "$MAIN"
require_text "PULP_FAT_ALGORITHM_FALLBACK_ENABLED" "$MAIN"
require_text "reader_file_browser_ux_changed: false" "$MAIN"
require_text "app_navigation_behavior_changed: false" "$MAIN"

require_text "spi_physical_native_driver_full_migration=ok" "$PHYSICAL_DIR/spi_physical_native_driver.rs"
require_text "PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false" "$PHYSICAL_DIR/spi_physical_native_driver.rs"
require_text "IMPORTED_PULP_SPI_RUNTIME_ACTIVE: bool = false" "$PHYSICAL_DIR/spi_physical_native_driver.rs"
require_text "display_physical_ssd1677_full_migration=ok" "$PHYSICAL_DIR/display_physical_ssd1677_native_driver.rs"
require_text "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED: bool = false" "$PHYSICAL_DIR/display_physical_ssd1677_native_driver.rs"
require_text "IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE: bool = false" "$PHYSICAL_DIR/display_physical_ssd1677_native_driver.rs"
require_text "storage_physical_sd_mmc_full_migration=ok" "$PHYSICAL_DIR/storage_physical_sd_mmc_native_driver.rs"
require_text "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED: bool = false" "$PHYSICAL_DIR/storage_physical_sd_mmc_native_driver.rs"
require_text "IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE: bool = false" "$PHYSICAL_DIR/storage_physical_sd_mmc_native_driver.rs"
require_text "storage_fat_algorithm_full_migration=ok" "$PHYSICAL_DIR/storage_fat_algorithm_native_driver.rs"
require_text "PULP_FAT_ALGORITHM_FALLBACK_ENABLED: bool = false" "$PHYSICAL_DIR/storage_fat_algorithm_native_driver.rs"
require_text "IMPORTED_PULP_FAT_RUNTIME_ACTIVE: bool = false" "$PHYSICAL_DIR/storage_fat_algorithm_native_driver.rs"
require_text "input_physical_sampling_native_driver=ok" "$PHYSICAL_DIR/input_physical_sampling_native_driver.rs"
require_text "RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK: bool = true" "$PHYSICAL_DIR/input_physical_sampling_native_driver.rs"

require_text "Hardware Physical Full Migration Consolidation" "$DOC"
require_text "VaachakNativeSpiPhysicalDriver" "$DOC"
require_text "VaachakNativeSsd1677PhysicalDriver" "$DOC"
require_text "VaachakNativeSdMmcPhysicalDriver" "$DOC"
require_text "VaachakNativeFatAlgorithmDriver" "$DOC"
require_text "hardware_physical_full_migration_consolidation=ok" "$DOC"

reject_text "PulpCompatibility as active SPI" "$MAIN"
reject_text "PulpCompatibility as active display" "$MAIN"
reject_text "PulpCompatibility as active SD" "$MAIN"
reject_text "PulpCompatibility as active FAT" "$MAIN"

if [ "$missing" -ne 0 ]; then
  exit 1
fi

echo "hardware_physical_full_migration_consolidation=ok"
