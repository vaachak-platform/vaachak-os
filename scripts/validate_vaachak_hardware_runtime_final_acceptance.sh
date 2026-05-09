#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "vaachak_hardware_runtime_final_acceptance validation failed: $*" >&2
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

require_not_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    fail "forbidden text '$text' found in $file"
  fi
}

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical/vaachak_hardware_runtime_final_acceptance.rs"
CONTRACT="target-xteink-x4/src/vaachak_x4/contracts/vaachak_hardware_runtime_final_acceptance_smoke.rs"
DOC="docs/architecture/vaachak-hardware-runtime-final-acceptance.md"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACT_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"

require_file "$PHYSICAL"
require_file "$CONTRACT"
require_file "$DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACT_MOD"
[ -d "vendor/pulp-os" ] || fail "missing directory vendor/pulp-os"

require_text "$PHYSICAL_MOD" "pub mod vaachak_hardware_runtime_final_acceptance;"
require_text "$CONTRACT_MOD" "pub mod vaachak_hardware_runtime_final_acceptance_smoke;"

require_text "$PHYSICAL" "VaachakHardwareRuntimeFinalAcceptance"
require_text "$PHYSICAL" "vaachak_hardware_runtime_final_acceptance=ok"
require_text "$PHYSICAL" "VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok()"
require_text "$PHYSICAL" "VaachakHardwarePhysicalFullMigrationCleanup::cleanup_ok()"
require_text "$PHYSICAL" "VaachakPulpHardwareReferenceDeprecationAudit::audit_ok()"
require_text "$PHYSICAL" "VaachakPulpHardwareDeadPathQuarantine::quarantine_ok()"
require_text "$PHYSICAL" "VaachakPulpHardwareDeadPathRemoval::removal_ok()"
require_text "$PHYSICAL" "VaachakVendorPulpOsScopeReduction::scope_reduction_ok()"
require_text "$PHYSICAL" "ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING"
require_text "$PHYSICAL" "UNCLASSIFIED_VENDOR_PULP_HARDWARE_SURFACE_REMAINING"
require_text "$PHYSICAL" "DEVICE_SMOKE_REQUIRED_AFTER_FLASH"
require_text "$CONTRACT" "VaachakHardwareRuntimeFinalAcceptanceSmoke"
require_text "$DOC" "Vaachak Hardware Runtime Final Acceptance"
require_text "$DOC" "remains present"
require_text "$DOC" "Final hardware smoke"

# Verify final native migration source files still exist.
require_file "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/input_physical_sampling_native_driver.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_cleanup.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/vendor_pulp_os_scope_reduction.rs"

require_text "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs" "VaachakNativeSpiPhysicalDriver"
require_text "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs" "PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs" "VaachakNativeSsd1677PhysicalDriver"
require_text "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs" "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs" "VaachakNativeSdMmcPhysicalDriver"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs" "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs" "VaachakNativeFatAlgorithmDriver"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs" "PULP_FAT_ALGORITHM_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/input_physical_sampling_native_driver.rs" "VaachakInputPhysicalSamplingNativeDriver"

# Run the accepted checkpoint validators if they are available. These provide the
# load-bearing proof that this final acceptance gate is not bypassing previous gates.
for script in \
  scripts/validate_hardware_physical_full_migration_consolidation.sh \
  scripts/validate_pulp_hardware_reference_deprecation_audit.sh \
  scripts/validate_pulp_hardware_dead_path_quarantine.sh \
  scripts/validate_pulp_hardware_dead_path_removal.sh \
  scripts/validate_vendor_pulp_os_scope_reduction.sh
  do
    [ -x "$script" ] || fail "missing executable validator $script"
    "$script" >/tmp/vaachak-final-acceptance-$(basename "$script").log
  done

# Ensure the final acceptance source itself does not try to re-enable any Pulp hardware fallback.
require_not_text "$PHYSICAL" "FALLBACK_ENABLED = true"
require_not_text "$PHYSICAL" "IMPORTED_PULP_SPI_RUNTIME_ACTIVE = true"
require_not_text "$PHYSICAL" "IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE = true"
require_not_text "$PHYSICAL" "IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE = true"
require_not_text "$PHYSICAL" "IMPORTED_PULP_FAT_RUNTIME_ACTIVE = true"

echo "vaachak_hardware_runtime_final_acceptance=ok"
