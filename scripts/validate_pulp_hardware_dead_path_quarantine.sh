#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "pulp_hardware_dead_path_quarantine validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local path="$1"
  local text="$2"
  grep -Fq "$text" "$path" || fail "missing text '$text' in $path"
}

require_absent_text() {
  local path="$1"
  local text="$2"
  if grep -Fq "$text" "$path"; then
    fail "forbidden text '$text' in $path"
  fi
}

QUARANTINE="target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_dead_path_quarantine.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/pulp_hardware_dead_path_quarantine_smoke.rs"
DOC="docs/architecture/pulp-hardware-dead-path-quarantine.md"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
AUDIT="target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_reference_deprecation_audit.rs"
CONSOLIDATION="target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs"

require_file "$QUARANTINE"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACTS_MOD"
require_file "$AUDIT"
require_file "$CONSOLIDATION"
[ -d "vendor/pulp-os" ] || fail "vendor/pulp-os must remain present during quarantine"

require_text "$PHYSICAL_MOD" "pub mod pulp_hardware_dead_path_quarantine;"
require_text "$CONTRACTS_MOD" "pub mod pulp_hardware_dead_path_quarantine_smoke;"

require_text "$QUARANTINE" "VaachakPulpHardwareDeadPathQuarantine"
require_text "$QUARANTINE" "VaachakPulpHardwareQuarantineDisposition"
require_text "$QUARANTINE" "KeepRequiredRuntimeDependency"
require_text "$QUARANTINE" "KeepCompatibilityImportBoundary"
require_text "$QUARANTINE" "QuarantineDeadLegacyHardwarePath"
require_text "$QUARANTINE" "KeepDocumentationOnlyReference"
require_text "$QUARANTINE" "RemoveGeneratedOverlayScaffoldArtifact"
require_text "$QUARANTINE" "VaachakPulpHardwareReferenceDeprecationAudit::audit_ok()"
require_text "$QUARANTINE" "VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok()"
require_text "$QUARANTINE" "dead_legacy_hardware_paths_quarantined"
require_text "$QUARANTINE" "quarantined_hardware_paths_runtime_inactive"
require_text "$QUARANTINE" "UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE: bool = false"
require_text "$QUARANTINE" "VENDOR_PULP_OS_REMOVAL_DEFERRED: bool = true"
require_text "$QUARANTINE" "vendor_pulp_os_removed: false"
require_text "$QUARANTINE" "runtime_hardware_active: false"
require_text "$QUARANTINE" "deletion_performed: false"
require_text "$QUARANTINE" "inactive PULP_*_FALLBACK_ENABLED"
require_text "$QUARANTINE" "quarantine_ok()"
require_absent_text "$QUARANTINE" "remove_dir_all"
require_absent_text "$QUARANTINE" "std::fs::remove"
require_absent_text "$QUARANTINE" "IMPORTED_PULP_SPI_RUNTIME_ACTIVE = true"
require_absent_text "$QUARANTINE" "PULP_SPI_TRANSFER_FALLBACK_ENABLED = true"
require_absent_text "$QUARANTINE" "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED = true"
require_absent_text "$QUARANTINE" "PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED = true"
require_absent_text "$QUARANTINE" "PULP_FAT_ALGORITHM_FALLBACK_ENABLED = true"

require_text "$SMOKE" "VaachakPulpHardwareDeadPathQuarantineSmoke"
require_text "$SMOKE" "smoke_ok()"
require_text "$SMOKE" "pulp_hardware_dead_path_quarantine=ok"
require_text "$SMOKE" "QuarantineDeadLegacyHardwarePath"

require_text "$DOC" "Pulp Hardware Dead Path Quarantine"
require_text "$DOC" "vendor/pulp-os is not removed"
require_text "$DOC" "DeadLegacyHardwarePath"
require_text "$DOC" "QuarantineDeadLegacyHardwarePath"
require_text "$DOC" "no unclassified Pulp hardware path is active"

require_text "$AUDIT" "pulp_hardware_reference_deprecation_audit=ok"
require_text "$AUDIT" "DeadLegacyHardwarePath"
require_text "$AUDIT" "unclassified_pulp_hardware_fallback_active: false"

require_text "$CONSOLIDATION" "VaachakNativeSpiPhysicalDriver"
require_text "$CONSOLIDATION" "VaachakNativeSsd1677PhysicalDriver"
require_text "$CONSOLIDATION" "VaachakNativeSdMmcPhysicalDriver"
require_text "$CONSOLIDATION" "VaachakNativeFatAlgorithmDriver"
require_text "$CONSOLIDATION" "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"

echo "pulp_hardware_dead_path_quarantine=ok"
