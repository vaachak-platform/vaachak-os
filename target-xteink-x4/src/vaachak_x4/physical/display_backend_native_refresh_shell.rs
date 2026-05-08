use super::display_backend_native_refresh_command_executor::VaachakDisplayBackendNativeRefreshCommandExecutor;
#[allow(dead_code)]
use super::display_executor_bridge::{VaachakDisplayExecutorBridge, VaachakDisplayExecutorIntent};
use super::display_runtime_owner::VaachakDisplayRuntimeOwner;
use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_backend::{VaachakDisplayBackendOperation, VaachakDisplayRequest};
use super::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;

/// Vaachak-native display refresh shell.
///
/// This shell is the first display-native migration step. Vaachak owns refresh
/// command normalization, refresh intent mapping, and refresh handoff pre-routing.
/// The SSD1677 draw buffer, waveform, BUSY wait, full refresh, partial refresh,
/// clear, sleep, and physical SPI/chip-select execution remain Pulp-compatible.
pub struct VaachakDisplayBackendNativeRefreshShell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayNativeRefreshBackend {
    VaachakDisplayNativeRefreshShellWithPulpExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshExecutorFallback {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshShellOperation {
    CommandNormalization,
    RefreshIntentMapping,
    FullRefreshHandoff,
    PartialRefreshHandoff,
    ClearFrameHandoff,
    SleepFrameHandoff,
    RenderFrameMetadataHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshCommand {
    FullRefresh,
    PartialRefresh,
    ClearFrame,
    SleepFrame,
    RenderFrameMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshIntent {
    ExecuteFullRefreshThroughPulpCompatibility,
    ExecutePartialRefreshThroughPulpCompatibility,
    ExecuteClearFrameThroughPulpCompatibility,
    ExecuteSleepFrameThroughPulpCompatibility,
    ExecuteRenderFrameMetadataThroughPulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayNativeRefreshRoute {
    pub source_command: VaachakDisplayRefreshCommand,
    pub mapped_intent: VaachakDisplayRefreshIntent,
    pub backend_operation: VaachakDisplayBackendOperation,
    pub executor_bridge_intent: VaachakDisplayExecutorIntent,
    pub native_backend: VaachakDisplayNativeRefreshBackend,
    pub executor_fallback: VaachakDisplayRefreshExecutorFallback,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub refresh_shell_owned_by_vaachak: bool,
    pub refresh_intent_mapping_owned_by_vaachak: bool,
    pub ssd1677_executor_moved_to_vaachak: bool,
    pub draw_buffer_algorithm_rewritten: bool,
    pub full_refresh_algorithm_rewritten: bool,
    pub partial_refresh_algorithm_rewritten: bool,
    pub spi_transfer_or_chip_select_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayNativeRefreshShellReport {
    pub operation: VaachakDisplayRefreshShellOperation,
    pub active_native_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub refresh_executor_fallback_name: &'static str,
    pub refresh_executor_owner: &'static str,
    pub refresh_shell_owned_by_vaachak: bool,
    pub refresh_intent_mapping_owned_by_vaachak: bool,
    pub pulp_refresh_executor_available: bool,
    pub display_runtime_owner_ready: bool,
    pub display_executor_bridge_ready: bool,
    pub low_level_backend: VaachakHardwareExecutorBackend,
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

impl VaachakDisplayNativeRefreshRoute {
    pub const fn ok(self) -> bool {
        self.refresh_shell_owned_by_vaachak
            && self.refresh_intent_mapping_owned_by_vaachak
            && matches!(
                self.native_backend,
                VaachakDisplayNativeRefreshBackend::VaachakDisplayNativeRefreshShellWithPulpExecutor
            )
            && matches!(
                self.executor_fallback,
                VaachakDisplayRefreshExecutorFallback::PulpCompatibility
            )
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && !self.ssd1677_executor_moved_to_vaachak
            && !self.draw_buffer_algorithm_rewritten
            && !self.full_refresh_algorithm_rewritten
            && !self.partial_refresh_algorithm_rewritten
            && !self.spi_transfer_or_chip_select_changed
    }
}

impl VaachakDisplayNativeRefreshShellReport {
    pub const fn ok(self) -> bool {
        self.refresh_shell_owned_by_vaachak
            && self.refresh_intent_mapping_owned_by_vaachak
            && self.pulp_refresh_executor_available
            && self.display_runtime_owner_ready
            && self.display_executor_bridge_ready
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
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

impl VaachakDisplayBackendNativeRefreshShell {
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_MARKER: &'static str =
        "display_backend_native_refresh_shell=ok";
    pub const ACTIVE_NATIVE_BACKEND_NAME: &'static str =
        "VaachakDisplayNativeRefreshShellWithPulpExecutor";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const REFRESH_EXECUTOR_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const REFRESH_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK: bool = true;
    pub const REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK: bool = true;
    pub const PULP_REFRESH_EXECUTOR_AVAILABLE: bool = true;

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

    pub const fn map_command_to_intent(
        command: VaachakDisplayRefreshCommand,
    ) -> VaachakDisplayRefreshIntent {
        match command {
            VaachakDisplayRefreshCommand::FullRefresh => {
                VaachakDisplayRefreshIntent::ExecuteFullRefreshThroughPulpCompatibility
            }
            VaachakDisplayRefreshCommand::PartialRefresh => {
                VaachakDisplayRefreshIntent::ExecutePartialRefreshThroughPulpCompatibility
            }
            VaachakDisplayRefreshCommand::ClearFrame => {
                VaachakDisplayRefreshIntent::ExecuteClearFrameThroughPulpCompatibility
            }
            VaachakDisplayRefreshCommand::SleepFrame => {
                VaachakDisplayRefreshIntent::ExecuteSleepFrameThroughPulpCompatibility
            }
            VaachakDisplayRefreshCommand::RenderFrameMetadata => {
                VaachakDisplayRefreshIntent::ExecuteRenderFrameMetadataThroughPulpCompatibility
            }
        }
    }

    pub const fn backend_operation_for(
        command: VaachakDisplayRefreshCommand,
    ) -> VaachakDisplayBackendOperation {
        match command {
            VaachakDisplayRefreshCommand::FullRefresh => {
                VaachakDisplayBackendOperation::FullRefresh
            }
            VaachakDisplayRefreshCommand::PartialRefresh => {
                VaachakDisplayBackendOperation::PartialRefresh
            }
            VaachakDisplayRefreshCommand::ClearFrame => VaachakDisplayBackendOperation::Clear,
            VaachakDisplayRefreshCommand::SleepFrame => VaachakDisplayBackendOperation::Sleep,
            VaachakDisplayRefreshCommand::RenderFrameMetadata => {
                VaachakDisplayBackendOperation::RenderIntent
            }
        }
    }

    pub const fn executor_bridge_intent_for(
        command: VaachakDisplayRefreshCommand,
    ) -> VaachakDisplayExecutorIntent {
        match command {
            VaachakDisplayRefreshCommand::FullRefresh => VaachakDisplayExecutorIntent::FullRefresh,
            VaachakDisplayRefreshCommand::PartialRefresh => {
                VaachakDisplayExecutorIntent::PartialRefresh
            }
            VaachakDisplayRefreshCommand::ClearFrame => VaachakDisplayExecutorIntent::ClearFrame,
            VaachakDisplayRefreshCommand::SleepFrame => VaachakDisplayExecutorIntent::SleepFrame,
            VaachakDisplayRefreshCommand::RenderFrameMetadata => {
                VaachakDisplayExecutorIntent::RenderFrameMetadata
            }
        }
    }

    pub const fn route_command(
        command: VaachakDisplayRefreshCommand,
    ) -> VaachakDisplayNativeRefreshRoute {
        VaachakDisplayNativeRefreshRoute {
            source_command: command,
            mapped_intent: Self::map_command_to_intent(command),
            backend_operation: Self::backend_operation_for(command),
            executor_bridge_intent: Self::executor_bridge_intent_for(command),
            native_backend:
                VaachakDisplayNativeRefreshBackend::VaachakDisplayNativeRefreshShellWithPulpExecutor,
            executor_fallback: VaachakDisplayRefreshExecutorFallback::PulpCompatibility,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            refresh_shell_owned_by_vaachak: Self::REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK,
            refresh_intent_mapping_owned_by_vaachak: Self::REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK,
            ssd1677_executor_moved_to_vaachak: Self::SSD1677_EXECUTOR_MOVED_TO_VAACHAK,
            draw_buffer_algorithm_rewritten: Self::DRAW_BUFFER_ALGORITHM_REWRITTEN,
            full_refresh_algorithm_rewritten: Self::FULL_REFRESH_ALGORITHM_REWRITTEN,
            partial_refresh_algorithm_rewritten: Self::PARTIAL_REFRESH_ALGORITHM_REWRITTEN,
            spi_transfer_or_chip_select_changed: Self::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
        }
    }

    pub const fn display_request_for(
        command: VaachakDisplayRefreshCommand,
    ) -> VaachakDisplayRequest {
        VaachakDisplayRequest {
            operation: Self::backend_operation_for(command),
            spi_handoff_required: true,
            display_draw_algorithm_rewrite_allowed: false,
        }
    }

    pub const fn report(
        operation: VaachakDisplayRefreshShellOperation,
    ) -> VaachakDisplayNativeRefreshShellReport {
        VaachakDisplayNativeRefreshShellReport {
            operation,
            active_native_backend_name: Self::ACTIVE_NATIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            refresh_executor_fallback_name: Self::REFRESH_EXECUTOR_FALLBACK_NAME,
            refresh_executor_owner: Self::REFRESH_EXECUTOR_OWNER,
            refresh_shell_owned_by_vaachak: Self::REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK,
            refresh_intent_mapping_owned_by_vaachak: Self::REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK,
            pulp_refresh_executor_available: Self::PULP_REFRESH_EXECUTOR_AVAILABLE,
            display_runtime_owner_ready: VaachakDisplayRuntimeOwner::ownership_ok(),
            display_executor_bridge_ready: VaachakDisplayExecutorBridge::bridge_ok(),
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            ssd1677_executor_moved_to_vaachak: Self::SSD1677_EXECUTOR_MOVED_TO_VAACHAK,
            draw_buffer_algorithm_rewritten: Self::DRAW_BUFFER_ALGORITHM_REWRITTEN,
            full_refresh_algorithm_rewritten: Self::FULL_REFRESH_ALGORITHM_REWRITTEN,
            partial_refresh_algorithm_rewritten: Self::PARTIAL_REFRESH_ALGORITHM_REWRITTEN,
            busy_wait_algorithm_rewritten: Self::BUSY_WAIT_ALGORITHM_REWRITTEN,
            spi_transfer_or_chip_select_changed: Self::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn execute_full_refresh_handoff() -> VaachakDisplayNativeRefreshShellReport {
        let _command_execution =
            VaachakDisplayBackendNativeRefreshCommandExecutor::execute_full_refresh_command();
        Self::report(VaachakDisplayRefreshShellOperation::FullRefreshHandoff)
    }

    pub fn execute_partial_refresh_handoff() -> VaachakDisplayNativeRefreshShellReport {
        let _command_execution =
            VaachakDisplayBackendNativeRefreshCommandExecutor::execute_partial_refresh_command();
        Self::report(VaachakDisplayRefreshShellOperation::PartialRefreshHandoff)
    }

    pub fn execute_clear_handoff() -> VaachakDisplayNativeRefreshShellReport {
        let _command_execution =
            VaachakDisplayBackendNativeRefreshCommandExecutor::execute_clear_command();
        Self::report(VaachakDisplayRefreshShellOperation::ClearFrameHandoff)
    }

    pub fn execute_sleep_handoff() -> VaachakDisplayNativeRefreshShellReport {
        let _command_execution =
            VaachakDisplayBackendNativeRefreshCommandExecutor::execute_sleep_command();
        Self::report(VaachakDisplayRefreshShellOperation::SleepFrameHandoff)
    }

    pub fn execute_render_metadata_handoff() -> VaachakDisplayNativeRefreshShellReport {
        let _command_execution =
            VaachakDisplayBackendNativeRefreshCommandExecutor::execute_render_metadata_command();
        Self::report(VaachakDisplayRefreshShellOperation::RenderFrameMetadataHandoff)
    }

    pub const fn native_refresh_mappings_ok() -> bool {
        matches!(
            Self::route_command(VaachakDisplayRefreshCommand::FullRefresh).mapped_intent,
            VaachakDisplayRefreshIntent::ExecuteFullRefreshThroughPulpCompatibility
        ) && matches!(
            Self::route_command(VaachakDisplayRefreshCommand::PartialRefresh).mapped_intent,
            VaachakDisplayRefreshIntent::ExecutePartialRefreshThroughPulpCompatibility
        ) && matches!(
            Self::route_command(VaachakDisplayRefreshCommand::ClearFrame).mapped_intent,
            VaachakDisplayRefreshIntent::ExecuteClearFrameThroughPulpCompatibility
        ) && matches!(
            Self::route_command(VaachakDisplayRefreshCommand::SleepFrame).mapped_intent,
            VaachakDisplayRefreshIntent::ExecuteSleepFrameThroughPulpCompatibility
        ) && matches!(
            Self::route_command(VaachakDisplayRefreshCommand::RenderFrameMetadata).mapped_intent,
            VaachakDisplayRefreshIntent::ExecuteRenderFrameMetadataThroughPulpCompatibility
        )
    }

    pub const fn route_safety_ok() -> bool {
        Self::route_command(VaachakDisplayRefreshCommand::FullRefresh).ok()
            && Self::route_command(VaachakDisplayRefreshCommand::PartialRefresh).ok()
            && Self::route_command(VaachakDisplayRefreshCommand::ClearFrame).ok()
            && Self::route_command(VaachakDisplayRefreshCommand::SleepFrame).ok()
            && Self::route_command(VaachakDisplayRefreshCommand::RenderFrameMetadata).ok()
    }

    pub fn display_native_refresh_command_executor_ready() -> bool {
        VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok()
    }

    pub fn native_refresh_shell_ok() -> bool {
        VaachakDisplayRuntimeOwner::ownership_ok()
            && VaachakDisplayExecutorBridge::bridge_ok()
            && VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok()
            && Self::native_refresh_mappings_ok()
            && Self::route_safety_ok()
            && Self::display_native_refresh_command_executor_ready()
            && Self::execute_full_refresh_handoff().ok()
            && Self::execute_partial_refresh_handoff().ok()
            && Self::execute_clear_handoff().ok()
            && Self::execute_sleep_handoff().ok()
            && Self::execute_render_metadata_handoff().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        VaachakDisplayBackendNativeRefreshShell, VaachakDisplayRefreshCommand,
        VaachakDisplayRefreshIntent,
    };

    #[test]
    fn display_backend_native_refresh_shell_is_ready() {
        assert!(VaachakDisplayBackendNativeRefreshShell::native_refresh_shell_ok());
    }

    #[test]
    fn full_and_partial_refresh_map_to_pulp_compatible_execution() {
        assert_eq!(
            VaachakDisplayBackendNativeRefreshShell::route_command(
                VaachakDisplayRefreshCommand::FullRefresh
            )
            .mapped_intent,
            VaachakDisplayRefreshIntent::ExecuteFullRefreshThroughPulpCompatibility
        );
        assert_eq!(
            VaachakDisplayBackendNativeRefreshShell::route_command(
                VaachakDisplayRefreshCommand::PartialRefresh
            )
            .mapped_intent,
            VaachakDisplayRefreshIntent::ExecutePartialRefreshThroughPulpCompatibility
        );
    }
}
