#![allow(dead_code)]

use super::hardware_physical_full_migration_consolidation::{
    VaachakHardwarePhysicalFullMigrationConsolidation, VaachakHardwarePhysicalMigrationMap,
};

/// Classification audit for remaining Pulp references after the full Vaachak
/// hardware migration.
///
/// This module is intentionally audit-only. It does not delete `vendor/pulp-os`,
/// does not change runtime dispatch, and does not alter reader/file-browser,
/// app navigation, display, input, SPI, SD/MMC, or FAT behavior. It records the
/// remaining Pulp reference classes so the next step can quarantine or remove
/// dead hardware paths safely.
pub struct VaachakPulpHardwareReferenceDeprecationAudit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPulpReferenceClassification {
    StillRequiredRuntimeDependency,
    CompatibilityImportBoundary,
    DeadLegacyHardwarePath,
    DocumentationOnlyReference,
    SafeToRemoveOverlayScaffoldArtifact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPulpReferenceScope {
    VendorPulpOsTree,
    ImportedPulpReaderRuntimeBoundary,
    HistoricalHardwareFallbackConstants,
    ArchitectureDocs,
    GeneratedOverlayArtifacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPulpReferenceAuditEntry {
    pub scope: VaachakPulpReferenceScope,
    pub classification: VaachakPulpReferenceClassification,
    pub path_hint: &'static str,
    pub action: &'static str,
    pub hardware_runtime_active: bool,
    pub classified: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPulpHardwareReferenceAuditReport {
    pub marker: &'static str,
    pub migration_map_ok: bool,
    pub spi_native_backend_selected: bool,
    pub display_native_backend_selected: bool,
    pub storage_sd_mmc_native_backend_selected: bool,
    pub storage_fat_native_backend_selected: bool,
    pub input_sampling_native_backend_selected: bool,
    pub vendor_pulp_os_kept: bool,
    pub vendor_pulp_os_removed: bool,
    pub remaining_pulp_references_classified: bool,
    pub unclassified_pulp_hardware_fallback_active: bool,
    pub app_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub display_input_storage_behavior_changed: bool,
}

impl VaachakPulpReferenceAuditEntry {
    pub const fn ok(self) -> bool {
        self.classified
            && !self.hardware_runtime_active
            && !matches!(
                self.classification,
                VaachakPulpReferenceClassification::DeadLegacyHardwarePath
            )
    }

    pub const fn classified_ok(self) -> bool {
        self.classified && !self.hardware_runtime_active
    }
}

impl VaachakPulpHardwareReferenceAuditReport {
    pub const fn ok(self) -> bool {
        self.migration_map_ok
            && self.spi_native_backend_selected
            && self.display_native_backend_selected
            && self.storage_sd_mmc_native_backend_selected
            && self.storage_fat_native_backend_selected
            && self.input_sampling_native_backend_selected
            && self.vendor_pulp_os_kept
            && !self.vendor_pulp_os_removed
            && self.remaining_pulp_references_classified
            && !self.unclassified_pulp_hardware_fallback_active
            && !self.app_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.display_input_storage_behavior_changed
    }
}

impl VaachakPulpHardwareReferenceDeprecationAudit {
    pub const MARKER: &'static str = "pulp_hardware_reference_deprecation_audit=ok";
    pub const AUDIT_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const VENDOR_PULP_OS_PATH: &'static str = "vendor/pulp-os";
    pub const VENDOR_PULP_OS_REMOVAL_DEFERRED: bool = true;
    pub const APP_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const DISPLAY_INPUT_STORAGE_BEHAVIOR_CHANGED: bool = false;

    pub const AUDIT_ENTRIES: [VaachakPulpReferenceAuditEntry; 5] = [
        VaachakPulpReferenceAuditEntry {
            scope: VaachakPulpReferenceScope::VendorPulpOsTree,
            classification: VaachakPulpReferenceClassification::StillRequiredRuntimeDependency,
            path_hint: "vendor/pulp-os",
            action: "keep until non-hardware Pulp runtime dependencies are separated",
            hardware_runtime_active: false,
            classified: true,
        },
        VaachakPulpReferenceAuditEntry {
            scope: VaachakPulpReferenceScope::ImportedPulpReaderRuntimeBoundary,
            classification: VaachakPulpReferenceClassification::CompatibilityImportBoundary,
            path_hint: "target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs",
            action: "keep as reader/runtime compatibility boundary until reader app migration",
            hardware_runtime_active: false,
            classified: true,
        },
        VaachakPulpReferenceAuditEntry {
            scope: VaachakPulpReferenceScope::HistoricalHardwareFallbackConstants,
            classification: VaachakPulpReferenceClassification::DeadLegacyHardwarePath,
            path_hint: "inactive PULP_*_FALLBACK_ENABLED / IMPORTED_PULP_*_RUNTIME_ACTIVE constants",
            action: "quarantine before removal; active values must remain false",
            hardware_runtime_active: false,
            classified: true,
        },
        VaachakPulpReferenceAuditEntry {
            scope: VaachakPulpReferenceScope::ArchitectureDocs,
            classification: VaachakPulpReferenceClassification::DocumentationOnlyReference,
            path_hint: "docs/architecture",
            action: "retain only where documenting migration history and deprecation state",
            hardware_runtime_active: false,
            classified: true,
        },
        VaachakPulpReferenceAuditEntry {
            scope: VaachakPulpReferenceScope::GeneratedOverlayArtifacts,
            classification: VaachakPulpReferenceClassification::SafeToRemoveOverlayScaffoldArtifact,
            path_hint: "*_overlay folders, deliverable zip files, validator-fix folders",
            action: "remove generated artifacts only; never remove repo source/docs/scripts",
            hardware_runtime_active: false,
            classified: true,
        },
    ];

    pub fn migration_map() -> VaachakHardwarePhysicalMigrationMap {
        VaachakHardwarePhysicalFullMigrationConsolidation::migration_map()
    }

    pub fn all_references_classified() -> bool {
        Self::AUDIT_ENTRIES
            .iter()
            .all(|entry| entry.classified_ok())
    }

    pub fn report() -> VaachakPulpHardwareReferenceAuditReport {
        let map = Self::migration_map();
        VaachakPulpHardwareReferenceAuditReport {
            marker: Self::MARKER,
            migration_map_ok: map.ok()
                && VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok(),
            spi_native_backend_selected: map.spi_full_migration_ok
                && map.spi_backend_name == "VaachakNativeSpiPhysicalDriver"
                && !map.imported_pulp_spi_runtime_active,
            display_native_backend_selected: map.display_full_migration_ok
                && map.display_backend_name == "VaachakNativeSsd1677PhysicalDriver"
                && !map.imported_pulp_display_runtime_active,
            storage_sd_mmc_native_backend_selected: map.storage_sd_mmc_full_migration_ok
                && map.storage_sd_mmc_backend_name == "VaachakNativeSdMmcPhysicalDriver"
                && !map.imported_pulp_sd_mmc_runtime_active,
            storage_fat_native_backend_selected: map.storage_fat_full_migration_ok
                && map.storage_fat_backend_name == "VaachakNativeFatAlgorithmDriver"
                && !map.imported_pulp_fat_runtime_active,
            input_sampling_native_backend_selected: map.input_physical_sampling_native_ok
                && map.input_sampling_backend_name
                    == "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"
                && map.input_adc_gpio_read_fallback_remains,
            vendor_pulp_os_kept: Self::VENDOR_PULP_OS_REMOVAL_DEFERRED,
            vendor_pulp_os_removed: false,
            remaining_pulp_references_classified: Self::all_references_classified(),
            unclassified_pulp_hardware_fallback_active: false,
            app_behavior_changed: Self::APP_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            display_input_storage_behavior_changed: Self::DISPLAY_INPUT_STORAGE_BEHAVIOR_CHANGED,
        }
    }

    pub fn audit_ok() -> bool {
        Self::report().ok()
    }
}
