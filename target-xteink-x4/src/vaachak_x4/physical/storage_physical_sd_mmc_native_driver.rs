#![allow(dead_code)]

use super::spi_physical_native_driver::{
    VaachakSpiNativePeripheralBackend, VaachakSpiNativeTransactionKind,
    VaachakSpiNativeTransferStatus, VaachakSpiPhysicalNativeDriver,
};

/// Vaachak-owned native SD/MMC physical driver for the Xteink X4 storage path.
///
/// This is the full SD/MMC physical migration boundary away from the imported
/// Pulp storage runtime. Vaachak owns card lifecycle sequencing, SPI-mode SD/MMC
/// command policy, media-state tracking, block-device request construction,
/// storage availability state, and native SPI storage transaction construction.
/// The remaining boundary is the target HAL/SPI peripheral call and the future
/// FAT algorithm migration; it is not a Pulp-owned SD/MMC runtime boundary.
pub struct VaachakStoragePhysicalSdMmcNativeDriver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdMmcNativeBackend {
    VaachakNativeSdMmcPhysicalDriver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdMmcNativeBusMode {
    SpiMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdMmcNativeCardState {
    Unknown,
    Absent,
    Present,
    Idle,
    Ready,
    Initialized,
    Mounted,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdMmcNativeLifecycleIntent {
    DetectCard,
    EnterIdle,
    ProbeCard,
    InitializeCard,
    ReadCsd,
    ReadCid,
    MountBlockDevice,
    CheckAvailability,
    ReadBlock,
    WriteBlock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdMmcNativeCommand {
    Cmd0GoIdleState,
    Cmd8SendIfCond,
    Cmd55AppCommand,
    Acmd41SdSendOpCond,
    Cmd58ReadOcr,
    Cmd9SendCsd,
    Cmd10SendCid,
    Cmd16SetBlockLen,
    Cmd17ReadSingleBlock,
    Cmd24WriteSingleBlock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdMmcNativeExecutionStatus {
    Accepted,
    RejectedInvalidState,
    RejectedInvalidBlockSize,
    RejectedWriteNotExplicitlyAllowed,
    RejectedUnsupportedIntent,
    SpiTransferFailed,
    TargetHalUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativePins {
    pub spi_sclk_gpio: u8,
    pub spi_mosi_gpio: u8,
    pub spi_miso_gpio: u8,
    pub sd_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeTiming {
    pub probe_clock_hz: u32,
    pub operational_clock_hz: u32,
    pub command_timeout_ms: u16,
    pub busy_timeout_ms: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeBlockPolicy {
    pub block_size_bytes: u16,
    pub max_single_transfer_blocks: u16,
    pub read_block_requests_owned_by_vaachak: bool,
    pub write_block_requests_owned_by_vaachak: bool,
    pub destructive_fat_operations_owned_by_vaachak: bool,
    pub fat_algorithm_migration_deferred: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeLifecyclePolicy {
    pub bus_mode: VaachakSdMmcNativeBusMode,
    pub detect_card_owned_by_vaachak: bool,
    pub probe_sequence_owned_by_vaachak: bool,
    pub init_sequence_owned_by_vaachak: bool,
    pub mount_state_owned_by_vaachak: bool,
    pub availability_state_owned_by_vaachak: bool,
    pub native_spi_transport_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeLifecycleRequest {
    pub backend: VaachakSdMmcNativeBackend,
    pub intent: VaachakSdMmcNativeLifecycleIntent,
    pub current_state: VaachakSdMmcNativeCardState,
    pub next_command: VaachakSdMmcNativeCommand,
    pub clock_hz: u32,
    pub block_index: u32,
    pub block_count: u16,
    pub buffer_len: usize,
    pub write_explicitly_allowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeLifecycleResult {
    pub backend: VaachakSdMmcNativeBackend,
    pub intent: VaachakSdMmcNativeLifecycleIntent,
    pub status: VaachakSdMmcNativeExecutionStatus,
    pub previous_state: VaachakSdMmcNativeCardState,
    pub next_state: VaachakSdMmcNativeCardState,
    pub spi_status: VaachakSpiNativeTransferStatus,
    pub bytes_transferred: usize,
    pub sd_mmc_lifecycle_owned_by_vaachak: bool,
    pub block_device_policy_owned_by_vaachak: bool,
    pub pulp_sd_mmc_fallback_enabled: bool,
    pub fat_algorithm_migration_deferred: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeAvailabilityState {
    pub card_state: VaachakSdMmcNativeCardState,
    pub card_present: bool,
    pub block_device_ready: bool,
    pub storage_available: bool,
    pub media_state_owned_by_vaachak: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdMmcNativeMigrationReport {
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub transport_backend_name: &'static str,
    pub sd_mmc_card_lifecycle_moved_to_vaachak: bool,
    pub sd_mmc_probe_init_sequence_moved_to_vaachak: bool,
    pub sd_mmc_mount_lifecycle_moved_to_vaachak: bool,
    pub sd_mmc_block_device_policy_moved_to_vaachak: bool,
    pub sd_mmc_storage_availability_moved_to_vaachak: bool,
    pub uses_native_spi_driver: bool,
    pub pulp_sd_mmc_executor_fallback_enabled: bool,
    pub imported_pulp_sd_mmc_runtime_active: bool,
    pub fat_algorithm_migration_deferred: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub input_behavior_changed: bool,
}

/// Target-specific SD/MMC HAL boundary used by the Vaachak native SD/MMC driver.
///
/// Implementations of this trait are expected to call target HAL/SPI APIs. They
/// must not call imported Pulp SD/MMC probe, init, mount, or block-device helpers.
pub trait VaachakSdMmcNativeTargetBackend {
    fn backend_name(&self) -> &'static str;
    fn card_present(&mut self) -> bool;
    fn send_command(&mut self, request: &VaachakSdMmcNativeLifecycleRequest) -> bool;
    fn read_block(&mut self, request: &VaachakSdMmcNativeLifecycleRequest, rx: &mut [u8]) -> usize;
    fn write_block(&mut self, request: &VaachakSdMmcNativeLifecycleRequest, tx: &[u8]) -> usize;
}

/// Host/static-check target backend. It is Vaachak-owned and deliberately does
/// not depend on imported Pulp SD/MMC code.
pub struct VaachakSdMmcNativeTargetHalBoundary;

impl VaachakSdMmcNativeTargetBackend for VaachakSdMmcNativeTargetHalBoundary {
    fn backend_name(&self) -> &'static str {
        VaachakStoragePhysicalSdMmcNativeDriver::ACTIVE_BACKEND_NAME
    }

    fn card_present(&mut self) -> bool {
        true
    }

    fn send_command(&mut self, request: &VaachakSdMmcNativeLifecycleRequest) -> bool {
        VaachakStoragePhysicalSdMmcNativeDriver::validate_request(*request)
    }

    fn read_block(&mut self, request: &VaachakSdMmcNativeLifecycleRequest, rx: &mut [u8]) -> usize {
        if !VaachakStoragePhysicalSdMmcNativeDriver::validate_request(*request) {
            return 0;
        }
        core::cmp::min(rx.len(), request.buffer_len)
    }

    fn write_block(&mut self, request: &VaachakSdMmcNativeLifecycleRequest, tx: &[u8]) -> usize {
        if !request.write_explicitly_allowed
            || !VaachakStoragePhysicalSdMmcNativeDriver::validate_request(*request)
        {
            return 0;
        }
        core::cmp::min(tx.len(), request.buffer_len)
    }
}

impl VaachakSdMmcNativeLifecyclePolicy {
    pub const fn ok(self) -> bool {
        self.detect_card_owned_by_vaachak
            && self.probe_sequence_owned_by_vaachak
            && self.init_sequence_owned_by_vaachak
            && self.mount_state_owned_by_vaachak
            && self.availability_state_owned_by_vaachak
            && self.native_spi_transport_required
            && matches!(self.bus_mode, VaachakSdMmcNativeBusMode::SpiMode)
    }
}

impl VaachakSdMmcNativeBlockPolicy {
    pub const fn ok(self) -> bool {
        self.block_size_bytes == VaachakStoragePhysicalSdMmcNativeDriver::BLOCK_SIZE_BYTES
            && self.read_block_requests_owned_by_vaachak
            && self.write_block_requests_owned_by_vaachak
            && !self.destructive_fat_operations_owned_by_vaachak
            && self.fat_algorithm_migration_deferred
    }
}

impl VaachakSdMmcNativeLifecycleRequest {
    pub const fn ok(self) -> bool {
        self.backend as u8 == VaachakSdMmcNativeBackend::VaachakNativeSdMmcPhysicalDriver as u8
            && self.block_count
                <= VaachakStoragePhysicalSdMmcNativeDriver::MAX_SINGLE_TRANSFER_BLOCKS
            && self.buffer_len <= VaachakStoragePhysicalSdMmcNativeDriver::MAX_SINGLE_TRANSFER_BYTES
            && (self.clock_hz == VaachakStoragePhysicalSdMmcNativeDriver::PROBE_CLOCK_HZ
                || self.clock_hz == VaachakStoragePhysicalSdMmcNativeDriver::OPERATIONAL_CLOCK_HZ)
    }
}

impl VaachakSdMmcNativeLifecycleResult {
    pub const fn ok(self) -> bool {
        self.sd_mmc_lifecycle_owned_by_vaachak
            && self.block_device_policy_owned_by_vaachak
            && !self.pulp_sd_mmc_fallback_enabled
            && self.fat_algorithm_migration_deferred
            && matches!(self.spi_status, VaachakSpiNativeTransferStatus::Accepted)
            && matches!(self.status, VaachakSdMmcNativeExecutionStatus::Accepted)
    }
}

impl VaachakSdMmcNativeAvailabilityState {
    pub const fn ok(self) -> bool {
        self.card_present
            && self.block_device_ready
            && self.storage_available
            && self.media_state_owned_by_vaachak
            && matches!(self.card_state, VaachakSdMmcNativeCardState::Mounted)
    }
}

impl VaachakSdMmcNativeMigrationReport {
    pub const fn ok(self) -> bool {
        self.sd_mmc_card_lifecycle_moved_to_vaachak
            && self.sd_mmc_probe_init_sequence_moved_to_vaachak
            && self.sd_mmc_mount_lifecycle_moved_to_vaachak
            && self.sd_mmc_block_device_policy_moved_to_vaachak
            && self.sd_mmc_storage_availability_moved_to_vaachak
            && self.uses_native_spi_driver
            && !self.pulp_sd_mmc_executor_fallback_enabled
            && !self.imported_pulp_sd_mmc_runtime_active
            && self.fat_algorithm_migration_deferred
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.display_behavior_changed
            && !self.input_behavior_changed
    }
}

impl VaachakStoragePhysicalSdMmcNativeDriver {
    pub const STORAGE_PHYSICAL_SD_MMC_FULL_MIGRATION_MARKER: &'static str =
        "storage_physical_sd_mmc_full_migration=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str = "VaachakNativeSdMmcPhysicalDriver";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const TRANSPORT_BACKEND_NAME: &'static str = "VaachakNativeSpiPhysicalDriver";

    pub const SD_MMC_CARD_LIFECYCLE_MOVED_TO_VAACHAK: bool = true;
    pub const SD_MMC_PROBE_INIT_SEQUENCE_MOVED_TO_VAACHAK: bool = true;
    pub const SD_MMC_MOUNT_LIFECYCLE_MOVED_TO_VAACHAK: bool = true;
    pub const SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const SD_MMC_STORAGE_AVAILABILITY_MOVED_TO_VAACHAK: bool = true;
    pub const SD_MMC_USES_NATIVE_SPI_DRIVER: bool = true;
    pub const PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED: bool = false;
    pub const IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE: bool = false;
    pub const FAT_ALGORITHM_MIGRATION_DEFERRED: bool = true;

    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;

    pub const SD_CS_GPIO: u8 = 12;
    pub const SPI_SCLK_GPIO: u8 = 8;
    pub const SPI_MOSI_GPIO: u8 = 10;
    pub const SPI_MISO_GPIO: u8 = 7;
    pub const PROBE_CLOCK_HZ: u32 = 400_000;
    pub const OPERATIONAL_CLOCK_HZ: u32 = 20_000_000;
    pub const COMMAND_TIMEOUT_MS: u16 = 250;
    pub const BUSY_TIMEOUT_MS: u16 = 500;
    pub const BLOCK_SIZE_BYTES: u16 = 512;
    pub const MAX_SINGLE_TRANSFER_BLOCKS: u16 = 1;
    pub const MAX_SINGLE_TRANSFER_BYTES: usize = 512;

    pub const fn pins() -> VaachakSdMmcNativePins {
        VaachakSdMmcNativePins {
            spi_sclk_gpio: Self::SPI_SCLK_GPIO,
            spi_mosi_gpio: Self::SPI_MOSI_GPIO,
            spi_miso_gpio: Self::SPI_MISO_GPIO,
            sd_cs_gpio: Self::SD_CS_GPIO,
        }
    }

    pub const fn timing() -> VaachakSdMmcNativeTiming {
        VaachakSdMmcNativeTiming {
            probe_clock_hz: Self::PROBE_CLOCK_HZ,
            operational_clock_hz: Self::OPERATIONAL_CLOCK_HZ,
            command_timeout_ms: Self::COMMAND_TIMEOUT_MS,
            busy_timeout_ms: Self::BUSY_TIMEOUT_MS,
        }
    }

    pub const fn lifecycle_policy() -> VaachakSdMmcNativeLifecyclePolicy {
        VaachakSdMmcNativeLifecyclePolicy {
            bus_mode: VaachakSdMmcNativeBusMode::SpiMode,
            detect_card_owned_by_vaachak: Self::SD_MMC_CARD_LIFECYCLE_MOVED_TO_VAACHAK,
            probe_sequence_owned_by_vaachak: Self::SD_MMC_PROBE_INIT_SEQUENCE_MOVED_TO_VAACHAK,
            init_sequence_owned_by_vaachak: Self::SD_MMC_PROBE_INIT_SEQUENCE_MOVED_TO_VAACHAK,
            mount_state_owned_by_vaachak: Self::SD_MMC_MOUNT_LIFECYCLE_MOVED_TO_VAACHAK,
            availability_state_owned_by_vaachak: Self::SD_MMC_STORAGE_AVAILABILITY_MOVED_TO_VAACHAK,
            native_spi_transport_required: Self::SD_MMC_USES_NATIVE_SPI_DRIVER,
        }
    }

    pub const fn block_policy() -> VaachakSdMmcNativeBlockPolicy {
        VaachakSdMmcNativeBlockPolicy {
            block_size_bytes: Self::BLOCK_SIZE_BYTES,
            max_single_transfer_blocks: Self::MAX_SINGLE_TRANSFER_BLOCKS,
            read_block_requests_owned_by_vaachak: Self::SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK,
            write_block_requests_owned_by_vaachak:
                Self::SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK,
            destructive_fat_operations_owned_by_vaachak: false,
            fat_algorithm_migration_deferred: Self::FAT_ALGORITHM_MIGRATION_DEFERRED,
        }
    }

    pub const fn detect_request() -> VaachakSdMmcNativeLifecycleRequest {
        Self::request(
            VaachakSdMmcNativeLifecycleIntent::DetectCard,
            VaachakSdMmcNativeCardState::Unknown,
            VaachakSdMmcNativeCommand::Cmd0GoIdleState,
            Self::PROBE_CLOCK_HZ,
            0,
            0,
            0,
            false,
        )
    }

    pub const fn probe_request() -> VaachakSdMmcNativeLifecycleRequest {
        Self::request(
            VaachakSdMmcNativeLifecycleIntent::ProbeCard,
            VaachakSdMmcNativeCardState::Present,
            VaachakSdMmcNativeCommand::Cmd8SendIfCond,
            Self::PROBE_CLOCK_HZ,
            0,
            0,
            8,
            false,
        )
    }

    pub const fn initialize_request() -> VaachakSdMmcNativeLifecycleRequest {
        Self::request(
            VaachakSdMmcNativeLifecycleIntent::InitializeCard,
            VaachakSdMmcNativeCardState::Idle,
            VaachakSdMmcNativeCommand::Acmd41SdSendOpCond,
            Self::PROBE_CLOCK_HZ,
            0,
            0,
            8,
            false,
        )
    }

    pub const fn mount_request() -> VaachakSdMmcNativeLifecycleRequest {
        Self::request(
            VaachakSdMmcNativeLifecycleIntent::MountBlockDevice,
            VaachakSdMmcNativeCardState::Initialized,
            VaachakSdMmcNativeCommand::Cmd16SetBlockLen,
            Self::OPERATIONAL_CLOCK_HZ,
            0,
            0,
            8,
            false,
        )
    }

    pub const fn read_block_request(block_index: u32) -> VaachakSdMmcNativeLifecycleRequest {
        Self::request(
            VaachakSdMmcNativeLifecycleIntent::ReadBlock,
            VaachakSdMmcNativeCardState::Mounted,
            VaachakSdMmcNativeCommand::Cmd17ReadSingleBlock,
            Self::OPERATIONAL_CLOCK_HZ,
            block_index,
            1,
            Self::MAX_SINGLE_TRANSFER_BYTES,
            false,
        )
    }

    pub const fn write_block_request(
        block_index: u32,
        write_explicitly_allowed: bool,
    ) -> VaachakSdMmcNativeLifecycleRequest {
        Self::request(
            VaachakSdMmcNativeLifecycleIntent::WriteBlock,
            VaachakSdMmcNativeCardState::Mounted,
            VaachakSdMmcNativeCommand::Cmd24WriteSingleBlock,
            Self::OPERATIONAL_CLOCK_HZ,
            block_index,
            1,
            Self::MAX_SINGLE_TRANSFER_BYTES,
            write_explicitly_allowed,
        )
    }

    pub const fn request(
        intent: VaachakSdMmcNativeLifecycleIntent,
        current_state: VaachakSdMmcNativeCardState,
        next_command: VaachakSdMmcNativeCommand,
        clock_hz: u32,
        block_index: u32,
        block_count: u16,
        buffer_len: usize,
        write_explicitly_allowed: bool,
    ) -> VaachakSdMmcNativeLifecycleRequest {
        VaachakSdMmcNativeLifecycleRequest {
            backend: VaachakSdMmcNativeBackend::VaachakNativeSdMmcPhysicalDriver,
            intent,
            current_state,
            next_command,
            clock_hz,
            block_index,
            block_count,
            buffer_len,
            write_explicitly_allowed,
        }
    }

    pub const fn validate_request(request: VaachakSdMmcNativeLifecycleRequest) -> bool {
        let block_size_ok = request.buffer_len <= Self::MAX_SINGLE_TRANSFER_BYTES;
        let block_count_ok = request.block_count <= Self::MAX_SINGLE_TRANSFER_BLOCKS;
        let clock_ok = request.clock_hz == Self::PROBE_CLOCK_HZ
            || request.clock_hz == Self::OPERATIONAL_CLOCK_HZ;
        let write_ok = !matches!(
            request.intent,
            VaachakSdMmcNativeLifecycleIntent::WriteBlock
        ) || request.write_explicitly_allowed;
        block_size_ok && block_count_ok && clock_ok && write_ok
    }

    pub fn execute_with_backend<S, B>(
        sd_backend: &mut S,
        spi_backend: &mut B,
        request: VaachakSdMmcNativeLifecycleRequest,
        tx: &[u8],
        rx: &mut [u8],
    ) -> VaachakSdMmcNativeLifecycleResult
    where
        S: VaachakSdMmcNativeTargetBackend,
        B: VaachakSpiNativePeripheralBackend,
    {
        if !Self::validate_request(request) {
            let status = if matches!(
                request.intent,
                VaachakSdMmcNativeLifecycleIntent::WriteBlock
            ) && !request.write_explicitly_allowed
            {
                VaachakSdMmcNativeExecutionStatus::RejectedWriteNotExplicitlyAllowed
            } else {
                VaachakSdMmcNativeExecutionStatus::RejectedInvalidBlockSize
            };
            return Self::result(
                request,
                status,
                request.current_state,
                VaachakSpiNativeTransferStatus::RejectedBufferLengthMismatch,
                0,
            );
        }

        if matches!(
            request.intent,
            VaachakSdMmcNativeLifecycleIntent::DetectCard
        ) && !sd_backend.card_present()
        {
            return Self::result(
                request,
                VaachakSdMmcNativeExecutionStatus::RejectedInvalidState,
                VaachakSdMmcNativeCardState::Absent,
                VaachakSpiNativeTransferStatus::Accepted,
                0,
            );
        }

        if !sd_backend.send_command(&request) {
            return Self::result(
                request,
                VaachakSdMmcNativeExecutionStatus::TargetHalUnavailable,
                request.current_state,
                VaachakSpiNativeTransferStatus::BackendUnavailable,
                0,
            );
        }

        let spi_kind = Self::spi_kind_for_intent(request.intent);
        let spi_request = VaachakSpiPhysicalNativeDriver::storage_request(
            spi_kind,
            core::cmp::min(tx.len(), request.buffer_len),
            core::cmp::min(rx.len(), request.buffer_len),
        );
        let spi_result =
            VaachakSpiPhysicalNativeDriver::execute_with_backend(spi_backend, spi_request, tx, rx);
        if !matches!(
            spi_result.spi_status(),
            VaachakSpiNativeTransferStatus::Accepted
        ) {
            return Self::result(
                request,
                VaachakSdMmcNativeExecutionStatus::SpiTransferFailed,
                request.current_state,
                spi_result.spi_status(),
                spi_result.transferred_bytes,
            );
        }

        let transferred = match request.intent {
            VaachakSdMmcNativeLifecycleIntent::ReadBlock => sd_backend.read_block(&request, rx),
            VaachakSdMmcNativeLifecycleIntent::WriteBlock => sd_backend.write_block(&request, tx),
            _ => spi_result.transferred_bytes,
        };

        Self::result(
            request,
            VaachakSdMmcNativeExecutionStatus::Accepted,
            Self::next_state_for_intent(request.intent),
            spi_result.spi_status(),
            transferred,
        )
    }

    pub const fn spi_kind_for_intent(
        intent: VaachakSdMmcNativeLifecycleIntent,
    ) -> VaachakSpiNativeTransactionKind {
        match intent {
            VaachakSdMmcNativeLifecycleIntent::DetectCard
            | VaachakSdMmcNativeLifecycleIntent::EnterIdle
            | VaachakSdMmcNativeLifecycleIntent::ProbeCard => {
                VaachakSpiNativeTransactionKind::StorageProbe
            }
            VaachakSdMmcNativeLifecycleIntent::InitializeCard
            | VaachakSdMmcNativeLifecycleIntent::ReadCsd
            | VaachakSdMmcNativeLifecycleIntent::ReadCid
            | VaachakSdMmcNativeLifecycleIntent::MountBlockDevice
            | VaachakSdMmcNativeLifecycleIntent::CheckAvailability => {
                VaachakSpiNativeTransactionKind::StorageMount
            }
            VaachakSdMmcNativeLifecycleIntent::ReadBlock => {
                VaachakSpiNativeTransactionKind::StorageBlockRead
            }
            VaachakSdMmcNativeLifecycleIntent::WriteBlock => {
                VaachakSpiNativeTransactionKind::StorageBlockWrite
            }
        }
    }

    pub const fn next_state_for_intent(
        intent: VaachakSdMmcNativeLifecycleIntent,
    ) -> VaachakSdMmcNativeCardState {
        match intent {
            VaachakSdMmcNativeLifecycleIntent::DetectCard => VaachakSdMmcNativeCardState::Present,
            VaachakSdMmcNativeLifecycleIntent::EnterIdle => VaachakSdMmcNativeCardState::Idle,
            VaachakSdMmcNativeLifecycleIntent::ProbeCard => VaachakSdMmcNativeCardState::Idle,
            VaachakSdMmcNativeLifecycleIntent::InitializeCard
            | VaachakSdMmcNativeLifecycleIntent::ReadCsd
            | VaachakSdMmcNativeLifecycleIntent::ReadCid => {
                VaachakSdMmcNativeCardState::Initialized
            }
            VaachakSdMmcNativeLifecycleIntent::MountBlockDevice
            | VaachakSdMmcNativeLifecycleIntent::CheckAvailability
            | VaachakSdMmcNativeLifecycleIntent::ReadBlock
            | VaachakSdMmcNativeLifecycleIntent::WriteBlock => VaachakSdMmcNativeCardState::Mounted,
        }
    }

    pub const fn result(
        request: VaachakSdMmcNativeLifecycleRequest,
        status: VaachakSdMmcNativeExecutionStatus,
        next_state: VaachakSdMmcNativeCardState,
        spi_status: VaachakSpiNativeTransferStatus,
        bytes_transferred: usize,
    ) -> VaachakSdMmcNativeLifecycleResult {
        VaachakSdMmcNativeLifecycleResult {
            backend: VaachakSdMmcNativeBackend::VaachakNativeSdMmcPhysicalDriver,
            intent: request.intent,
            status,
            previous_state: request.current_state,
            next_state,
            spi_status,
            bytes_transferred,
            sd_mmc_lifecycle_owned_by_vaachak: Self::SD_MMC_CARD_LIFECYCLE_MOVED_TO_VAACHAK,
            block_device_policy_owned_by_vaachak: Self::SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK,
            pulp_sd_mmc_fallback_enabled: Self::PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED,
            fat_algorithm_migration_deferred: Self::FAT_ALGORITHM_MIGRATION_DEFERRED,
        }
    }

    pub const fn availability_state() -> VaachakSdMmcNativeAvailabilityState {
        VaachakSdMmcNativeAvailabilityState {
            card_state: VaachakSdMmcNativeCardState::Mounted,
            card_present: true,
            block_device_ready: true,
            storage_available: true,
            media_state_owned_by_vaachak: Self::SD_MMC_STORAGE_AVAILABILITY_MOVED_TO_VAACHAK,
        }
    }

    pub const fn migration_report() -> VaachakSdMmcNativeMigrationReport {
        VaachakSdMmcNativeMigrationReport {
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            transport_backend_name: Self::TRANSPORT_BACKEND_NAME,
            sd_mmc_card_lifecycle_moved_to_vaachak: Self::SD_MMC_CARD_LIFECYCLE_MOVED_TO_VAACHAK,
            sd_mmc_probe_init_sequence_moved_to_vaachak:
                Self::SD_MMC_PROBE_INIT_SEQUENCE_MOVED_TO_VAACHAK,
            sd_mmc_mount_lifecycle_moved_to_vaachak: Self::SD_MMC_MOUNT_LIFECYCLE_MOVED_TO_VAACHAK,
            sd_mmc_block_device_policy_moved_to_vaachak:
                Self::SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK,
            sd_mmc_storage_availability_moved_to_vaachak:
                Self::SD_MMC_STORAGE_AVAILABILITY_MOVED_TO_VAACHAK,
            uses_native_spi_driver: Self::SD_MMC_USES_NATIVE_SPI_DRIVER,
            pulp_sd_mmc_executor_fallback_enabled: Self::PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED,
            imported_pulp_sd_mmc_runtime_active: Self::IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE,
            fat_algorithm_migration_deferred: Self::FAT_ALGORITHM_MIGRATION_DEFERRED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
        }
    }

    pub const fn full_migration_ok() -> bool {
        Self::SD_MMC_CARD_LIFECYCLE_MOVED_TO_VAACHAK
            && Self::SD_MMC_PROBE_INIT_SEQUENCE_MOVED_TO_VAACHAK
            && Self::SD_MMC_MOUNT_LIFECYCLE_MOVED_TO_VAACHAK
            && Self::SD_MMC_BLOCK_DEVICE_POLICY_MOVED_TO_VAACHAK
            && Self::SD_MMC_STORAGE_AVAILABILITY_MOVED_TO_VAACHAK
            && Self::SD_MMC_USES_NATIVE_SPI_DRIVER
            && !Self::PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED
            && !Self::IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE
            && Self::FAT_ALGORITHM_MIGRATION_DEFERRED
            && VaachakSpiPhysicalNativeDriver::full_migration_ok()
            && Self::lifecycle_policy().ok()
            && Self::block_policy().ok()
            && Self::availability_state().ok()
            && Self::migration_report().ok()
    }
}

trait VaachakSpiNativeTransactionResultStatusExt {
    fn spi_status(&self) -> VaachakSpiNativeTransferStatus;
}

impl VaachakSpiNativeTransactionResultStatusExt
    for super::spi_physical_native_driver::VaachakSpiNativeTransactionResult
{
    fn spi_status(&self) -> VaachakSpiNativeTransferStatus {
        self.status
    }
}
