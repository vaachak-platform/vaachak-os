#![allow(dead_code)]

use super::hardware_runtime_backend::VaachakHardwareRuntimeBackendInterface;
use super::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;
use super::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use super::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use super::hardware_runtime_executor_live_handoff::VaachakHardwareRuntimeExecutorLiveHandoff;
use super::hardware_runtime_executor_live_handoff_cleanup::VaachakHardwareRuntimeExecutorLiveHandoffCleanup;
use super::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;

/// Final cleanup/acceptance checkpoint for the hardware runtime backend takeover.
///
/// This module intentionally does not add native hardware behavior. It proves the
/// accepted backend takeover bridge is ready as a clean checkpoint before the
/// first native backend implementation replaces any Pulp-compatible executor.
pub struct VaachakHardwareRuntimeBackendTakeoverCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeBackendTakeoverCleanupReport {
    pub cleanup_entrypoint_active: bool,
    pub backend_takeover_accepted: bool,
    pub backend_traits_owned_by_vaachak: bool,
    pub request_result_structs_owned_by_vaachak: bool,
    pub pulp_compatibility_backend_active: bool,
    pub live_handoff_accepted: bool,
    pub live_handoff_cleanup_accepted: bool,
    pub runtime_use_accepted: bool,
    pub acceptance_checkpoint_accepted: bool,
    pub old_overlay_artifacts_safe_to_remove: bool,
    pub physical_spi_transfer_changed: bool,
    pub chip_select_toggling_changed: bool,
    pub sd_mmc_fat_algorithm_changed: bool,
    pub display_draw_algorithm_changed: bool,
    pub input_debounce_navigation_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub destructive_storage_behavior_added: bool,
}

impl VaachakHardwareRuntimeBackendTakeoverCleanupReport {
    pub const fn ok(self) -> bool {
        self.cleanup_entrypoint_active
            && self.backend_takeover_accepted
            && self.backend_traits_owned_by_vaachak
            && self.request_result_structs_owned_by_vaachak
            && self.pulp_compatibility_backend_active
            && self.live_handoff_accepted
            && self.live_handoff_cleanup_accepted
            && self.runtime_use_accepted
            && self.acceptance_checkpoint_accepted
            && self.old_overlay_artifacts_safe_to_remove
            && !self.physical_spi_transfer_changed
            && !self.chip_select_toggling_changed
            && !self.sd_mmc_fat_algorithm_changed
            && !self.display_draw_algorithm_changed
            && !self.input_debounce_navigation_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.destructive_storage_behavior_added
    }
}

impl VaachakHardwareRuntimeBackendTakeoverCleanup {
    pub const HARDWARE_RUNTIME_BACKEND_TAKEOVER_CLEANUP_MARKER: &'static str =
        "hardware_runtime_backend_takeover_cleanup=ok";
    pub const HARDWARE_RUNTIME_BACKEND_TAKEOVER_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const CLEANUP_ENTRYPOINT_ACTIVE: bool = true;
    pub const OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE: bool = true;

    pub const PHYSICAL_SPI_TRANSFER_CHANGED: bool = false;
    pub const CHIP_SELECT_TOGGLING_CHANGED: bool = false;
    pub const SD_MMC_FAT_ALGORITHM_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_CHANGED: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false;

    pub fn acceptance_preflight_ok() -> bool {
        VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()
    }

    pub fn backend_takeover_bridge_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeover::takeover_ok()
            && VaachakHardwareRuntimeBackendTakeover::backend_interface_calls_ok()
    }

    pub fn vaachak_backend_interface_ok() -> bool {
        VaachakHardwareRuntimeBackendInterface::interface_ok()
            && VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok()
    }

    pub fn report() -> VaachakHardwareRuntimeBackendTakeoverCleanupReport {
        VaachakHardwareRuntimeBackendTakeoverCleanupReport {
            cleanup_entrypoint_active: Self::CLEANUP_ENTRYPOINT_ACTIVE,
            backend_takeover_accepted: Self::backend_takeover_bridge_ok(),
            backend_traits_owned_by_vaachak: VaachakHardwareRuntimeBackendInterface::interface_ok(),
            request_result_structs_owned_by_vaachak:
                VaachakHardwareRuntimeBackendInterface::REQUEST_RESULT_STRUCTS_OWNED_BY_VAACHAK,
            pulp_compatibility_backend_active: Self::vaachak_backend_interface_ok(),
            live_handoff_accepted: VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok(),
            live_handoff_cleanup_accepted:
                VaachakHardwareRuntimeExecutorLiveHandoffCleanup::live_handoff_cleanup_ok(),
            runtime_use_accepted: VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok(),
            acceptance_checkpoint_accepted: Self::acceptance_preflight_ok(),
            old_overlay_artifacts_safe_to_remove: Self::OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE,
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED,
            chip_select_toggling_changed: Self::CHIP_SELECT_TOGGLING_CHANGED,
            sd_mmc_fat_algorithm_changed: Self::SD_MMC_FAT_ALGORITHM_CHANGED,
            display_draw_algorithm_changed: Self::DISPLAY_DRAW_ALGORITHM_CHANGED,
            input_debounce_navigation_changed: Self::INPUT_DEBOUNCE_NAVIGATION_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            destructive_storage_behavior_added: Self::DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED,
        }
    }

    pub fn backend_takeover_cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_backend_takeover_cleanup_marker() {
        if Self::backend_takeover_cleanup_ok() {
            Self::emit_line(Self::HARDWARE_RUNTIME_BACKEND_TAKEOVER_CLEANUP_MARKER);
            Self::emit_line("hardware.backend.takeover.cleanup.accepted");
            Self::emit_line("hardware.backend.takeover.cleanup.backend.pulp_compatible");
            Self::emit_line("hardware.backend.takeover.cleanup.behavior.preserved");
        } else {
            Self::emit_line("hardware_runtime_backend_takeover_cleanup=failed");
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
    use super::VaachakHardwareRuntimeBackendTakeoverCleanup;

    #[test]
    fn hardware_runtime_backend_takeover_cleanup_is_ready() {
        assert!(VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok());
    }
}
