#![allow(dead_code)]

use crate::vaachak_x4::physical::spi_bus_runtime::VaachakSpiBusRuntimeBridge;

/// Vaachak-owned metadata contract for the Xteink X4 SD probe/mount lifecycle.
///
/// This module intentionally records ownership and sequencing facts only. It does
/// not initialize SD, select SPI chip-select lines, create a FAT volume manager,
/// open files, read sectors, write sectors, mount media, or probe hardware. The
/// active SD/FAT lifecycle remains owned by the imported Pulp runtime.
pub struct VaachakStorageProbeMountContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageLifecycleStep {
    RuntimeBoot,
    SharedSpiAvailable,
    SlowSdIdentification,
    CardAvailabilityKnown,
    FatVolumeAvailable,
    ReadOnlyFacadeAvailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageBehaviorOwner {
    VendorPulpRuntime,
    VaachakContractMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageLifecycleOwner {
    pub step: VaachakStorageLifecycleStep,
    pub description: &'static str,
    pub active_behavior_owner: VaachakStorageBehaviorOwner,
    pub vaachak_role: VaachakStorageBehaviorOwner,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountReport {
    pub lifecycle_documented: bool,
    pub shared_spi_dependency_documented: bool,
    pub active_sd_runtime_still_imported: bool,
    pub active_fat_runtime_still_imported: bool,
    pub active_spi_arbitration_still_imported: bool,
    pub active_display_runtime_still_imported: bool,
    pub no_runtime_behavior_moved: bool,
}

impl VaachakStorageProbeMountReport {
    pub const fn contract_ok(self) -> bool {
        self.lifecycle_documented
            && self.shared_spi_dependency_documented
            && self.active_sd_runtime_still_imported
            && self.active_fat_runtime_still_imported
            && self.active_spi_arbitration_still_imported
            && self.active_display_runtime_still_imported
            && self.no_runtime_behavior_moved
    }
}

impl VaachakStorageProbeMountContract {
    pub const STORAGE_PROBE_MOUNT_CONTRACT_MARKER: &'static str =
        "x4-storage-probe-mount-contract-ok";

    pub const CONTRACT_OWNER: &'static str = "Vaachak storage probe/mount metadata contract";
    pub const ACTIVE_SD_PROBE_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_SD_MOUNT_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_FAT_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_SPI_ARBITRATION_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_DISPLAY_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const SHARED_SPI_CONTRACT_DOC: &'static str =
        "docs/architecture/spi-bus-runtime-contract.md";

    /// These flags are intentionally false. They document that this slice is a
    /// contract/metadata checkpoint, not a hardware ownership move.
    pub const SD_DRIVER_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_MOUNT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_WRITE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;

    pub const STORAGE_SD_CS_GPIO: u8 = 12;
    pub const SD_IDENTIFICATION_KHZ: u32 = 400;
    pub const OPERATIONAL_SPI_MHZ: u32 = 20;

    pub const LIFECYCLE: [VaachakStorageLifecycleOwner; 6] = [
        VaachakStorageLifecycleOwner {
            step: VaachakStorageLifecycleStep::RuntimeBoot,
            description: "Pulp runtime starts the active firmware path",
            active_behavior_owner: VaachakStorageBehaviorOwner::VendorPulpRuntime,
            vaachak_role: VaachakStorageBehaviorOwner::VaachakContractMetadata,
        },
        VaachakStorageLifecycleOwner {
            step: VaachakStorageLifecycleStep::SharedSpiAvailable,
            description: "Shared SPI bus is governed by the existing Pulp arbitration path",
            active_behavior_owner: VaachakStorageBehaviorOwner::VendorPulpRuntime,
            vaachak_role: VaachakStorageBehaviorOwner::VaachakContractMetadata,
        },
        VaachakStorageLifecycleOwner {
            step: VaachakStorageLifecycleStep::SlowSdIdentification,
            description: "SD card identification uses the existing slow-clock Pulp behavior",
            active_behavior_owner: VaachakStorageBehaviorOwner::VendorPulpRuntime,
            vaachak_role: VaachakStorageBehaviorOwner::VaachakContractMetadata,
        },
        VaachakStorageLifecycleOwner {
            step: VaachakStorageLifecycleStep::CardAvailabilityKnown,
            description: "Card availability remains discovered by the imported runtime",
            active_behavior_owner: VaachakStorageBehaviorOwner::VendorPulpRuntime,
            vaachak_role: VaachakStorageBehaviorOwner::VaachakContractMetadata,
        },
        VaachakStorageLifecycleOwner {
            step: VaachakStorageLifecycleStep::FatVolumeAvailable,
            description: "FAT volume availability remains owned by the imported runtime",
            active_behavior_owner: VaachakStorageBehaviorOwner::VendorPulpRuntime,
            vaachak_role: VaachakStorageBehaviorOwner::VaachakContractMetadata,
        },
        VaachakStorageLifecycleOwner {
            step: VaachakStorageLifecycleStep::ReadOnlyFacadeAvailable,
            description: "Vaachak read-only storage facade may observe files after Pulp makes storage available",
            active_behavior_owner: VaachakStorageBehaviorOwner::VendorPulpRuntime,
            vaachak_role: VaachakStorageBehaviorOwner::VaachakContractMetadata,
        },
    ];

    pub const fn active_runtime_owner_for_step(
        step: VaachakStorageLifecycleStep,
    ) -> VaachakStorageBehaviorOwner {
        match step {
            VaachakStorageLifecycleStep::RuntimeBoot
            | VaachakStorageLifecycleStep::SharedSpiAvailable
            | VaachakStorageLifecycleStep::SlowSdIdentification
            | VaachakStorageLifecycleStep::CardAvailabilityKnown
            | VaachakStorageLifecycleStep::FatVolumeAvailable
            | VaachakStorageLifecycleStep::ReadOnlyFacadeAvailable => {
                VaachakStorageBehaviorOwner::VendorPulpRuntime
            }
        }
    }

    pub fn contract_report() -> VaachakStorageProbeMountReport {
        VaachakStorageProbeMountReport {
            lifecycle_documented: Self::lifecycle_is_documented(),
            shared_spi_dependency_documented: Self::shared_spi_dependency_is_documented(),
            active_sd_runtime_still_imported: Self::active_sd_runtime_is_still_imported(),
            active_fat_runtime_still_imported: Self::active_fat_runtime_is_still_imported(),
            active_spi_arbitration_still_imported: Self::active_spi_arbitration_is_still_imported(),
            active_display_runtime_still_imported: Self::active_display_runtime_is_still_imported(),
            no_runtime_behavior_moved: Self::no_runtime_behavior_moved(),
        }
    }

    pub fn contract_ok() -> bool {
        Self::contract_report().contract_ok()
    }

    pub fn lifecycle_is_documented() -> bool {
        Self::LIFECYCLE.len() == 6
            && Self::active_runtime_owner_for_step(VaachakStorageLifecycleStep::RuntimeBoot)
                == VaachakStorageBehaviorOwner::VendorPulpRuntime
            && Self::active_runtime_owner_for_step(VaachakStorageLifecycleStep::FatVolumeAvailable)
                == VaachakStorageBehaviorOwner::VendorPulpRuntime
            && Self::LIFECYCLE.iter().all(|entry| {
                entry.active_behavior_owner == VaachakStorageBehaviorOwner::VendorPulpRuntime
                    && entry.vaachak_role == VaachakStorageBehaviorOwner::VaachakContractMetadata
            })
    }

    pub fn shared_spi_dependency_is_documented() -> bool {
        VaachakSpiBusRuntimeBridge::PINS.sd_cs_gpio == Self::STORAGE_SD_CS_GPIO
            && VaachakSpiBusRuntimeBridge::TIMING.sd_probe_khz == Self::SD_IDENTIFICATION_KHZ
            && VaachakSpiBusRuntimeBridge::TIMING.operational_mhz == Self::OPERATIONAL_SPI_MHZ
            && Self::SHARED_SPI_CONTRACT_DOC == "docs/architecture/spi-bus-runtime-contract.md"
    }

    pub const fn active_sd_runtime_is_still_imported() -> bool {
        !Self::SD_DRIVER_MOVED_TO_VAACHAK
            && !Self::SD_PROBE_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::SD_MOUNT_BEHAVIOR_MOVED_TO_VAACHAK
    }

    pub const fn active_fat_runtime_is_still_imported() -> bool {
        !Self::FAT_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::FAT_READ_BEHAVIOR_MOVED_TO_VAACHAK
            && !Self::FAT_WRITE_BEHAVIOR_MOVED_TO_VAACHAK
    }

    pub const fn active_spi_arbitration_is_still_imported() -> bool {
        !Self::SPI_ARBITRATION_MOVED_TO_VAACHAK
    }

    pub const fn active_display_runtime_is_still_imported() -> bool {
        !Self::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK
    }

    pub const fn no_runtime_behavior_moved() -> bool {
        Self::active_sd_runtime_is_still_imported()
            && Self::active_fat_runtime_is_still_imported()
            && Self::active_spi_arbitration_is_still_imported()
            && Self::active_display_runtime_is_still_imported()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageProbeMountContract;

    #[test]
    fn storage_probe_mount_contract_is_metadata_only() {
        assert!(VaachakStorageProbeMountContract::contract_ok());
    }

    #[test]
    fn storage_probe_mount_contract_keeps_runtime_imported() {
        let report = VaachakStorageProbeMountContract::contract_report();
        assert!(report.active_sd_runtime_still_imported);
        assert!(report.active_fat_runtime_still_imported);
        assert!(report.active_spi_arbitration_still_imported);
        assert!(report.active_display_runtime_still_imported);
        assert!(report.no_runtime_behavior_moved);
    }
}
