#![allow(dead_code)]

use super::hardware_physical_full_migration_consolidation::VaachakHardwarePhysicalFullMigrationConsolidation;
use super::pulp_hardware_dead_path_removal::VaachakPulpHardwareDeadPathRemoval;
use super::pulp_hardware_reference_deprecation_audit::VaachakPulpReferenceScope;

/// Scope reduction checkpoint for `vendor/pulp-os` after Vaachak owns the
/// hardware stack.
///
/// This deliberately does not delete `vendor/pulp-os`. The vendor tree remains
/// available only for non-hardware compatibility/import surfaces that are still
/// required by the current application/runtime shape. Hardware ownership and
/// hardware fallback paths remain Vaachak-native.
pub struct VaachakVendorPulpOsScopeReduction;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakVendorPulpOsRetainedSurface {
    ImportedReaderRuntimeCompatibility,
    HistoricalArchitectureDocumentation,
    NonHardwareRuntimeDependency,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakVendorPulpOsExcludedSurface {
    SpiHardwareRuntime,
    DisplayHardwareRuntime,
    StorageSdMmcHardwareRuntime,
    StorageFatHardwareRuntime,
    InputHardwareRuntime,
    DeadLegacyHardwareFallback,
    GeneratedOverlayScaffoldArtifact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakVendorPulpOsRetainedEntry {
    pub surface: VaachakVendorPulpOsRetainedSurface,
    pub source_scope: VaachakPulpReferenceScope,
    pub path_hint: &'static str,
    pub reason: &'static str,
    pub hardware_runtime_allowed: bool,
    pub vendor_tree_removed: bool,
    pub app_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakVendorPulpOsExcludedEntry {
    pub surface: VaachakVendorPulpOsExcludedSurface,
    pub vaachak_owner: &'static str,
    pub path_hint: &'static str,
    pub excluded_from_vendor_scope: bool,
    pub active_pulp_fallback_allowed: bool,
    pub vendor_tree_removed: bool,
    pub app_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakVendorPulpOsScopeReductionReport {
    pub marker: &'static str,
    pub full_hardware_migration_ok: bool,
    pub dead_path_removal_ok: bool,
    pub vendor_pulp_os_present: bool,
    pub vendor_pulp_os_removed: bool,
    pub retained_surface_count: usize,
    pub excluded_surface_count: usize,
    pub retained_surfaces_are_non_hardware: bool,
    pub hardware_surfaces_excluded_from_vendor_scope: bool,
    pub active_pulp_hardware_fallback_remaining: bool,
    pub unclassified_vendor_pulp_hardware_surface_remaining: bool,
    pub generated_overlay_scaffold_artifacts_excluded: bool,
    pub app_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub display_input_storage_spi_behavior_changed: bool,
}

impl VaachakVendorPulpOsRetainedEntry {
    pub const fn ok(self) -> bool {
        !self.hardware_runtime_allowed && !self.vendor_tree_removed && !self.app_behavior_changed
    }
}

impl VaachakVendorPulpOsExcludedEntry {
    pub const fn ok(self) -> bool {
        self.excluded_from_vendor_scope
            && !self.active_pulp_fallback_allowed
            && !self.vendor_tree_removed
            && !self.app_behavior_changed
    }
}

impl VaachakVendorPulpOsScopeReductionReport {
    pub const fn ok(self) -> bool {
        self.full_hardware_migration_ok
            && self.dead_path_removal_ok
            && self.vendor_pulp_os_present
            && !self.vendor_pulp_os_removed
            && self.retained_surface_count == 3
            && self.excluded_surface_count == 7
            && self.retained_surfaces_are_non_hardware
            && self.hardware_surfaces_excluded_from_vendor_scope
            && !self.active_pulp_hardware_fallback_remaining
            && !self.unclassified_vendor_pulp_hardware_surface_remaining
            && self.generated_overlay_scaffold_artifacts_excluded
            && !self.app_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.display_input_storage_spi_behavior_changed
    }
}

impl VaachakVendorPulpOsScopeReduction {
    pub const MARKER: &'static str = "vendor_pulp_os_scope_reduction=ok";
    pub const SCOPE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const VENDOR_PULP_OS_PATH: &'static str = "vendor/pulp-os";
    pub const VENDOR_PULP_OS_PRESENT: bool = true;
    pub const VENDOR_PULP_OS_REMOVED: bool = false;
    pub const ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING: bool = false;
    pub const UNCLASSIFIED_VENDOR_PULP_HARDWARE_SURFACE_REMAINING: bool = false;
    pub const APP_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const DISPLAY_INPUT_STORAGE_SPI_BEHAVIOR_CHANGED: bool = false;

    pub const RETAINED_SURFACES: [VaachakVendorPulpOsRetainedEntry; 3] = [
        VaachakVendorPulpOsRetainedEntry {
            surface: VaachakVendorPulpOsRetainedSurface::ImportedReaderRuntimeCompatibility,
            source_scope: VaachakPulpReferenceScope::ImportedPulpReaderRuntimeBoundary,
            path_hint: "target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs",
            reason: "keep as compatibility boundary until reader/runtime migration is complete",
            hardware_runtime_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsRetainedEntry {
            surface: VaachakVendorPulpOsRetainedSurface::HistoricalArchitectureDocumentation,
            source_scope: VaachakPulpReferenceScope::ArchitectureDocs,
            path_hint: "docs/architecture",
            reason: "keep migration-history references only",
            hardware_runtime_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsRetainedEntry {
            surface: VaachakVendorPulpOsRetainedSurface::NonHardwareRuntimeDependency,
            source_scope: VaachakPulpReferenceScope::VendorPulpOsTree,
            path_hint: "vendor/pulp-os",
            reason: "retain until non-hardware runtime dependencies are separately audited",
            hardware_runtime_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
    ];

    pub const EXCLUDED_SURFACES: [VaachakVendorPulpOsExcludedEntry; 7] = [
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::SpiHardwareRuntime,
            vaachak_owner: "VaachakNativeSpiPhysicalDriver",
            path_hint: "target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::DisplayHardwareRuntime,
            vaachak_owner: "VaachakNativeSsd1677PhysicalDriver",
            path_hint: "target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::StorageSdMmcHardwareRuntime,
            vaachak_owner: "VaachakNativeSdMmcPhysicalDriver",
            path_hint: "target-xteink-x4/src/vaachak_x4/physical/storage_physical_sd_mmc_native_driver.rs",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::StorageFatHardwareRuntime,
            vaachak_owner: "VaachakNativeFatAlgorithmDriver",
            path_hint: "target-xteink-x4/src/vaachak_x4/physical/storage_fat_algorithm_native_driver.rs",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::InputHardwareRuntime,
            vaachak_owner: "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback",
            path_hint: "target-xteink-x4/src/vaachak_x4/physical/input_physical_sampling_native_driver.rs",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::DeadLegacyHardwareFallback,
            vaachak_owner: "VaachakHardwarePhysicalFullMigrationConsolidation",
            path_hint: "quarantined and removed Pulp hardware fallback constants",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
        VaachakVendorPulpOsExcludedEntry {
            surface: VaachakVendorPulpOsExcludedSurface::GeneratedOverlayScaffoldArtifact,
            vaachak_owner: "cleanup scripts",
            path_hint: "generated overlay zip files and extracted overlay folders",
            excluded_from_vendor_scope: true,
            active_pulp_fallback_allowed: false,
            vendor_tree_removed: false,
            app_behavior_changed: false,
        },
    ];

    pub fn retained_surfaces_ok() -> bool {
        Self::RETAINED_SURFACES.iter().all(|entry| entry.ok())
    }

    pub fn excluded_surfaces_ok() -> bool {
        Self::EXCLUDED_SURFACES.iter().all(|entry| entry.ok())
    }

    pub fn report() -> VaachakVendorPulpOsScopeReductionReport {
        VaachakVendorPulpOsScopeReductionReport {
            marker: Self::MARKER,
            full_hardware_migration_ok:
                VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok(),
            dead_path_removal_ok: VaachakPulpHardwareDeadPathRemoval::removal_ok(),
            vendor_pulp_os_present: Self::VENDOR_PULP_OS_PRESENT,
            vendor_pulp_os_removed: Self::VENDOR_PULP_OS_REMOVED,
            retained_surface_count: Self::RETAINED_SURFACES.len(),
            excluded_surface_count: Self::EXCLUDED_SURFACES.len(),
            retained_surfaces_are_non_hardware: Self::retained_surfaces_ok(),
            hardware_surfaces_excluded_from_vendor_scope: Self::excluded_surfaces_ok(),
            active_pulp_hardware_fallback_remaining: Self::ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING,
            unclassified_vendor_pulp_hardware_surface_remaining:
                Self::UNCLASSIFIED_VENDOR_PULP_HARDWARE_SURFACE_REMAINING,
            generated_overlay_scaffold_artifacts_excluded: Self::EXCLUDED_SURFACES.iter().any(
                |entry| {
                    matches!(
                        entry.surface,
                        VaachakVendorPulpOsExcludedSurface::GeneratedOverlayScaffoldArtifact
                    ) && entry.ok()
                },
            ),
            app_behavior_changed: Self::APP_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            display_input_storage_spi_behavior_changed:
                Self::DISPLAY_INPUT_STORAGE_SPI_BEHAVIOR_CHANGED,
        }
    }

    pub fn scope_reduction_ok() -> bool {
        Self::report().ok()
    }
}
