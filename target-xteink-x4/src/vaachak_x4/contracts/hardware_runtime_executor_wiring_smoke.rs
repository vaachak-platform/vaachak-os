#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor::VaachakHardwareRuntimeExecutor;
use crate::vaachak_x4::physical::hardware_runtime_executor_wiring::VaachakHardwareRuntimeExecutorWiring;
use crate::vaachak_x4::physical::hardware_runtime_wiring_pulp_backend::VaachakHardwareRuntimeWiringPulpBackend;

/// Contract smoke for wiring selected runtime paths through the consolidated
/// Vaachak hardware runtime executor.
pub struct VaachakHardwareRuntimeExecutorWiringSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorWiringSmokeReport {
    pub wiring_entrypoint_present: bool,
    pub consolidated_executor_present: bool,
    pub pulp_backend_active: bool,
    pub selected_path_count_ok: bool,
    pub spi_paths_wired: bool,
    pub storage_paths_wired: bool,
    pub display_paths_wired: bool,
    pub input_paths_wired: bool,
    pub reader_file_browser_ux_unchanged: bool,
    pub app_navigation_unchanged: bool,
    pub no_display_draw_rewrite: bool,
    pub no_input_debounce_navigation_rewrite: bool,
    pub no_fat_destructive_behavior: bool,
}

impl VaachakHardwareRuntimeExecutorWiringSmokeReport {
    pub const fn ok(self) -> bool {
        self.wiring_entrypoint_present
            && self.consolidated_executor_present
            && self.pulp_backend_active
            && self.selected_path_count_ok
            && self.spi_paths_wired
            && self.storage_paths_wired
            && self.display_paths_wired
            && self.input_paths_wired
            && self.reader_file_browser_ux_unchanged
            && self.app_navigation_unchanged
            && self.no_display_draw_rewrite
            && self.no_input_debounce_navigation_rewrite
            && self.no_fat_destructive_behavior
    }
}

impl VaachakHardwareRuntimeExecutorWiringSmoke {
    pub const MARKER: &'static str =
        VaachakHardwareRuntimeExecutorWiring::HARDWARE_RUNTIME_EXECUTOR_WIRING_MARKER;

    pub const fn report() -> VaachakHardwareRuntimeExecutorWiringSmokeReport {
        VaachakHardwareRuntimeExecutorWiringSmokeReport {
            wiring_entrypoint_present: VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            consolidated_executor_present: VaachakHardwareRuntimeExecutor::extraction_ok(),
            pulp_backend_active: VaachakHardwareRuntimeWiringPulpBackend::backend_ok(),
            selected_path_count_ok:
                VaachakHardwareRuntimeExecutorWiring::SELECTED_RUNTIME_PATH_COUNT == 10,
            spi_paths_wired: VaachakHardwareRuntimeExecutorWiring::spi_paths_wired(),
            storage_paths_wired: VaachakHardwareRuntimeExecutorWiring::storage_paths_wired(),
            display_paths_wired: VaachakHardwareRuntimeExecutorWiring::display_paths_wired(),
            input_paths_wired: VaachakHardwareRuntimeExecutorWiring::input_paths_wired(),
            reader_file_browser_ux_unchanged:
                !VaachakHardwareRuntimeExecutorWiring::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_unchanged:
                !VaachakHardwareRuntimeExecutorWiring::APP_NAVIGATION_BEHAVIOR_CHANGED,
            no_display_draw_rewrite:
                !VaachakHardwareRuntimeExecutorWiring::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            no_input_debounce_navigation_rewrite:
                !VaachakHardwareRuntimeExecutorWiring::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            no_fat_destructive_behavior:
                !VaachakHardwareRuntimeExecutorWiring::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorWiringSmoke;

    #[test]
    fn hardware_runtime_executor_wiring_smoke_passes() {
        assert!(VaachakHardwareRuntimeExecutorWiringSmoke::ok());
    }
}
