#![allow(dead_code)]

use crate::vaachak_x4::physical::display_executor_bridge::VaachakDisplayExecutorBridge;
use crate::vaachak_x4::physical::hardware_executor_pulp_backend::VaachakHardwareExecutorPulpBackend;
use crate::vaachak_x4::physical::hardware_runtime_executor::VaachakHardwareRuntimeExecutor;
use crate::vaachak_x4::physical::input_executor_bridge::VaachakInputExecutorBridge;
use crate::vaachak_x4::physical::spi_executor_bridge::VaachakSpiExecutorBridge;
use crate::vaachak_x4::physical::storage_executor_bridge::VaachakStorageExecutorBridge;

/// Contract smoke for the broad Vaachak hardware runtime executor extraction.
pub struct VaachakHardwareRuntimeExecutorSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorSmokeReport {
    pub consolidated_entrypoint_present: bool,
    pub pulp_backend_active: bool,
    pub spi_bridge_present: bool,
    pub storage_bridge_present: bool,
    pub display_bridge_present: bool,
    pub input_bridge_present: bool,
    pub reader_file_browser_ux_unchanged: bool,
    pub app_navigation_unchanged: bool,
    pub no_display_draw_rewrite: bool,
    pub no_input_debounce_navigation_rewrite: bool,
    pub no_fat_destructive_behavior: bool,
}

impl VaachakHardwareRuntimeExecutorSmokeReport {
    pub const fn ok(self) -> bool {
        self.consolidated_entrypoint_present
            && self.pulp_backend_active
            && self.spi_bridge_present
            && self.storage_bridge_present
            && self.display_bridge_present
            && self.input_bridge_present
            && self.reader_file_browser_ux_unchanged
            && self.app_navigation_unchanged
            && self.no_display_draw_rewrite
            && self.no_input_debounce_navigation_rewrite
            && self.no_fat_destructive_behavior
    }
}

impl VaachakHardwareRuntimeExecutorSmoke {
    pub const MARKER: &'static str =
        VaachakHardwareRuntimeExecutor::HARDWARE_RUNTIME_EXECUTOR_EXTRACTION_MARKER;

    pub const fn report() -> VaachakHardwareRuntimeExecutorSmokeReport {
        VaachakHardwareRuntimeExecutorSmokeReport {
            consolidated_entrypoint_present: VaachakHardwareRuntimeExecutor::extraction_ok(),
            pulp_backend_active: VaachakHardwareExecutorPulpBackend::backend_ok(),
            spi_bridge_present: VaachakSpiExecutorBridge::bridge_ok(),
            storage_bridge_present: VaachakStorageExecutorBridge::bridge_ok(),
            display_bridge_present: VaachakDisplayExecutorBridge::bridge_ok(),
            input_bridge_present: VaachakInputExecutorBridge::bridge_ok(),
            reader_file_browser_ux_unchanged:
                !VaachakHardwareRuntimeExecutor::READER_FILE_BROWSER_UX_BEHAVIOR_CHANGED,
            app_navigation_unchanged:
                !VaachakHardwareRuntimeExecutor::APP_NAVIGATION_BEHAVIOR_CHANGED,
            no_display_draw_rewrite:
                !VaachakHardwareRuntimeExecutor::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            no_input_debounce_navigation_rewrite:
                !VaachakHardwareRuntimeExecutor::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            no_fat_destructive_behavior:
                !VaachakHardwareRuntimeExecutor::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorSmoke;

    #[test]
    fn hardware_runtime_executor_smoke_passes() {
        assert!(VaachakHardwareRuntimeExecutorSmoke::ok());
    }
}
