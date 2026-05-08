#![allow(dead_code)]

use crate::vaachak_x4::physical::spi_bus_arbitration_pulp_backend::VaachakSpiArbitrationPulpBackend;
use crate::vaachak_x4::physical::spi_bus_arbitration_runtime_owner::VaachakSpiBusArbitrationRuntimeOwner;
use crate::vaachak_x4::physical::spi_bus_runtime_owner::VaachakSpiBusRuntimeOwner;

pub struct VaachakSpiBusArbitrationRuntimeOwnershipSmoke;

impl VaachakSpiBusArbitrationRuntimeOwnershipSmoke {
    pub const MARKER: &'static str = "x4-spi-bus-arbitration-runtime-ownership-smoke-ok";

    pub const SPI_BUS_OWNER_READY: bool = VaachakSpiBusRuntimeOwner::ownership_bridge_ok();
    pub const ARBITRATION_RUNTIME_OWNER_READY: bool =
        VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok();
    pub const PULP_COMPATIBILITY_BACKEND_ACTIVE: bool =
        VaachakSpiArbitrationPulpBackend::backend_ok();

    pub const SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn smoke_ok() -> bool {
        Self::SPI_BUS_OWNER_READY
            && Self::ARBITRATION_RUNTIME_OWNER_READY
            && Self::PULP_COMPATIBILITY_BACKEND_ACTIVE
            && Self::SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && Self::SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK
            && !Self::PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::DISPLAY_BEHAVIOR_CHANGED
            && !Self::STORAGE_BEHAVIOR_CHANGED
            && !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSpiBusArbitrationRuntimeOwnershipSmoke;

    #[test]
    fn spi_bus_arbitration_runtime_ownership_smoke_passes() {
        assert!(VaachakSpiBusArbitrationRuntimeOwnershipSmoke::smoke_ok());
    }
}
