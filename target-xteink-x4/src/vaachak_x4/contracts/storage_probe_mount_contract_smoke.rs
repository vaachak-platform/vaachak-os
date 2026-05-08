#![allow(dead_code)]

use crate::vaachak_x4::physical::storage_probe_mount_contract::VaachakStorageProbeMountContract;

pub struct VaachakStorageProbeMountContractSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountContractSmokeReport {
    pub contract_ok: bool,
    pub lifecycle_documented: bool,
    pub shared_spi_dependency_documented: bool,
    pub active_sd_runtime_still_imported: bool,
    pub active_fat_runtime_still_imported: bool,
    pub active_spi_arbitration_still_imported: bool,
    pub active_display_runtime_still_imported: bool,
    pub no_runtime_behavior_moved: bool,
}

impl VaachakStorageProbeMountContractSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.contract_ok
            && self.lifecycle_documented
            && self.shared_spi_dependency_documented
            && self.active_sd_runtime_still_imported
            && self.active_fat_runtime_still_imported
            && self.active_spi_arbitration_still_imported
            && self.active_display_runtime_still_imported
            && self.no_runtime_behavior_moved
    }
}

impl VaachakStorageProbeMountContractSmoke {
    pub const STORAGE_PROBE_MOUNT_CONTRACT_SMOKE_MARKER: &'static str =
        "storage_probe_mount_contract=ok";

    pub fn report() -> VaachakStorageProbeMountContractSmokeReport {
        let report = VaachakStorageProbeMountContract::contract_report();
        VaachakStorageProbeMountContractSmokeReport {
            contract_ok: VaachakStorageProbeMountContract::contract_ok(),
            lifecycle_documented: report.lifecycle_documented,
            shared_spi_dependency_documented: report.shared_spi_dependency_documented,
            active_sd_runtime_still_imported: report.active_sd_runtime_still_imported,
            active_fat_runtime_still_imported: report.active_fat_runtime_still_imported,
            active_spi_arbitration_still_imported: report.active_spi_arbitration_still_imported,
            active_display_runtime_still_imported: report.active_display_runtime_still_imported,
            no_runtime_behavior_moved: report.no_runtime_behavior_moved,
        }
    }

    pub fn smoke_ok() -> bool {
        Self::report().smoke_ok()
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!("{}", Self::STORAGE_PROBE_MOUNT_CONTRACT_SMOKE_MARKER);
        } else {
            esp_println::println!("storage_probe_mount_contract=failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!("{}", Self::STORAGE_PROBE_MOUNT_CONTRACT_SMOKE_MARKER);
        } else {
            println!("storage_probe_mount_contract=failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageProbeMountContractSmoke;

    #[test]
    fn storage_probe_mount_contract_smoke_is_ok() {
        assert!(VaachakStorageProbeMountContractSmoke::smoke_ok());
    }

    #[test]
    fn storage_probe_mount_contract_smoke_reports_imported_runtime() {
        let report = VaachakStorageProbeMountContractSmoke::report();
        assert!(report.active_sd_runtime_still_imported);
        assert!(report.active_fat_runtime_still_imported);
        assert!(report.active_spi_arbitration_still_imported);
        assert!(report.active_display_runtime_still_imported);
        assert!(report.no_runtime_behavior_moved);
    }
}
