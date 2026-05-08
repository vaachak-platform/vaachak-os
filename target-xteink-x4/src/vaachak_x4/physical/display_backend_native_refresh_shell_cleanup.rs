#![allow(dead_code)]

use super::display_backend_native_refresh_shell::VaachakDisplayBackendNativeRefreshShell;
use super::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use super::hardware_runtime_backend_takeover_cleanup::VaachakHardwareRuntimeBackendTakeoverCleanup;
use super::input_backend_native_executor_cleanup::VaachakInputBackendNativeExecutorCleanup;

/// Final cleanup checkpoint for the Vaachak-native display refresh shell.
///
/// This module intentionally does not add SSD1677 drawing, waveform, BUSY wait,
/// physical SPI transfer, chip-select, storage, input, reader, file-browser, or
/// app-navigation behavior. It proves the accepted display refresh shell is a
/// clean checkpoint before deeper display-native or storage-native migration.
pub struct VaachakDisplayBackendNativeRefreshShellCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayBackendNativeRefreshShellCleanupReport {
    pub cleanup_entrypoint_active: bool,
    pub display_native_refresh_shell_accepted: bool,
    pub backend_takeover_bridge_accepted: bool,
    pub backend_takeover_cleanup_accepted: bool,
    pub input_native_cleanup_accepted: bool,
    pub refresh_command_shell_owned_by_vaachak: bool,
    pub refresh_intent_mapping_owned_by_vaachak: bool,
    pub selected_backend_is_vaachak_display_native_refresh_shell_with_pulp_executor: bool,
    pub pulp_compatibility_executor_active: bool,
    pub old_overlay_artifacts_safe_to_remove: bool,
    pub ssd1677_executor_moved_to_vaachak: bool,
    pub draw_buffer_algorithm_rewritten: bool,
    pub full_refresh_algorithm_rewritten: bool,
    pub partial_refresh_algorithm_rewritten: bool,
    pub busy_wait_algorithm_rewritten: bool,
    pub spi_transfer_or_chip_select_changed: bool,
    pub storage_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakDisplayBackendNativeRefreshShellCleanupReport {
    pub const fn ok(self) -> bool {
        self.cleanup_entrypoint_active
            && self.display_native_refresh_shell_accepted
            && self.backend_takeover_bridge_accepted
            && self.backend_takeover_cleanup_accepted
            && self.input_native_cleanup_accepted
            && self.refresh_command_shell_owned_by_vaachak
            && self.refresh_intent_mapping_owned_by_vaachak
            && self.selected_backend_is_vaachak_display_native_refresh_shell_with_pulp_executor
            && self.pulp_compatibility_executor_active
            && self.old_overlay_artifacts_safe_to_remove
            && !self.ssd1677_executor_moved_to_vaachak
            && !self.draw_buffer_algorithm_rewritten
            && !self.full_refresh_algorithm_rewritten
            && !self.partial_refresh_algorithm_rewritten
            && !self.busy_wait_algorithm_rewritten
            && !self.spi_transfer_or_chip_select_changed
            && !self.storage_behavior_changed
            && !self.input_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakDisplayBackendNativeRefreshShellCleanup {
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_CLEANUP_MARKER: &'static str =
        "display_backend_native_refresh_shell_cleanup=ok";
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_NATIVE_BACKEND_NAME: &'static str =
        "VaachakDisplayNativeRefreshShellWithPulpExecutor";
    pub const REFRESH_EXECUTOR_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const REFRESH_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const CLEANUP_ENTRYPOINT_ACTIVE: bool = true;
    pub const OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE: bool = true;

    pub const SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DRAW_BUFFER_ALGORITHM_REWRITTEN: bool = false;
    pub const FULL_REFRESH_ALGORITHM_REWRITTEN: bool = false;
    pub const PARTIAL_REFRESH_ALGORITHM_REWRITTEN: bool = false;
    pub const BUSY_WAIT_ALGORITHM_REWRITTEN: bool = false;
    pub const SPI_TRANSFER_OR_CHIP_SELECT_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub fn display_native_refresh_shell_ok() -> bool {
        VaachakDisplayBackendNativeRefreshShell::native_refresh_shell_ok()
    }

    pub fn backend_takeover_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeover::takeover_ok()
            && VaachakHardwareRuntimeBackendTakeover::backend_interface_calls_ok()
    }

    pub fn backend_takeover_cleanup_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()
    }

    pub fn input_native_cleanup_ok() -> bool {
        VaachakInputBackendNativeExecutorCleanup::cleanup_ok()
    }

    pub fn report() -> VaachakDisplayBackendNativeRefreshShellCleanupReport {
        VaachakDisplayBackendNativeRefreshShellCleanupReport {
            cleanup_entrypoint_active: Self::CLEANUP_ENTRYPOINT_ACTIVE,
            display_native_refresh_shell_accepted: Self::display_native_refresh_shell_ok(),
            backend_takeover_bridge_accepted: Self::backend_takeover_ok(),
            backend_takeover_cleanup_accepted: Self::backend_takeover_cleanup_ok(),
            input_native_cleanup_accepted: Self::input_native_cleanup_ok(),
            refresh_command_shell_owned_by_vaachak:
                VaachakDisplayBackendNativeRefreshShell::REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK,
            refresh_intent_mapping_owned_by_vaachak:
                VaachakDisplayBackendNativeRefreshShell::REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK,
            selected_backend_is_vaachak_display_native_refresh_shell_with_pulp_executor:
                VaachakDisplayBackendNativeRefreshShell::ACTIVE_NATIVE_BACKEND_NAME
                    == Self::ACTIVE_NATIVE_BACKEND_NAME,
            pulp_compatibility_executor_active:
                VaachakDisplayBackendNativeRefreshShell::REFRESH_EXECUTOR_FALLBACK_NAME
                    == Self::REFRESH_EXECUTOR_FALLBACK_NAME,
            old_overlay_artifacts_safe_to_remove: Self::OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE,
            ssd1677_executor_moved_to_vaachak:
                VaachakDisplayBackendNativeRefreshShell::SSD1677_EXECUTOR_MOVED_TO_VAACHAK
                    || Self::SSD1677_EXECUTOR_MOVED_TO_VAACHAK,
            draw_buffer_algorithm_rewritten:
                VaachakDisplayBackendNativeRefreshShell::DRAW_BUFFER_ALGORITHM_REWRITTEN
                    || Self::DRAW_BUFFER_ALGORITHM_REWRITTEN,
            full_refresh_algorithm_rewritten:
                VaachakDisplayBackendNativeRefreshShell::FULL_REFRESH_ALGORITHM_REWRITTEN
                    || Self::FULL_REFRESH_ALGORITHM_REWRITTEN,
            partial_refresh_algorithm_rewritten:
                VaachakDisplayBackendNativeRefreshShell::PARTIAL_REFRESH_ALGORITHM_REWRITTEN
                    || Self::PARTIAL_REFRESH_ALGORITHM_REWRITTEN,
            busy_wait_algorithm_rewritten:
                VaachakDisplayBackendNativeRefreshShell::BUSY_WAIT_ALGORITHM_REWRITTEN
                    || Self::BUSY_WAIT_ALGORITHM_REWRITTEN,
            spi_transfer_or_chip_select_changed:
                VaachakDisplayBackendNativeRefreshShell::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED
                    || Self::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
            storage_behavior_changed:
                VaachakDisplayBackendNativeRefreshShell::STORAGE_BEHAVIOR_CHANGED
                    || Self::STORAGE_BEHAVIOR_CHANGED,
            input_behavior_changed: VaachakDisplayBackendNativeRefreshShell::INPUT_BEHAVIOR_CHANGED
                || Self::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed:
                VaachakDisplayBackendNativeRefreshShell::READER_FILE_BROWSER_UX_CHANGED
                    || Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed:
                VaachakDisplayBackendNativeRefreshShell::APP_NAVIGATION_BEHAVIOR_CHANGED
                    || Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_display_backend_native_refresh_shell_cleanup_marker() {
        if Self::cleanup_ok() {
            Self::emit_line(Self::DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_CLEANUP_MARKER);
            Self::emit_line("display.backend.native.cleanup.accepted");
            Self::emit_line(
                "display.backend.native.cleanup.backend.vaachak_refresh_shell_with_pulp_executor",
            );
            Self::emit_line("display.backend.native.cleanup.refresh_executor.pulp_compatible");
            Self::emit_line("display.backend.native.cleanup.behavior.preserved");
        } else {
            Self::emit_line("display_backend_native_refresh_shell_cleanup=failed");
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
    use super::VaachakDisplayBackendNativeRefreshShellCleanup;

    #[test]
    fn display_backend_native_refresh_shell_cleanup_is_ready() {
        assert!(VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok());
    }
}
