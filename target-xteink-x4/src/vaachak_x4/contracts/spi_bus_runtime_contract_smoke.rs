#![allow(dead_code)]

use crate::vaachak_x4::physical::spi_bus_runtime_contract::VaachakSpiBusRuntimeContract;

/// Smoke contract for the consolidated SPI bus runtime metadata layer.
///
/// This module intentionally reaches only into Vaachak contract metadata. It does
/// not initialize SPI, mount SD, probe FAT, initialize SSD1677, or refresh the display.
pub struct VaachakSpiBusRuntimeContractSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpiBusRuntimeContractSmokeReport {
    pub contract_ok: bool,
    pub shared_users_documented: bool,
    pub arbitration_still_imported: bool,
    pub sd_runtime_still_imported: bool,
    pub display_runtime_still_imported: bool,
    pub no_runtime_behavior_moved: bool,
}

impl SpiBusRuntimeContractSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.contract_ok
            && self.shared_users_documented
            && self.arbitration_still_imported
            && self.sd_runtime_still_imported
            && self.display_runtime_still_imported
            && self.no_runtime_behavior_moved
    }
}

impl VaachakSpiBusRuntimeContractSmoke {
    pub const SPI_BUS_RUNTIME_CONTRACT_SMOKE_MARKER: &'static str =
        "x4-spi-bus-runtime-contract-smoke-ok";

    pub const CONTRACT_SOURCE: &'static str = "vaachak_x4/physical/spi_bus_runtime_contract.rs";
    pub const RUNTIME_FACADE_SOURCE: &'static str = "vaachak_x4/physical/spi_bus_runtime.rs";
    pub const ACTIVE_ARBITRATION_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_SD_RUNTIME_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_DISPLAY_RUNTIME_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const SD_RUNTIME_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_RUNTIME_MOVED_TO_VAACHAK: bool = false;

    pub fn report() -> SpiBusRuntimeContractSmokeReport {
        let contract_report = VaachakSpiBusRuntimeContract::report();

        SpiBusRuntimeContractSmokeReport {
            contract_ok: contract_report.contract_ok(),
            shared_users_documented: contract_report.shared_users_documented,
            arbitration_still_imported: contract_report.arbitration_still_imported,
            sd_runtime_still_imported: contract_report.sd_runtime_still_imported,
            display_runtime_still_imported: contract_report.display_runtime_still_imported,
            no_runtime_behavior_moved: contract_report.no_runtime_behavior_moved,
        }
    }

    pub fn smoke_ok() -> bool {
        Self::report().smoke_ok()
            && !Self::SPI_ARBITRATION_MOVED_TO_VAACHAK
            && !Self::SD_RUNTIME_MOVED_TO_VAACHAK
            && !Self::DISPLAY_RUNTIME_MOVED_TO_VAACHAK
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!("{}", Self::SPI_BUS_RUNTIME_CONTRACT_SMOKE_MARKER);
        } else {
            esp_println::println!("spi-bus-runtime-contract-smoke-failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!("{}", Self::SPI_BUS_RUNTIME_CONTRACT_SMOKE_MARKER);
        } else {
            println!("spi-bus-runtime-contract-smoke-failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSpiBusRuntimeContractSmoke;

    #[test]
    fn spi_bus_contract_smoke_is_ok() {
        assert!(VaachakSpiBusRuntimeContractSmoke::smoke_ok());
    }

    #[test]
    fn spi_bus_contract_smoke_keeps_runtime_imported() {
        let report = VaachakSpiBusRuntimeContractSmoke::report();
        assert!(report.arbitration_still_imported);
        assert!(report.sd_runtime_still_imported);
        assert!(report.display_runtime_still_imported);
        assert!(report.no_runtime_behavior_moved);
    }
}
