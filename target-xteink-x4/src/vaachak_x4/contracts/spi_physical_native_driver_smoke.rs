#![allow(dead_code)]

use crate::vaachak_x4::physical::spi_physical_native_driver::{
    VaachakSpiNativeBackend, VaachakSpiNativeChipSelect, VaachakSpiNativeDevice,
    VaachakSpiNativeTransactionKind, VaachakSpiPhysicalNativeDriver,
};

pub struct VaachakSpiPhysicalNativeDriverSmoke;

impl VaachakSpiPhysicalNativeDriverSmoke {
    pub const MARKER: &'static str = "spi_physical_native_driver_full_migration=ok";

    pub const fn smoke_ok() -> bool {
        let pins = VaachakSpiPhysicalNativeDriver::pins();
        let timing = VaachakSpiPhysicalNativeDriver::timing();
        let policy = VaachakSpiPhysicalNativeDriver::chip_select_policy();
        let display = VaachakSpiPhysicalNativeDriver::display_request(
            VaachakSpiNativeTransactionKind::DisplayRefreshControl,
            4,
            4,
        );
        let storage = VaachakSpiPhysicalNativeDriver::storage_request(
            VaachakSpiNativeTransactionKind::StorageProbe,
            1,
            1,
        );

        pins.sclk_gpio == 8
            && pins.mosi_gpio == 10
            && pins.miso_gpio == 7
            && pins.display_cs_gpio == 21
            && pins.storage_cs_gpio == 12
            && timing.storage_probe_hz == 400_000
            && timing.operational_hz == 20_000_000
            && policy.display_cs == VaachakSpiNativeChipSelect::DisplayGpio21
            && policy.storage_cs == VaachakSpiNativeChipSelect::StorageGpio12
            && policy.assert_before_transfer
            && policy.deassert_after_transfer
            && policy.never_assert_two_devices
            && display.backend == VaachakSpiNativeBackend::VaachakNativeSpiPhysicalDriver
            && display.device == VaachakSpiNativeDevice::DisplaySsd1677
            && storage.device == VaachakSpiNativeDevice::StorageSdCard
            && VaachakSpiPhysicalNativeDriver::validate_request(display)
            && VaachakSpiPhysicalNativeDriver::validate_request(storage)
            && VaachakSpiPhysicalNativeDriver::full_migration_ok()
    }
}
