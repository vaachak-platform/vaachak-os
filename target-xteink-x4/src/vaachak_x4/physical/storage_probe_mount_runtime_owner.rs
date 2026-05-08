#![allow(dead_code)]

use super::spi_bus_runtime_owner::{
    VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind,
};
use super::storage_probe_mount_pulp_backend::VaachakStorageProbeMountPulpBackend;

/// Vaachak-owned SD probe/mount runtime owner for Xteink X4.
///
/// This is the first SD lifecycle hardware-ownership move after the SPI
/// ownership bridge. Vaachak now owns the runtime authority boundary for SD
/// probe/mount lifecycle metadata and sequencing intent. The existing imported
/// Pulp runtime remains the active executor for card detection, SD
/// identification, FAT availability, and file I/O. This module does not perform
/// SD card initialization, mount a FAT volume, access files, move SPI
/// arbitration, or change display rendering.
pub struct VaachakStorageProbeMountRuntimeOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountRuntimeBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageRuntimeStep {
    RuntimeBoot,
    SharedSpiReady,
    CardDetectAuthority,
    SlowIdentificationAuthority,
    CardAvailabilityAuthority,
    FatVolumeAvailabilityObserved,
    ReadOnlyBoundaryObserved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageRuntimeAuthority {
    VaachakRuntimeOwner,
    PulpCompatibilityExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageRuntimeLifecycleEntry {
    pub step: VaachakStorageRuntimeStep,
    pub description: &'static str,
    pub authority_owner: VaachakStorageRuntimeAuthority,
    pub active_executor: VaachakStorageRuntimeBackend,
    pub active_executor_owner: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountOwnershipReport {
    pub ownership_authority_moved_to_vaachak: bool,
    pub active_backend_is_pulp_compatibility: bool,
    pub shared_spi_owner_available: bool,
    pub storage_user_registered_on_spi: bool,
    pub storage_chip_select_ok: bool,
    pub slow_identification_timing_ok: bool,
    pub lifecycle_authority_ok: bool,
    pub backend_bridge_ok: bool,
    pub sd_executor_moved_to_vaachak: bool,
    pub fat_behavior_moved_to_vaachak: bool,
    pub spi_arbitration_moved_to_vaachak: bool,
    pub display_behavior_moved_to_vaachak: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakStorageProbeMountOwnershipReport {
    pub const fn ownership_ok(self) -> bool {
        self.ownership_authority_moved_to_vaachak
            && self.active_backend_is_pulp_compatibility
            && self.shared_spi_owner_available
            && self.storage_user_registered_on_spi
            && self.storage_chip_select_ok
            && self.slow_identification_timing_ok
            && self.lifecycle_authority_ok
            && self.backend_bridge_ok
            && !self.sd_executor_moved_to_vaachak
            && !self.fat_behavior_moved_to_vaachak
            && !self.spi_arbitration_moved_to_vaachak
            && !self.display_behavior_moved_to_vaachak
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakStorageProbeMountRuntimeOwner {
    pub const STORAGE_PROBE_MOUNT_RUNTIME_OWNERSHIP_MARKER: &'static str =
        "x4-storage-probe-mount-runtime-owner-ok";

    pub const STORAGE_PROBE_MOUNT_IDENTITY: &'static str = "xteink-x4-sd-probe-mount-runtime";
    pub const STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;

    pub const PULP_COMPATIBILITY_BACKEND: VaachakStorageProbeMountRuntimeBackend =
        VaachakStorageProbeMountRuntimeBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND: VaachakStorageProbeMountRuntimeBackend =
        Self::PULP_COMPATIBILITY_BACKEND;
    pub const ACTIVE_BACKEND_NAME: &'static str = VaachakStorageProbeMountPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const STORAGE_SD_CS_GPIO: u8 = 12;
    pub const SD_IDENTIFICATION_KHZ: u32 = 400;
    pub const OPERATIONAL_SPI_MHZ: u32 = 20;

    pub const SD_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_WRITE_LIST_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const LIFECYCLE: [VaachakStorageRuntimeLifecycleEntry; 7] = [
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::RuntimeBoot,
            description: "firmware enters the existing Pulp-compatible runtime path",
            authority_owner: VaachakStorageRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::SharedSpiReady,
            description: "storage lifecycle depends on the accepted Vaachak SPI ownership bridge",
            authority_owner: VaachakStorageRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::CardDetectAuthority,
            description: "Vaachak owns card-detection lifecycle authority metadata",
            authority_owner: VaachakStorageRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::SlowIdentificationAuthority,
            description: "Vaachak owns slow-identification lifecycle authority metadata",
            authority_owner: VaachakStorageRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::CardAvailabilityAuthority,
            description: "Vaachak owns card-availability lifecycle authority metadata",
            authority_owner: VaachakStorageRuntimeAuthority::VaachakRuntimeOwner,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::FatVolumeAvailabilityObserved,
            description: "FAT availability remains executed by Pulp and only observed by Vaachak",
            authority_owner: VaachakStorageRuntimeAuthority::PulpCompatibilityExecutor,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
        VaachakStorageRuntimeLifecycleEntry {
            step: VaachakStorageRuntimeStep::ReadOnlyBoundaryObserved,
            description: "read-only adapter observes storage after Pulp exposes the working path",
            authority_owner: VaachakStorageRuntimeAuthority::PulpCompatibilityExecutor,
            active_executor: VaachakStorageProbeMountRuntimeBackend::PulpCompatibility,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        },
    ];

    pub const fn lifecycle_entry(
        step: VaachakStorageRuntimeStep,
    ) -> VaachakStorageRuntimeLifecycleEntry {
        match step {
            VaachakStorageRuntimeStep::RuntimeBoot => Self::LIFECYCLE[0],
            VaachakStorageRuntimeStep::SharedSpiReady => Self::LIFECYCLE[1],
            VaachakStorageRuntimeStep::CardDetectAuthority => Self::LIFECYCLE[2],
            VaachakStorageRuntimeStep::SlowIdentificationAuthority => Self::LIFECYCLE[3],
            VaachakStorageRuntimeStep::CardAvailabilityAuthority => Self::LIFECYCLE[4],
            VaachakStorageRuntimeStep::FatVolumeAvailabilityObserved => Self::LIFECYCLE[5],
            VaachakStorageRuntimeStep::ReadOnlyBoundaryObserved => Self::LIFECYCLE[6],
        }
    }

    pub const fn shared_spi_owner_available() -> bool {
        VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
    }

    pub const fn storage_user_registered_on_spi() -> bool {
        VaachakSpiBusRuntimeOwner::storage_user_registered()
    }

    pub const fn storage_chip_select_ok() -> bool {
        VaachakSpiBusRuntimeOwner::chip_select_gpio(VaachakSpiRuntimeUser::Storage)
            == Self::STORAGE_SD_CS_GPIO
    }

    pub const fn storage_spi_metadata_ok(step: VaachakStorageRuntimeStep) -> bool {
        let transaction_kind = match step {
            VaachakStorageRuntimeStep::RuntimeBoot
            | VaachakStorageRuntimeStep::SharedSpiReady
            | VaachakStorageRuntimeStep::CardDetectAuthority
            | VaachakStorageRuntimeStep::SlowIdentificationAuthority
            | VaachakStorageRuntimeStep::CardAvailabilityAuthority => {
                VaachakSpiTransactionKind::StorageProbeMetadata
            }
            VaachakStorageRuntimeStep::FatVolumeAvailabilityObserved
            | VaachakStorageRuntimeStep::ReadOnlyBoundaryObserved => {
                VaachakSpiTransactionKind::StorageMountMetadata
            }
        };
        let metadata = VaachakSpiBusRuntimeOwner::transaction_ownership(
            VaachakSpiRuntimeUser::Storage,
            transaction_kind,
        );
        VaachakSpiBusRuntimeOwner::transaction_metadata_is_safe(metadata)
    }

    pub const fn slow_identification_timing_ok() -> bool {
        Self::SD_IDENTIFICATION_KHZ == 400 && Self::OPERATIONAL_SPI_MHZ == 20
    }

    pub const fn lifecycle_authority_ok() -> bool {
        Self::LIFECYCLE.len() == 7
            && matches!(
                Self::lifecycle_entry(VaachakStorageRuntimeStep::CardDetectAuthority)
                    .authority_owner,
                VaachakStorageRuntimeAuthority::VaachakRuntimeOwner
            )
            && matches!(
                Self::lifecycle_entry(VaachakStorageRuntimeStep::SlowIdentificationAuthority)
                    .authority_owner,
                VaachakStorageRuntimeAuthority::VaachakRuntimeOwner
            )
            && matches!(
                Self::lifecycle_entry(VaachakStorageRuntimeStep::CardAvailabilityAuthority)
                    .authority_owner,
                VaachakStorageRuntimeAuthority::VaachakRuntimeOwner
            )
            && matches!(
                Self::lifecycle_entry(VaachakStorageRuntimeStep::FatVolumeAvailabilityObserved)
                    .authority_owner,
                VaachakStorageRuntimeAuthority::PulpCompatibilityExecutor
            )
            && matches!(
                Self::lifecycle_entry(VaachakStorageRuntimeStep::ReadOnlyBoundaryObserved)
                    .authority_owner,
                VaachakStorageRuntimeAuthority::PulpCompatibilityExecutor
            )
    }

    pub const fn report() -> VaachakStorageProbeMountOwnershipReport {
        VaachakStorageProbeMountOwnershipReport {
            ownership_authority_moved_to_vaachak:
                Self::STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            active_backend_is_pulp_compatibility: matches!(
                Self::ACTIVE_BACKEND,
                VaachakStorageProbeMountRuntimeBackend::PulpCompatibility
            ),
            shared_spi_owner_available: Self::shared_spi_owner_available(),
            storage_user_registered_on_spi: Self::storage_user_registered_on_spi(),
            storage_chip_select_ok: Self::storage_chip_select_ok(),
            slow_identification_timing_ok: Self::slow_identification_timing_ok(),
            lifecycle_authority_ok: Self::lifecycle_authority_ok(),
            backend_bridge_ok: VaachakStorageProbeMountPulpBackend::bridge_ok(),
            sd_executor_moved_to_vaachak: Self::SD_EXECUTOR_MOVED_TO_VAACHAK,
            fat_behavior_moved_to_vaachak: Self::FAT_BEHAVIOR_MOVED_TO_VAACHAK,
            spi_arbitration_moved_to_vaachak: Self::SPI_ARBITRATION_MOVED_TO_VAACHAK,
            display_behavior_moved_to_vaachak: Self::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn ownership_ok() -> bool {
        Self::report().ownership_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakStorageProbeMountRuntimeOwner, VaachakStorageRuntimeStep};

    #[test]
    fn storage_probe_mount_runtime_owner_is_active() {
        assert!(VaachakStorageProbeMountRuntimeOwner::ownership_ok());
    }

    #[test]
    fn storage_lifecycle_authority_moved_without_fat_move() {
        let report = VaachakStorageProbeMountRuntimeOwner::report();
        assert!(report.ownership_authority_moved_to_vaachak);
        assert!(report.active_backend_is_pulp_compatibility);
        assert!(!report.fat_behavior_moved_to_vaachak);
        assert!(!report.display_behavior_moved_to_vaachak);
    }

    #[test]
    fn storage_spi_metadata_is_safe() {
        assert!(
            VaachakStorageProbeMountRuntimeOwner::storage_spi_metadata_ok(
                VaachakStorageRuntimeStep::CardDetectAuthority,
            )
        );
        assert!(
            VaachakStorageProbeMountRuntimeOwner::storage_spi_metadata_ok(
                VaachakStorageRuntimeStep::FatVolumeAvailabilityObserved,
            )
        );
    }
}
