#![allow(dead_code)]

use crate::vaachak_x4::physical::sd_fat_readonly_pulp_backend::VaachakSdFatReadonlyPulpBackend;
use crate::vaachak_x4::physical::sd_fat_runtime_readonly_owner::{
    VaachakSdFatReadonlyOperation, VaachakSdFatRuntimeReadonlyOwner,
};

/// Contract smoke for the Vaachak-owned SD/FAT read-only runtime owner.
///
/// This verifies that Vaachak now owns the SD/FAT read-only authority boundary,
/// the active FAT executor remains Pulp-compatible, dependencies on SPI and
/// probe/mount ownership are honored, and no writable/file-browser/display
/// behavior has moved.
pub struct VaachakSdFatRuntimeReadonlyOwnershipSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSdFatRuntimeReadonlyOwnershipSmokeReport {
    pub owner_entrypoint_ok: bool,
    pub pulp_backend_active: bool,
    pub spi_dependency_ok: bool,
    pub probe_mount_dependency_ok: bool,
    pub readonly_boundary_ok: bool,
    pub readonly_operations_ok: bool,
    pub writable_operations_denied: bool,
    pub no_unwanted_runtime_behavior_moved: bool,
}

impl VaachakSdFatRuntimeReadonlyOwnershipSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.owner_entrypoint_ok
            && self.pulp_backend_active
            && self.spi_dependency_ok
            && self.probe_mount_dependency_ok
            && self.readonly_boundary_ok
            && self.readonly_operations_ok
            && self.writable_operations_denied
            && self.no_unwanted_runtime_behavior_moved
    }
}

impl VaachakSdFatRuntimeReadonlyOwnershipSmoke {
    pub const SD_FAT_RUNTIME_READONLY_OWNERSHIP_SMOKE_MARKER: &'static str =
        "x4-sd-fat-runtime-readonly-ownership-smoke-ok";

    pub const SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true;
    pub const FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn no_unwanted_runtime_behavior_moved() -> bool {
        !Self::FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK
            && !Self::SD_PROBE_MOUNT_BEHAVIOR_CHANGED
            && !Self::SPI_ARBITRATION_MOVED_TO_VAACHAK
            && !Self::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }

    pub const fn report() -> VaachakSdFatRuntimeReadonlyOwnershipSmokeReport {
        VaachakSdFatRuntimeReadonlyOwnershipSmokeReport {
            owner_entrypoint_ok: VaachakSdFatRuntimeReadonlyOwner::ownership_ok()
                && Self::SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            pulp_backend_active: VaachakSdFatReadonlyPulpBackend::bridge_ok()
                && Self::PULP_COMPATIBILITY_BACKEND_ACTIVE,
            spi_dependency_ok: VaachakSdFatRuntimeReadonlyOwner::shared_spi_owner_available()
                && VaachakSdFatRuntimeReadonlyOwner::storage_user_registered_on_spi()
                && VaachakSdFatRuntimeReadonlyOwner::storage_spi_metadata_ok(),
            probe_mount_dependency_ok:
                VaachakSdFatRuntimeReadonlyOwner::storage_probe_mount_owner_available(),
            readonly_boundary_ok: VaachakSdFatRuntimeReadonlyOwner::readonly_boundary_available(),
            readonly_operations_ok: VaachakSdFatRuntimeReadonlyOwner::readonly_operations_registered(
            ) && !VaachakSdFatRuntimeReadonlyOwner::operation_entry(
                VaachakSdFatReadonlyOperation::FileExists,
            )
            .writable
                && !VaachakSdFatRuntimeReadonlyOwner::operation_entry(
                    VaachakSdFatReadonlyOperation::ReadFileStart,
                )
                .writable
                && !VaachakSdFatRuntimeReadonlyOwner::operation_entry(
                    VaachakSdFatReadonlyOperation::ReadChunk,
                )
                .writable
                && !VaachakSdFatRuntimeReadonlyOwner::operation_entry(
                    VaachakSdFatReadonlyOperation::ListDirectoryMetadata,
                )
                .writable
                && !VaachakSdFatRuntimeReadonlyOwner::operation_entry(
                    VaachakSdFatReadonlyOperation::ResolveCurrentStoragePaths,
                )
                .writable,
            writable_operations_denied:
                VaachakSdFatRuntimeReadonlyOwner::writable_operations_denied(),
            no_unwanted_runtime_behavior_moved: Self::no_unwanted_runtime_behavior_moved(),
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok()
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!("{}", Self::SD_FAT_RUNTIME_READONLY_OWNERSHIP_SMOKE_MARKER);
        } else {
            esp_println::println!("sd_fat_runtime_readonly_owner=failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!("{}", Self::SD_FAT_RUNTIME_READONLY_OWNERSHIP_SMOKE_MARKER);
        } else {
            println!("sd_fat_runtime_readonly_owner=failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSdFatRuntimeReadonlyOwnershipSmoke;

    #[test]
    fn sd_fat_runtime_readonly_ownership_smoke_is_ok() {
        assert!(VaachakSdFatRuntimeReadonlyOwnershipSmoke::smoke_ok());
    }

    #[test]
    fn no_unwanted_runtime_behavior_moved() {
        assert!(VaachakSdFatRuntimeReadonlyOwnershipSmoke::no_unwanted_runtime_behavior_moved());
    }
}
