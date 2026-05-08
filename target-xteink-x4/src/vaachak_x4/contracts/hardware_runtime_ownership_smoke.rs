#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_ownership::{
    VaachakHardwareRuntimeOwnerKind, VaachakHardwareRuntimeOwnership,
};

pub struct VaachakHardwareRuntimeOwnershipSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeOwnershipSmokeReport {
    pub marker_present: bool,
    pub spi_bus_owner_present: bool,
    pub storage_probe_mount_owner_present: bool,
    pub sd_fat_readonly_owner_present: bool,
    pub display_owner_present: bool,
    pub input_owner_present: bool,
    pub consolidation_ok: bool,
}

impl VaachakHardwareRuntimeOwnershipSmokeReport {
    pub const fn ok(self) -> bool {
        self.marker_present
            && self.spi_bus_owner_present
            && self.storage_probe_mount_owner_present
            && self.sd_fat_readonly_owner_present
            && self.display_owner_present
            && self.input_owner_present
            && self.consolidation_ok
    }
}

impl VaachakHardwareRuntimeOwnershipSmoke {
    pub const fn report() -> VaachakHardwareRuntimeOwnershipSmokeReport {
        let entries = VaachakHardwareRuntimeOwnership::entries();
        VaachakHardwareRuntimeOwnershipSmokeReport {
            marker_present:
                VaachakHardwareRuntimeOwnership::HARDWARE_RUNTIME_OWNERSHIP_CONSOLIDATION_MARKER
                    .len()
                    > 0,
            spi_bus_owner_present: matches!(
                entries[0].owner,
                VaachakHardwareRuntimeOwnerKind::SpiBus
            ),
            storage_probe_mount_owner_present: matches!(
                entries[1].owner,
                VaachakHardwareRuntimeOwnerKind::StorageProbeMount
            ),
            sd_fat_readonly_owner_present: matches!(
                entries[2].owner,
                VaachakHardwareRuntimeOwnerKind::SdFatReadonly
            ),
            display_owner_present: matches!(
                entries[3].owner,
                VaachakHardwareRuntimeOwnerKind::Display
            ),
            input_owner_present: matches!(entries[4].owner, VaachakHardwareRuntimeOwnerKind::Input),
            consolidation_ok: VaachakHardwareRuntimeOwnership::consolidation_ok(),
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeOwnershipSmoke;

    #[test]
    fn hardware_runtime_ownership_smoke_is_ok() {
        assert!(VaachakHardwareRuntimeOwnershipSmoke::ok());
    }
}
