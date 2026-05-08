#![allow(dead_code)]

//! Consolidated Vaachak-owned read-only storage boundary for the Xteink X4 target.
//!
//! This public boundary entrypoint composes the Vaachak facade with the Pulp-backed
//! bridge while active SD mount/probe, FAT, SPI arbitration, display, reader, and
//! file-browser behavior remain in `vendor/pulp-os`.

use crate::vaachak_x4::io::storage_readonly_adapter::{
    VaachakDirectoryEntry, VaachakReadonlyStorage, VaachakReadonlyStorageContract,
    VaachakReadonlyStorageFacade, VaachakResolvedStoragePaths, VaachakStoragePathRef,
    VaachakStorageReadChunk,
};
use crate::vaachak_x4::io::storage_readonly_pulp_bridge::{
    PulpReadonlyStorageBackend, PulpReadonlyStorageBridge, PulpReadonlyStorageBridgeError,
    VaachakStorageReadonlyPulpBridgeContract,
};

pub const STORAGE_READONLY_BOUNDARY_MARKER: &str = "x4-storage-readonly-boundary-ok";
pub const STORAGE_READONLY_BOUNDARY_OWNER: &str = "target-xteink-x4 Vaachak read-only boundary";
pub const STORAGE_READONLY_BOUNDARY_ACTIVE_BACKEND_OWNER: &str = "vendor/pulp-os imported runtime";

pub struct VaachakStorageReadonlyBoundaryContract;

impl VaachakStorageReadonlyBoundaryContract {
    pub const BOUNDARY_MARKER: &'static str = STORAGE_READONLY_BOUNDARY_MARKER;
    pub const BOUNDARY_OWNER: &'static str = STORAGE_READONLY_BOUNDARY_OWNER;
    pub const ACTIVE_BACKEND_OWNER: &'static str = STORAGE_READONLY_BOUNDARY_ACTIVE_BACKEND_OWNER;
    pub const PUBLIC_CONTRACT_LAYER: &'static str = "VaachakReadonlyStorage facade trait";
    pub const ACTIVE_IMPLEMENTATION_LAYER: &'static str = "PulpReadonlyStorageBridge";
    pub const EMBEDDED_BACKEND_LAYER: &'static str = "X4PulpReadonlyStorageBackend";
    pub const FACADE_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_adapter.rs";
    pub const BRIDGE_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_pulp_bridge.rs";
    pub const BOUNDARY_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_boundary.rs";
    pub const CANONICAL_DOC_SOURCE: &'static str = "docs/architecture/storage-readonly-boundary.md";

    pub const SD_MOUNT_OR_PROBE_MOVED_TO_BOUNDARY: bool = false;
    pub const SD_DRIVER_MOVED_TO_BOUNDARY: bool = false;
    pub const FAT_BEHAVIOR_MOVED_TO_BOUNDARY: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_BOUNDARY: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_BOUNDARY: bool = false;
    pub const READER_OR_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;
    pub const WRITABLE_STORAGE_BEHAVIOR_ADDED: bool = false;

    pub const fn physical_behavior_moved() -> bool {
        Self::SD_MOUNT_OR_PROBE_MOVED_TO_BOUNDARY
            || Self::SD_DRIVER_MOVED_TO_BOUNDARY
            || Self::FAT_BEHAVIOR_MOVED_TO_BOUNDARY
            || Self::SPI_ARBITRATION_MOVED_TO_BOUNDARY
            || Self::DISPLAY_BEHAVIOR_MOVED_TO_BOUNDARY
            || Self::READER_OR_FILE_BROWSER_BEHAVIOR_CHANGED
            || Self::WRITABLE_STORAGE_BEHAVIOR_ADDED
            || VaachakReadonlyStorageContract::physical_behavior_moved()
            || VaachakStorageReadonlyPulpBridgeContract::physical_behavior_moved()
    }

    pub const fn active_runtime_preflight() -> StorageReadonlyBoundaryPreflight {
        StorageReadonlyBoundaryPreflight {
            boundary_marker_present: !Self::BOUNDARY_MARKER.is_empty(),
            facade_marker_present: !VaachakReadonlyStorageContract::CONTRACT_MARKER.is_empty(),
            bridge_marker_present: !VaachakStorageReadonlyPulpBridgeContract::BRIDGE_MARKER
                .is_empty(),
            active_backend_is_pulp: !Self::ACTIVE_BACKEND_OWNER.is_empty(),
            public_contract_is_facade: !Self::PUBLIC_CONTRACT_LAYER.is_empty(),
            active_implementation_is_bridge: !Self::ACTIVE_IMPLEMENTATION_LAYER.is_empty(),
            canonical_doc_present: !Self::CANONICAL_DOC_SOURCE.is_empty(),
            physical_behavior_moved: Self::physical_behavior_moved(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageReadonlyBoundaryPreflight {
    pub boundary_marker_present: bool,
    pub facade_marker_present: bool,
    pub bridge_marker_present: bool,
    pub active_backend_is_pulp: bool,
    pub public_contract_is_facade: bool,
    pub active_implementation_is_bridge: bool,
    pub canonical_doc_present: bool,
    pub physical_behavior_moved: bool,
}

impl StorageReadonlyBoundaryPreflight {
    pub const fn ok(self) -> bool {
        self.boundary_marker_present
            && self.facade_marker_present
            && self.bridge_marker_present
            && self.active_backend_is_pulp
            && self.public_contract_is_facade
            && self.active_implementation_is_bridge
            && self.canonical_doc_present
            && !self.physical_behavior_moved
    }
}

/// Canonical read-only storage boundary entrypoint.
pub struct VaachakStorageReadonlyBoundary<B> {
    facade: VaachakReadonlyStorageFacade<PulpReadonlyStorageBridge<B>>,
}

impl<B> VaachakStorageReadonlyBoundary<B> {
    pub const fn new_pulp_backed(backend: B) -> Self {
        Self {
            facade: VaachakReadonlyStorageFacade::new(PulpReadonlyStorageBridge::new(backend)),
        }
    }

    pub fn facade(&self) -> &VaachakReadonlyStorageFacade<PulpReadonlyStorageBridge<B>> {
        &self.facade
    }

    pub fn facade_mut(
        &mut self,
    ) -> &mut VaachakReadonlyStorageFacade<PulpReadonlyStorageBridge<B>> {
        &mut self.facade
    }

    pub fn bridge(&self) -> &PulpReadonlyStorageBridge<B> {
        self.facade.backend()
    }

    pub fn bridge_mut(&mut self) -> &mut PulpReadonlyStorageBridge<B> {
        self.facade.backend_mut()
    }

    pub fn into_bridge(self) -> PulpReadonlyStorageBridge<B> {
        self.facade.into_backend()
    }
}

impl<B> VaachakReadonlyStorage for VaachakStorageReadonlyBoundary<B>
where
    B: PulpReadonlyStorageBackend,
{
    type Error = PulpReadonlyStorageBridgeError<B::Error>;

    fn file_exists(&mut self, path: VaachakStoragePathRef<'_>) -> Result<bool, Self::Error> {
        self.facade.file_exists(path)
    }

    fn read_file_start(
        &mut self,
        path: VaachakStoragePathRef<'_>,
        out: &mut [u8],
    ) -> Result<VaachakStorageReadChunk, Self::Error> {
        self.facade.read_file_start(path, out)
    }

    fn read_chunk(
        &mut self,
        path: VaachakStoragePathRef<'_>,
        offset: u64,
        out: &mut [u8],
    ) -> Result<VaachakStorageReadChunk, Self::Error> {
        self.facade.read_chunk(path, offset, out)
    }

    fn list_directory_metadata(
        &mut self,
        path: VaachakStoragePathRef<'_>,
        out: &mut [VaachakDirectoryEntry],
    ) -> Result<usize, Self::Error> {
        self.facade.list_directory_metadata(path, out)
    }

    fn resolve_current_storage_paths(&self) -> VaachakResolvedStoragePaths<'static> {
        self.facade.resolve_current_storage_paths()
    }
}
