#![allow(dead_code)]

use crate::vaachak_x4::physical::display_pulp_backend::VaachakDisplayPulpBackend;
use crate::vaachak_x4::physical::display_runtime_owner::{
    VaachakDisplayRuntimeOperation, VaachakDisplayRuntimeOwner,
};

/// Contract smoke for the display runtime ownership bridge.
///
/// This smoke proves that Vaachak owns the display runtime ownership entrypoint
/// while the active SSD1677/e-paper executor remains the Pulp compatibility
/// backend. It must not execute draw, refresh, partial-refresh, SPI, storage, or
/// reader/file-browser behavior.
pub struct VaachakDisplayRuntimeOwnershipSmoke;

impl VaachakDisplayRuntimeOwnershipSmoke {
    pub const DISPLAY_RUNTIME_OWNERSHIP_SMOKE_MARKER: &'static str =
        "x4-display-runtime-ownership-smoke-ok";

    pub const DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true;
    pub const SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn smoke_ok() -> bool {
        VaachakDisplayRuntimeOwner::ownership_ok()
            && VaachakDisplayPulpBackend::bridge_ok()
            && Self::DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && Self::PULP_COMPATIBILITY_BACKEND_ACTIVE
            && !Self::SSD1677_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::DISPLAY_SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::STORAGE_BEHAVIOR_CHANGED
            && !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }

    pub const fn full_refresh_metadata_is_safe() -> bool {
        let metadata = VaachakDisplayRuntimeOwner::operation_ownership(
            VaachakDisplayRuntimeOperation::FullRefreshMetadata,
        );
        VaachakDisplayRuntimeOwner::operation_metadata_is_safe(metadata)
    }

    pub const fn partial_refresh_metadata_is_safe() -> bool {
        let metadata = VaachakDisplayRuntimeOwner::operation_ownership(
            VaachakDisplayRuntimeOperation::PartialRefreshMetadata,
        );
        VaachakDisplayRuntimeOwner::operation_metadata_is_safe(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakDisplayRuntimeOwnershipSmoke;

    #[test]
    fn display_runtime_ownership_smoke_passes() {
        assert!(VaachakDisplayRuntimeOwnershipSmoke::smoke_ok());
        assert!(VaachakDisplayRuntimeOwnershipSmoke::full_refresh_metadata_is_safe());
        assert!(VaachakDisplayRuntimeOwnershipSmoke::partial_refresh_metadata_is_safe());
    }
}
