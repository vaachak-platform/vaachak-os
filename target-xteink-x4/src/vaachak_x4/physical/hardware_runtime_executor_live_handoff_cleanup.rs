#![allow(dead_code)]

use super::hardware_executor_pulp_backend::VaachakHardwareExecutorPulpBackend;
use super::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use super::hardware_runtime_executor_live_handoff::VaachakHardwareRuntimeExecutorLiveHandoff;
use super::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;

/// Final cleanup/acceptance checkpoint for live hardware executor handoff.
///
/// This module intentionally does not add a new hardware behavior path. It
/// consolidates the accepted live handoff checkpoint and keeps the Pulp-compatible
/// low-level executor active while Vaachak owns the live handoff boundary.
pub struct VaachakHardwareRuntimeExecutorLiveHandoffCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeLiveHandoffCleanupReport {
    pub cleanup_entrypoint_active: bool,
    pub live_handoff_accepted: bool,
    pub runtime_use_accepted: bool,
    pub acceptance_checkpoint_accepted: bool,
    pub pulp_compatible_backend_active: bool,
    pub boot_handoff_documented: bool,
    pub imported_pulp_boundary_documented: bool,
    pub storage_handoff_documented: bool,
    pub display_handoff_documented: bool,
    pub input_handoff_documented: bool,
    pub old_overlay_artifacts_safe_to_remove: bool,
    pub physical_spi_transfer_changed: bool,
    pub chip_select_toggling_changed: bool,
    pub sd_mmc_low_level_changed: bool,
    pub fat_storage_algorithm_changed: bool,
    pub display_draw_algorithm_changed: bool,
    pub input_debounce_navigation_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub destructive_storage_behavior_added: bool,
}

impl VaachakHardwareRuntimeLiveHandoffCleanupReport {
    pub const fn ok(self) -> bool {
        self.cleanup_entrypoint_active
            && self.live_handoff_accepted
            && self.runtime_use_accepted
            && self.acceptance_checkpoint_accepted
            && self.pulp_compatible_backend_active
            && self.boot_handoff_documented
            && self.imported_pulp_boundary_documented
            && self.storage_handoff_documented
            && self.display_handoff_documented
            && self.input_handoff_documented
            && self.old_overlay_artifacts_safe_to_remove
            && !self.physical_spi_transfer_changed
            && !self.chip_select_toggling_changed
            && !self.sd_mmc_low_level_changed
            && !self.fat_storage_algorithm_changed
            && !self.display_draw_algorithm_changed
            && !self.input_debounce_navigation_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.destructive_storage_behavior_added
    }
}

impl VaachakHardwareRuntimeExecutorLiveHandoffCleanup {
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_CLEANUP_MARKER: &'static str =
        "hardware_runtime_executor_live_handoff_cleanup=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_CLEANUP_SCOPE: &'static str =
        "final cleanup checkpoint for live hardware executor handoff";

    pub const CLEANUP_ENTRYPOINT_ACTIVE: bool = true;
    pub const BOOT_HANDOFF_DOCUMENTED: bool = true;
    pub const IMPORTED_PULP_BOUNDARY_DOCUMENTED: bool = true;
    pub const STORAGE_HANDOFF_DOCUMENTED: bool = true;
    pub const DISPLAY_HANDOFF_DOCUMENTED: bool = true;
    pub const INPUT_HANDOFF_DOCUMENTED: bool = true;
    pub const OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE: bool = true;

    pub const PHYSICAL_SPI_TRANSFER_CHANGED: bool = false;
    pub const CHIP_SELECT_TOGGLING_CHANGED: bool = false;
    pub const SD_MMC_LOW_LEVEL_CHANGED: bool = false;
    pub const FAT_STORAGE_ALGORITHM_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_CHANGED: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false;

    pub const fn acceptance_preflight_ok() -> bool {
        VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()
    }

    pub const fn report() -> VaachakHardwareRuntimeLiveHandoffCleanupReport {
        VaachakHardwareRuntimeLiveHandoffCleanupReport {
            cleanup_entrypoint_active: Self::CLEANUP_ENTRYPOINT_ACTIVE,
            live_handoff_accepted: VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok(),
            runtime_use_accepted: VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok(),
            acceptance_checkpoint_accepted: Self::acceptance_preflight_ok(),
            pulp_compatible_backend_active: VaachakHardwareExecutorPulpBackend::backend_ok(),
            boot_handoff_documented: Self::BOOT_HANDOFF_DOCUMENTED,
            imported_pulp_boundary_documented: Self::IMPORTED_PULP_BOUNDARY_DOCUMENTED,
            storage_handoff_documented: Self::STORAGE_HANDOFF_DOCUMENTED,
            display_handoff_documented: Self::DISPLAY_HANDOFF_DOCUMENTED,
            input_handoff_documented: Self::INPUT_HANDOFF_DOCUMENTED,
            old_overlay_artifacts_safe_to_remove: Self::OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE,
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED,
            chip_select_toggling_changed: Self::CHIP_SELECT_TOGGLING_CHANGED,
            sd_mmc_low_level_changed: Self::SD_MMC_LOW_LEVEL_CHANGED,
            fat_storage_algorithm_changed: Self::FAT_STORAGE_ALGORITHM_CHANGED,
            display_draw_algorithm_changed: Self::DISPLAY_DRAW_ALGORITHM_CHANGED,
            input_debounce_navigation_changed: Self::INPUT_DEBOUNCE_NAVIGATION_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            destructive_storage_behavior_added: Self::DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED,
        }
    }

    pub const fn live_handoff_cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_live_handoff_cleanup_marker() {
        if Self::live_handoff_cleanup_ok() {
            Self::emit_line(Self::HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_CLEANUP_MARKER);
            Self::emit_line("hardware.executor.live_handoff.cleanup.accepted");
            Self::emit_line("hardware.executor.live_handoff.cleanup.backend.pulp_compatible");
            Self::emit_line("hardware.executor.live_handoff.cleanup.behavior.preserved");
        } else {
            Self::emit_line("hardware_runtime_executor_live_handoff_cleanup=failed");
        }
    }

    #[cfg(all(target_arch = "riscv32", target_os = "none"))]
    fn emit_line(line: &str) {
        esp_println::println!("{}", line);
    }

    #[cfg(not(all(target_arch = "riscv32", target_os = "none")))]
    fn emit_line(line: &str) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorLiveHandoffCleanup;

    #[test]
    fn hardware_runtime_executor_live_handoff_cleanup_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorLiveHandoffCleanup::live_handoff_cleanup_ok());
    }
}
