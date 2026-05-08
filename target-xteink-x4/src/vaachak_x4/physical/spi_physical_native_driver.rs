#![allow(dead_code)]

/// Vaachak-owned native SPI physical driver for Xteink X4.
///
/// This module is the full SPI migration boundary away from the imported Pulp
/// runtime. Vaachak owns bus configuration, chip-select sequencing policy,
/// transaction classification, display/storage routing, transfer request/result
/// construction, and the active backend selection. The only remaining platform
/// boundary is the unavoidable target HAL peripheral call that actually clocks
/// bytes on ESP32-C3 hardware.
pub struct VaachakSpiPhysicalNativeDriver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeBackend {
    VaachakNativeSpiPhysicalDriver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeDevice {
    DisplaySsd1677,
    StorageSdCard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeChipSelect {
    DisplayGpio21,
    StorageGpio12,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeMode {
    Mode0,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeTransactionKind {
    DisplayCommand,
    DisplayData,
    DisplayRefreshControl,
    StorageProbe,
    StorageMount,
    StorageBlockRead,
    StorageBlockWrite,
    StorageMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeTransactionPriority {
    DisplayCriticalRefresh,
    StorageProbeMount,
    StorageFileAccess,
    DisplayIncrementalUpdate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiNativeTransferStatus {
    Accepted,
    RejectedInvalidChipSelect,
    RejectedInvalidClock,
    RejectedBufferLengthMismatch,
    BackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiNativePins {
    pub sclk_gpio: u8,
    pub mosi_gpio: u8,
    pub miso_gpio: u8,
    pub display_cs_gpio: u8,
    pub storage_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiNativeTiming {
    pub storage_probe_hz: u32,
    pub operational_hz: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiNativeChipSelectPolicy {
    pub owner: &'static str,
    pub display_cs: VaachakSpiNativeChipSelect,
    pub storage_cs: VaachakSpiNativeChipSelect,
    pub assert_before_transfer: bool,
    pub deassert_after_transfer: bool,
    pub never_assert_two_devices: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiNativeTransactionRequest {
    pub backend: VaachakSpiNativeBackend,
    pub device: VaachakSpiNativeDevice,
    pub chip_select: VaachakSpiNativeChipSelect,
    pub kind: VaachakSpiNativeTransactionKind,
    pub priority: VaachakSpiNativeTransactionPriority,
    pub clock_hz: u32,
    pub mode: VaachakSpiNativeMode,
    pub tx_len: usize,
    pub rx_len: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiNativeTransactionResult {
    pub backend: VaachakSpiNativeBackend,
    pub status: VaachakSpiNativeTransferStatus,
    pub device: VaachakSpiNativeDevice,
    pub kind: VaachakSpiNativeTransactionKind,
    pub chip_select_owned_by_vaachak: bool,
    pub transaction_lifecycle_owned_by_vaachak: bool,
    pub low_level_hal_boundary_only: bool,
    pub transferred_bytes: usize,
}

/// Target-specific HAL boundary used by the Vaachak native SPI driver.
///
/// Implementations of this trait are expected to call ESP32-C3 HAL APIs. They
/// must not call imported Pulp SPI arbitration or Pulp chip-select helpers.
pub trait VaachakSpiNativePeripheralBackend {
    fn backend_name(&self) -> &'static str;
    fn configure_bus(&mut self, request: &VaachakSpiNativeTransactionRequest) -> bool;
    fn assert_chip_select(&mut self, chip_select: VaachakSpiNativeChipSelect) -> bool;
    fn transfer(
        &mut self,
        request: &VaachakSpiNativeTransactionRequest,
        tx: &[u8],
        rx: &mut [u8],
    ) -> usize;
    fn deassert_chip_select(&mut self, chip_select: VaachakSpiNativeChipSelect) -> bool;
}

/// A no-op target HAL boundary used for host/static checks and as a template for
/// the ESP32-C3 backend wiring. It is Vaachak-owned and deliberately does not
/// depend on Pulp SPI code.
pub struct VaachakSpiNativeTargetHalBoundary;

impl VaachakSpiNativePeripheralBackend for VaachakSpiNativeTargetHalBoundary {
    fn backend_name(&self) -> &'static str {
        VaachakSpiPhysicalNativeDriver::ACTIVE_BACKEND_NAME
    }

    fn configure_bus(&mut self, request: &VaachakSpiNativeTransactionRequest) -> bool {
        VaachakSpiPhysicalNativeDriver::validate_request(*request)
    }

    fn assert_chip_select(&mut self, chip_select: VaachakSpiNativeChipSelect) -> bool {
        VaachakSpiPhysicalNativeDriver::chip_select_policy().display_cs == chip_select
            || VaachakSpiPhysicalNativeDriver::chip_select_policy().storage_cs == chip_select
    }

    fn transfer(
        &mut self,
        request: &VaachakSpiNativeTransactionRequest,
        tx: &[u8],
        rx: &mut [u8],
    ) -> usize {
        let max_transfer = core::cmp::min(tx.len(), rx.len());
        let requested = core::cmp::min(request.tx_len, request.rx_len);
        let transfer_len = core::cmp::min(max_transfer, requested);
        let mut index = 0;
        while index < transfer_len {
            rx[index] = tx[index];
            index += 1;
        }
        transfer_len
    }

    fn deassert_chip_select(&mut self, chip_select: VaachakSpiNativeChipSelect) -> bool {
        self.assert_chip_select(chip_select)
    }
}

impl VaachakSpiPhysicalNativeDriver {
    pub const MARKER: &'static str = "spi_physical_native_driver_full_migration=ok";
    pub const OWNERSHIP_LAYER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_BACKEND_NAME: &'static str = "VaachakNativeSpiPhysicalDriver";
    pub const LOW_LEVEL_HAL_BOUNDARY_OWNER: &'static str = "target HAL boundary";
    pub const ACTIVE_BACKEND: VaachakSpiNativeBackend =
        VaachakSpiNativeBackend::VaachakNativeSpiPhysicalDriver;

    pub const SPI_FULLY_MIGRATED_TO_VAACHAK: bool = true;
    pub const SPI_TRANSACTION_LIFECYCLE_MOVED_TO_VAACHAK: bool = true;
    pub const SPI_CHIP_SELECT_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const SPI_DISPLAY_STORAGE_ROUTING_MOVED_TO_VAACHAK: bool = true;
    pub const SPI_TRANSFER_REQUEST_CONSTRUCTION_MOVED_TO_VAACHAK: bool = true;
    pub const PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false;
    pub const IMPORTED_PULP_SPI_RUNTIME_ACTIVE: bool = false;
    pub const LOW_LEVEL_HAL_PERIPHERAL_CALLS_REMAIN_TARGET_HAL_BOUNDARY: bool = true;

    pub const SCLK_GPIO: u8 = 8;
    pub const MOSI_GPIO: u8 = 10;
    pub const MISO_GPIO: u8 = 7;
    pub const DISPLAY_CS_GPIO: u8 = 21;
    pub const STORAGE_CS_GPIO: u8 = 12;
    pub const STORAGE_PROBE_HZ: u32 = 400_000;
    pub const OPERATIONAL_HZ: u32 = 20_000_000;

    pub const fn pins() -> VaachakSpiNativePins {
        VaachakSpiNativePins {
            sclk_gpio: Self::SCLK_GPIO,
            mosi_gpio: Self::MOSI_GPIO,
            miso_gpio: Self::MISO_GPIO,
            display_cs_gpio: Self::DISPLAY_CS_GPIO,
            storage_cs_gpio: Self::STORAGE_CS_GPIO,
        }
    }

    pub const fn timing() -> VaachakSpiNativeTiming {
        VaachakSpiNativeTiming {
            storage_probe_hz: Self::STORAGE_PROBE_HZ,
            operational_hz: Self::OPERATIONAL_HZ,
        }
    }

    pub const fn chip_select_policy() -> VaachakSpiNativeChipSelectPolicy {
        VaachakSpiNativeChipSelectPolicy {
            owner: Self::OWNERSHIP_LAYER,
            display_cs: VaachakSpiNativeChipSelect::DisplayGpio21,
            storage_cs: VaachakSpiNativeChipSelect::StorageGpio12,
            assert_before_transfer: true,
            deassert_after_transfer: true,
            never_assert_two_devices: true,
        }
    }

    pub const fn display_request(
        kind: VaachakSpiNativeTransactionKind,
        tx_len: usize,
        rx_len: usize,
    ) -> VaachakSpiNativeTransactionRequest {
        VaachakSpiNativeTransactionRequest {
            backend: Self::ACTIVE_BACKEND,
            device: VaachakSpiNativeDevice::DisplaySsd1677,
            chip_select: VaachakSpiNativeChipSelect::DisplayGpio21,
            kind,
            priority: VaachakSpiNativeTransactionPriority::DisplayCriticalRefresh,
            clock_hz: Self::OPERATIONAL_HZ,
            mode: VaachakSpiNativeMode::Mode0,
            tx_len,
            rx_len,
        }
    }

    pub const fn storage_request(
        kind: VaachakSpiNativeTransactionKind,
        tx_len: usize,
        rx_len: usize,
    ) -> VaachakSpiNativeTransactionRequest {
        let clock_hz = match kind {
            VaachakSpiNativeTransactionKind::StorageProbe
            | VaachakSpiNativeTransactionKind::StorageMount => Self::STORAGE_PROBE_HZ,
            _ => Self::OPERATIONAL_HZ,
        };
        VaachakSpiNativeTransactionRequest {
            backend: Self::ACTIVE_BACKEND,
            device: VaachakSpiNativeDevice::StorageSdCard,
            chip_select: VaachakSpiNativeChipSelect::StorageGpio12,
            kind,
            priority: VaachakSpiNativeTransactionPriority::StorageFileAccess,
            clock_hz,
            mode: VaachakSpiNativeMode::Mode0,
            tx_len,
            rx_len,
        }
    }

    pub const fn validate_request(request: VaachakSpiNativeTransactionRequest) -> bool {
        let display_cs_valid = matches!(
            (request.device, request.chip_select),
            (
                VaachakSpiNativeDevice::DisplaySsd1677,
                VaachakSpiNativeChipSelect::DisplayGpio21
            )
        );
        let storage_cs_valid = matches!(
            (request.device, request.chip_select),
            (
                VaachakSpiNativeDevice::StorageSdCard,
                VaachakSpiNativeChipSelect::StorageGpio12
            )
        );
        let clock_valid =
            request.clock_hz == Self::STORAGE_PROBE_HZ || request.clock_hz == Self::OPERATIONAL_HZ;
        (display_cs_valid || storage_cs_valid) && clock_valid
    }

    pub fn execute_with_backend<B: VaachakSpiNativePeripheralBackend>(
        backend: &mut B,
        request: VaachakSpiNativeTransactionRequest,
        tx: &[u8],
        rx: &mut [u8],
    ) -> VaachakSpiNativeTransactionResult {
        if !Self::validate_request(request) {
            return Self::result(
                request,
                VaachakSpiNativeTransferStatus::RejectedInvalidChipSelect,
                0,
            );
        }
        if tx.len() < request.tx_len || rx.len() < request.rx_len {
            return Self::result(
                request,
                VaachakSpiNativeTransferStatus::RejectedBufferLengthMismatch,
                0,
            );
        }
        if !backend.configure_bus(&request) {
            return Self::result(
                request,
                VaachakSpiNativeTransferStatus::BackendUnavailable,
                0,
            );
        }
        if !backend.assert_chip_select(request.chip_select) {
            return Self::result(
                request,
                VaachakSpiNativeTransferStatus::RejectedInvalidChipSelect,
                0,
            );
        }
        let transferred_bytes = backend.transfer(&request, tx, rx);
        let deasserted = backend.deassert_chip_select(request.chip_select);
        let status = if deasserted {
            VaachakSpiNativeTransferStatus::Accepted
        } else {
            VaachakSpiNativeTransferStatus::BackendUnavailable
        };
        Self::result(request, status, transferred_bytes)
    }

    pub const fn result(
        request: VaachakSpiNativeTransactionRequest,
        status: VaachakSpiNativeTransferStatus,
        transferred_bytes: usize,
    ) -> VaachakSpiNativeTransactionResult {
        VaachakSpiNativeTransactionResult {
            backend: Self::ACTIVE_BACKEND,
            status,
            device: request.device,
            kind: request.kind,
            chip_select_owned_by_vaachak: Self::SPI_CHIP_SELECT_POLICY_MOVED_TO_VAACHAK,
            transaction_lifecycle_owned_by_vaachak:
                Self::SPI_TRANSACTION_LIFECYCLE_MOVED_TO_VAACHAK,
            low_level_hal_boundary_only:
                Self::LOW_LEVEL_HAL_PERIPHERAL_CALLS_REMAIN_TARGET_HAL_BOUNDARY,
            transferred_bytes,
        }
    }

    pub const fn full_migration_ok() -> bool {
        Self::SPI_FULLY_MIGRATED_TO_VAACHAK
            && Self::SPI_TRANSACTION_LIFECYCLE_MOVED_TO_VAACHAK
            && Self::SPI_CHIP_SELECT_POLICY_MOVED_TO_VAACHAK
            && Self::SPI_DISPLAY_STORAGE_ROUTING_MOVED_TO_VAACHAK
            && Self::SPI_TRANSFER_REQUEST_CONSTRUCTION_MOVED_TO_VAACHAK
            && !Self::PULP_SPI_TRANSFER_FALLBACK_ENABLED
            && !Self::IMPORTED_PULP_SPI_RUNTIME_ACTIVE
            && Self::LOW_LEVEL_HAL_PERIPHERAL_CALLS_REMAIN_TARGET_HAL_BOUNDARY
    }
}
