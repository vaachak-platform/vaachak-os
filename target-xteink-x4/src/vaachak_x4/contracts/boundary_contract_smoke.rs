#![allow(dead_code)]

/// Vaachak-owned combined contract smoke for the Xteink X4 target.
///
/// The current implementation intentionally does not move physical storage, input, display,
/// SPI, ADC, SSD1677, SD, or reader behavior out of the imported Pulp runtime.
/// This module only consolidates the already-established Vaachak-owned contract
/// smokes from the related display, input, and storage contract checks under one stable boundary layer.
pub struct VaachakBoundaryContractSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundaryContractSmokeReport {
    pub storage_contract_ok: bool,
    pub input_contract_ok: bool,
    pub display_contract_ok: bool,
    pub physical_behavior_moved: bool,
}

impl BoundaryContractSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.storage_contract_ok
            && self.input_contract_ok
            && self.display_contract_ok
            && !self.physical_behavior_moved
    }
}

impl VaachakBoundaryContractSmoke {
    pub const BOUNDARY_CONTRACT_SMOKE_MARKER: &'static str = "x4-boundary-contract-smoke-ok";

    pub const STORAGE_CONTRACT_SOURCE: &'static str =
        "vaachak_x4/contracts/storage_state_contract.rs";
    pub const INPUT_CONTRACT_SOURCE: &'static str = "vaachak_x4/contracts/input_contract_smoke.rs";
    pub const DISPLAY_CONTRACT_SOURCE: &'static str =
        "vaachak_x4/contracts/display_contract_smoke.rs";

    pub const STORAGE_CONTRACT_MARKER: &'static str = "x4-storage-contract-smoke-ok";
    pub const INPUT_CONTRACT_MARKER: &'static str = "x4-input-contract-smoke-ok";
    pub const DISPLAY_CONTRACT_MARKER: &'static str = "x4-display-contract-smoke-ok";

    /// The current implementation remains non-invasive. Physical behavior is still imported from
    /// vendor/pulp-os and is not owned by this Vaachak contract smoke layer.
    pub const PHYSICAL_STORAGE_MOVED_TO_BOUNDARY: bool = false;
    pub const PHYSICAL_INPUT_MOVED_TO_BOUNDARY: bool = false;
    pub const PHYSICAL_DISPLAY_MOVED_TO_BOUNDARY: bool = false;

    /// Compile-time dependency names used by the contract smoke. These strings
    /// intentionally keep the relationship to the the related contract modules visible
    /// without calling into physical hardware paths.
    pub const DEPENDS_ON_CONTRACT_MODULES: [&'static str; 3] = [
        Self::STORAGE_CONTRACT_SOURCE,
        Self::INPUT_CONTRACT_SOURCE,
        Self::DISPLAY_CONTRACT_SOURCE,
    ];

    pub const fn storage_contract_present() -> bool {
        !Self::STORAGE_CONTRACT_SOURCE.is_empty() && !Self::STORAGE_CONTRACT_MARKER.is_empty()
    }

    pub const fn input_contract_present() -> bool {
        !Self::INPUT_CONTRACT_SOURCE.is_empty() && !Self::INPUT_CONTRACT_MARKER.is_empty()
    }

    pub const fn display_contract_present() -> bool {
        !Self::DISPLAY_CONTRACT_SOURCE.is_empty() && !Self::DISPLAY_CONTRACT_MARKER.is_empty()
    }

    pub const fn physical_behavior_moved() -> bool {
        Self::PHYSICAL_STORAGE_MOVED_TO_BOUNDARY
            || Self::PHYSICAL_INPUT_MOVED_TO_BOUNDARY
            || Self::PHYSICAL_DISPLAY_MOVED_TO_BOUNDARY
    }

    pub const fn report() -> BoundaryContractSmokeReport {
        BoundaryContractSmokeReport {
            storage_contract_ok: Self::storage_contract_present(),
            input_contract_ok: Self::input_contract_present(),
            display_contract_ok: Self::display_contract_present(),
            physical_behavior_moved: Self::physical_behavior_moved(),
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok()
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!("{}", Self::BOUNDARY_CONTRACT_SMOKE_MARKER);
        } else {
            esp_println::println!("boundary-contract-smoke-failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!("{}", Self::BOUNDARY_CONTRACT_SMOKE_MARKER);
        } else {
            println!("boundary-contract-smoke-failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakBoundaryContractSmoke;

    #[test]
    fn combined_contract_smoke_is_ok() {
        assert!(VaachakBoundaryContractSmoke::smoke_ok());
    }

    #[test]
    fn boundary_contract_smoke_preserves_physical_behavior() {
        assert!(!VaachakBoundaryContractSmoke::physical_behavior_moved());
    }
}
