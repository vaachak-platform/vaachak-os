#![allow(dead_code)]

/// Pulp compatibility backend descriptor for Vaachak SD/FAT read-only ownership.
///
/// This backend is intentionally metadata-only. It records that the existing
/// imported Pulp runtime remains the active executor for FAT file existence,
/// read-start, read-chunk, directory metadata listing, and storage path
/// resolution. It does not mount SD, initialize cards, access FAT directly,
/// perform writes, or touch display/SPI behavior.
pub struct VaachakSdFatReadonlyPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdFatReadonlyPulpBackendReport {
    pub backend_name_present: bool,
    pub active_hardware_executor: bool,
    pub active_readonly_executor_owner_present: bool,
    pub active_fat_executor_owner_present: bool,
    pub active_path_resolution_owner_present: bool,
    pub readonly_executor_moved_to_vaachak: bool,
    pub writable_behavior_moved_to_vaachak: bool,
    pub sd_probe_mount_moved_to_vaachak: bool,
    pub spi_arbitration_moved_to_vaachak: bool,
    pub display_runtime_moved_to_vaachak: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakSdFatReadonlyPulpBackendReport {
    pub const fn bridge_ok(self) -> bool {
        self.backend_name_present
            && self.active_hardware_executor
            && self.active_readonly_executor_owner_present
            && self.active_fat_executor_owner_present
            && self.active_path_resolution_owner_present
            && !self.readonly_executor_moved_to_vaachak
            && !self.writable_behavior_moved_to_vaachak
            && !self.sd_probe_mount_moved_to_vaachak
            && !self.spi_arbitration_moved_to_vaachak
            && !self.display_runtime_moved_to_vaachak
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakSdFatReadonlyPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const BACKEND_SOURCE: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_HARDWARE_EXECUTOR: bool = true;

    pub const ACTIVE_READONLY_FILE_EXISTS_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_READONLY_FILE_START_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_READONLY_CHUNK_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_DIRECTORY_METADATA_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_PATH_RESOLUTION_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_FAT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const READONLY_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_RUNTIME_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn report() -> VaachakSdFatReadonlyPulpBackendReport {
        VaachakSdFatReadonlyPulpBackendReport {
            backend_name_present: !Self::BACKEND_NAME.is_empty(),
            active_hardware_executor: Self::ACTIVE_HARDWARE_EXECUTOR,
            active_readonly_executor_owner_present:
                !Self::ACTIVE_READONLY_FILE_EXISTS_EXECUTOR_OWNER.is_empty()
                    && !Self::ACTIVE_READONLY_FILE_START_EXECUTOR_OWNER.is_empty()
                    && !Self::ACTIVE_READONLY_CHUNK_EXECUTOR_OWNER.is_empty()
                    && !Self::ACTIVE_DIRECTORY_METADATA_EXECUTOR_OWNER.is_empty(),
            active_fat_executor_owner_present: !Self::ACTIVE_FAT_EXECUTOR_OWNER.is_empty(),
            active_path_resolution_owner_present: !Self::ACTIVE_PATH_RESOLUTION_EXECUTOR_OWNER
                .is_empty(),
            readonly_executor_moved_to_vaachak: Self::READONLY_EXECUTOR_MOVED_TO_VAACHAK,
            writable_behavior_moved_to_vaachak: Self::WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK
                || Self::WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK,
            sd_probe_mount_moved_to_vaachak: Self::SD_PROBE_MOUNT_MOVED_TO_VAACHAK,
            spi_arbitration_moved_to_vaachak: Self::SPI_ARBITRATION_MOVED_TO_VAACHAK,
            display_runtime_moved_to_vaachak: Self::DISPLAY_RUNTIME_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn bridge_ok() -> bool {
        Self::report().bridge_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSdFatReadonlyPulpBackend;

    #[test]
    fn pulp_backend_descriptor_remains_safe() {
        assert!(VaachakSdFatReadonlyPulpBackend::bridge_ok());
    }
}
