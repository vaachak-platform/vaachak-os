#![allow(dead_code)]

use super::display_backend_native_refresh_shell::{
    VaachakDisplayBackendNativeRefreshShell, VaachakDisplayRefreshCommand,
};
use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_backend::{
    VaachakDisplayBackendOperation, VaachakDisplayRequest, VaachakHardwareBackendHandoffResult,
};
use super::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;

/// Vaachak-native display refresh command executor.
///
/// This is the first display behavior move beyond refresh-shell metadata:
/// Vaachak now owns refresh command selection, partial-to-full escalation policy,
/// clear/sleep/render command classification, request construction, and safe
/// executor routing. The low-level SSD1677 draw buffer, waveform, BUSY wait,
/// physical SPI transfer, and chip-select execution remain Pulp-compatible.
pub struct VaachakDisplayBackendNativeRefreshCommandExecutor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshCommandExecutorBackend {
    VaachakDisplayRefreshCommandExecutorWithPulpExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshCommandReason {
    FullRefreshRequested,
    PartialRefreshRequested,
    PartialRefreshUnsafeEscalatedToFull,
    ClearRequested,
    SleepRequested,
    RenderMetadataRequested,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRefreshCommandSafety {
    SafeForRequestedRefresh,
    EscalatedToFullRefresh,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRefreshCommandContext {
    pub requested_command: VaachakDisplayRefreshCommand,
    pub partial_refresh_safe: bool,
    pub force_full_refresh: bool,
    pub display_spi_handoff_required: bool,
    pub draw_algorithm_rewrite_allowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRefreshCommandDecision {
    pub requested_command: VaachakDisplayRefreshCommand,
    pub selected_command: VaachakDisplayRefreshCommand,
    pub backend_operation: VaachakDisplayBackendOperation,
    pub reason: VaachakDisplayRefreshCommandReason,
    pub safety: VaachakDisplayRefreshCommandSafety,
    pub backend: VaachakDisplayRefreshCommandExecutorBackend,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub command_selection_owned_by_vaachak: bool,
    pub partial_escalation_owned_by_vaachak: bool,
    pub display_request_owned_by_vaachak: bool,
    pub pulp_executor_selected: bool,
    pub ssd1677_draw_algorithm_moved_to_vaachak: bool,
    pub waveform_or_busy_wait_moved_to_vaachak: bool,
    pub spi_transfer_or_chip_select_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRefreshCommandExecution {
    pub decision: VaachakDisplayRefreshCommandDecision,
    pub display_request: VaachakDisplayRequest,
    pub pulp_handoff_result: VaachakHardwareBackendHandoffResult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRefreshCommandExecutorReport {
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub low_level_executor_owner: &'static str,
    pub command_selection_owned_by_vaachak: bool,
    pub partial_refresh_escalation_owned_by_vaachak: bool,
    pub display_request_construction_owned_by_vaachak: bool,
    pub native_refresh_shell_ready: bool,
    pub pulp_executor_available: bool,
    pub full_refresh_command_ok: bool,
    pub partial_refresh_command_ok: bool,
    pub partial_escalation_command_ok: bool,
    pub clear_command_ok: bool,
    pub sleep_command_ok: bool,
    pub render_metadata_command_ok: bool,
    pub ssd1677_draw_algorithm_moved_to_vaachak: bool,
    pub waveform_or_busy_wait_moved_to_vaachak: bool,
    pub spi_transfer_or_chip_select_changed: bool,
    pub storage_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakDisplayRefreshCommandDecision {
    pub const fn ok(self) -> bool {
        self.command_selection_owned_by_vaachak
            && self.partial_escalation_owned_by_vaachak
            && self.display_request_owned_by_vaachak
            && self.pulp_executor_selected
            && matches!(
                self.backend,
                VaachakDisplayRefreshCommandExecutorBackend::VaachakDisplayRefreshCommandExecutorWithPulpExecutor
            )
            && matches!(self.low_level_backend, VaachakHardwareExecutorBackend::PulpCompatibility)
            && !self.ssd1677_draw_algorithm_moved_to_vaachak
            && !self.waveform_or_busy_wait_moved_to_vaachak
            && !self.spi_transfer_or_chip_select_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakDisplayRefreshCommandExecution {
    pub const fn ok(self) -> bool {
        self.decision.ok()
            && !self.display_request.display_draw_algorithm_rewrite_allowed
            && self.display_request.spi_handoff_required
            && self.pulp_handoff_result.ok()
    }
}

impl VaachakDisplayRefreshCommandExecutorReport {
    pub const fn ok(self) -> bool {
        self.command_selection_owned_by_vaachak
            && self.partial_refresh_escalation_owned_by_vaachak
            && self.display_request_construction_owned_by_vaachak
            && self.native_refresh_shell_ready
            && self.pulp_executor_available
            && self.full_refresh_command_ok
            && self.partial_refresh_command_ok
            && self.partial_escalation_command_ok
            && self.clear_command_ok
            && self.sleep_command_ok
            && self.render_metadata_command_ok
            && !self.ssd1677_draw_algorithm_moved_to_vaachak
            && !self.waveform_or_busy_wait_moved_to_vaachak
            && !self.spi_transfer_or_chip_select_changed
            && !self.storage_behavior_changed
            && !self.input_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakDisplayBackendNativeRefreshCommandExecutor {
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_MARKER: &'static str =
        "display_backend_native_refresh_command_executor=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str =
        "VaachakDisplayRefreshCommandExecutorWithPulpExecutor";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const COMMAND_SELECTION_OWNED_BY_VAACHAK: bool = true;
    pub const PARTIAL_REFRESH_ESCALATION_OWNED_BY_VAACHAK: bool = true;
    pub const DISPLAY_REQUEST_CONSTRUCTION_OWNED_BY_VAACHAK: bool = true;
    pub const PULP_EXECUTOR_AVAILABLE: bool = true;

    pub const SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK: bool = false;
    pub const WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_TRANSFER_OR_CHIP_SELECT_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub const FULL_REFRESH_CONTEXT: VaachakDisplayRefreshCommandContext =
        VaachakDisplayRefreshCommandContext {
            requested_command: VaachakDisplayRefreshCommand::FullRefresh,
            partial_refresh_safe: true,
            force_full_refresh: true,
            display_spi_handoff_required: true,
            draw_algorithm_rewrite_allowed: false,
        };

    pub const PARTIAL_REFRESH_CONTEXT: VaachakDisplayRefreshCommandContext =
        VaachakDisplayRefreshCommandContext {
            requested_command: VaachakDisplayRefreshCommand::PartialRefresh,
            partial_refresh_safe: true,
            force_full_refresh: false,
            display_spi_handoff_required: true,
            draw_algorithm_rewrite_allowed: false,
        };

    pub const PARTIAL_UNSAFE_CONTEXT: VaachakDisplayRefreshCommandContext =
        VaachakDisplayRefreshCommandContext {
            requested_command: VaachakDisplayRefreshCommand::PartialRefresh,
            partial_refresh_safe: false,
            force_full_refresh: false,
            display_spi_handoff_required: true,
            draw_algorithm_rewrite_allowed: false,
        };

    pub fn decide_refresh_command(
        context: VaachakDisplayRefreshCommandContext,
    ) -> VaachakDisplayRefreshCommandDecision {
        let selected_command = match context.requested_command {
            VaachakDisplayRefreshCommand::PartialRefresh
                if context.force_full_refresh || !context.partial_refresh_safe =>
            {
                VaachakDisplayRefreshCommand::FullRefresh
            }
            command => command,
        };

        let reason = match (context.requested_command, selected_command) {
            (
                VaachakDisplayRefreshCommand::FullRefresh,
                VaachakDisplayRefreshCommand::FullRefresh,
            ) => VaachakDisplayRefreshCommandReason::FullRefreshRequested,
            (
                VaachakDisplayRefreshCommand::PartialRefresh,
                VaachakDisplayRefreshCommand::PartialRefresh,
            ) => VaachakDisplayRefreshCommandReason::PartialRefreshRequested,
            (
                VaachakDisplayRefreshCommand::PartialRefresh,
                VaachakDisplayRefreshCommand::FullRefresh,
            ) => VaachakDisplayRefreshCommandReason::PartialRefreshUnsafeEscalatedToFull,
            (
                VaachakDisplayRefreshCommand::ClearFrame,
                VaachakDisplayRefreshCommand::ClearFrame,
            ) => VaachakDisplayRefreshCommandReason::ClearRequested,
            (
                VaachakDisplayRefreshCommand::SleepFrame,
                VaachakDisplayRefreshCommand::SleepFrame,
            ) => VaachakDisplayRefreshCommandReason::SleepRequested,
            (
                VaachakDisplayRefreshCommand::RenderFrameMetadata,
                VaachakDisplayRefreshCommand::RenderFrameMetadata,
            ) => VaachakDisplayRefreshCommandReason::RenderMetadataRequested,
            _ => VaachakDisplayRefreshCommandReason::PartialRefreshUnsafeEscalatedToFull,
        };

        let safety = match reason {
            VaachakDisplayRefreshCommandReason::PartialRefreshUnsafeEscalatedToFull => {
                VaachakDisplayRefreshCommandSafety::EscalatedToFullRefresh
            }
            _ => VaachakDisplayRefreshCommandSafety::SafeForRequestedRefresh,
        };

        VaachakDisplayRefreshCommandDecision {
            requested_command: context.requested_command,
            selected_command,
            backend_operation: VaachakDisplayBackendNativeRefreshShell::backend_operation_for(
                selected_command,
            ),
            reason,
            safety,
            backend: VaachakDisplayRefreshCommandExecutorBackend::VaachakDisplayRefreshCommandExecutorWithPulpExecutor,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            command_selection_owned_by_vaachak: Self::COMMAND_SELECTION_OWNED_BY_VAACHAK,
            partial_escalation_owned_by_vaachak: Self::PARTIAL_REFRESH_ESCALATION_OWNED_BY_VAACHAK,
            display_request_owned_by_vaachak: Self::DISPLAY_REQUEST_CONSTRUCTION_OWNED_BY_VAACHAK,
            pulp_executor_selected: Self::PULP_EXECUTOR_AVAILABLE,
            ssd1677_draw_algorithm_moved_to_vaachak: Self::SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK,
            waveform_or_busy_wait_moved_to_vaachak: Self::WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK,
            spi_transfer_or_chip_select_changed: Self::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn display_request_for_decision(
        decision: VaachakDisplayRefreshCommandDecision,
    ) -> VaachakDisplayRequest {
        VaachakDisplayRequest {
            operation: decision.backend_operation,
            spi_handoff_required: true,
            display_draw_algorithm_rewrite_allowed: false,
        }
    }

    pub fn execute_refresh_command(
        context: VaachakDisplayRefreshCommandContext,
    ) -> VaachakDisplayRefreshCommandExecution {
        let decision = Self::decide_refresh_command(context);
        let display_request = Self::display_request_for_decision(decision);
        let backend = VaachakHardwareRuntimePulpCompatibilityBackend;
        let pulp_handoff_result =
            super::hardware_runtime_backend::VaachakDisplayExecutor::execute_display(
                &backend,
                display_request,
            );

        VaachakDisplayRefreshCommandExecution {
            decision,
            display_request,
            pulp_handoff_result,
        }
    }

    pub fn execute_full_refresh_command() -> VaachakDisplayRefreshCommandExecution {
        Self::execute_refresh_command(Self::FULL_REFRESH_CONTEXT)
    }

    pub fn execute_partial_refresh_command() -> VaachakDisplayRefreshCommandExecution {
        Self::execute_refresh_command(Self::PARTIAL_REFRESH_CONTEXT)
    }

    pub fn execute_partial_refresh_or_escalate_command() -> VaachakDisplayRefreshCommandExecution {
        Self::execute_refresh_command(Self::PARTIAL_UNSAFE_CONTEXT)
    }

    pub fn execute_clear_command() -> VaachakDisplayRefreshCommandExecution {
        Self::execute_refresh_command(VaachakDisplayRefreshCommandContext {
            requested_command: VaachakDisplayRefreshCommand::ClearFrame,
            partial_refresh_safe: true,
            force_full_refresh: false,
            display_spi_handoff_required: true,
            draw_algorithm_rewrite_allowed: false,
        })
    }

    pub fn execute_sleep_command() -> VaachakDisplayRefreshCommandExecution {
        Self::execute_refresh_command(VaachakDisplayRefreshCommandContext {
            requested_command: VaachakDisplayRefreshCommand::SleepFrame,
            partial_refresh_safe: true,
            force_full_refresh: false,
            display_spi_handoff_required: true,
            draw_algorithm_rewrite_allowed: false,
        })
    }

    pub fn execute_render_metadata_command() -> VaachakDisplayRefreshCommandExecution {
        Self::execute_refresh_command(VaachakDisplayRefreshCommandContext {
            requested_command: VaachakDisplayRefreshCommand::RenderFrameMetadata,
            partial_refresh_safe: true,
            force_full_refresh: false,
            display_spi_handoff_required: true,
            draw_algorithm_rewrite_allowed: false,
        })
    }

    pub fn report() -> VaachakDisplayRefreshCommandExecutorReport {
        VaachakDisplayRefreshCommandExecutorReport {
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            low_level_executor_owner: Self::LOW_LEVEL_EXECUTOR_OWNER,
            command_selection_owned_by_vaachak: Self::COMMAND_SELECTION_OWNED_BY_VAACHAK,
            partial_refresh_escalation_owned_by_vaachak:
                Self::PARTIAL_REFRESH_ESCALATION_OWNED_BY_VAACHAK,
            display_request_construction_owned_by_vaachak:
                Self::DISPLAY_REQUEST_CONSTRUCTION_OWNED_BY_VAACHAK,
            native_refresh_shell_ready:
                VaachakDisplayBackendNativeRefreshShell::native_refresh_mappings_ok()
                    && VaachakDisplayBackendNativeRefreshShell::route_safety_ok(),
            pulp_executor_available: VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok(),
            full_refresh_command_ok: Self::execute_full_refresh_command().ok(),
            partial_refresh_command_ok: Self::execute_partial_refresh_command().ok(),
            partial_escalation_command_ok: Self::execute_partial_refresh_or_escalate_command().ok(),
            clear_command_ok: Self::execute_clear_command().ok(),
            sleep_command_ok: Self::execute_sleep_command().ok(),
            render_metadata_command_ok: Self::execute_render_metadata_command().ok(),
            ssd1677_draw_algorithm_moved_to_vaachak: Self::SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK,
            waveform_or_busy_wait_moved_to_vaachak: Self::WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK,
            spi_transfer_or_chip_select_changed: Self::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn command_executor_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_display_backend_native_refresh_command_executor_marker() {
        if Self::command_executor_ok() {
            Self::emit_line(Self::DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_MARKER);
            Self::emit_line("display.backend.native.command_executor.command_selection.vaachak");
            Self::emit_line("display.backend.native.command_executor.partial_escalation.vaachak");
            Self::emit_line("display.backend.native.command_executor.low_level.pulp_compatible");
            Self::emit_line("display.backend.native.command_executor.behavior.preserved");
        } else {
            Self::emit_line("display_backend_native_refresh_command_executor=failed");
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
    use super::{
        VaachakDisplayBackendNativeRefreshCommandExecutor, VaachakDisplayRefreshCommandReason,
        VaachakDisplayRefreshCommandSafety,
    };

    #[test]
    fn display_backend_native_refresh_command_executor_is_ready() {
        assert!(VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok());
    }

    #[test]
    fn unsafe_partial_refresh_escalates_to_full_refresh() {
        let execution = VaachakDisplayBackendNativeRefreshCommandExecutor::execute_partial_refresh_or_escalate_command();
        assert_eq!(
            execution.decision.reason,
            VaachakDisplayRefreshCommandReason::PartialRefreshUnsafeEscalatedToFull
        );
        assert_eq!(
            execution.decision.safety,
            VaachakDisplayRefreshCommandSafety::EscalatedToFullRefresh
        );
        assert!(execution.ok());
    }
}
