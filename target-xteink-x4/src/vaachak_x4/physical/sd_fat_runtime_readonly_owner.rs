#![allow(dead_code)]

use crate::vaachak_x4::io::storage_readonly_boundary::VaachakStorageReadonlyBoundaryContract;

use super::sd_fat_readonly_pulp_backend::VaachakSdFatReadonlyPulpBackend;
use super::spi_bus_runtime_owner::{
    VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind,
};
use super::storage_probe_mount_runtime_owner::VaachakStorageProbeMountRuntimeOwner;

/// Vaachak-owned SD/FAT read-only runtime owner for Xteink X4.
///
/// This is the first SD/FAT hardware-ownership move above probe/mount lifecycle.
/// Vaachak owns the read-only FAT runtime authority boundary and allowed
/// operation metadata. The existing imported Pulp runtime remains the active FAT
/// executor for file existence, file read-start, read-chunk, directory metadata,
/// and current path resolution. This module does not perform FAT I/O directly,
/// add writes, change SD probe/mount, move SPI arbitration, or change display,
/// reader, or file-browser behavior.
pub struct VaachakSdFatRuntimeReadonlyOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdFatRuntimeReadonlyBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdFatReadonlyOperation {
    FileExists,
    ReadFileStart,
    ReadChunk,
    ListDirectoryMetadata,
    ResolveCurrentStoragePaths,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdFatReadonlyLifecycleStep {
    SharedSpiOwnershipReady,
    ProbeMountOwnershipReady,
    FatVolumeAvailabilityObserved,
    ReadonlyBoundaryReady,
    ReadonlyOperationAuthorityReady,
    WritableOperationsDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSdFatRuntimeAuthority {
    VaachakRuntimeOwner,
    PulpCompatibilityExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdFatReadonlyOperationEntry {
    pub operation: VaachakSdFatReadonlyOperation,
    pub description: &'static str,
    pub authority_owner: VaachakSdFatRuntimeAuthority,
    pub active_executor: VaachakSdFatRuntimeReadonlyBackend,
    pub active_executor_owner: &'static str,
    pub writable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdFatReadonlyLifecycleEntry {
    pub step: VaachakSdFatReadonlyLifecycleStep,
    pub description: &'static str,
    pub authority_owner: VaachakSdFatRuntimeAuthority,
    pub active_executor: VaachakSdFatRuntimeReadonlyBackend,
    pub active_executor_owner: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdFatReadonlyOwnershipReport {
    pub ownership_authority_moved_to_vaachak: bool,
    pub active_backend_is_pulp_compatibility: bool,
    pub shared_spi_owner_available: bool,
    pub storage_user_registered_on_spi: bool,
    pub storage_probe_mount_owner_available: bool,
    pub readonly_boundary_available: bool,
    pub readonly_operations_registered: bool,
    pub writable_operations_denied: bool,
    pub backend_bridge_ok: bool,
    pub fat_readonly_executor_moved_to_vaachak: bool,
    pub fat_writable_behavior_moved_to_vaachak: bool,
    pub sd_probe_mount_behavior_changed: bool,
    pub spi_arbitration_moved_to_vaachak: bool,
    pub display_behavior_moved_to_vaachak: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakSdFatReadonlyOwnershipReport {
    pub const fn ownership_ok(self) -> bool {
        self.ownership_authority_moved_to_vaachak
            && self.active_backend_is_pulp_compatibility
            && self.shared_spi_owner_available
            && self.storage_user_registered_on_spi
            && self.storage_probe_mount_owner_available
            && self.readonly_boundary_available
            && self.readonly_operations_registered
            && self.writable_operations_denied
            && self.backend_bridge_ok
            && !self.fat_readonly_executor_moved_to_vaachak
            && !self.fat_writable_behavior_moved_to_vaachak
            && !self.sd_probe_mount_behavior_changed
            && !self.spi_arbitration_moved_to_vaachak
            && !self.display_behavior_moved_to_vaachak
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakSdFatRuntimeReadonlyOwner {
    pub const SD_FAT_RUNTIME_READONLY_OWNERSHIP_MARKER: &'static str =
        "x4-sd-fat-runtime-readonly-owner-ok";

    pub const SD_FAT_RUNTIME_READONLY_IDENTITY: &'static str = "xteink-x4-sd-fat-runtime-readonly";
    pub const SD_FAT_READONLY_OWNERSHIP_AUTHORITY: &'static str = "target-xteink-x4 Vaachak layer";
    pub const SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;

    pub const PULP_COMPATIBILITY_BACKEND: VaachakSdFatRuntimeReadonlyBackend =
        VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND: VaachakSdFatRuntimeReadonlyBackend = Self::PULP_COMPATIBILITY_BACKEND;
    pub const ACTIVE_BACKEND_NAME: &'static str = VaachakSdFatReadonlyPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const STORAGE_SD_CS_GPIO: u8 = 12;
    pub const SD_IDENTIFICATION_KHZ: u32 = 400;
    pub const OPERATIONAL_SPI_MHZ: u32 = 20;

    pub const FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const READONLY_OPERATIONS: [VaachakSdFatReadonlyOperationEntry; 5] = [
        VaachakSdFatReadonlyOperationEntry {
            operation: VaachakSdFatReadonlyOperation::FileExists,
            description: "read-only file existence authority exposed through Vaachak boundary",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            writable: false,
        },
        VaachakSdFatReadonlyOperationEntry {
            operation: VaachakSdFatReadonlyOperation::ReadFileStart,
            description: "read-only start-of-file authority exposed through Vaachak boundary",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            writable: false,
        },
        VaachakSdFatReadonlyOperationEntry {
            operation: VaachakSdFatReadonlyOperation::ReadChunk,
            description: "read-only chunk authority exposed through Vaachak boundary",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            writable: false,
        },
        VaachakSdFatReadonlyOperationEntry {
            operation: VaachakSdFatReadonlyOperation::ListDirectoryMetadata,
            description: "read-only directory metadata authority exposed through Vaachak boundary",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            writable: false,
        },
        VaachakSdFatReadonlyOperationEntry {
            operation: VaachakSdFatReadonlyOperation::ResolveCurrentStoragePaths,
            description: "read-only storage path authority exposed through Vaachak boundary",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            writable: false,
        },
    ];

    pub const LIFECYCLE: [VaachakSdFatReadonlyLifecycleEntry; 6] = [
        VaachakSdFatReadonlyLifecycleEntry {
            step: VaachakSdFatReadonlyLifecycleStep::SharedSpiOwnershipReady,
            description: "read-only FAT authority depends on the accepted Vaachak SPI ownership bridge",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakSdFatReadonlyLifecycleEntry {
            step: VaachakSdFatReadonlyLifecycleStep::ProbeMountOwnershipReady,
            description: "read-only FAT authority depends on the accepted SD probe/mount owner",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakSdFatReadonlyLifecycleEntry {
            step: VaachakSdFatReadonlyLifecycleStep::FatVolumeAvailabilityObserved,
            description: "FAT volume availability remains executed by Pulp and observed by Vaachak",
            authority_owner: VaachakSdFatRuntimeAuthority::PulpCompatibilityExecutor,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakSdFatReadonlyLifecycleEntry {
            step: VaachakSdFatReadonlyLifecycleStep::ReadonlyBoundaryReady,
            description: "Vaachak read-only storage boundary is the public contract layer",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakSdFatReadonlyLifecycleEntry {
            step: VaachakSdFatReadonlyLifecycleStep::ReadonlyOperationAuthorityReady,
            description: "Vaachak owns read-only operation authority metadata",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakSdFatReadonlyLifecycleEntry {
            step: VaachakSdFatReadonlyLifecycleStep::WritableOperationsDenied,
            description: "write, append, delete, rename, mkdir, mount, and format remain outside this owner",
            authority_owner: VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
    ];

    pub const fn operation_entry(
        operation: VaachakSdFatReadonlyOperation,
    ) -> VaachakSdFatReadonlyOperationEntry {
        match operation {
            VaachakSdFatReadonlyOperation::FileExists => Self::READONLY_OPERATIONS[0],
            VaachakSdFatReadonlyOperation::ReadFileStart => Self::READONLY_OPERATIONS[1],
            VaachakSdFatReadonlyOperation::ReadChunk => Self::READONLY_OPERATIONS[2],
            VaachakSdFatReadonlyOperation::ListDirectoryMetadata => Self::READONLY_OPERATIONS[3],
            VaachakSdFatReadonlyOperation::ResolveCurrentStoragePaths => {
                Self::READONLY_OPERATIONS[4]
            }
        }
    }

    pub const fn lifecycle_entry(
        step: VaachakSdFatReadonlyLifecycleStep,
    ) -> VaachakSdFatReadonlyLifecycleEntry {
        match step {
            VaachakSdFatReadonlyLifecycleStep::SharedSpiOwnershipReady => Self::LIFECYCLE[0],
            VaachakSdFatReadonlyLifecycleStep::ProbeMountOwnershipReady => Self::LIFECYCLE[1],
            VaachakSdFatReadonlyLifecycleStep::FatVolumeAvailabilityObserved => Self::LIFECYCLE[2],
            VaachakSdFatReadonlyLifecycleStep::ReadonlyBoundaryReady => Self::LIFECYCLE[3],
            VaachakSdFatReadonlyLifecycleStep::ReadonlyOperationAuthorityReady => {
                Self::LIFECYCLE[4]
            }
            VaachakSdFatReadonlyLifecycleStep::WritableOperationsDenied => Self::LIFECYCLE[5],
        }
    }

    pub const fn shared_spi_owner_available() -> bool {
        VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
    }

    pub const fn storage_user_registered_on_spi() -> bool {
        VaachakSpiBusRuntimeOwner::storage_user_registered()
    }

    pub const fn storage_spi_metadata_ok() -> bool {
        let metadata = VaachakSpiBusRuntimeOwner::transaction_ownership(
            VaachakSpiRuntimeUser::Storage,
            VaachakSpiTransactionKind::StorageFatIoMetadata,
        );
        VaachakSpiBusRuntimeOwner::transaction_metadata_is_safe(metadata)
    }

    pub const fn storage_probe_mount_owner_available() -> bool {
        VaachakStorageProbeMountRuntimeOwner::ownership_ok()
    }

    pub const fn readonly_boundary_available() -> bool {
        VaachakStorageReadonlyBoundaryContract::active_runtime_preflight().ok()
    }

    pub const fn readonly_operations_registered() -> bool {
        let file_exists = Self::operation_entry(VaachakSdFatReadonlyOperation::FileExists);
        let read_start = Self::operation_entry(VaachakSdFatReadonlyOperation::ReadFileStart);
        let read_chunk = Self::operation_entry(VaachakSdFatReadonlyOperation::ReadChunk);
        let list_metadata =
            Self::operation_entry(VaachakSdFatReadonlyOperation::ListDirectoryMetadata);
        let resolve_paths =
            Self::operation_entry(VaachakSdFatReadonlyOperation::ResolveCurrentStoragePaths);

        !file_exists.writable
            && !read_start.writable
            && !read_chunk.writable
            && !list_metadata.writable
            && !resolve_paths.writable
            && matches!(
                file_exists.active_executor,
                VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility
            )
            && matches!(
                read_start.active_executor,
                VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility
            )
            && matches!(
                read_chunk.active_executor,
                VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility
            )
            && matches!(
                list_metadata.active_executor,
                VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility
            )
            && matches!(
                resolve_paths.active_executor,
                VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility
            )
    }

    pub const fn writable_operations_denied() -> bool {
        !Self::FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK
    }

    pub const fn lifecycle_authority_ok() -> bool {
        let spi_ready =
            Self::lifecycle_entry(VaachakSdFatReadonlyLifecycleStep::SharedSpiOwnershipReady);
        let probe_ready =
            Self::lifecycle_entry(VaachakSdFatReadonlyLifecycleStep::ProbeMountOwnershipReady);
        let fat_observed =
            Self::lifecycle_entry(VaachakSdFatReadonlyLifecycleStep::FatVolumeAvailabilityObserved);
        let readonly_ready =
            Self::lifecycle_entry(VaachakSdFatReadonlyLifecycleStep::ReadonlyBoundaryReady);
        let operation_ready = Self::lifecycle_entry(
            VaachakSdFatReadonlyLifecycleStep::ReadonlyOperationAuthorityReady,
        );
        let writable_denied =
            Self::lifecycle_entry(VaachakSdFatReadonlyLifecycleStep::WritableOperationsDenied);

        matches!(
            spi_ready.authority_owner,
            VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner
        ) && matches!(
            probe_ready.authority_owner,
            VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner
        ) && matches!(
            fat_observed.authority_owner,
            VaachakSdFatRuntimeAuthority::PulpCompatibilityExecutor
        ) && matches!(
            readonly_ready.authority_owner,
            VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner
        ) && matches!(
            operation_ready.authority_owner,
            VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner
        ) && matches!(
            writable_denied.authority_owner,
            VaachakSdFatRuntimeAuthority::VaachakRuntimeOwner
        )
    }

    pub const fn report() -> VaachakSdFatReadonlyOwnershipReport {
        VaachakSdFatReadonlyOwnershipReport {
            ownership_authority_moved_to_vaachak:
                Self::SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            active_backend_is_pulp_compatibility: matches!(
                Self::ACTIVE_BACKEND,
                VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility
            ),
            shared_spi_owner_available: Self::shared_spi_owner_available()
                && Self::storage_spi_metadata_ok(),
            storage_user_registered_on_spi: Self::storage_user_registered_on_spi(),
            storage_probe_mount_owner_available: Self::storage_probe_mount_owner_available(),
            readonly_boundary_available: Self::readonly_boundary_available(),
            readonly_operations_registered: Self::readonly_operations_registered(),
            writable_operations_denied: Self::writable_operations_denied(),
            backend_bridge_ok: VaachakSdFatReadonlyPulpBackend::bridge_ok(),
            fat_readonly_executor_moved_to_vaachak: Self::FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK,
            fat_writable_behavior_moved_to_vaachak: Self::FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK
                || Self::WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK,
            sd_probe_mount_behavior_changed: Self::SD_PROBE_MOUNT_BEHAVIOR_CHANGED,
            spi_arbitration_moved_to_vaachak: Self::SPI_ARBITRATION_MOVED_TO_VAACHAK,
            display_behavior_moved_to_vaachak: Self::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn ownership_ok() -> bool {
        Self::report().ownership_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakSdFatReadonlyOperation, VaachakSdFatRuntimeReadonlyOwner};

    #[test]
    fn sd_fat_readonly_runtime_owner_is_active() {
        assert!(VaachakSdFatRuntimeReadonlyOwner::ownership_ok());
    }

    #[test]
    fn registered_operations_are_readonly() {
        assert!(VaachakSdFatRuntimeReadonlyOwner::readonly_operations_registered());
        assert!(
            !VaachakSdFatRuntimeReadonlyOwner::operation_entry(
                VaachakSdFatReadonlyOperation::ReadChunk
            )
            .writable
        );
    }
}
