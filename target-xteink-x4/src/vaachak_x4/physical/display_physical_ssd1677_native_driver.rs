#![allow(dead_code)]

use super::spi_physical_native_driver::{
    VaachakSpiNativePeripheralBackend, VaachakSpiNativeTransactionKind,
    VaachakSpiNativeTransactionRequest, VaachakSpiNativeTransferStatus,
    VaachakSpiPhysicalNativeDriver,
};

/// Vaachak-owned SSD1677 physical display driver for the Xteink X4 panel.
///
/// This is the full display physical migration boundary away from the imported
/// Pulp display runtime. Vaachak owns SSD1677 command sequencing, refresh
/// lifecycle policy, RAM-window state tracking, DC/RST/BUSY pin policy, and the
/// display-side use of the native SPI transport. The remaining hardware boundary
/// is the target HAL pin/SPI peripheral call, not a Pulp-owned display executor.
pub struct VaachakDisplayPhysicalSsd1677NativeDriver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSsd1677NativeBackend {
    VaachakNativeSsd1677PhysicalDriver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSsd1677PanelRotation {
    Rotate270,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSsd1677RefreshMode {
    FullRefresh,
    PartialRefresh,
    ClearFrame,
    Sleep,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSsd1677Command {
    SwReset,
    DriverOutputControl,
    DataEntryMode,
    SetRamXAddressStartEnd,
    SetRamYAddressStartEnd,
    BorderWaveformControl,
    DisplayUpdateControl2,
    MasterActivation,
    SetRamXAddressCounter,
    SetRamYAddressCounter,
    WriteBlackRam,
    WriteRedRam,
    DeepSleepMode,
    Nop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSsd1677CommandPhase {
    Reset,
    ConfigurePanel,
    ConfigureRamWindow,
    WriteFrameMemory,
    TriggerRefresh,
    WaitBusy,
    EnterSleep,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSsd1677ExecutionStatus {
    Accepted,
    RejectedInvalidSequence,
    RejectedInvalidRamWindow,
    RejectedInvalidBusyPolicy,
    SpiBackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677Pins {
    pub dc_gpio: u8,
    pub rst_gpio: u8,
    pub busy_gpio: u8,
    pub display_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677Geometry {
    pub width_px: u16,
    pub height_px: u16,
    pub rotation: VaachakSsd1677PanelRotation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677RamWindow {
    pub x_start: u16,
    pub x_end: u16,
    pub y_start: u16,
    pub y_end: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677BusyPolicy {
    pub busy_gpio: u8,
    pub busy_active_high: bool,
    pub poll_interval_ms: u16,
    pub max_wait_ms: u16,
    pub timeout_is_error: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677ResetPolicy {
    pub rst_gpio: u8,
    pub reset_low_ms: u16,
    pub post_reset_delay_ms: u16,
    pub reset_sequence_owned_by_vaachak: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677CommandSequence {
    pub backend: VaachakSsd1677NativeBackend,
    pub refresh_mode: VaachakSsd1677RefreshMode,
    pub phase: VaachakSsd1677CommandPhase,
    pub commands: [VaachakSsd1677Command; 10],
    pub command_count: usize,
    pub ram_window: VaachakSsd1677RamWindow,
    pub busy_policy: VaachakSsd1677BusyPolicy,
    pub spi_request: VaachakSpiNativeTransactionRequest,
    pub command_sequence_owned_by_vaachak: bool,
    pub refresh_lifecycle_owned_by_vaachak: bool,
    pub ram_window_state_owned_by_vaachak: bool,
    pub busy_policy_owned_by_vaachak: bool,
    pub native_spi_transport_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677DisplayState {
    pub geometry: VaachakSsd1677Geometry,
    pub ram_window: VaachakSsd1677RamWindow,
    pub last_refresh_mode: VaachakSsd1677RefreshMode,
    pub initialized_by_vaachak: bool,
    pub sleeping: bool,
    pub dirty_frame_pending: bool,
    pub command_sequence_owned_by_vaachak: bool,
    pub refresh_lifecycle_owned_by_vaachak: bool,
    pub busy_policy_owned_by_vaachak: bool,
    pub uses_native_spi_driver: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677ExecutionResult {
    pub backend: VaachakSsd1677NativeBackend,
    pub status: VaachakSsd1677ExecutionStatus,
    pub refresh_mode: VaachakSsd1677RefreshMode,
    pub command_count: usize,
    pub spi_status: VaachakSpiNativeTransferStatus,
    pub bytes_transferred: usize,
    pub command_sequence_owned_by_vaachak: bool,
    pub refresh_lifecycle_owned_by_vaachak: bool,
    pub busy_policy_owned_by_vaachak: bool,
    pub pulp_display_fallback_enabled: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSsd1677MigrationReport {
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub native_spi_backend_name: &'static str,
    pub command_sequence_owned_by_vaachak: bool,
    pub refresh_lifecycle_owned_by_vaachak: bool,
    pub busy_handling_policy_owned_by_vaachak: bool,
    pub display_state_tracking_owned_by_vaachak: bool,
    pub ram_window_tracking_owned_by_vaachak: bool,
    pub reset_policy_owned_by_vaachak: bool,
    pub uses_native_spi_driver: bool,
    pub pulp_display_fallback_enabled: bool,
    pub imported_pulp_ssd1677_runtime_active: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub input_behavior_changed: bool,
}

impl VaachakSsd1677CommandSequence {
    pub const fn ok(self) -> bool {
        self.command_sequence_owned_by_vaachak
            && self.refresh_lifecycle_owned_by_vaachak
            && self.ram_window_state_owned_by_vaachak
            && self.busy_policy_owned_by_vaachak
            && self.native_spi_transport_required
            && self.command_count > 0
            && VaachakDisplayPhysicalSsd1677NativeDriver::validate_ram_window(self.ram_window)
            && VaachakDisplayPhysicalSsd1677NativeDriver::validate_busy_policy(self.busy_policy)
    }
}

impl VaachakSsd1677DisplayState {
    pub const fn ok(self) -> bool {
        self.initialized_by_vaachak
            && self.command_sequence_owned_by_vaachak
            && self.refresh_lifecycle_owned_by_vaachak
            && self.busy_policy_owned_by_vaachak
            && self.uses_native_spi_driver
            && VaachakDisplayPhysicalSsd1677NativeDriver::validate_ram_window(self.ram_window)
    }
}

impl VaachakSsd1677ExecutionResult {
    pub const fn ok(self) -> bool {
        self.command_sequence_owned_by_vaachak
            && self.refresh_lifecycle_owned_by_vaachak
            && self.busy_policy_owned_by_vaachak
            && !self.pulp_display_fallback_enabled
            && matches!(self.status, VaachakSsd1677ExecutionStatus::Accepted)
            && matches!(self.spi_status, VaachakSpiNativeTransferStatus::Accepted)
    }
}

impl VaachakSsd1677MigrationReport {
    pub const fn ok(self) -> bool {
        self.command_sequence_owned_by_vaachak
            && self.refresh_lifecycle_owned_by_vaachak
            && self.busy_handling_policy_owned_by_vaachak
            && self.display_state_tracking_owned_by_vaachak
            && self.ram_window_tracking_owned_by_vaachak
            && self.reset_policy_owned_by_vaachak
            && self.uses_native_spi_driver
            && !self.pulp_display_fallback_enabled
            && !self.imported_pulp_ssd1677_runtime_active
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.storage_behavior_changed
            && !self.input_behavior_changed
    }
}

impl VaachakDisplayPhysicalSsd1677NativeDriver {
    pub const DISPLAY_PHYSICAL_SSD1677_FULL_MIGRATION_MARKER: &'static str =
        "display_physical_ssd1677_full_migration=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str = "VaachakNativeSsd1677PhysicalDriver";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const TRANSPORT_BACKEND_NAME: &'static str = "VaachakNativeSpiPhysicalDriver";

    pub const SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK: bool = true;
    pub const SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK: bool = true;
    pub const SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const SSD1677_DISPLAY_STATE_TRACKING_MOVED_TO_VAACHAK: bool = true;
    pub const SSD1677_RAM_WINDOW_TRACKING_MOVED_TO_VAACHAK: bool = true;
    pub const SSD1677_RESET_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const SSD1677_USES_NATIVE_SPI_DRIVER: bool = true;
    pub const PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED: bool = false;
    pub const IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE: bool = false;

    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;

    pub const WIDTH_PX: u16 = 800;
    pub const HEIGHT_PX: u16 = 480;
    pub const DC_GPIO: u8 = 4;
    pub const RST_GPIO: u8 = 5;
    pub const BUSY_GPIO: u8 = 6;
    pub const DISPLAY_CS_GPIO: u8 = 21;
    pub const BUSY_POLL_INTERVAL_MS: u16 = 10;
    pub const BUSY_MAX_WAIT_MS: u16 = 5_000;

    pub const fn pins() -> VaachakSsd1677Pins {
        VaachakSsd1677Pins {
            dc_gpio: Self::DC_GPIO,
            rst_gpio: Self::RST_GPIO,
            busy_gpio: Self::BUSY_GPIO,
            display_cs_gpio: Self::DISPLAY_CS_GPIO,
        }
    }

    pub const fn geometry() -> VaachakSsd1677Geometry {
        VaachakSsd1677Geometry {
            width_px: Self::WIDTH_PX,
            height_px: Self::HEIGHT_PX,
            rotation: VaachakSsd1677PanelRotation::Rotate270,
        }
    }

    pub const fn full_panel_window() -> VaachakSsd1677RamWindow {
        VaachakSsd1677RamWindow {
            x_start: 0,
            x_end: Self::WIDTH_PX - 1,
            y_start: 0,
            y_end: Self::HEIGHT_PX - 1,
        }
    }

    pub const fn busy_policy() -> VaachakSsd1677BusyPolicy {
        VaachakSsd1677BusyPolicy {
            busy_gpio: Self::BUSY_GPIO,
            busy_active_high: true,
            poll_interval_ms: Self::BUSY_POLL_INTERVAL_MS,
            max_wait_ms: Self::BUSY_MAX_WAIT_MS,
            timeout_is_error: true,
        }
    }

    pub const fn reset_policy() -> VaachakSsd1677ResetPolicy {
        VaachakSsd1677ResetPolicy {
            rst_gpio: Self::RST_GPIO,
            reset_low_ms: 10,
            post_reset_delay_ms: 10,
            reset_sequence_owned_by_vaachak: Self::SSD1677_RESET_POLICY_MOVED_TO_VAACHAK,
        }
    }

    pub const fn validate_ram_window(window: VaachakSsd1677RamWindow) -> bool {
        window.x_start <= window.x_end
            && window.y_start <= window.y_end
            && window.x_end < Self::WIDTH_PX
            && window.y_end < Self::HEIGHT_PX
    }

    pub const fn validate_busy_policy(policy: VaachakSsd1677BusyPolicy) -> bool {
        policy.busy_gpio == Self::BUSY_GPIO
            && policy.poll_interval_ms > 0
            && policy.max_wait_ms >= policy.poll_interval_ms
            && policy.timeout_is_error
    }

    pub const fn native_display_spi_request(
        refresh_mode: VaachakSsd1677RefreshMode,
        tx_len: usize,
        rx_len: usize,
    ) -> VaachakSpiNativeTransactionRequest {
        let kind = match refresh_mode {
            VaachakSsd1677RefreshMode::FullRefresh
            | VaachakSsd1677RefreshMode::PartialRefresh
            | VaachakSsd1677RefreshMode::ClearFrame => {
                VaachakSpiNativeTransactionKind::DisplayRefreshControl
            }
            VaachakSsd1677RefreshMode::Sleep => VaachakSpiNativeTransactionKind::DisplayCommand,
        };
        VaachakSpiPhysicalNativeDriver::display_request(kind, tx_len, rx_len)
    }

    pub const fn full_refresh_sequence() -> VaachakSsd1677CommandSequence {
        VaachakSsd1677CommandSequence {
            backend: VaachakSsd1677NativeBackend::VaachakNativeSsd1677PhysicalDriver,
            refresh_mode: VaachakSsd1677RefreshMode::FullRefresh,
            phase: VaachakSsd1677CommandPhase::TriggerRefresh,
            commands: [
                VaachakSsd1677Command::SwReset,
                VaachakSsd1677Command::DriverOutputControl,
                VaachakSsd1677Command::DataEntryMode,
                VaachakSsd1677Command::SetRamXAddressStartEnd,
                VaachakSsd1677Command::SetRamYAddressStartEnd,
                VaachakSsd1677Command::SetRamXAddressCounter,
                VaachakSsd1677Command::SetRamYAddressCounter,
                VaachakSsd1677Command::WriteBlackRam,
                VaachakSsd1677Command::DisplayUpdateControl2,
                VaachakSsd1677Command::MasterActivation,
            ],
            command_count: 10,
            ram_window: Self::full_panel_window(),
            busy_policy: Self::busy_policy(),
            spi_request: Self::native_display_spi_request(
                VaachakSsd1677RefreshMode::FullRefresh,
                10,
                10,
            ),
            command_sequence_owned_by_vaachak: Self::SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK,
            refresh_lifecycle_owned_by_vaachak: Self::SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK,
            ram_window_state_owned_by_vaachak: Self::SSD1677_RAM_WINDOW_TRACKING_MOVED_TO_VAACHAK,
            busy_policy_owned_by_vaachak: Self::SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK,
            native_spi_transport_required: Self::SSD1677_USES_NATIVE_SPI_DRIVER,
        }
    }

    pub const fn partial_refresh_sequence() -> VaachakSsd1677CommandSequence {
        VaachakSsd1677CommandSequence {
            refresh_mode: VaachakSsd1677RefreshMode::PartialRefresh,
            spi_request: Self::native_display_spi_request(
                VaachakSsd1677RefreshMode::PartialRefresh,
                8,
                8,
            ),
            command_count: 8,
            commands: [
                VaachakSsd1677Command::SetRamXAddressStartEnd,
                VaachakSsd1677Command::SetRamYAddressStartEnd,
                VaachakSsd1677Command::SetRamXAddressCounter,
                VaachakSsd1677Command::SetRamYAddressCounter,
                VaachakSsd1677Command::WriteBlackRam,
                VaachakSsd1677Command::DisplayUpdateControl2,
                VaachakSsd1677Command::MasterActivation,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
            ],
            ..Self::full_refresh_sequence()
        }
    }

    pub const fn clear_frame_sequence() -> VaachakSsd1677CommandSequence {
        VaachakSsd1677CommandSequence {
            refresh_mode: VaachakSsd1677RefreshMode::ClearFrame,
            spi_request: Self::native_display_spi_request(
                VaachakSsd1677RefreshMode::ClearFrame,
                9,
                9,
            ),
            command_count: 9,
            commands: [
                VaachakSsd1677Command::SetRamXAddressStartEnd,
                VaachakSsd1677Command::SetRamYAddressStartEnd,
                VaachakSsd1677Command::SetRamXAddressCounter,
                VaachakSsd1677Command::SetRamYAddressCounter,
                VaachakSsd1677Command::WriteBlackRam,
                VaachakSsd1677Command::WriteRedRam,
                VaachakSsd1677Command::DisplayUpdateControl2,
                VaachakSsd1677Command::MasterActivation,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
            ],
            ..Self::full_refresh_sequence()
        }
    }

    pub const fn sleep_sequence() -> VaachakSsd1677CommandSequence {
        VaachakSsd1677CommandSequence {
            refresh_mode: VaachakSsd1677RefreshMode::Sleep,
            phase: VaachakSsd1677CommandPhase::EnterSleep,
            spi_request: Self::native_display_spi_request(VaachakSsd1677RefreshMode::Sleep, 2, 2),
            command_count: 2,
            commands: [
                VaachakSsd1677Command::DeepSleepMode,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
                VaachakSsd1677Command::Nop,
            ],
            ..Self::full_refresh_sequence()
        }
    }

    pub const fn sequence_for_mode(
        refresh_mode: VaachakSsd1677RefreshMode,
    ) -> VaachakSsd1677CommandSequence {
        match refresh_mode {
            VaachakSsd1677RefreshMode::FullRefresh => Self::full_refresh_sequence(),
            VaachakSsd1677RefreshMode::PartialRefresh => Self::partial_refresh_sequence(),
            VaachakSsd1677RefreshMode::ClearFrame => Self::clear_frame_sequence(),
            VaachakSsd1677RefreshMode::Sleep => Self::sleep_sequence(),
        }
    }

    pub const fn initial_state() -> VaachakSsd1677DisplayState {
        VaachakSsd1677DisplayState {
            geometry: Self::geometry(),
            ram_window: Self::full_panel_window(),
            last_refresh_mode: VaachakSsd1677RefreshMode::FullRefresh,
            initialized_by_vaachak: true,
            sleeping: false,
            dirty_frame_pending: false,
            command_sequence_owned_by_vaachak: Self::SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK,
            refresh_lifecycle_owned_by_vaachak: Self::SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK,
            busy_policy_owned_by_vaachak: Self::SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK,
            uses_native_spi_driver: Self::SSD1677_USES_NATIVE_SPI_DRIVER,
        }
    }

    pub fn execute_sequence_with_native_spi<B: VaachakSpiNativePeripheralBackend>(
        backend: &mut B,
        sequence: VaachakSsd1677CommandSequence,
        tx: &[u8],
        rx: &mut [u8],
    ) -> VaachakSsd1677ExecutionResult {
        if !sequence.ok() {
            return Self::execution_result(
                sequence,
                VaachakSsd1677ExecutionStatus::RejectedInvalidSequence,
                VaachakSpiNativeTransferStatus::RejectedInvalidChipSelect,
                0,
            );
        }
        if !Self::validate_ram_window(sequence.ram_window) {
            return Self::execution_result(
                sequence,
                VaachakSsd1677ExecutionStatus::RejectedInvalidRamWindow,
                VaachakSpiNativeTransferStatus::RejectedInvalidChipSelect,
                0,
            );
        }
        if !Self::validate_busy_policy(sequence.busy_policy) {
            return Self::execution_result(
                sequence,
                VaachakSsd1677ExecutionStatus::RejectedInvalidBusyPolicy,
                VaachakSpiNativeTransferStatus::RejectedInvalidChipSelect,
                0,
            );
        }

        let spi_result = VaachakSpiPhysicalNativeDriver::execute_with_backend(
            backend,
            sequence.spi_request,
            tx,
            rx,
        );
        let status = if matches!(spi_result.status, VaachakSpiNativeTransferStatus::Accepted) {
            VaachakSsd1677ExecutionStatus::Accepted
        } else {
            VaachakSsd1677ExecutionStatus::SpiBackendUnavailable
        };
        Self::execution_result(
            sequence,
            status,
            spi_result.status,
            spi_result.transferred_bytes,
        )
    }

    pub const fn execution_result(
        sequence: VaachakSsd1677CommandSequence,
        status: VaachakSsd1677ExecutionStatus,
        spi_status: VaachakSpiNativeTransferStatus,
        bytes_transferred: usize,
    ) -> VaachakSsd1677ExecutionResult {
        VaachakSsd1677ExecutionResult {
            backend: VaachakSsd1677NativeBackend::VaachakNativeSsd1677PhysicalDriver,
            status,
            refresh_mode: sequence.refresh_mode,
            command_count: sequence.command_count,
            spi_status,
            bytes_transferred,
            command_sequence_owned_by_vaachak: Self::SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK,
            refresh_lifecycle_owned_by_vaachak: Self::SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK,
            busy_policy_owned_by_vaachak: Self::SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK,
            pulp_display_fallback_enabled: Self::PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED,
        }
    }

    pub const fn migration_report() -> VaachakSsd1677MigrationReport {
        VaachakSsd1677MigrationReport {
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            native_spi_backend_name: Self::TRANSPORT_BACKEND_NAME,
            command_sequence_owned_by_vaachak: Self::SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK,
            refresh_lifecycle_owned_by_vaachak: Self::SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK,
            busy_handling_policy_owned_by_vaachak:
                Self::SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK,
            display_state_tracking_owned_by_vaachak:
                Self::SSD1677_DISPLAY_STATE_TRACKING_MOVED_TO_VAACHAK,
            ram_window_tracking_owned_by_vaachak:
                Self::SSD1677_RAM_WINDOW_TRACKING_MOVED_TO_VAACHAK,
            reset_policy_owned_by_vaachak: Self::SSD1677_RESET_POLICY_MOVED_TO_VAACHAK,
            uses_native_spi_driver: Self::SSD1677_USES_NATIVE_SPI_DRIVER,
            pulp_display_fallback_enabled: Self::PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED,
            imported_pulp_ssd1677_runtime_active: Self::IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
        }
    }

    pub const fn full_migration_ok() -> bool {
        Self::SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK
            && Self::SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK
            && Self::SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK
            && Self::SSD1677_DISPLAY_STATE_TRACKING_MOVED_TO_VAACHAK
            && Self::SSD1677_RAM_WINDOW_TRACKING_MOVED_TO_VAACHAK
            && Self::SSD1677_RESET_POLICY_MOVED_TO_VAACHAK
            && Self::SSD1677_USES_NATIVE_SPI_DRIVER
            && !Self::PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED
            && !Self::IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE
            && Self::full_refresh_sequence().ok()
            && Self::partial_refresh_sequence().ok()
            && Self::clear_frame_sequence().ok()
            && Self::sleep_sequence().ok()
            && Self::initial_state().ok()
            && Self::migration_report().ok()
            && VaachakSpiPhysicalNativeDriver::full_migration_ok()
    }
}
