#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "pulp_hardware_dead_path_removal validation failed: $*" >&2
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

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_dead_path_removal.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/pulp_hardware_dead_path_removal_smoke.rs"
DOC="docs/architecture/pulp-hardware-dead-path-removal.md"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"

require_file "$PHYSICAL"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACTS_MOD"
require_file "target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_dead_path_quarantine.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_reference_deprecation_audit.rs"
require_file "target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs"

require_text "$PHYSICAL_MOD" "pub mod pulp_hardware_dead_path_removal;"
require_text "$CONTRACTS_MOD" "pub mod pulp_hardware_dead_path_removal_smoke;"

require_text "$PHYSICAL" "pulp_hardware_dead_path_removal=ok"
require_text "$PHYSICAL" "VaachakPulpHardwareDeadPathRemoval"
require_text "$PHYSICAL" "VaachakPulpHardwareDeadPathQuarantine"
require_text "$PHYSICAL" "quarantine_ok"
require_text "$PHYSICAL" "VaachakHardwarePhysicalFullMigrationConsolidation"
require_text "$PHYSICAL" "consolidation_ok"
require_text "$PHYSICAL" "DEAD_LEGACY_PULP_HARDWARE_RUNTIME_PATHS_REMOVED"
require_text "$PHYSICAL" "ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING"
require_text "$PHYSICAL" "UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE"
require_text "$PHYSICAL" "VENDOR_PULP_OS_REMOVED"
require_text "$PHYSICAL" "RemovedFromVaachakHardwareIntegration"
require_text "$PHYSICAL" "KeptRequiredRuntimeDependency"
require_text "$PHYSICAL" "KeptCompatibilityImportBoundary"
require_text "$PHYSICAL" "KeptDocumentationOnlyReference"
require_text "$PHYSICAL" "RemovedGeneratedOverlayScaffoldArtifact"
require_text "$PHYSICAL" "runtime_behavior_changed: false"
require_text "$PHYSICAL" "compatibility_boundary_removed: false"
require_text "$PHYSICAL" "vendor_tree_removed: false"
require_text "$SMOKE" "VaachakPulpHardwareDeadPathRemoval::removal_ok()"
require_text "$DOC" "vendor/pulp-os remains present"
require_text "$DOC" "VaachakNativeSpiPhysicalDriver"
require_text "$DOC" "VaachakNativeSsd1677PhysicalDriver"
require_text "$DOC" "VaachakNativeSdMmcPhysicalDriver"
require_text "$DOC" "VaachakNativeFatAlgorithmDriver"

[[ -d vendor/pulp-os ]] || fail "vendor/pulp-os must remain present"

require_text "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs" "PULP_SPI_TRANSFER_FALLBACK_ENABLED"
require_text "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs" "false"
require_text "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs" "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED"
require_text "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs" "false"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs" "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs" "false"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs" "PULP_FAT_ALGORITHM_FALLBACK_ENABLED"
require_text "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs" "false"

# This removal checkpoint must not edit app or reader/file-browser UX files.
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  changed_app_files=$(git status --short -- src/apps vendor/pulp-os 2>/dev/null || true)
  if [[ -n "$changed_app_files" ]]; then
    fail "unexpected changes under src/apps or vendor/pulp-os: $changed_app_files"
  fi
fi

# The removal checkpoint must not introduce a new active Pulp hardware fallback token.
if grep -R "ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING: bool = true\|UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE: bool = true\|VENDOR_PULP_OS_REMOVED: bool = true" \
  target-xteink-x4/src/vaachak_x4/physical >/dev/null 2>&1; then
  fail "active/unclassified Pulp hardware fallback or vendor removal flag detected"
fi

echo "pulp_hardware_dead_path_removal=ok"
