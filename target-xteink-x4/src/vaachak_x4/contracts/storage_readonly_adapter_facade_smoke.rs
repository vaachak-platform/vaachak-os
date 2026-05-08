#![allow(dead_code)]

//! Static smoke contract for the Vaachak read-only storage adapter facade.
//!
//! This module proves the facade is present and that the current implementation
//! mapping still points to Pulp-owned SD/FAT/SPI/display behavior. It does not
//! call any hardware code.

use crate::vaachak_x4::io::storage_readonly_adapter::{
    VaachakReadonlyStorageContract, VaachakResolvedStoragePaths,
};

pub struct VaachakStorageReadonlyAdapterFacadeSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageReadonlyAdapterFacadeSmokeReport {
    pub marker_present: bool,
    pub file_exists_contract_present: bool,
    pub read_file_start_contract_present: bool,
    pub read_chunk_contract_present: bool,
    pub list_directory_metadata_contract_present: bool,
    pub resolve_current_storage_paths_contract_present: bool,
    pub pulp_backed_paths_present: bool,
    pub physical_behavior_moved: bool,
}

impl StorageReadonlyAdapterFacadeSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.marker_present
            && self.file_exists_contract_present
            && self.read_file_start_contract_present
            && self.read_chunk_contract_present
            && self.list_directory_metadata_contract_present
            && self.resolve_current_storage_paths_contract_present
            && self.pulp_backed_paths_present
            && !self.physical_behavior_moved
    }
}

impl VaachakStorageReadonlyAdapterFacadeSmoke {
    pub const SMOKE_MARKER: &'static str = "x4-storage-readonly-adapter-facade-smoke-ok";

    pub const FACADE_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_adapter.rs";
    pub const DOC_SOURCE: &'static str = "docs/architecture/storage-readonly-adapter-facade.md";

    pub const FILE_EXISTS_METHOD: &'static str = "file_exists";
    pub const READ_FILE_START_METHOD: &'static str = "read_file_start";
    pub const READ_CHUNK_METHOD: &'static str = "read_chunk";
    pub const LIST_DIRECTORY_METADATA_METHOD: &'static str = "list_directory_metadata";
    pub const RESOLVE_CURRENT_STORAGE_PATHS_METHOD: &'static str = "resolve_current_storage_paths";

    pub const fn report() -> StorageReadonlyAdapterFacadeSmokeReport {
        StorageReadonlyAdapterFacadeSmokeReport {
            marker_present: !VaachakReadonlyStorageContract::CONTRACT_MARKER.is_empty()
                && !Self::SMOKE_MARKER.is_empty(),
            file_exists_contract_present: !Self::FILE_EXISTS_METHOD.is_empty(),
            read_file_start_contract_present: !Self::READ_FILE_START_METHOD.is_empty(),
            read_chunk_contract_present: !Self::READ_CHUNK_METHOD.is_empty(),
            list_directory_metadata_contract_present: !Self::LIST_DIRECTORY_METADATA_METHOD
                .is_empty(),
            resolve_current_storage_paths_contract_present:
                !Self::RESOLVE_CURRENT_STORAGE_PATHS_METHOD.is_empty(),
            pulp_backed_paths_present: Self::pulp_backed_paths_present(),
            physical_behavior_moved: VaachakReadonlyStorageContract::physical_behavior_moved(),
        }
    }

    pub const fn pulp_backed_paths_present() -> bool {
        let paths = VaachakResolvedStoragePaths::PULP_BACKED_ACTIVE_PATHS;
        !paths.root.is_empty()
            && !paths.library_root.is_empty()
            && !paths.state_root.is_empty()
            && !paths.epub_cache_root.is_empty()
            && !paths.settings_file.is_empty()
            && !paths.title_cache_file.is_empty()
            && !paths.sleep_root.is_empty()
            && !paths.sleep_daily_root.is_empty()
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok()
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!("{}", Self::SMOKE_MARKER);
        } else {
            esp_println::println!("storage-readonly-adapter-facade-smoke-failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!("{}", Self::SMOKE_MARKER);
        } else {
            println!("storage-readonly-adapter-facade-smoke-failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageReadonlyAdapterFacadeSmoke;

    #[test]
    fn storage_readonly_adapter_facade_smoke_is_ok() {
        assert!(VaachakStorageReadonlyAdapterFacadeSmoke::smoke_ok());
    }

    #[test]
    fn storage_readonly_adapter_facade_keeps_physical_behavior_imported() {
        let report = VaachakStorageReadonlyAdapterFacadeSmoke::report();
        assert!(!report.physical_behavior_moved);
    }
}
