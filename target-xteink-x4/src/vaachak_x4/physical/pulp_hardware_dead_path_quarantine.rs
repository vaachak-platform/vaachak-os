#![allow(dead_code)]

use super::hardware_physical_full_migration_consolidation::VaachakHardwarePhysicalFullMigrationConsolidation;
use super::pulp_hardware_reference_deprecation_audit::{
    VaachakPulpHardwareReferenceDeprecationAudit, VaachakPulpReferenceClassification,
    VaachakPulpReferenceScope,
};

/// Quarantine plan for dead legacy Pulp hardware references after full Vaachak
/// hardware migration.
///
/// This module is intentionally non-destructive. It does not remove
/// `vendor/pulp-os`, does not delete imported runtime boundaries, and does not
/// change app, reader/file-browser, display, input, SPI, SD/MMC, or FAT
/// behavior. It marks classified dead hardware references as quarantined so a
/// later removal deliverable can delete only proven-dead paths.
pub struct VaachakPulpHardwareDeadPathQuarantine;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPulpHardwareQuarantineDisposition {
    KeepRequiredRuntimeDependency,
    KeepCompatibilityImportBoundary,
    QuarantineDeadLegacyHardwarePath,
    KeepDocumentationOnlyReference,
    RemoveGeneratedOverlayScaffoldArtifact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPulpHardwareQuarantineEntry {
    pub scope: VaachakPulpReferenceScope,
    pub source_classification: VaachakPulpReferenceClassification,
    pub disposition: VaachakPulpHardwareQuarantineDisposition,
    pub path_hint: &'static str,
    pub quarantine_reason: &'static str,
    pub runtime_hardware_active: bool,
    pub deletion_performed: bool,
    pub vendor_tree_required: bool,
    pub quarantined: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPulpHardwareDeadPathQuarantineReport {
    pub marker: &'static str,
    pub audit_ok: bool,
    pub native_physical_migration_ok: bool,
    pub vendor_pulp_os_kept: bool,
    pub vendor_pulp_os_removed: bool,
    pub dead_legacy_hardware_paths_quarantined: bool,
    pub quarantined_hardware_paths_runtime_inactive: bool,
    pub unclassified_pulp_hardware_path_active: bool,
    pub app_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub display_input_storage_behavior_changed: bool,
    pub spi_storage_display_input_backends_remain_native: bool,
}

impl VaachakPulpHardwareQuarantineEntry {
    pub const fn ok(self) -> bool {
        match self.disposition {
            VaachakPulpHardwareQuarantineDisposition::QuarantineDeadLegacyHardwarePath => {
                self.quarantined && !self.runtime_hardware_active && !self.deletion_performed
            }
            VaachakPulpHardwareQuarantineDisposition::RemoveGeneratedOverlayScaffoldArtifact => {
                self.quarantined && !self.runtime_hardware_active
            }
            VaachakPulpHardwareQuarantineDisposition::KeepRequiredRuntimeDependency
            | VaachakPulpHardwareQuarantineDisposition::KeepCompatibilityImportBoundary
            | VaachakPulpHardwareQuarantineDisposition::KeepDocumentationOnlyReference => {
                !self.runtime_hardware_active && !self.deletion_performed
            }
        }
    }

    pub const fn is_dead_hardware_quarantine(self) -> bool {
        matches!(
            self.disposition,
            VaachakPulpHardwareQuarantineDisposition::QuarantineDeadLegacyHardwarePath
        ) && self.quarantined
            && !self.runtime_hardware_active
            && !self.deletion_performed
    }
}

impl VaachakPulpHardwareDeadPathQuarantineReport {
    pub const fn ok(self) -> bool {
        self.audit_ok
            && self.native_physical_migration_ok
            && self.vendor_pulp_os_kept
            && !self.vendor_pulp_os_removed
            && self.dead_legacy_hardware_paths_quarantined
            && self.quarantined_hardware_paths_runtime_inactive
            && !self.unclassified_pulp_hardware_path_active
            && !self.app_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.display_input_storage_behavior_changed
            && self.spi_storage_display_input_backends_remain_native
    }
}

impl VaachakPulpHardwareDeadPathQuarantine {
    pub const MARKER: &'static str = "pulp_hardware_dead_path_quarantine=ok";
    pub const QUARANTINE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const VENDOR_PULP_OS_PATH: &'static str = "vendor/pulp-os";
    pub const VENDOR_PULP_OS_REMOVAL_DEFERRED: bool = true;
    pub const APP_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const DISPLAY_INPUT_STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE: bool = false;

    pub const QUARANTINE_ENTRIES: [VaachakPulpHardwareQuarantineEntry; 5] = [
        VaachakPulpHardwareQuarantineEntry {
            scope: VaachakPulpReferenceScope::VendorPulpOsTree,
            source_classification:
                VaachakPulpReferenceClassification::StillRequiredRuntimeDependency,
            disposition: VaachakPulpHardwareQuarantineDisposition::KeepRequiredRuntimeDependency,
            path_hint: "vendor/pulp-os",
            quarantine_reason: "vendor tree remains until non-hardware runtime dependencies are separated",
            runtime_hardware_active: false,
            deletion_performed: false,
            vendor_tree_required: true,
            quarantined: false,
        },
        VaachakPulpHardwareQuarantineEntry {
            scope: VaachakPulpReferenceScope::ImportedPulpReaderRuntimeBoundary,
            source_classification: VaachakPulpReferenceClassification::CompatibilityImportBoundary,
            disposition: VaachakPulpHardwareQuarantineDisposition::KeepCompatibilityImportBoundary,
            path_hint: "target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs",
            quarantine_reason: "reader/runtime import boundary is not a hardware executor path",
            runtime_hardware_active: false,
            deletion_performed: false,
            vendor_tree_required: true,
            quarantined: false,
        },
        VaachakPulpHardwareQuarantineEntry {
            scope: VaachakPulpReferenceScope::HistoricalHardwareFallbackConstants,
            source_classification: VaachakPulpReferenceClassification::DeadLegacyHardwarePath,
            disposition: VaachakPulpHardwareQuarantineDisposition::QuarantineDeadLegacyHardwarePath,
            path_hint: "inactive PULP_*_FALLBACK_ENABLED and IMPORTED_PULP_*_RUNTIME_ACTIVE constants",
            quarantine_reason: "native Vaachak hardware backends are selected and Pulp hardware fallback is inactive",
            runtime_hardware_active: false,
            deletion_performed: false,
            vendor_tree_required: false,
            quarantined: true,
        },
        VaachakPulpHardwareQuarantineEntry {
            scope: VaachakPulpReferenceScope::ArchitectureDocs,
            source_classification: VaachakPulpReferenceClassification::DocumentationOnlyReference,
            disposition: VaachakPulpHardwareQuarantineDisposition::KeepDocumentationOnlyReference,
            path_hint: "docs/architecture",
            quarantine_reason: "documentation references are retained only for migration history",
            runtime_hardware_active: false,
            deletion_performed: false,
            vendor_tree_required: false,
            quarantined: false,
        },
        VaachakPulpHardwareQuarantineEntry {
            scope: VaachakPulpReferenceScope::GeneratedOverlayArtifacts,
            source_classification:
                VaachakPulpReferenceClassification::SafeToRemoveOverlayScaffoldArtifact,
            disposition:
                VaachakPulpHardwareQuarantineDisposition::RemoveGeneratedOverlayScaffoldArtifact,
            path_hint: "generated deliverable overlay folders and zip files",
            quarantine_reason: "generated artifacts are safe to remove outside repo source/docs/scripts",
            runtime_hardware_active: false,
            deletion_performed: false,
            vendor_tree_required: false,
            quarantined: true,
        },
    ];

    pub fn all_entries_ok() -> bool {
        Self::QUARANTINE_ENTRIES.iter().all(|entry| entry.ok())
    }

    pub fn dead_hardware_quarantined() -> bool {
        Self::QUARANTINE_ENTRIES
            .iter()
            .any(|entry| entry.is_dead_hardware_quarantine())
    }

    pub fn report() -> VaachakPulpHardwareDeadPathQuarantineReport {
        let migration_map = VaachakHardwarePhysicalFullMigrationConsolidation::migration_map();
        VaachakPulpHardwareDeadPathQuarantineReport {
            marker: Self::MARKER,
            audit_ok: VaachakPulpHardwareReferenceDeprecationAudit::audit_ok(),
            native_physical_migration_ok: migration_map.ok()
                && VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok(),
            vendor_pulp_os_kept: Self::VENDOR_PULP_OS_REMOVAL_DEFERRED,
            vendor_pulp_os_removed: false,
            dead_legacy_hardware_paths_quarantined: Self::dead_hardware_quarantined(),
            quarantined_hardware_paths_runtime_inactive: Self::all_entries_ok(),
            unclassified_pulp_hardware_path_active: Self::UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE,
            app_behavior_changed: Self::APP_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            display_input_storage_behavior_changed: Self::DISPLAY_INPUT_STORAGE_BEHAVIOR_CHANGED,
            spi_storage_display_input_backends_remain_native: migration_map.spi_full_migration_ok
                && migration_map.display_full_migration_ok
                && migration_map.storage_sd_mmc_full_migration_ok
                && migration_map.storage_fat_full_migration_ok
                && migration_map.input_physical_sampling_native_ok,
        }
    }

    pub fn quarantine_ok() -> bool {
        Self::report().ok()
    }
}
