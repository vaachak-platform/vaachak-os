#![allow(dead_code)]

use crate::vaachak_x4::physical::storage_physical_sd_mmc_native_driver::{
    VaachakSdMmcNativeCardState, VaachakSdMmcNativeExecutionStatus,
    VaachakSdMmcNativeLifecycleIntent, VaachakStoragePhysicalSdMmcNativeDriver,
};

pub struct VaachakStoragePhysicalSdMmcNativeDriverSmoke;

impl VaachakStoragePhysicalSdMmcNativeDriverSmoke {
    pub const MARKER: &'static str = "storage_physical_sd_mmc_full_migration=ok";

    pub const fn run() -> bool {
        let detect = VaachakStoragePhysicalSdMmcNativeDriver::detect_request();
        let probe = VaachakStoragePhysicalSdMmcNativeDriver::probe_request();
        let init = VaachakStoragePhysicalSdMmcNativeDriver::initialize_request();
        let mount = VaachakStoragePhysicalSdMmcNativeDriver::mount_request();
        let read = VaachakStoragePhysicalSdMmcNativeDriver::read_block_request(0);
        let rejected_write = VaachakStoragePhysicalSdMmcNativeDriver::write_block_request(0, false);
        let accepted_write = VaachakStoragePhysicalSdMmcNativeDriver::write_block_request(0, true);
        let report = VaachakStoragePhysicalSdMmcNativeDriver::migration_report();
        let ok_result = VaachakStoragePhysicalSdMmcNativeDriver::result(
            mount,
            VaachakSdMmcNativeExecutionStatus::Accepted,
            VaachakSdMmcNativeCardState::Mounted,
            super::super::physical::spi_physical_native_driver::VaachakSpiNativeTransferStatus::Accepted,
            8,
        );
        VaachakStoragePhysicalSdMmcNativeDriver::full_migration_ok()
            && detect.ok()
            && probe.ok()
            && init.ok()
            && mount.ok()
            && read.ok()
            && !rejected_write.ok()
            && accepted_write.ok()
            && report.ok()
            && ok_result.ok()
            && matches!(detect.intent, VaachakSdMmcNativeLifecycleIntent::DetectCard)
            && matches!(
                mount.intent,
                VaachakSdMmcNativeLifecycleIntent::MountBlockDevice
            )
    }
}

pub const STORAGE_PHYSICAL_SD_MMC_NATIVE_DRIVER_SMOKE_OK: bool =
    VaachakStoragePhysicalSdMmcNativeDriverSmoke::run();
