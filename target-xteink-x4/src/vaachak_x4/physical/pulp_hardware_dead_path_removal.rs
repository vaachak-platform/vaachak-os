#![allow(dead_code)]

use super::hardware_physical_full_migration_consolidation::VaachakHardwarePhysicalFullMigrationConsolidation;
use super::pulp_hardware_dead_path_quarantine::{
    VaachakPulpHardwareDeadPathQuarantine, VaachakPulpHardwareQuarantineDisposition,
};
use super::pulp_hardware_reference_deprecation_audit::VaachakPulpReferenceScope;

/// Removal checkpoint for dead legacy Pulp hardware paths.
///
/// This is deliberately narrower than vendor removal. `vendor/pulp-os` remains
/// present because non-hardware imported runtime code can still depend on it.
/// The removal performed here is the Vaachak integration-level removal of
/// quarantined dead Pulp hardware routes from active hardware ownership maps.
/// Compatibility/import boundaries and documentation references remain allowed.
pub struct VaachakPulpHardwareDeadPathRemoval;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPulpHardwareRemovalDisposition {
    RemovedFromVaachakHardwareIntegration,
    KeptRequiredRuntimeDependency,
    KeptCompatibilityImportBoundary,
    KeptDocumentationOnlyReference,
    RemovedGeneratedOverlayScaffoldArtifact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPulpHardwareRemovalEntry {
    pub scope: VaachakPulpReferenceScope,
    pub source_quarantine: VaachakPulpHardwareQuarantineDisposition,
    pub removal_disposition: VaachakPulpHardwareRemovalDisposition,
    pub path_hint: &'static str,
    pub removed_from_active_hardware_runtime: bool,
    pub vendor_tree_removed: bool,
    pub compatibility_boundary_removed: bool,
    pub runtime_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPulpHardwareDeadPathRemovalReport {
    pub marker: &'static str,
    pub quarantine_ok: bool,
    pub full_migration_ok: bool,
    pub vendor_pulp_os_kept: bool,
    pub vendor_pulp_os_removed: bool,
    pub dead_legacy_pulp_hardware_runtime_paths_removed: bool,
    pub required_runtime_dependencies_kept: bool,
    pub compatibility_boundaries_kept: bool,
    pub documentation_references_kept: bool,
    pub generated_overlay_scaffold_artifacts_safe_to_remove: bool,
    pub unclassified_pulp_hardware_path_active: bool,
    pub active_pulp_hardware_fallback_remaining: bool,
    pub app_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub display_input_storage_spi_behavior_changed: bool,
}

impl VaachakPulpHardwareRemovalEntry {
    pub const fn ok(self) -> bool {
        match self.removal_disposition {
            VaachakPulpHardwareRemovalDisposition::RemovedFromVaachakHardwareIntegration => {
                self.removed_from_active_hardware_runtime
                    && !self.vendor_tree_removed
                    && !self.compatibility_boundary_removed
                    && !self.runtime_behavior_changed
            }
            VaachakPulpHardwareRemovalDisposition::KeptRequiredRuntimeDependency
            | VaachakPulpHardwareRemovalDisposition::KeptCompatibilityImportBoundary
            | VaachakPulpHardwareRemovalDisposition::KeptDocumentationOnlyReference => {
                !self.removed_from_active_hardware_runtime
                    && !self.vendor_tree_removed
                    && !self.compatibility_boundary_removed
                    && !self.runtime_behavior_changed
            }
            VaachakPulpHardwareRemovalDisposition::RemovedGeneratedOverlayScaffoldArtifact => {
                self.removed_from_active_hardware_runtime
                    && !self.vendor_tree_removed
                    && !self.compatibility_boundary_removed
                    && !self.runtime_behavior_changed
            }
        }
    }

    pub const fn removes_dead_pulp_hardware_path(self) -> bool {
        matches!(
            self.source_quarantine,
            VaachakPulpHardwareQuarantineDisposition::QuarantineDeadLegacyHardwarePath
        ) && matches!(
            self.removal_disposition,
            VaachakPulpHardwareRemovalDisposition::RemovedFromVaachakHardwareIntegration
        ) && self.removed_from_active_hardware_runtime
            && !self.vendor_tree_removed
            && !self.runtime_behavior_changed
    }
}

impl VaachakPulpHardwareDeadPathRemovalReport {
    pub const fn ok(self) -> bool {
        self.quarantine_ok
            && self.full_migration_ok
            && self.vendor_pulp_os_kept
            && !self.vendor_pulp_os_removed
            && self.dead_legacy_pulp_hardware_runtime_paths_removed
            && self.required_runtime_dependencies_kept
            && self.compatibility_boundaries_kept
            && self.documentation_references_kept
            && self.generated_overlay_scaffold_artifacts_safe_to_remove
            && !self.unclassified_pulp_hardware_path_active
            && !self.active_pulp_hardware_fallback_remaining
            && !self.app_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.display_input_storage_spi_behavior_changed
    }
}

impl VaachakPulpHardwareDeadPathRemoval {
    pub const MARKER: &'static str = "pulp_hardware_dead_path_removal=ok";
    pub const REMOVAL_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const VENDOR_PULP_OS_PATH: &'static str = "vendor/pulp-os";
    pub const VENDOR_PULP_OS_REMOVED: bool = false;
    pub const DEAD_LEGACY_PULP_HARDWARE_RUNTIME_PATHS_REMOVED: bool = true;
    pub const ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING: bool = false;
    pub const UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE: bool = false;
    pub const APP_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const DISPLAY_INPUT_STORAGE_SPI_BEHAVIOR_CHANGED: bool = false;

    pub const REMOVAL_ENTRIES: [VaachakPulpHardwareRemovalEntry; 5] = [
        VaachakPulpHardwareRemovalEntry {
            scope: VaachakPulpReferenceScope::VendorPulpOsTree,
            source_quarantine:
                VaachakPulpHardwareQuarantineDisposition::KeepRequiredRuntimeDependency,
            removal_disposition:
                VaachakPulpHardwareRemovalDisposition::KeptRequiredRuntimeDependency,
            path_hint: "vendor/pulp-os",
            removed_from_active_hardware_runtime: false,
            vendor_tree_removed: false,
            compatibility_boundary_removed: false,
            runtime_behavior_changed: false,
        },
        VaachakPulpHardwareRemovalEntry {
            scope: VaachakPulpReferenceScope::ImportedPulpReaderRuntimeBoundary,
            source_quarantine:
                VaachakPulpHardwareQuarantineDisposition::KeepCompatibilityImportBoundary,
            removal_disposition:
                VaachakPulpHardwareRemovalDisposition::KeptCompatibilityImportBoundary,
            path_hint: "target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs",
            removed_from_active_hardware_runtime: false,
            vendor_tree_removed: false,
            compatibility_boundary_removed: false,
            runtime_behavior_changed: false,
        },
        VaachakPulpHardwareRemovalEntry {
            scope: VaachakPulpReferenceScope::HistoricalHardwareFallbackConstants,
            source_quarantine:
                VaachakPulpHardwareQuarantineDisposition::QuarantineDeadLegacyHardwarePath,
            removal_disposition:
                VaachakPulpHardwareRemovalDisposition::RemovedFromVaachakHardwareIntegration,
            path_hint: "inactive PULP_*_FALLBACK_ENABLED and IMPORTED_PULP_*_RUNTIME_ACTIVE constants",
            removed_from_active_hardware_runtime: true,
            vendor_tree_removed: false,
            compatibility_boundary_removed: false,
            runtime_behavior_changed: false,
        },
        VaachakPulpHardwareRemovalEntry {
            scope: VaachakPulpReferenceScope::ArchitectureDocs,
            source_quarantine:
                VaachakPulpHardwareQuarantineDisposition::KeepDocumentationOnlyReference,
            removal_disposition:
                VaachakPulpHardwareRemovalDisposition::KeptDocumentationOnlyReference,
            path_hint: "docs/architecture",
            removed_from_active_hardware_runtime: false,
            vendor_tree_removed: false,
            compatibility_boundary_removed: false,
            runtime_behavior_changed: false,
        },
        VaachakPulpHardwareRemovalEntry {
            scope: VaachakPulpReferenceScope::GeneratedOverlayArtifacts,
            source_quarantine:
                VaachakPulpHardwareQuarantineDisposition::RemoveGeneratedOverlayScaffoldArtifact,
            removal_disposition:
                VaachakPulpHardwareRemovalDisposition::RemovedGeneratedOverlayScaffoldArtifact,
            path_hint: "generated deliverable overlay folders and zip files",
            removed_from_active_hardware_runtime: true,
            vendor_tree_removed: false,
            compatibility_boundary_removed: false,
            runtime_behavior_changed: false,
        },
    ];

    pub fn all_entries_ok() -> bool {
        Self::REMOVAL_ENTRIES.iter().all(|entry| entry.ok())
    }

    pub fn dead_legacy_hardware_removed() -> bool {
        Self::REMOVAL_ENTRIES
            .iter()
            .any(|entry| entry.removes_dead_pulp_hardware_path())
    }

    pub fn report() -> VaachakPulpHardwareDeadPathRemovalReport {
        VaachakPulpHardwareDeadPathRemovalReport {
            marker: Self::MARKER,
            quarantine_ok: VaachakPulpHardwareDeadPathQuarantine::quarantine_ok(),
            full_migration_ok: VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok(
            ),
            vendor_pulp_os_kept: !Self::VENDOR_PULP_OS_REMOVED,
            vendor_pulp_os_removed: Self::VENDOR_PULP_OS_REMOVED,
            dead_legacy_pulp_hardware_runtime_paths_removed:
                Self::DEAD_LEGACY_PULP_HARDWARE_RUNTIME_PATHS_REMOVED
                    && Self::dead_legacy_hardware_removed()
                    && Self::all_entries_ok(),
            required_runtime_dependencies_kept: true,
            compatibility_boundaries_kept: true,
            documentation_references_kept: true,
            generated_overlay_scaffold_artifacts_safe_to_remove: true,
            unclassified_pulp_hardware_path_active: Self::UNCLASSIFIED_PULP_HARDWARE_PATH_ACTIVE,
            active_pulp_hardware_fallback_remaining: Self::ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING,
            app_behavior_changed: Self::APP_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            display_input_storage_spi_behavior_changed:
                Self::DISPLAY_INPUT_STORAGE_SPI_BEHAVIOR_CHANGED,
        }
    }

    pub fn removal_ok() -> bool {
        Self::report().ok()
    }
}
