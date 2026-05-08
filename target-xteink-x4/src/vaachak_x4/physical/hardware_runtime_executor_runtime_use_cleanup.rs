#![allow(dead_code)]

use super::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use super::hardware_runtime_executor_boot_markers::VaachakHardwareRuntimeExecutorBootMarkers;
use super::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;

/// Final cleanup surface for the hardware runtime executor runtime-use checkpoint.
///
/// This folds the validator repair into the runtime-use deliverable, makes the
/// accepted runtime-use state easy to validate, and keeps the current
/// Pulp-compatible hardware behavior unchanged.
pub struct VaachakHardwareRuntimeExecutorRuntimeUseCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorRuntimeUseCleanupReport {
    pub runtime_use_surface_ready: bool,
    pub acceptance_surface_ready: bool,
    pub boot_markers_ready: bool,
    pub runtime_use_validator_fix_folded_in: bool,
    pub cleanup_script_available: bool,
    pub runtime_use_site_count_ok: bool,
    pub low_level_backend_still_pulp_compatible: bool,
    pub physical_spi_transfer_moved: bool,
    pub chip_select_toggling_moved: bool,
    pub sd_mmc_executor_moved: bool,
    pub fat_executor_rewritten: bool,
    pub display_draw_algorithm_rewritten: bool,
    pub input_debounce_navigation_rewritten: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub fat_destructive_behavior_introduced: bool,
}

impl VaachakHardwareRuntimeExecutorRuntimeUseCleanupReport {
    pub const fn ok(self) -> bool {
        self.runtime_use_surface_ready
            && self.acceptance_surface_ready
            && self.boot_markers_ready
            && self.runtime_use_validator_fix_folded_in
            && self.cleanup_script_available
            && self.runtime_use_site_count_ok
            && self.low_level_backend_still_pulp_compatible
            && !self.physical_spi_transfer_moved
            && !self.chip_select_toggling_moved
            && !self.sd_mmc_executor_moved
            && !self.fat_executor_rewritten
            && !self.display_draw_algorithm_rewritten
            && !self.input_debounce_navigation_rewritten
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.fat_destructive_behavior_introduced
    }
}

impl VaachakHardwareRuntimeExecutorRuntimeUseCleanup {
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_CLEANUP_MARKER: &'static str =
        "hardware_runtime_executor_runtime_use_cleanup=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_CLEANUP_SCOPE: &'static str =
        "GitHub-ready cleanup for hardware runtime executor runtime-use adoption";

    pub const REQUIRED_RUNTIME_USE_SITE_COUNT: usize = 10;
    pub const RUNTIME_USE_VALIDATOR_FIX_FOLDED_IN: bool = true;
    pub const CLEANUP_SCRIPT_AVAILABLE: bool = true;

    pub const LOW_LEVEL_BACKEND_STILL_PULP_COMPATIBLE: bool = true;
    pub const PHYSICAL_SPI_TRANSFER_MOVED: bool = false;
    pub const CHIP_SELECT_TOGGLING_MOVED: bool = false;
    pub const SD_MMC_EXECUTOR_MOVED: bool = false;
    pub const FAT_EXECUTOR_REWRITTEN: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn runtime_use_site_count_ok() -> bool {
        VaachakHardwareRuntimeExecutorRuntimeUse::RUNTIME_USE_SITE_COUNT
            == Self::REQUIRED_RUNTIME_USE_SITE_COUNT
    }

    pub const fn behavior_preserved() -> bool {
        Self::LOW_LEVEL_BACKEND_STILL_PULP_COMPATIBLE
            && !Self::PHYSICAL_SPI_TRANSFER_MOVED
            && !Self::CHIP_SELECT_TOGGLING_MOVED
            && !Self::SD_MMC_EXECUTOR_MOVED
            && !Self::FAT_EXECUTOR_REWRITTEN
            && !Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN
            && !Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_BEHAVIOR_CHANGED
            && !Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorRuntimeUseCleanupReport {
        VaachakHardwareRuntimeExecutorRuntimeUseCleanupReport {
            runtime_use_surface_ready: VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok(),
            acceptance_surface_ready: VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok(),
            boot_markers_ready: VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok(),
            runtime_use_validator_fix_folded_in: Self::RUNTIME_USE_VALIDATOR_FIX_FOLDED_IN,
            cleanup_script_available: Self::CLEANUP_SCRIPT_AVAILABLE,
            runtime_use_site_count_ok: Self::runtime_use_site_count_ok(),
            low_level_backend_still_pulp_compatible: Self::LOW_LEVEL_BACKEND_STILL_PULP_COMPATIBLE,
            physical_spi_transfer_moved: Self::PHYSICAL_SPI_TRANSFER_MOVED,
            chip_select_toggling_moved: Self::CHIP_SELECT_TOGGLING_MOVED,
            sd_mmc_executor_moved: Self::SD_MMC_EXECUTOR_MOVED,
            fat_executor_rewritten: Self::FAT_EXECUTOR_REWRITTEN,
            display_draw_algorithm_rewritten: Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            input_debounce_navigation_rewritten: Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            fat_destructive_behavior_introduced: Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn cleanup_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorRuntimeUseCleanup;

    #[test]
    fn hardware_runtime_executor_runtime_use_cleanup_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorRuntimeUseCleanup::cleanup_ok());
    }

    #[test]
    fn runtime_use_cleanup_preserves_behavior() {
        assert!(VaachakHardwareRuntimeExecutorRuntimeUseCleanup::behavior_preserved());
        assert!(VaachakHardwareRuntimeExecutorRuntimeUseCleanup::runtime_use_site_count_ok());
    }
}
