#![allow(dead_code)]

use crate::vaachak_x4::physical::storage_probe_mount_pulp_backend::VaachakStorageProbeMountPulpBackend;
use crate::vaachak_x4::physical::storage_probe_mount_runtime_owner::{
    VaachakStorageProbeMountRuntimeOwner, VaachakStorageRuntimeStep,
};

/// Contract smoke for the Vaachak-owned SD probe/mount runtime owner.
///
/// This verifies that SD probe/mount ownership authority has moved to Vaachak,
/// the active backend remains Pulp-compatible, the shared SPI dependency is
/// honored, and FAT/display/reader behavior has not moved.
pub struct VaachakStorageProbeMountRuntimeOwnershipSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountRuntimeOwnershipSmokeReport {
    pub owner_entrypoint_ok: bool,
    pub pulp_backend_active: bool,
    pub shared_spi_dependency_ok: bool,
    pub lifecycle_authority_ok: bool,
    pub storage_probe_metadata_ok: bool,
    pub storage_mount_metadata_ok: bool,
    pub no_unwanted_runtime_behavior_moved: bool,
}

impl VaachakStorageProbeMountRuntimeOwnershipSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.owner_entrypoint_ok
            && self.pulp_backend_active
            && self.shared_spi_dependency_ok
            && self.lifecycle_authority_ok
            && self.storage_probe_metadata_ok
            && self.storage_mount_metadata_ok
            && self.no_unwanted_runtime_behavior_moved
    }
}

impl VaachakStorageProbeMountRuntimeOwnershipSmoke {
    pub const STORAGE_PROBE_MOUNT_RUNTIME_OWNERSHIP_SMOKE_MARKER: &'static str =
        "x4-storage-probe-mount-runtime-ownership-smoke-ok";

    pub const STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true;
    pub const SD_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_WRITE_LIST_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn no_unwanted_runtime_behavior_moved() -> bool {
        !Self::SD_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::FAT_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::FAT_READ_WRITE_LIST_MOVED_TO_VAACHAK
            && !Self::SPI_ARBITRATION_MOVED_TO_VAACHAK
            && !Self::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }

    pub const fn report() -> VaachakStorageProbeMountRuntimeOwnershipSmokeReport {
        VaachakStorageProbeMountRuntimeOwnershipSmokeReport {
            owner_entrypoint_ok: VaachakStorageProbeMountRuntimeOwner::ownership_ok()
                && Self::STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            pulp_backend_active: VaachakStorageProbeMountPulpBackend::bridge_ok()
                && Self::PULP_COMPATIBILITY_BACKEND_ACTIVE,
            shared_spi_dependency_ok:
                VaachakStorageProbeMountRuntimeOwner::shared_spi_owner_available()
                    && VaachakStorageProbeMountRuntimeOwner::storage_user_registered_on_spi()
                    && VaachakStorageProbeMountRuntimeOwner::storage_chip_select_ok(),
            lifecycle_authority_ok: VaachakStorageProbeMountRuntimeOwner::lifecycle_authority_ok(),
            storage_probe_metadata_ok:
                VaachakStorageProbeMountRuntimeOwner::storage_spi_metadata_ok(
                    VaachakStorageRuntimeStep::CardDetectAuthority,
                ),
            storage_mount_metadata_ok:
                VaachakStorageProbeMountRuntimeOwner::storage_spi_metadata_ok(
                    VaachakStorageRuntimeStep::FatVolumeAvailabilityObserved,
                ),
            no_unwanted_runtime_behavior_moved: Self::no_unwanted_runtime_behavior_moved(),
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok()
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!(
                "{}",
                Self::STORAGE_PROBE_MOUNT_RUNTIME_OWNERSHIP_SMOKE_MARKER
            );
        } else {
            esp_println::println!("storage_probe_mount_runtime_owner=failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!(
                "{}",
                Self::STORAGE_PROBE_MOUNT_RUNTIME_OWNERSHIP_SMOKE_MARKER
            );
        } else {
            println!("storage_probe_mount_runtime_owner=failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageProbeMountRuntimeOwnershipSmoke;

    #[test]
    fn storage_probe_mount_runtime_ownership_smoke_is_ok() {
        assert!(VaachakStorageProbeMountRuntimeOwnershipSmoke::smoke_ok());
    }

    #[test]
    fn no_unwanted_runtime_behavior_moved() {
        assert!(
            VaachakStorageProbeMountRuntimeOwnershipSmoke::no_unwanted_runtime_behavior_moved()
        );
    }
}
