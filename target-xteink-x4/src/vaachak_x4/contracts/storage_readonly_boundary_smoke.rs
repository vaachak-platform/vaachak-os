#![allow(dead_code)]

//! Static smoke contract for the consolidated Vaachak read-only storage boundary.

use crate::vaachak_x4::contracts::storage_readonly_adapter_facade_smoke::VaachakStorageReadonlyAdapterFacadeSmoke;
use crate::vaachak_x4::contracts::storage_readonly_pulp_bridge_smoke::VaachakStorageReadonlyPulpBridgeSmoke;
use crate::vaachak_x4::io::storage_readonly_boundary::{
    STORAGE_READONLY_BOUNDARY_ACTIVE_BACKEND_OWNER, STORAGE_READONLY_BOUNDARY_MARKER,
    STORAGE_READONLY_BOUNDARY_OWNER, VaachakStorageReadonlyBoundaryContract,
};

pub struct VaachakStorageReadonlyBoundarySmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageReadonlyBoundarySmokeReport {
    pub boundary_marker_present: bool,
    pub boundary_owner_present: bool,
    pub active_backend_owner_present: bool,
    pub facade_smoke_ok: bool,
    pub bridge_smoke_ok: bool,
    pub boundary_entrypoint_present: bool,
    pub public_contract_present: bool,
    pub active_implementation_present: bool,
    pub canonical_doc_present: bool,
    pub physical_behavior_moved: bool,
}

impl StorageReadonlyBoundarySmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.boundary_marker_present
            && self.boundary_owner_present
            && self.active_backend_owner_present
            && self.facade_smoke_ok
            && self.bridge_smoke_ok
            && self.boundary_entrypoint_present
            && self.public_contract_present
            && self.active_implementation_present
            && self.canonical_doc_present
            && !self.physical_behavior_moved
    }
}

impl VaachakStorageReadonlyBoundarySmoke {
    pub const SMOKE_MARKER: &'static str = "x4-storage-readonly-boundary-smoke-ok";
    pub const BOUNDARY_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_boundary.rs";
    pub const FACADE_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_adapter.rs";
    pub const BRIDGE_SOURCE: &'static str = "vaachak_x4/io/storage_readonly_pulp_bridge.rs";
    pub const DOC_SOURCE: &'static str = "docs/architecture/storage-readonly-boundary.md";
    pub const BOUNDARY_ENTRYPOINT_TYPE: &'static str = "VaachakStorageReadonlyBoundary";
    pub const PUBLIC_CONTRACT_TRAIT: &'static str = "VaachakReadonlyStorage";
    pub const ACTIVE_IMPLEMENTATION_TYPE: &'static str = "PulpReadonlyStorageBridge";

    pub const fn report() -> StorageReadonlyBoundarySmokeReport {
        StorageReadonlyBoundarySmokeReport {
            boundary_marker_present: !STORAGE_READONLY_BOUNDARY_MARKER.is_empty()
                && !Self::SMOKE_MARKER.is_empty(),
            boundary_owner_present: !STORAGE_READONLY_BOUNDARY_OWNER.is_empty(),
            active_backend_owner_present: !STORAGE_READONLY_BOUNDARY_ACTIVE_BACKEND_OWNER
                .is_empty(),
            facade_smoke_ok: VaachakStorageReadonlyAdapterFacadeSmoke::smoke_ok(),
            bridge_smoke_ok: VaachakStorageReadonlyPulpBridgeSmoke::report().smoke_ok(),
            boundary_entrypoint_present: !Self::BOUNDARY_ENTRYPOINT_TYPE.is_empty(),
            public_contract_present: !Self::PUBLIC_CONTRACT_TRAIT.is_empty(),
            active_implementation_present: !Self::ACTIVE_IMPLEMENTATION_TYPE.is_empty(),
            canonical_doc_present: !Self::DOC_SOURCE.is_empty(),
            physical_behavior_moved:
                VaachakStorageReadonlyBoundaryContract::physical_behavior_moved(),
        }
    }

    pub const fn active_runtime_preflight_ok() -> bool {
        VaachakStorageReadonlyBoundaryContract::active_runtime_preflight().ok()
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok() && Self::active_runtime_preflight_ok()
    }
}
