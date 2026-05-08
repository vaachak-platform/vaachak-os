#![allow(dead_code)]

use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::sd_fat_runtime_readonly_owner::{
    VaachakSdFatReadonlyOperation, VaachakSdFatRuntimeReadonlyOwner,
};
use super::storage_probe_mount_runtime_executor_bridge::{
    VaachakStorageProbeMountLifecycleIntent, VaachakStorageProbeMountRuntimeExecutorBridge,
};

/// Vaachak-owned storage executor bridge.
///
/// This bridge consolidates storage lifecycle and FAT/file intent routing under a
/// single Vaachak executor entrypoint. It does not replace the active low-level
/// Pulp FAT executor and does not introduce destructive file operations.
pub struct VaachakStorageExecutorBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageExecutorIntent {
    CardPresent,
    ProbeCard,
    MountStorage,
    StorageAvailableState,
    LibraryFileMetadataAccess,
    FileOpenReadIntent,
    FileReadChunkIntent,
    DirectoryListingIntent,
    StateCachePathResolution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageExecutorDecision {
    RoutedToPulpCompatibilityExecutor,
    RejectedBeforeHardwareExecution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageExecutorRoute {
    pub intent: VaachakStorageExecutorIntent,
    pub decision: VaachakStorageExecutorDecision,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub probe_mount_lifecycle_ready: bool,
    pub fat_storage_authority_ready: bool,
    pub destructive_behavior_introduced: bool,
    pub reader_file_browser_ux_changed: bool,
}

impl VaachakStorageExecutorBridge {
    pub const STORAGE_EXECUTOR_BRIDGE_MARKER: &'static str = "x4-storage-executor-bridge-ok";
    pub const STORAGE_EXECUTOR_BRIDGE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";

    pub const DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const FAT_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;

    pub const fn lifecycle_intent_for(
        intent: VaachakStorageExecutorIntent,
    ) -> VaachakStorageProbeMountLifecycleIntent {
        match intent {
            VaachakStorageExecutorIntent::CardPresent => {
                VaachakStorageProbeMountLifecycleIntent::DetectCard
            }
            VaachakStorageExecutorIntent::ProbeCard => {
                VaachakStorageProbeMountLifecycleIntent::IdentifyCardAtSafeSpeed
            }
            VaachakStorageExecutorIntent::MountStorage => {
                VaachakStorageProbeMountLifecycleIntent::ObserveFatVolumeAvailability
            }
            _ => VaachakStorageProbeMountLifecycleIntent::ObserveCardAvailability,
        }
    }

    pub const fn fat_operation_for(
        intent: VaachakStorageExecutorIntent,
    ) -> VaachakSdFatReadonlyOperation {
        match intent {
            VaachakStorageExecutorIntent::LibraryFileMetadataAccess => {
                VaachakSdFatReadonlyOperation::FileExists
            }
            VaachakStorageExecutorIntent::FileOpenReadIntent => {
                VaachakSdFatReadonlyOperation::ReadFileStart
            }
            VaachakStorageExecutorIntent::FileReadChunkIntent => {
                VaachakSdFatReadonlyOperation::ReadChunk
            }
            VaachakStorageExecutorIntent::DirectoryListingIntent => {
                VaachakSdFatReadonlyOperation::ListDirectoryMetadata
            }
            VaachakStorageExecutorIntent::StateCachePathResolution => {
                VaachakSdFatReadonlyOperation::ResolveCurrentStoragePaths
            }
            _ => VaachakSdFatReadonlyOperation::FileExists,
        }
    }

    pub const fn intent_is_lifecycle(intent: VaachakStorageExecutorIntent) -> bool {
        matches!(
            intent,
            VaachakStorageExecutorIntent::CardPresent
                | VaachakStorageExecutorIntent::ProbeCard
                | VaachakStorageExecutorIntent::MountStorage
                | VaachakStorageExecutorIntent::StorageAvailableState
        )
    }

    pub const fn route_intent(intent: VaachakStorageExecutorIntent) -> VaachakStorageExecutorRoute {
        let backend_route =
            VaachakHardwareExecutorPulpBackend::route_for(if Self::intent_is_lifecycle(intent) {
                VaachakHardwareExecutorDomain::StorageProbeMount
            } else {
                VaachakHardwareExecutorDomain::FatStorage
            });
        let lifecycle_ready = VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok();
        let fat_ready = VaachakSdFatRuntimeReadonlyOwner::ownership_ok();
        let decision = if lifecycle_ready
            && fat_ready
            && VaachakHardwareExecutorPulpBackend::route_is_pulp_compatible(backend_route)
        {
            VaachakStorageExecutorDecision::RoutedToPulpCompatibilityExecutor
        } else {
            VaachakStorageExecutorDecision::RejectedBeforeHardwareExecution
        };

        VaachakStorageExecutorRoute {
            intent,
            decision,
            backend: backend_route.backend,
            backend_name: backend_route.backend_name,
            active_executor_owner: backend_route.active_executor_owner,
            probe_mount_lifecycle_ready: lifecycle_ready,
            fat_storage_authority_ready: fat_ready,
            destructive_behavior_introduced: Self::DESTRUCTIVE_BEHAVIOR_INTRODUCED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
        }
    }

    pub const fn route_is_safe(route: VaachakStorageExecutorRoute) -> bool {
        matches!(
            route.decision,
            VaachakStorageExecutorDecision::RoutedToPulpCompatibilityExecutor
        ) && matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && route.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && route.probe_mount_lifecycle_ready
            && route.fat_storage_authority_ready
            && !route.destructive_behavior_introduced
            && !route.reader_file_browser_ux_changed
            && !Self::FAT_EXECUTOR_REWRITTEN_IN_VAACHAK
    }

    pub const fn lifecycle_routes_ready() -> bool {
        Self::route_is_safe(Self::route_intent(
            VaachakStorageExecutorIntent::CardPresent,
        )) && Self::route_is_safe(Self::route_intent(VaachakStorageExecutorIntent::ProbeCard))
            && Self::route_is_safe(Self::route_intent(
                VaachakStorageExecutorIntent::MountStorage,
            ))
            && Self::route_is_safe(Self::route_intent(
                VaachakStorageExecutorIntent::StorageAvailableState,
            ))
    }

    pub const fn fat_storage_routes_ready() -> bool {
        Self::route_is_safe(Self::route_intent(
            VaachakStorageExecutorIntent::LibraryFileMetadataAccess,
        )) && Self::route_is_safe(Self::route_intent(
            VaachakStorageExecutorIntent::FileOpenReadIntent,
        )) && Self::route_is_safe(Self::route_intent(
            VaachakStorageExecutorIntent::FileReadChunkIntent,
        )) && Self::route_is_safe(Self::route_intent(
            VaachakStorageExecutorIntent::DirectoryListingIntent,
        )) && Self::route_is_safe(Self::route_intent(
            VaachakStorageExecutorIntent::StateCachePathResolution,
        ))
    }

    pub const fn bridge_ok() -> bool {
        VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok()
            && VaachakSdFatRuntimeReadonlyOwner::ownership_ok()
            && VaachakHardwareExecutorPulpBackend::storage_probe_mount_route_ok()
            && VaachakHardwareExecutorPulpBackend::fat_storage_route_ok()
            && Self::lifecycle_routes_ready()
            && Self::fat_storage_routes_ready()
    }
}
