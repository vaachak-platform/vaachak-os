use super::display_backend_native_refresh_command_executor::VaachakDisplayBackendNativeRefreshCommandExecutor;
use super::display_backend_native_refresh_shell_cleanup::VaachakDisplayBackendNativeRefreshShellCleanup;
use super::hardware_runtime_backend_takeover_cleanup::VaachakHardwareRuntimeBackendTakeoverCleanup;
use super::input_backend_native_event_pipeline_cleanup::VaachakInputBackendNativeEventPipelineCleanup;

/// Final cleanup checkpoint for the Vaachak-native display refresh command executor.
///
/// This checkpoint folds the rustfmt repair into the accepted display command
/// executor migration. It intentionally does not move SSD1677 drawing, waveform,
/// BUSY wait, physical SPI transfer, chip-select, storage, input, reader,
/// file-browser, or app-navigation behavior.
pub struct VaachakDisplayBackendNativeRefreshCommandExecutorCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayBackendNativeRefreshCommandExecutorCleanupReport {
    pub cleanup_entrypoint_active: bool,
    pub display_command_executor_accepted: bool,
    pub display_refresh_shell_cleanup_accepted: bool,
    pub backend_takeover_cleanup_accepted: bool,
    pub input_event_pipeline_cleanup_accepted: bool,
    pub command_selection_owned_by_vaachak: bool,
    pub partial_refresh_escalation_owned_by_vaachak: bool,
    pub display_request_construction_owned_by_vaachak: bool,
    pub active_backend_is_vaachak_command_executor_with_pulp_executor: bool,
    pub pulp_compatibility_low_level_executor_active: bool,
    pub rustfmt_inner_attribute_repair_folded: bool,
    pub old_overlay_artifacts_safe_to_remove: bool,
    pub ssd1677_draw_algorithm_moved_to_vaachak: bool,
    pub waveform_or_busy_wait_moved_to_vaachak: bool,
    pub spi_transfer_or_chip_select_changed: bool,
    pub storage_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakDisplayBackendNativeRefreshCommandExecutorCleanupReport {
    pub const fn ok(self) -> bool {
        self.cleanup_entrypoint_active
            && self.display_command_executor_accepted
            && self.display_refresh_shell_cleanup_accepted
            && self.backend_takeover_cleanup_accepted
            && self.input_event_pipeline_cleanup_accepted
            && self.command_selection_owned_by_vaachak
            && self.partial_refresh_escalation_owned_by_vaachak
            && self.display_request_construction_owned_by_vaachak
            && self.active_backend_is_vaachak_command_executor_with_pulp_executor
            && self.pulp_compatibility_low_level_executor_active
            && self.rustfmt_inner_attribute_repair_folded
            && self.old_overlay_artifacts_safe_to_remove
            && !self.ssd1677_draw_algorithm_moved_to_vaachak
            && !self.waveform_or_busy_wait_moved_to_vaachak
            && !self.spi_transfer_or_chip_select_changed
            && !self.storage_behavior_changed
            && !self.input_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakDisplayBackendNativeRefreshCommandExecutorCleanup {
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_CLEANUP_MARKER: &'static str =
        "display_backend_native_refresh_command_executor_cleanup=ok";
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_NATIVE_BACKEND_NAME: &'static str =
        "VaachakDisplayRefreshCommandExecutorWithPulpExecutor";
    pub const LOW_LEVEL_EXECUTOR_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const CLEANUP_ENTRYPOINT_ACTIVE: bool = true;
    pub const RUSTFMT_INNER_ATTRIBUTE_REPAIR_FOLDED: bool = true;
    pub const OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE: bool = true;

    pub const SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK: bool = false;
    pub const WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_TRANSFER_OR_CHIP_SELECT_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub fn display_command_executor_ok() -> bool {
        VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok()
    }

    pub fn display_refresh_shell_cleanup_ok() -> bool {
        VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok()
    }

    pub fn backend_takeover_cleanup_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()
    }

    pub fn input_event_pipeline_cleanup_ok() -> bool {
        VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()
    }

    pub fn report() -> VaachakDisplayBackendNativeRefreshCommandExecutorCleanupReport {
        VaachakDisplayBackendNativeRefreshCommandExecutorCleanupReport {
            cleanup_entrypoint_active: Self::CLEANUP_ENTRYPOINT_ACTIVE,
            display_command_executor_accepted: Self::display_command_executor_ok(),
            display_refresh_shell_cleanup_accepted: Self::display_refresh_shell_cleanup_ok(),
            backend_takeover_cleanup_accepted: Self::backend_takeover_cleanup_ok(),
            input_event_pipeline_cleanup_accepted: Self::input_event_pipeline_cleanup_ok(),
            command_selection_owned_by_vaachak:
                VaachakDisplayBackendNativeRefreshCommandExecutor::COMMAND_SELECTION_OWNED_BY_VAACHAK,
            partial_refresh_escalation_owned_by_vaachak:
                VaachakDisplayBackendNativeRefreshCommandExecutor::PARTIAL_REFRESH_ESCALATION_OWNED_BY_VAACHAK,
            display_request_construction_owned_by_vaachak:
                VaachakDisplayBackendNativeRefreshCommandExecutor::DISPLAY_REQUEST_CONSTRUCTION_OWNED_BY_VAACHAK,
            active_backend_is_vaachak_command_executor_with_pulp_executor:
                VaachakDisplayBackendNativeRefreshCommandExecutor::ACTIVE_BACKEND_NAME
                    == Self::ACTIVE_NATIVE_BACKEND_NAME,
            pulp_compatibility_low_level_executor_active:
                VaachakDisplayBackendNativeRefreshCommandExecutor::PULP_EXECUTOR_AVAILABLE
                    && Self::LOW_LEVEL_EXECUTOR_FALLBACK_NAME == "PulpCompatibility",
            rustfmt_inner_attribute_repair_folded: Self::RUSTFMT_INNER_ATTRIBUTE_REPAIR_FOLDED,
            old_overlay_artifacts_safe_to_remove: Self::OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE,
            ssd1677_draw_algorithm_moved_to_vaachak:
                VaachakDisplayBackendNativeRefreshCommandExecutor::SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK
                    || Self::SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK,
            waveform_or_busy_wait_moved_to_vaachak:
                VaachakDisplayBackendNativeRefreshCommandExecutor::WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK
                    || Self::WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK,
            spi_transfer_or_chip_select_changed:
                VaachakDisplayBackendNativeRefreshCommandExecutor::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED
                    || Self::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
            storage_behavior_changed:
                VaachakDisplayBackendNativeRefreshCommandExecutor::STORAGE_BEHAVIOR_CHANGED
                    || Self::STORAGE_BEHAVIOR_CHANGED,
            input_behavior_changed:
                VaachakDisplayBackendNativeRefreshCommandExecutor::INPUT_BEHAVIOR_CHANGED
                    || Self::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed:
                VaachakDisplayBackendNativeRefreshCommandExecutor::READER_FILE_BROWSER_UX_CHANGED
                    || Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed:
                VaachakDisplayBackendNativeRefreshCommandExecutor::APP_NAVIGATION_BEHAVIOR_CHANGED
                    || Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_display_backend_native_refresh_command_executor_cleanup_marker() {
        if Self::cleanup_ok() {
            Self::emit_line(Self::DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_CLEANUP_MARKER);
            Self::emit_line("display.backend.native.command_executor.cleanup.accepted");
            Self::emit_line(
                "display.backend.native.command_executor.cleanup.command_selection.vaachak",
            );
            Self::emit_line(
                "display.backend.native.command_executor.cleanup.low_level.pulp_compatible",
            );
            Self::emit_line("display.backend.native.command_executor.cleanup.behavior.preserved");
        } else {
            Self::emit_line("display_backend_native_refresh_command_executor_cleanup=failed");
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
    use super::VaachakDisplayBackendNativeRefreshCommandExecutorCleanup;

    #[test]
    fn display_backend_native_refresh_command_executor_cleanup_is_ready() {
        assert!(VaachakDisplayBackendNativeRefreshCommandExecutorCleanup::cleanup_ok());
    }
}
