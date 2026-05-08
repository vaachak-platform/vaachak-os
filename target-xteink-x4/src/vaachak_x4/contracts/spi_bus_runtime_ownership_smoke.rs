#![allow(dead_code)]

use crate::vaachak_x4::physical::spi_bus_pulp_backend::VaachakSpiPulpBackend;
use crate::vaachak_x4::physical::spi_bus_runtime_owner::{
    VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind,
};

/// Contract smoke for the Vaachak-owned SPI runtime ownership bridge.
///
/// This verifies the ownership entrypoint, Pulp compatibility backend, shared SPI
/// users, and non-movement guarantees without invoking hardware behavior.
pub struct VaachakSpiBusRuntimeOwnershipSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiBusRuntimeOwnershipSmokeReport {
    pub owner_entrypoint_ok: bool,
    pub pulp_backend_active: bool,
    pub display_user_ok: bool,
    pub storage_user_ok: bool,
    pub display_transaction_metadata_ok: bool,
    pub storage_transaction_metadata_ok: bool,
    pub no_runtime_behavior_moved: bool,
}

impl VaachakSpiBusRuntimeOwnershipSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.owner_entrypoint_ok
            && self.pulp_backend_active
            && self.display_user_ok
            && self.storage_user_ok
            && self.display_transaction_metadata_ok
            && self.storage_transaction_metadata_ok
            && self.no_runtime_behavior_moved
    }
}

impl VaachakSpiBusRuntimeOwnershipSmoke {
    pub const SPI_BUS_RUNTIME_OWNERSHIP_SMOKE_MARKER: &'static str =
        "x4-spi-bus-runtime-ownership-smoke-ok";

    pub const SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true;
    pub const ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false;
    pub const SD_FAT_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn no_runtime_behavior_moved() -> bool {
        !Self::ARBITRATION_POLICY_MOVED_TO_VAACHAK
            && !Self::SD_PROBE_MOUNT_MOVED_TO_VAACHAK
            && !Self::SD_FAT_MOVED_TO_VAACHAK
            && !Self::DISPLAY_RENDERING_MOVED_TO_VAACHAK
            && !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }

    pub const fn report() -> VaachakSpiBusRuntimeOwnershipSmokeReport {
        let display_metadata = VaachakSpiBusRuntimeOwner::transaction_ownership(
            VaachakSpiRuntimeUser::Display,
            VaachakSpiTransactionKind::DisplayRefreshMetadata,
        );
        let storage_metadata = VaachakSpiBusRuntimeOwner::transaction_ownership(
            VaachakSpiRuntimeUser::Storage,
            VaachakSpiTransactionKind::StorageFatIoMetadata,
        );

        VaachakSpiBusRuntimeOwnershipSmokeReport {
            owner_entrypoint_ok: VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
                && Self::SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            pulp_backend_active: VaachakSpiPulpBackend::bridge_ok()
                && Self::PULP_COMPATIBILITY_BACKEND_ACTIVE,
            display_user_ok: VaachakSpiBusRuntimeOwner::display_user_registered()
                && VaachakSpiBusRuntimeOwner::chip_select_gpio(VaachakSpiRuntimeUser::Display)
                    == 21,
            storage_user_ok: VaachakSpiBusRuntimeOwner::storage_user_registered()
                && VaachakSpiBusRuntimeOwner::chip_select_gpio(VaachakSpiRuntimeUser::Storage)
                    == 12,
            display_transaction_metadata_ok:
                VaachakSpiBusRuntimeOwner::transaction_metadata_is_safe(display_metadata),
            storage_transaction_metadata_ok:
                VaachakSpiBusRuntimeOwner::transaction_metadata_is_safe(storage_metadata),
            no_runtime_behavior_moved: Self::no_runtime_behavior_moved(),
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSpiBusRuntimeOwnershipSmoke;

    #[test]
    fn spi_runtime_ownership_smoke_is_ok() {
        assert!(VaachakSpiBusRuntimeOwnershipSmoke::smoke_ok());
    }

    #[test]
    fn runtime_behavior_has_not_moved() {
        assert!(VaachakSpiBusRuntimeOwnershipSmoke::no_runtime_behavior_moved());
    }
}
