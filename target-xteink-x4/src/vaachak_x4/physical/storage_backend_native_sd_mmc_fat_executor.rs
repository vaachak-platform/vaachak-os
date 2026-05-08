use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_backend::{
    VaachakHardwareBackendDomain, VaachakHardwareBackendHandoffResult,
    VaachakStorageAccessBackendOperation, VaachakStorageAccessRequest,
    VaachakStorageFatAccessExecutor, VaachakStoragePathRole, VaachakStorageProbeMountBackendIntent,
    VaachakStorageProbeMountExecutor, VaachakStorageProbeMountRequest,
};
use super::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;

/// Vaachak-native SD/MMC/FAT executor behavior.
///
/// This module is the first storage behavior move out of the Pulp-owned runtime
/// path. Vaachak now owns storage media state interpretation, probe/mount
/// lifecycle command decisions, FAT operation classification, path-role policy,
/// destructive-operation denial, and Pulp-compatible request construction. The
/// physical SD/MMC block driver and low-level FAT implementation remain behind
/// the PulpCompatibility fallback for this slice.
pub struct VaachakStorageBackendNativeSdMmcFatExecutor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageNativeBackend {
    VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageNativeMediaState {
    CardAbsent,
    CardPresentUnprobed,
    CardProbed,
    Mounted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageNativeLifecycleIntent {
    CardAvailability,
    ProbeCard,
    MountFilesystem,
    StorageAvailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageNativeFatOperation {
    FileExists,
    DirectoryListing,
    FileOpen,
    FileReadChunk,
    StateCachePathResolution,
    CreateFileDenied,
    AppendFileDenied,
    DeleteFileDenied,
    RenameFileDenied,
    MakeDirDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageNativeDecisionKind {
    LifecycleDecision,
    FatAccessDecision,
    DestructiveOperationDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageNativeAccessMode {
    MetadataOnly,
    ReadExistingData,
    DestructiveDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageNativeRequest {
    pub media_state: VaachakStorageNativeMediaState,
    pub lifecycle_intent: Option<VaachakStorageNativeLifecycleIntent>,
    pub fat_operation: Option<VaachakStorageNativeFatOperation>,
    pub path_role: VaachakStoragePathRole,
    pub destructive_operation_requested: bool,
    pub physical_sampling_or_block_io_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageNativeDecision {
    pub decision_kind: VaachakStorageNativeDecisionKind,
    pub selected_backend: VaachakStorageNativeBackend,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub access_mode: VaachakStorageNativeAccessMode,
    pub path_role: VaachakStoragePathRole,
    pub vaachak_owns_sd_mmc_fat_command_decision: bool,
    pub vaachak_owns_probe_mount_state_machine: bool,
    pub vaachak_owns_fat_operation_classification: bool,
    pub vaachak_owns_path_role_policy: bool,
    pub vaachak_owns_destructive_operation_guard: bool,
    pub pulp_low_level_executor_selected: bool,
    pub low_level_sd_mmc_block_driver_moved_to_vaachak: bool,
    pub low_level_fat_algorithm_moved_to_vaachak: bool,
    pub physical_spi_transfer_changed: bool,
    pub display_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageNativeExecutionResult {
    pub request: VaachakStorageNativeRequest,
    pub decision: VaachakStorageNativeDecision,
    pub pulp_handoff_result: VaachakHardwareBackendHandoffResult,
    pub destructive_operation_denied: bool,
    pub storage_behavior_moved_to_vaachak: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageNativeSdMmcFatExecutorReport {
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub low_level_executor_owner: &'static str,
    pub sd_mmc_fat_command_decision_moved_to_vaachak: bool,
    pub probe_mount_state_machine_moved_to_vaachak: bool,
    pub fat_operation_classification_moved_to_vaachak: bool,
    pub path_role_policy_moved_to_vaachak: bool,
    pub destructive_operation_guard_moved_to_vaachak: bool,
    pub card_availability_decision_ok: bool,
    pub probe_decision_ok: bool,
    pub mount_decision_ok: bool,
    pub directory_listing_decision_ok: bool,
    pub file_open_decision_ok: bool,
    pub file_read_decision_ok: bool,
    pub path_resolution_decision_ok: bool,
    pub destructive_operation_denial_ok: bool,
    pub low_level_sd_mmc_block_driver_moved_to_vaachak: bool,
    pub low_level_fat_algorithm_moved_to_vaachak: bool,
    pub physical_spi_transfer_changed: bool,
    pub display_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakStorageNativeDecision {
    pub const fn ok(self) -> bool {
        self.vaachak_owns_sd_mmc_fat_command_decision
            && self.vaachak_owns_probe_mount_state_machine
            && self.vaachak_owns_fat_operation_classification
            && self.vaachak_owns_path_role_policy
            && self.vaachak_owns_destructive_operation_guard
            && self.pulp_low_level_executor_selected
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && !self.low_level_sd_mmc_block_driver_moved_to_vaachak
            && !self.low_level_fat_algorithm_moved_to_vaachak
            && !self.physical_spi_transfer_changed
            && !self.display_behavior_changed
            && !self.input_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakStorageNativeExecutionResult {
    pub const fn ok(self) -> bool {
        self.decision.ok()
            && self.storage_behavior_moved_to_vaachak
            && if self.request.destructive_operation_requested {
                self.destructive_operation_denied
                    && !self.pulp_handoff_result.accepted
                    && !self.pulp_handoff_result.destructive_behavior_added
            } else {
                !self.destructive_operation_denied && self.pulp_handoff_result.ok()
            }
    }
}

impl VaachakStorageNativeSdMmcFatExecutorReport {
    pub const fn ok(self) -> bool {
        self.sd_mmc_fat_command_decision_moved_to_vaachak
            && self.probe_mount_state_machine_moved_to_vaachak
            && self.fat_operation_classification_moved_to_vaachak
            && self.path_role_policy_moved_to_vaachak
            && self.destructive_operation_guard_moved_to_vaachak
            && self.card_availability_decision_ok
            && self.probe_decision_ok
            && self.mount_decision_ok
            && self.directory_listing_decision_ok
            && self.file_open_decision_ok
            && self.file_read_decision_ok
            && self.path_resolution_decision_ok
            && self.destructive_operation_denial_ok
            && !self.low_level_sd_mmc_block_driver_moved_to_vaachak
            && !self.low_level_fat_algorithm_moved_to_vaachak
            && !self.physical_spi_transfer_changed
            && !self.display_behavior_changed
            && !self.input_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakStorageBackendNativeSdMmcFatExecutor {
    pub const STORAGE_BACKEND_NATIVE_SD_MMC_FAT_EXECUTOR_MARKER: &'static str =
        "storage_backend_native_sd_mmc_fat_executor=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str =
        "VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const SD_MMC_FAT_COMMAND_DECISION_MOVED_TO_VAACHAK: bool = true;
    pub const PROBE_MOUNT_STATE_MACHINE_MOVED_TO_VAACHAK: bool = true;
    pub const FAT_OPERATION_CLASSIFICATION_MOVED_TO_VAACHAK: bool = true;
    pub const PATH_ROLE_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const DESTRUCTIVE_OPERATION_GUARD_MOVED_TO_VAACHAK: bool = true;

    pub const LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK: bool = false;
    pub const LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK: bool = false;
    pub const PHYSICAL_SPI_TRANSFER_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    fn backend() -> VaachakHardwareRuntimePulpCompatibilityBackend {
        VaachakHardwareRuntimePulpCompatibilityBackend
    }

    pub const fn media_state_from_flags(
        card_present: bool,
        probed: bool,
        mounted: bool,
    ) -> VaachakStorageNativeMediaState {
        if mounted {
            VaachakStorageNativeMediaState::Mounted
        } else if probed {
            VaachakStorageNativeMediaState::CardProbed
        } else if card_present {
            VaachakStorageNativeMediaState::CardPresentUnprobed
        } else {
            VaachakStorageNativeMediaState::CardAbsent
        }
    }

    pub const fn fat_access_mode(
        operation: VaachakStorageNativeFatOperation,
    ) -> VaachakStorageNativeAccessMode {
        match operation {
            VaachakStorageNativeFatOperation::FileExists
            | VaachakStorageNativeFatOperation::DirectoryListing
            | VaachakStorageNativeFatOperation::StateCachePathResolution => {
                VaachakStorageNativeAccessMode::MetadataOnly
            }
            VaachakStorageNativeFatOperation::FileOpen
            | VaachakStorageNativeFatOperation::FileReadChunk => {
                VaachakStorageNativeAccessMode::ReadExistingData
            }
            VaachakStorageNativeFatOperation::CreateFileDenied
            | VaachakStorageNativeFatOperation::AppendFileDenied
            | VaachakStorageNativeFatOperation::DeleteFileDenied
            | VaachakStorageNativeFatOperation::RenameFileDenied
            | VaachakStorageNativeFatOperation::MakeDirDenied => {
                VaachakStorageNativeAccessMode::DestructiveDenied
            }
        }
    }

    pub const fn is_destructive_fat_operation(operation: VaachakStorageNativeFatOperation) -> bool {
        matches!(
            operation,
            VaachakStorageNativeFatOperation::CreateFileDenied
                | VaachakStorageNativeFatOperation::AppendFileDenied
                | VaachakStorageNativeFatOperation::DeleteFileDenied
                | VaachakStorageNativeFatOperation::RenameFileDenied
                | VaachakStorageNativeFatOperation::MakeDirDenied
        )
    }

    pub const fn storage_access_backend_operation(
        operation: VaachakStorageNativeFatOperation,
    ) -> Option<VaachakStorageAccessBackendOperation> {
        match operation {
            VaachakStorageNativeFatOperation::FileExists
            | VaachakStorageNativeFatOperation::DirectoryListing => {
                Some(VaachakStorageAccessBackendOperation::DirectoryListing)
            }
            VaachakStorageNativeFatOperation::FileOpen => {
                Some(VaachakStorageAccessBackendOperation::FileOpen)
            }
            VaachakStorageNativeFatOperation::FileReadChunk => {
                Some(VaachakStorageAccessBackendOperation::FileReadChunk)
            }
            VaachakStorageNativeFatOperation::StateCachePathResolution => {
                Some(VaachakStorageAccessBackendOperation::StateCachePathResolution)
            }
            VaachakStorageNativeFatOperation::CreateFileDenied
            | VaachakStorageNativeFatOperation::AppendFileDenied
            | VaachakStorageNativeFatOperation::DeleteFileDenied
            | VaachakStorageNativeFatOperation::RenameFileDenied
            | VaachakStorageNativeFatOperation::MakeDirDenied => None,
        }
    }

    pub const fn probe_mount_backend_intent(
        intent: VaachakStorageNativeLifecycleIntent,
    ) -> VaachakStorageProbeMountBackendIntent {
        match intent {
            VaachakStorageNativeLifecycleIntent::CardAvailability => {
                VaachakStorageProbeMountBackendIntent::CardAvailability
            }
            VaachakStorageNativeLifecycleIntent::ProbeCard => {
                VaachakStorageProbeMountBackendIntent::ProbeCard
            }
            VaachakStorageNativeLifecycleIntent::MountFilesystem
            | VaachakStorageNativeLifecycleIntent::StorageAvailable => {
                VaachakStorageProbeMountBackendIntent::MountReadiness
            }
        }
    }

    pub fn decide_storage_operation(
        request: VaachakStorageNativeRequest,
    ) -> VaachakStorageNativeDecision {
        let operation = request
            .fat_operation
            .unwrap_or(VaachakStorageNativeFatOperation::StateCachePathResolution);
        let access_mode = Self::fat_access_mode(operation);
        let destructive = request.destructive_operation_requested
            || Self::is_destructive_fat_operation(operation)
            || matches!(
                access_mode,
                VaachakStorageNativeAccessMode::DestructiveDenied
            );
        let decision_kind = if destructive {
            VaachakStorageNativeDecisionKind::DestructiveOperationDenied
        } else if request.lifecycle_intent.is_some() {
            VaachakStorageNativeDecisionKind::LifecycleDecision
        } else {
            VaachakStorageNativeDecisionKind::FatAccessDecision
        };

        VaachakStorageNativeDecision {
            decision_kind,
            selected_backend:
                VaachakStorageNativeBackend::VaachakSdMmcFatNativeExecutorWithPulpLowLevelFallback,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            access_mode,
            path_role: request.path_role,
            vaachak_owns_sd_mmc_fat_command_decision:
                Self::SD_MMC_FAT_COMMAND_DECISION_MOVED_TO_VAACHAK,
            vaachak_owns_probe_mount_state_machine:
                Self::PROBE_MOUNT_STATE_MACHINE_MOVED_TO_VAACHAK,
            vaachak_owns_fat_operation_classification:
                Self::FAT_OPERATION_CLASSIFICATION_MOVED_TO_VAACHAK,
            vaachak_owns_path_role_policy: Self::PATH_ROLE_POLICY_MOVED_TO_VAACHAK,
            vaachak_owns_destructive_operation_guard:
                Self::DESTRUCTIVE_OPERATION_GUARD_MOVED_TO_VAACHAK,
            pulp_low_level_executor_selected: true,
            low_level_sd_mmc_block_driver_moved_to_vaachak:
                Self::LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK,
            low_level_fat_algorithm_moved_to_vaachak:
                Self::LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK,
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    fn fallback_result(
        domain: VaachakHardwareBackendDomain,
        accepted: bool,
    ) -> VaachakHardwareBackendHandoffResult {
        VaachakHardwareBackendHandoffResult {
            domain,
            accepted,
            backend: Self::LOW_LEVEL_BACKEND,
            backend_name: "PulpCompatibility",
            backend_owner: Self::BACKEND_OWNER,
            low_level_executor_owner: Self::LOW_LEVEL_EXECUTOR_OWNER,
            pulp_compatible_backend_active: true,
            low_level_behavior_changed: false,
            destructive_behavior_added: false,
            ux_behavior_changed: false,
        }
    }

    pub fn execute_storage_operation(
        request: VaachakStorageNativeRequest,
    ) -> VaachakStorageNativeExecutionResult {
        let decision = Self::decide_storage_operation(request);
        let destructive_denied = matches!(
            decision.decision_kind,
            VaachakStorageNativeDecisionKind::DestructiveOperationDenied
        );
        let pulp_handoff_result = if destructive_denied {
            Self::fallback_result(VaachakHardwareBackendDomain::StorageFatAccess, false)
        } else if let Some(intent) = request.lifecycle_intent {
            Self::backend().execute_storage_probe_mount(VaachakStorageProbeMountRequest {
                intent: Self::probe_mount_backend_intent(intent),
                card_present_expected: !matches!(
                    request.media_state,
                    VaachakStorageNativeMediaState::CardAbsent
                ),
                mount_readiness_required: matches!(
                    intent,
                    VaachakStorageNativeLifecycleIntent::MountFilesystem
                        | VaachakStorageNativeLifecycleIntent::StorageAvailable
                ),
            })
        } else if let Some(operation) = request.fat_operation {
            if let Some(backend_operation) = Self::storage_access_backend_operation(operation) {
                Self::backend().execute_storage_access(VaachakStorageAccessRequest {
                    operation: backend_operation,
                    path_role: request.path_role,
                    destructive_operation_allowed: false,
                })
            } else {
                Self::fallback_result(VaachakHardwareBackendDomain::StorageFatAccess, false)
            }
        } else {
            Self::backend().execute_storage_access(VaachakStorageAccessRequest {
                operation: VaachakStorageAccessBackendOperation::StateCachePathResolution,
                path_role: request.path_role,
                destructive_operation_allowed: false,
            })
        };

        VaachakStorageNativeExecutionResult {
            request,
            decision,
            pulp_handoff_result,
            destructive_operation_denied: destructive_denied,
            storage_behavior_moved_to_vaachak: true,
        }
    }

    pub fn execute_card_availability_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::CardPresentUnprobed,
            lifecycle_intent: Some(VaachakStorageNativeLifecycleIntent::CardAvailability),
            fat_operation: None,
            path_role: VaachakStoragePathRole::LibraryRoot,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: true,
        })
    }

    pub fn execute_probe_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::CardPresentUnprobed,
            lifecycle_intent: Some(VaachakStorageNativeLifecycleIntent::ProbeCard),
            fat_operation: None,
            path_role: VaachakStoragePathRole::LibraryRoot,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: true,
        })
    }

    pub fn execute_mount_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::CardProbed,
            lifecycle_intent: Some(VaachakStorageNativeLifecycleIntent::MountFilesystem),
            fat_operation: None,
            path_role: VaachakStoragePathRole::LibraryRoot,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: true,
        })
    }

    pub fn execute_directory_listing_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::Mounted,
            lifecycle_intent: None,
            fat_operation: Some(VaachakStorageNativeFatOperation::DirectoryListing),
            path_role: VaachakStoragePathRole::LibraryRoot,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: true,
        })
    }

    pub fn execute_file_open_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::Mounted,
            lifecycle_intent: None,
            fat_operation: Some(VaachakStorageNativeFatOperation::FileOpen),
            path_role: VaachakStoragePathRole::ReaderBookPath,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: true,
        })
    }

    pub fn execute_file_read_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::Mounted,
            lifecycle_intent: None,
            fat_operation: Some(VaachakStorageNativeFatOperation::FileReadChunk),
            path_role: VaachakStoragePathRole::ReaderBookPath,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: true,
        })
    }

    pub fn execute_path_resolution_handoff() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::Mounted,
            lifecycle_intent: None,
            fat_operation: Some(VaachakStorageNativeFatOperation::StateCachePathResolution),
            path_role: VaachakStoragePathRole::CachePath,
            destructive_operation_requested: false,
            physical_sampling_or_block_io_required: false,
        })
    }

    pub fn execute_destructive_operation_denial() -> VaachakStorageNativeExecutionResult {
        Self::execute_storage_operation(VaachakStorageNativeRequest {
            media_state: VaachakStorageNativeMediaState::Mounted,
            lifecycle_intent: None,
            fat_operation: Some(VaachakStorageNativeFatOperation::DeleteFileDenied),
            path_role: VaachakStoragePathRole::ReaderBookPath,
            destructive_operation_requested: true,
            physical_sampling_or_block_io_required: false,
        })
    }

    pub fn adopt_storage_availability_handoff() -> bool {
        Self::execute_card_availability_handoff().ok()
            && Self::execute_probe_handoff().ok()
            && Self::execute_mount_handoff().ok()
    }

    pub fn adopt_storage_fat_access_handoff() -> bool {
        Self::execute_directory_listing_handoff().ok()
            && Self::execute_file_open_handoff().ok()
            && Self::execute_file_read_handoff().ok()
            && Self::execute_path_resolution_handoff().ok()
    }

    pub fn report() -> VaachakStorageNativeSdMmcFatExecutorReport {
        VaachakStorageNativeSdMmcFatExecutorReport {
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            low_level_executor_owner: Self::LOW_LEVEL_EXECUTOR_OWNER,
            sd_mmc_fat_command_decision_moved_to_vaachak:
                Self::SD_MMC_FAT_COMMAND_DECISION_MOVED_TO_VAACHAK,
            probe_mount_state_machine_moved_to_vaachak:
                Self::PROBE_MOUNT_STATE_MACHINE_MOVED_TO_VAACHAK,
            fat_operation_classification_moved_to_vaachak:
                Self::FAT_OPERATION_CLASSIFICATION_MOVED_TO_VAACHAK,
            path_role_policy_moved_to_vaachak: Self::PATH_ROLE_POLICY_MOVED_TO_VAACHAK,
            destructive_operation_guard_moved_to_vaachak:
                Self::DESTRUCTIVE_OPERATION_GUARD_MOVED_TO_VAACHAK,
            card_availability_decision_ok: Self::execute_card_availability_handoff().ok(),
            probe_decision_ok: Self::execute_probe_handoff().ok(),
            mount_decision_ok: Self::execute_mount_handoff().ok(),
            directory_listing_decision_ok: Self::execute_directory_listing_handoff().ok(),
            file_open_decision_ok: Self::execute_file_open_handoff().ok(),
            file_read_decision_ok: Self::execute_file_read_handoff().ok(),
            path_resolution_decision_ok: Self::execute_path_resolution_handoff().ok(),
            destructive_operation_denial_ok: Self::execute_destructive_operation_denial().ok(),
            low_level_sd_mmc_block_driver_moved_to_vaachak:
                Self::LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK,
            low_level_fat_algorithm_moved_to_vaachak:
                Self::LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK,
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn native_sd_mmc_fat_executor_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakStorageBackendNativeSdMmcFatExecutor, VaachakStorageNativeFatOperation};

    #[test]
    fn storage_backend_native_sd_mmc_fat_executor_is_ready() {
        assert!(VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok());
    }

    #[test]
    fn destructive_storage_operations_are_denied_before_pulp_handoff() {
        assert!(
            VaachakStorageBackendNativeSdMmcFatExecutor::is_destructive_fat_operation(
                VaachakStorageNativeFatOperation::DeleteFileDenied
            )
        );
        assert!(
            VaachakStorageBackendNativeSdMmcFatExecutor::execute_destructive_operation_denial()
                .ok()
        );
    }
}
