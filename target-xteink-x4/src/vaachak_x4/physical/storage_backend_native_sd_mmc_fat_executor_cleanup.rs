use super::storage_backend_native_sd_mmc_fat_executor::VaachakStorageBackendNativeSdMmcFatExecutor;

/// Cleanup checkpoint for the Vaachak-native SD/MMC/FAT executor migration.
///
/// This module does not add a new storage behavior path. It records that the
/// accepted storage behavior move is now consolidated for commit: Vaachak owns
/// storage command decisions, probe/mount state interpretation, FAT operation
/// classification, path-role policy, and destructive-operation denial, while
/// low-level SD/MMC block I/O and FAT algorithms remain Pulp-compatible.
pub struct VaachakStorageBackendNativeSdMmcFatExecutorCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageBackendNativeSdMmcFatExecutorCleanupReport {
    pub cleanup_marker: &'static str,
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub low_level_executor_owner: &'static str,
    pub native_sd_mmc_fat_executor_ready: bool,
    pub storage_availability_handoff_ready: bool,
    pub storage_fat_access_handoff_ready: bool,
    pub destructive_operation_denial_ready: bool,
    pub sd_mmc_fat_command_decision_moved_to_vaachak: bool,
    pub probe_mount_state_machine_moved_to_vaachak: bool,
    pub fat_operation_classification_moved_to_vaachak: bool,
    pub path_role_policy_moved_to_vaachak: bool,
    pub destructive_operation_guard_moved_to_vaachak: bool,
    pub low_level_sd_mmc_block_driver_moved_to_vaachak: bool,
    pub low_level_fat_algorithm_moved_to_vaachak: bool,
    pub physical_spi_transfer_changed: bool,
    pub display_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub cleanup_artifacts_script_declared: bool,
    pub canonical_cleanup_doc_declared: bool,
}

impl VaachakStorageBackendNativeSdMmcFatExecutorCleanupReport {
    pub const fn ok(self) -> bool {
        self.native_sd_mmc_fat_executor_ready
            && self.storage_availability_handoff_ready
            && self.storage_fat_access_handoff_ready
            && self.destructive_operation_denial_ready
            && self.sd_mmc_fat_command_decision_moved_to_vaachak
            && self.probe_mount_state_machine_moved_to_vaachak
            && self.fat_operation_classification_moved_to_vaachak
            && self.path_role_policy_moved_to_vaachak
            && self.destructive_operation_guard_moved_to_vaachak
            && !self.low_level_sd_mmc_block_driver_moved_to_vaachak
            && !self.low_level_fat_algorithm_moved_to_vaachak
            && !self.physical_spi_transfer_changed
            && !self.display_behavior_changed
            && !self.input_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && self.cleanup_artifacts_script_declared
            && self.canonical_cleanup_doc_declared
    }
}

impl VaachakStorageBackendNativeSdMmcFatExecutorCleanup {
    pub const STORAGE_BACKEND_NATIVE_SD_MMC_FAT_EXECUTOR_CLEANUP_MARKER: &'static str =
        "storage_backend_native_sd_mmc_fat_executor_cleanup=ok";
    pub const CLEANUP_ARTIFACTS_SCRIPT_DECLARED: bool = true;
    pub const CANONICAL_CLEANUP_DOC_DECLARED: bool = true;

    pub fn report() -> VaachakStorageBackendNativeSdMmcFatExecutorCleanupReport {
        VaachakStorageBackendNativeSdMmcFatExecutorCleanupReport {
            cleanup_marker: Self::STORAGE_BACKEND_NATIVE_SD_MMC_FAT_EXECUTOR_CLEANUP_MARKER,
            active_backend_name: VaachakStorageBackendNativeSdMmcFatExecutor::ACTIVE_BACKEND_NAME,
            backend_owner: VaachakStorageBackendNativeSdMmcFatExecutor::BACKEND_OWNER,
            low_level_executor_owner:
                VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_EXECUTOR_OWNER,
            native_sd_mmc_fat_executor_ready:
                VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok(),
            storage_availability_handoff_ready:
                VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_availability_handoff(),
            storage_fat_access_handoff_ready:
                VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_fat_access_handoff(),
            destructive_operation_denial_ready:
                VaachakStorageBackendNativeSdMmcFatExecutor::execute_destructive_operation_denial()
                    .ok(),
            sd_mmc_fat_command_decision_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::SD_MMC_FAT_COMMAND_DECISION_MOVED_TO_VAACHAK,
            probe_mount_state_machine_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::PROBE_MOUNT_STATE_MACHINE_MOVED_TO_VAACHAK,
            fat_operation_classification_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::FAT_OPERATION_CLASSIFICATION_MOVED_TO_VAACHAK,
            path_role_policy_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::PATH_ROLE_POLICY_MOVED_TO_VAACHAK,
            destructive_operation_guard_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::DESTRUCTIVE_OPERATION_GUARD_MOVED_TO_VAACHAK,
            low_level_sd_mmc_block_driver_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK,
            low_level_fat_algorithm_moved_to_vaachak:
                VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK,
            physical_spi_transfer_changed:
                VaachakStorageBackendNativeSdMmcFatExecutor::PHYSICAL_SPI_TRANSFER_CHANGED,
            display_behavior_changed:
                VaachakStorageBackendNativeSdMmcFatExecutor::DISPLAY_BEHAVIOR_CHANGED,
            input_behavior_changed:
                VaachakStorageBackendNativeSdMmcFatExecutor::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed:
                VaachakStorageBackendNativeSdMmcFatExecutor::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed:
                VaachakStorageBackendNativeSdMmcFatExecutor::APP_NAVIGATION_BEHAVIOR_CHANGED,
            cleanup_artifacts_script_declared: Self::CLEANUP_ARTIFACTS_SCRIPT_DECLARED,
            canonical_cleanup_doc_declared: Self::CANONICAL_CLEANUP_DOC_DECLARED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageBackendNativeSdMmcFatExecutorCleanup;

    #[test]
    fn storage_backend_native_sd_mmc_fat_executor_cleanup_is_ready() {
        assert!(VaachakStorageBackendNativeSdMmcFatExecutorCleanup::cleanup_ok());
    }
}
