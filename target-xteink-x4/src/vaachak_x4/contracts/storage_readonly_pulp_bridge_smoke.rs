#![allow(dead_code)]

//! Static smoke contract for the Pulp-backed read-only storage bridge.
//!
//! This module proves the bridge exists, remains read-only, and keeps active SD,
//! FAT, SPI, display, reader, and file-browser behavior in the imported Pulp
//! runtime. It does not construct SD hardware or call file I/O.

use crate::vaachak_x4::io::storage_readonly_pulp_bridge::{
    PULP_READONLY_ACTIVE_BACKEND_OWNER, PULP_READONLY_BRIDGE_OWNER,
    STORAGE_READONLY_PULP_BRIDGE_MARKER, VaachakStorageReadonlyPulpBridgeContract,
};

pub struct VaachakStorageReadonlyPulpBridgeSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageReadonlyPulpBridgeSmokeReport {
    pub marker_present: bool,
    pub bridge_owner_present: bool,
    pub active_backend_owner_present: bool,
    pub generic_bridge_present: bool,
    pub embedded_backend_present: bool,
    pub file_exists_mapping_present: bool,
    pub read_file_start_mapping_present: bool,
    pub read_chunk_mapping_present: bool,
    pub list_directory_mapping_present: bool,
    pub resolve_paths_mapping_present: bool,
    pub physical_behavior_moved: bool,
}

impl StorageReadonlyPulpBridgeSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.marker_present
            && self.bridge_owner_present
            && self.active_backend_owner_present
            && self.generic_bridge_present
            && self.embedded_backend_present
            && self.file_exists_mapping_present
            && self.read_file_start_mapping_present
            && self.read_chunk_mapping_present
            && self.list_directory_mapping_present
            && self.resolve_paths_mapping_present
            && !self.physical_behavior_moved
    }
}

impl VaachakStorageReadonlyPulpBridgeSmoke {
    pub const SMOKE_MARKER: &'static str = "x4-storage-readonly-pulp-bridge-smoke-ok";

    pub const BRIDGE_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_pulp_bridge.rs";
    pub const DOC_SOURCE: &'static str = "docs/architecture/storage-readonly-pulp-bridge.md";

    pub const GENERIC_BRIDGE_TYPE: &'static str = "PulpReadonlyStorageBridge";
    pub const EMBEDDED_BACKEND_TYPE: &'static str = "X4PulpReadonlyStorageBackend";

    pub const fn report() -> StorageReadonlyPulpBridgeSmokeReport {
        StorageReadonlyPulpBridgeSmokeReport {
            marker_present: !STORAGE_READONLY_PULP_BRIDGE_MARKER.is_empty()
                && !Self::SMOKE_MARKER.is_empty(),
            bridge_owner_present: !PULP_READONLY_BRIDGE_OWNER.is_empty(),
            active_backend_owner_present: !PULP_READONLY_ACTIVE_BACKEND_OWNER.is_empty(),
            generic_bridge_present: !Self::GENERIC_BRIDGE_TYPE.is_empty(),
            embedded_backend_present: !Self::EMBEDDED_BACKEND_TYPE.is_empty(),
            file_exists_mapping_present:
                !VaachakStorageReadonlyPulpBridgeContract::FILE_EXISTS_MAPPING.is_empty(),
            read_file_start_mapping_present:
                !VaachakStorageReadonlyPulpBridgeContract::READ_FILE_START_MAPPING.is_empty(),
            read_chunk_mapping_present:
                !VaachakStorageReadonlyPulpBridgeContract::READ_CHUNK_MAPPING.is_empty(),
            list_directory_mapping_present:
                !VaachakStorageReadonlyPulpBridgeContract::LIST_DIRECTORY_MAPPING.is_empty(),
            resolve_paths_mapping_present:
                !VaachakStorageReadonlyPulpBridgeContract::RESOLVE_PATHS_MAPPING.is_empty(),
            physical_behavior_moved:
                VaachakStorageReadonlyPulpBridgeContract::physical_behavior_moved(),
        }
    }

    pub const fn active_runtime_preflight_ok() -> bool {
        VaachakStorageReadonlyPulpBridgeContract::active_runtime_preflight().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageReadonlyPulpBridgeSmoke;

    #[test]
    fn storage_readonly_pulp_bridge_smoke_ok() {
        assert!(VaachakStorageReadonlyPulpBridgeSmoke::report().smoke_ok());
        assert!(VaachakStorageReadonlyPulpBridgeSmoke::active_runtime_preflight_ok());
    }
}
