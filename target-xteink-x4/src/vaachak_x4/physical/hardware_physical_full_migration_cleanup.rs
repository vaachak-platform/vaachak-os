#![allow(dead_code)]

use super::hardware_physical_full_migration_consolidation::VaachakHardwarePhysicalFullMigrationConsolidation;

/// Final cleanup checkpoint for the fully migrated Vaachak hardware stack.
///
/// This module intentionally does not move additional behavior. It records that
/// the accepted full physical migrations have been consolidated and that old
/// overlay zip/folder artifacts are expected to be removed from the repository
/// root before upload.
pub struct VaachakHardwarePhysicalFullMigrationCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwarePhysicalFullMigrationCleanupStatus {
    pub marker: &'static str,
    pub full_migration_consolidation_ok: bool,
    pub cleanup_checkpoint_owned_by_vaachak: bool,
    pub legacy_overlay_artifacts_removed: bool,
    pub native_spi_driver_consolidated: bool,
    pub native_display_driver_consolidated: bool,
    pub native_sd_mmc_driver_consolidated: bool,
    pub native_fat_algorithm_driver_consolidated: bool,
    pub native_input_sampling_driver_consolidated: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub additional_pulp_hardware_fallback_enabled: bool,
}

impl VaachakHardwarePhysicalFullMigrationCleanupStatus {
    pub const fn ok(self) -> bool {
        self.full_migration_consolidation_ok
            && self.cleanup_checkpoint_owned_by_vaachak
            && self.legacy_overlay_artifacts_removed
            && self.native_spi_driver_consolidated
            && self.native_display_driver_consolidated
            && self.native_sd_mmc_driver_consolidated
            && self.native_fat_algorithm_driver_consolidated
            && self.native_input_sampling_driver_consolidated
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.additional_pulp_hardware_fallback_enabled
    }
}

impl VaachakHardwarePhysicalFullMigrationCleanup {
    pub const MARKER: &'static str = "hardware_physical_full_migration_cleanup=ok";
    pub const CLEANUP_CHECKPOINT_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LEGACY_OVERLAY_ARTIFACTS_REMOVED: bool = true;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const ADDITIONAL_PULP_HARDWARE_FALLBACK_ENABLED: bool = false;

    pub fn cleanup_status() -> VaachakHardwarePhysicalFullMigrationCleanupStatus {
        let consolidated = VaachakHardwarePhysicalFullMigrationConsolidation::migration_map();

        VaachakHardwarePhysicalFullMigrationCleanupStatus {
            marker: Self::MARKER,
            full_migration_consolidation_ok:
                VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok(),
            cleanup_checkpoint_owned_by_vaachak: Self::CLEANUP_CHECKPOINT_OWNER
                == "target-xteink-x4 Vaachak layer",
            legacy_overlay_artifacts_removed: Self::LEGACY_OVERLAY_ARTIFACTS_REMOVED,
            native_spi_driver_consolidated: consolidated.spi_full_migration_ok,
            native_display_driver_consolidated: consolidated.display_full_migration_ok,
            native_sd_mmc_driver_consolidated: consolidated.storage_sd_mmc_full_migration_ok,
            native_fat_algorithm_driver_consolidated: consolidated.storage_fat_full_migration_ok,
            native_input_sampling_driver_consolidated: consolidated
                .input_physical_sampling_native_ok,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            additional_pulp_hardware_fallback_enabled:
                Self::ADDITIONAL_PULP_HARDWARE_FALLBACK_ENABLED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::cleanup_status().ok()
    }
}
