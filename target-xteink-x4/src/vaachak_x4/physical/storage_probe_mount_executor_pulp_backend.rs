#![allow(dead_code)]

/// Pulp compatibility executor descriptor for Vaachak-owned SD probe/mount
/// lifecycle intent routing.
///
/// This is deliberately a narrow executor bridge descriptor. It records that
/// Vaachak owns the lifecycle execution entrypoint while the active low-level
/// SD/MMC card-detect, identification, mount, and FAT implementation remains in
/// the imported Pulp runtime. This module does not call storage drivers, access
/// FAT, manipulate SPI GPIOs, render display frames, or change reader/file
/// browser behavior.
pub struct VaachakStorageProbeMountExecutorPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountExecutorBackendRole {
    CompatibilityLifecycleExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountExecutorPath {
    CardDetectIntent,
    CardIdentifyIntent,
    AvailabilityIntent,
    FatVolumeObservationIntent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountExecutorBackendReport {
    pub backend_active: bool,
    pub lifecycle_entrypoint_owned_by_vaachak: bool,
    pub low_level_card_detect_executor_is_pulp: bool,
    pub low_level_card_identification_executor_is_pulp: bool,
    pub low_level_mount_executor_is_pulp: bool,
    pub fat_executor_is_pulp: bool,
    pub fat_read_write_list_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakStorageProbeMountExecutorBackendReport {
    pub const fn backend_ok(self) -> bool {
        self.backend_active
            && self.lifecycle_entrypoint_owned_by_vaachak
            && self.low_level_card_detect_executor_is_pulp
            && self.low_level_card_identification_executor_is_pulp
            && self.low_level_mount_executor_is_pulp
            && self.fat_executor_is_pulp
            && !self.fat_read_write_list_behavior_changed
            && !self.display_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakStorageProbeMountExecutorPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const BACKEND_ROLE: VaachakStorageProbeMountExecutorBackendRole =
        VaachakStorageProbeMountExecutorBackendRole::CompatibilityLifecycleExecutor;

    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const LIFECYCLE_ENTRYPOINT_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LOW_LEVEL_CARD_DETECT_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_CARD_IDENTIFICATION_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_MOUNT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const FAT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK: bool = true;
    pub const LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn executor_owner_for(path: VaachakStorageProbeMountExecutorPath) -> &'static str {
        match path {
            VaachakStorageProbeMountExecutorPath::CardDetectIntent => {
                Self::LOW_LEVEL_CARD_DETECT_EXECUTOR_OWNER
            }
            VaachakStorageProbeMountExecutorPath::CardIdentifyIntent => {
                Self::LOW_LEVEL_CARD_IDENTIFICATION_EXECUTOR_OWNER
            }
            VaachakStorageProbeMountExecutorPath::AvailabilityIntent => {
                Self::LOW_LEVEL_MOUNT_EXECUTOR_OWNER
            }
            VaachakStorageProbeMountExecutorPath::FatVolumeObservationIntent => {
                Self::FAT_EXECUTOR_OWNER
            }
        }
    }

    pub const fn report() -> VaachakStorageProbeMountExecutorBackendReport {
        VaachakStorageProbeMountExecutorBackendReport {
            backend_active: true,
            lifecycle_entrypoint_owned_by_vaachak: Self::LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK,
            low_level_card_detect_executor_is_pulp:
                !Self::LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK,
            low_level_card_identification_executor_is_pulp:
                !Self::LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK,
            low_level_mount_executor_is_pulp: !Self::LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK,
            fat_executor_is_pulp: true,
            fat_read_write_list_behavior_changed: Self::FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn backend_ok() -> bool {
        Self::report().backend_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        VaachakStorageProbeMountExecutorPath, VaachakStorageProbeMountExecutorPulpBackend,
    };

    #[test]
    fn pulp_executor_backend_remains_active() {
        assert!(VaachakStorageProbeMountExecutorPulpBackend::backend_ok());
        assert_eq!(
            VaachakStorageProbeMountExecutorPulpBackend::executor_owner_for(
                VaachakStorageProbeMountExecutorPath::CardDetectIntent,
            ),
            "vendor/pulp-os imported runtime"
        );
    }
}
