#![allow(dead_code)]

/// Pulp compatibility backend for the Vaachak-owned SD probe/mount runtime owner.
///
/// This module is deliberately a bridge descriptor only. It does not initialize
/// the SD card, toggle chip-select, call into FAT, open files, read/write files,
/// or change display behavior. It records that Vaachak now owns the SD
/// probe/mount runtime authority boundary while the existing imported Pulp
/// runtime remains the active hardware executor.
pub struct VaachakStorageProbeMountPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountPulpBackendRole {
    CompatibilityExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountPulpBackendReport {
    pub backend_active: bool,
    pub active_card_detection_executor_is_pulp: bool,
    pub active_identification_executor_is_pulp: bool,
    pub active_mount_executor_is_pulp: bool,
    pub active_fat_executor_is_pulp: bool,
    pub storage_executor_moved_to_vaachak: bool,
    pub fat_executor_moved_to_vaachak: bool,
    pub spi_arbitration_moved_to_vaachak: bool,
    pub display_runtime_moved_to_vaachak: bool,
    pub reader_file_browser_changed: bool,
}

impl VaachakStorageProbeMountPulpBackendReport {
    pub const fn bridge_ok(self) -> bool {
        self.backend_active
            && self.active_card_detection_executor_is_pulp
            && self.active_identification_executor_is_pulp
            && self.active_mount_executor_is_pulp
            && self.active_fat_executor_is_pulp
            && !self.storage_executor_moved_to_vaachak
            && !self.fat_executor_moved_to_vaachak
            && !self.spi_arbitration_moved_to_vaachak
            && !self.display_runtime_moved_to_vaachak
            && !self.reader_file_browser_changed
    }
}

impl VaachakStorageProbeMountPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const BACKEND_ROLE: VaachakStorageProbeMountPulpBackendRole =
        VaachakStorageProbeMountPulpBackendRole::CompatibilityExecutor;

    pub const ACTIVE_HARDWARE_EXECUTOR: bool = true;
    pub const ACTIVE_HARDWARE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_CARD_DETECTION_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_SD_IDENTIFICATION_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_SD_MOUNT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_FAT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const STORAGE_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_WRITE_LIST_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_RUNTIME_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn report() -> VaachakStorageProbeMountPulpBackendReport {
        VaachakStorageProbeMountPulpBackendReport {
            backend_active: Self::ACTIVE_HARDWARE_EXECUTOR,
            active_card_detection_executor_is_pulp: true,
            active_identification_executor_is_pulp: true,
            active_mount_executor_is_pulp: true,
            active_fat_executor_is_pulp: true,
            storage_executor_moved_to_vaachak: Self::STORAGE_EXECUTOR_MOVED_TO_VAACHAK,
            fat_executor_moved_to_vaachak: Self::FAT_EXECUTOR_MOVED_TO_VAACHAK,
            spi_arbitration_moved_to_vaachak: Self::SPI_ARBITRATION_MOVED_TO_VAACHAK,
            display_runtime_moved_to_vaachak: Self::DISPLAY_RUNTIME_MOVED_TO_VAACHAK,
            reader_file_browser_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn bridge_ok() -> bool {
        Self::report().bridge_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageProbeMountPulpBackend;

    #[test]
    fn pulp_backend_remains_active_executor() {
        assert!(VaachakStorageProbeMountPulpBackend::bridge_ok());
        assert_eq!(
            VaachakStorageProbeMountPulpBackend::ACTIVE_HARDWARE_EXECUTOR_OWNER,
            "vendor/pulp-os imported runtime"
        );
    }
}
