#![allow(dead_code)]

/// Pulp compatibility backend for the Vaachak-owned SPI arbitration runtime.
///
/// The arbitration owner can decide which logical SPI user has the next safe
/// transaction slot, but this backend records that the physical SPI executor,
/// chip-select toggling, bus peripheral setup, display refresh, SD probe/mount,
/// and FAT behavior still run through the existing imported Pulp runtime.
pub struct VaachakSpiArbitrationPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiArbitrationBackendRole {
    PhysicalExecutorCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiArbitrationPulpBackendReport {
    pub backend_active: bool,
    pub physical_executor_is_pulp: bool,
    pub spi_transfer_executor_moved_to_vaachak: bool,
    pub chip_select_executor_moved_to_vaachak: bool,
    pub display_executor_moved_to_vaachak: bool,
    pub sd_probe_mount_executor_moved_to_vaachak: bool,
    pub sd_fat_executor_moved_to_vaachak: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakSpiArbitrationPulpBackendReport {
    pub const fn backend_ok(self) -> bool {
        self.backend_active
            && self.physical_executor_is_pulp
            && !self.spi_transfer_executor_moved_to_vaachak
            && !self.chip_select_executor_moved_to_vaachak
            && !self.display_executor_moved_to_vaachak
            && !self.sd_probe_mount_executor_moved_to_vaachak
            && !self.sd_fat_executor_moved_to_vaachak
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakSpiArbitrationPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const BACKEND_ROLE: VaachakSpiArbitrationBackendRole =
        VaachakSpiArbitrationBackendRole::PhysicalExecutorCompatibility;

    pub const ACTIVE_PHYSICAL_EXECUTOR: bool = true;
    pub const ACTIVE_PHYSICAL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_SPI_TRANSFER_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_CHIP_SELECT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_DISPLAY_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_SD_PROBE_MOUNT_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_SD_FAT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_FAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn report() -> VaachakSpiArbitrationPulpBackendReport {
        VaachakSpiArbitrationPulpBackendReport {
            backend_active: Self::ACTIVE_PHYSICAL_EXECUTOR,
            physical_executor_is_pulp: true,
            spi_transfer_executor_moved_to_vaachak: Self::SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK,
            chip_select_executor_moved_to_vaachak: Self::CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK,
            display_executor_moved_to_vaachak: Self::DISPLAY_EXECUTOR_MOVED_TO_VAACHAK,
            sd_probe_mount_executor_moved_to_vaachak:
                Self::SD_PROBE_MOUNT_EXECUTOR_MOVED_TO_VAACHAK,
            sd_fat_executor_moved_to_vaachak: Self::SD_FAT_EXECUTOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn backend_ok() -> bool {
        Self::report().backend_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSpiArbitrationPulpBackend;

    #[test]
    fn physical_executor_remains_pulp_compatibility() {
        assert!(VaachakSpiArbitrationPulpBackend::backend_ok());
        assert_eq!(
            VaachakSpiArbitrationPulpBackend::ACTIVE_PHYSICAL_EXECUTOR_OWNER,
            "vendor/pulp-os imported runtime"
        );
    }
}
