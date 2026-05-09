#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "vendor_pulp_os_scope_reduction validation failed: $*" >&2
  exit 1
}

require_file() {
  [[ -f "$1" ]] || fail "missing file $1"
}

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    fail "unexpected text '$text' in $file"
  fi
}

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical/vendor_pulp_os_scope_reduction.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/vendor_pulp_os_scope_reduction_smoke.rs"
DOC="docs/architecture/vendor-pulp-os-scope-reduction.md"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"

require_file "$PHYSICAL"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACTS_MOD"
require_file "target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_dead_path_removal.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_reference_deprecation_audit.rs"

require_text "$PHYSICAL_MOD" "pub mod vendor_pulp_os_scope_reduction;"
require_text "$CONTRACTS_MOD" "pub mod vendor_pulp_os_scope_reduction_smoke;"

require_text "$PHYSICAL" "vendor_pulp_os_scope_reduction=ok"
require_text "$PHYSICAL" "VaachakVendorPulpOsScopeReduction"
require_text "$PHYSICAL" "VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok()"
require_text "$PHYSICAL" "VaachakPulpHardwareDeadPathRemoval::removal_ok()"
require_text "$PHYSICAL" "VENDOR_PULP_OS_PRESENT: bool = true"
require_text "$PHYSICAL" "VENDOR_PULP_OS_REMOVED: bool = false"
require_text "$PHYSICAL" "ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING: bool = false"
require_text "$PHYSICAL" "UNCLASSIFIED_VENDOR_PULP_HARDWARE_SURFACE_REMAINING: bool = false"
require_text "$PHYSICAL" "ImportedReaderRuntimeCompatibility"
require_text "$PHYSICAL" "HistoricalArchitectureDocumentation"
require_text "$PHYSICAL" "NonHardwareRuntimeDependency"
require_text "$PHYSICAL" "SpiHardwareRuntime"
require_text "$PHYSICAL" "DisplayHardwareRuntime"
require_text "$PHYSICAL" "StorageSdMmcHardwareRuntime"
require_text "$PHYSICAL" "StorageFatHardwareRuntime"
require_text "$PHYSICAL" "InputHardwareRuntime"
require_text "$PHYSICAL" "GeneratedOverlayScaffoldArtifact"
require_text "$PHYSICAL" "VaachakNativeSpiPhysicalDriver"
require_text "$PHYSICAL" "VaachakNativeSsd1677PhysicalDriver"
require_text "$PHYSICAL" "VaachakNativeSdMmcPhysicalDriver"
require_text "$PHYSICAL" "VaachakNativeFatAlgorithmDriver"
require_text "$PHYSICAL" "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"
require_text "$PHYSICAL" "hardware_runtime_allowed: false"
require_text "$PHYSICAL" "active_pulp_fallback_allowed: false"
require_text "$PHYSICAL" "vendor_tree_removed: false"
require_text "$PHYSICAL" "app_behavior_changed: false"
require_text "$SMOKE" "VaachakVendorPulpOsScopeReduction::scope_reduction_ok()"
require_text "$DOC" "vendor_pulp_os_scope_reduction=ok"
require_text "$DOC" "vendor/pulp-os remains present"
require_text "$DOC" "VaachakNativeSpiPhysicalDriver"
require_text "$DOC" "VaachakNativeSsd1677PhysicalDriver"
require_text "$DOC" "VaachakNativeSdMmcPhysicalDriver"
require_text "$DOC" "VaachakNativeFatAlgorithmDriver"

[[ -d vendor/pulp-os ]] || fail "vendor/pulp-os must remain present for scope reduction"

require_text "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs" "PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs" "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs" "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs" "PULP_FAT_ALGORITHM_FALLBACK_ENABLED: bool = false"
require_text "target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs" "VaachakNativeSpiPhysicalDriver"

# This scope-reduction checkpoint must not edit app/runtime UX files or vendor/pulp-os.
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  changed_forbidden=$(git status --short -- src/apps vendor/pulp-os 2>/dev/null || true)
  if [[ -n "$changed_forbidden" ]]; then
    fail "unexpected changes under src/apps or vendor/pulp-os: $changed_forbidden"
  fi
fi

if grep -R "ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING: bool = true\|UNCLASSIFIED_VENDOR_PULP_HARDWARE_SURFACE_REMAINING: bool = true\|VENDOR_PULP_OS_REMOVED: bool = true" \
  target-xteink-x4/src/vaachak_x4/physical >/dev/null 2>&1; then
  fail "active/unclassified Pulp hardware fallback or vendor removal flag detected"
fi

echo "vendor_pulp_os_scope_reduction=ok"
