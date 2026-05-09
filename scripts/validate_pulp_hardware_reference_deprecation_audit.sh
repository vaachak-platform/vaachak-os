#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "pulp_hardware_reference_deprecation_audit validation failed: $*" >&2
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

AUDIT="target-xteink-x4/src/vaachak_x4/physical/pulp_hardware_reference_deprecation_audit.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/pulp_hardware_reference_deprecation_audit_smoke.rs"
DOC="docs/architecture/pulp-hardware-reference-deprecation-audit.md"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
CONSOLIDATION="target-xteink-x4/src/vaachak_x4/physical/hardware_physical_full_migration_consolidation.rs"

require_file "$AUDIT"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACTS_MOD"
require_file "$CONSOLIDATION"
[ -d "vendor/pulp-os" ] || fail "vendor/pulp-os must remain present for this audit checkpoint"

require_text "$PHYSICAL_MOD" "pub mod pulp_hardware_reference_deprecation_audit;"
require_text "$CONTRACTS_MOD" "pub mod pulp_hardware_reference_deprecation_audit_smoke;"

require_text "$AUDIT" "VaachakPulpHardwareReferenceDeprecationAudit"
require_text "$AUDIT" "VaachakPulpReferenceClassification"
require_text "$AUDIT" "StillRequiredRuntimeDependency"
require_text "$AUDIT" "CompatibilityImportBoundary"
require_text "$AUDIT" "DeadLegacyHardwarePath"
require_text "$AUDIT" "DocumentationOnlyReference"
require_text "$AUDIT" "SafeToRemoveOverlayScaffoldArtifact"
require_text "$AUDIT" "VENDOR_PULP_OS_REMOVAL_DEFERRED: bool = true"
require_text "$AUDIT" "unclassified_pulp_hardware_fallback_active: false"
require_text "$AUDIT" "vendor_pulp_os_removed: false"
require_text "$AUDIT" "VaachakNativeSpiPhysicalDriver"
require_text "$AUDIT" "VaachakNativeSsd1677PhysicalDriver"
require_text "$AUDIT" "VaachakNativeSdMmcPhysicalDriver"
require_text "$AUDIT" "VaachakNativeFatAlgorithmDriver"
require_text "$AUDIT" "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"
require_text "$AUDIT" "target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
require_text "$AUDIT" "vendor/pulp-os"
require_text "$AUDIT" "audit_ok()"
require_text "$AUDIT" "report().ok()"
require_absent_text "$AUDIT" "remove_dir_all"
require_absent_text "$AUDIT" "std::fs::remove"

require_text "$SMOKE" "VaachakPulpHardwareReferenceDeprecationAuditSmoke"
require_text "$SMOKE" "smoke_ok()"
require_text "$SMOKE" "pulp_hardware_reference_deprecation_audit=ok"

require_text "$DOC" "Pulp Hardware Reference Deprecation Audit"
require_text "$DOC" "vendor/pulp-os is not removed"
require_text "$DOC" "VaachakNativeSpiPhysicalDriver"
require_text "$DOC" "VaachakNativeSsd1677PhysicalDriver"
require_text "$DOC" "VaachakNativeSdMmcPhysicalDriver"
require_text "$DOC" "VaachakNativeFatAlgorithmDriver"
require_text "$DOC" "StillRequiredRuntimeDependency"
require_text "$DOC" "CompatibilityImportBoundary"
require_text "$DOC" "DeadLegacyHardwarePath"
require_text "$DOC" "DocumentationOnlyReference"
require_text "$DOC" "SafeToRemoveOverlayScaffoldArtifact"

require_text "$CONSOLIDATION" "VaachakNativeSpiPhysicalDriver"
require_text "$CONSOLIDATION" "VaachakNativeSsd1677PhysicalDriver"
require_text "$CONSOLIDATION" "VaachakNativeSdMmcPhysicalDriver"
require_text "$CONSOLIDATION" "VaachakNativeFatAlgorithmDriver"
require_text "$CONSOLIDATION" "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"
require_text "$CONSOLIDATION" "imported_pulp_spi_runtime_active"
require_text "$CONSOLIDATION" "imported_pulp_display_runtime_active"
require_text "$CONSOLIDATION" "imported_pulp_sd_mmc_runtime_active"
require_text "$CONSOLIDATION" "imported_pulp_fat_runtime_active"

echo "pulp_hardware_reference_deprecation_audit=ok"
