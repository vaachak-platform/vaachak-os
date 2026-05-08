#![allow(dead_code)]

use super::spi_bus_pulp_backend::VaachakSpiPulpBackend;

/// Vaachak-owned SPI runtime ownership entrypoint for Xteink X4.
///
/// This module moves SPI bus ownership authority into the Vaachak target layer
/// while keeping the existing imported Pulp runtime as the active hardware
/// executor. It owns identity, user registration, chip-select metadata, and safe
/// transaction ownership metadata. It does not implement SPI transfers,
/// chip-select toggling, SD probe/mount, FAT behavior, or display rendering.
pub struct VaachakSpiBusRuntimeOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiRuntimeBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiRuntimeUser {
    Display,
    Storage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiTransactionKind {
    SharedIdle,
    DisplayRefreshMetadata,
    StorageProbeMetadata,
    StorageMountMetadata,
    StorageFatIoMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiPinMap {
    pub sclk_gpio: u8,
    pub mosi_gpio: u8,
    pub miso_gpio: u8,
    pub display_cs_gpio: u8,
    pub storage_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiRegisteredUser {
    pub user: VaachakSpiRuntimeUser,
    pub device_name: &'static str,
    pub chip_select_gpio: u8,
    pub shared_bus: bool,
    pub active_behavior_owner: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiTransactionOwnership {
    pub user: VaachakSpiRuntimeUser,
    pub kind: VaachakSpiTransactionKind,
    pub chip_select_gpio: u8,
    pub backend: VaachakSpiRuntimeBackend,
    pub requires_exclusive_chip_select: bool,
    pub ownership_authority: &'static str,
    pub active_executor_owner: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiRuntimeOwnershipReport {
    pub ownership_authority_moved_to_vaachak: bool,
    pub active_backend_is_pulp_compatibility: bool,
    pub display_user_registered: bool,
    pub storage_user_registered: bool,
    pub pin_map_ok: bool,
    pub backend_bridge_ok: bool,
    pub arbitration_policy_moved_to_vaachak: bool,
    pub sd_probe_mount_moved_to_vaachak: bool,
    pub sd_fat_moved_to_vaachak: bool,
    pub display_rendering_moved_to_vaachak: bool,
}

impl VaachakSpiRuntimeOwnershipReport {
    pub const fn ownership_bridge_ok(self) -> bool {
        self.ownership_authority_moved_to_vaachak
            && self.active_backend_is_pulp_compatibility
            && self.display_user_registered
            && self.storage_user_registered
            && self.pin_map_ok
            && self.backend_bridge_ok
            && !self.arbitration_policy_moved_to_vaachak
            && !self.sd_probe_mount_moved_to_vaachak
            && !self.sd_fat_moved_to_vaachak
            && !self.display_rendering_moved_to_vaachak
    }
}

impl VaachakSpiBusRuntimeOwner {
    pub const SPI_BUS_RUNTIME_OWNERSHIP_MARKER: &'static str = "x4-spi-bus-runtime-ownership-ok";

    pub const SPI_BUS_IDENTITY: &'static str = "xteink-x4-shared-spi-bus";
    pub const SPI_BUS_OWNERSHIP_AUTHORITY: &'static str = "target-xteink-x4 Vaachak layer";
    pub const SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;

    pub const PULP_COMPATIBILITY_BACKEND: VaachakSpiRuntimeBackend =
        VaachakSpiRuntimeBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND: VaachakSpiRuntimeBackend = Self::PULP_COMPATIBILITY_BACKEND;
    pub const ACTIVE_BACKEND_NAME: &'static str = VaachakSpiPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const SPI_SCLK_GPIO: u8 = 8;
    pub const SPI_MOSI_GPIO: u8 = 10;
    pub const SPI_MISO_GPIO: u8 = 7;
    pub const DISPLAY_CS_GPIO: u8 = 21;
    pub const STORAGE_SD_CS_GPIO: u8 = 12;

    pub const DISPLAY_USER_NAME: &'static str = "SSD1677 display";
    pub const STORAGE_USER_NAME: &'static str = "microSD storage";

    pub const ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false;
    pub const SD_FAT_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn pin_map() -> VaachakSpiPinMap {
        VaachakSpiPinMap {
            sclk_gpio: Self::SPI_SCLK_GPIO,
            mosi_gpio: Self::SPI_MOSI_GPIO,
            miso_gpio: Self::SPI_MISO_GPIO,
            display_cs_gpio: Self::DISPLAY_CS_GPIO,
            storage_cs_gpio: Self::STORAGE_SD_CS_GPIO,
        }
    }

    pub const fn registered_user(user: VaachakSpiRuntimeUser) -> VaachakSpiRegisteredUser {
        match user {
            VaachakSpiRuntimeUser::Display => VaachakSpiRegisteredUser {
                user,
                device_name: Self::DISPLAY_USER_NAME,
                chip_select_gpio: Self::DISPLAY_CS_GPIO,
                shared_bus: true,
                active_behavior_owner: Self::ACTIVE_EXECUTOR_OWNER,
            },
            VaachakSpiRuntimeUser::Storage => VaachakSpiRegisteredUser {
                user,
                device_name: Self::STORAGE_USER_NAME,
                chip_select_gpio: Self::STORAGE_SD_CS_GPIO,
                shared_bus: true,
                active_behavior_owner: Self::ACTIVE_EXECUTOR_OWNER,
            },
        }
    }

    pub const fn display_user() -> VaachakSpiRegisteredUser {
        Self::registered_user(VaachakSpiRuntimeUser::Display)
    }

    pub const fn storage_user() -> VaachakSpiRegisteredUser {
        Self::registered_user(VaachakSpiRuntimeUser::Storage)
    }

    pub const fn chip_select_gpio(user: VaachakSpiRuntimeUser) -> u8 {
        match user {
            VaachakSpiRuntimeUser::Display => Self::DISPLAY_CS_GPIO,
            VaachakSpiRuntimeUser::Storage => Self::STORAGE_SD_CS_GPIO,
        }
    }

    pub const fn transaction_ownership(
        user: VaachakSpiRuntimeUser,
        kind: VaachakSpiTransactionKind,
    ) -> VaachakSpiTransactionOwnership {
        VaachakSpiTransactionOwnership {
            user,
            kind,
            chip_select_gpio: Self::chip_select_gpio(user),
            backend: Self::ACTIVE_BACKEND,
            requires_exclusive_chip_select: true,
            ownership_authority: Self::SPI_BUS_OWNERSHIP_AUTHORITY,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
        }
    }

    pub const fn transaction_metadata_is_safe(metadata: VaachakSpiTransactionOwnership) -> bool {
        metadata.requires_exclusive_chip_select
            && metadata.ownership_authority.len() == Self::SPI_BUS_OWNERSHIP_AUTHORITY.len()
            && metadata.active_executor_owner.len() == Self::ACTIVE_EXECUTOR_OWNER.len()
            && matches!(
                metadata.backend,
                VaachakSpiRuntimeBackend::PulpCompatibility
            )
            && ((matches!(metadata.user, VaachakSpiRuntimeUser::Display)
                && metadata.chip_select_gpio == Self::DISPLAY_CS_GPIO)
                || (matches!(metadata.user, VaachakSpiRuntimeUser::Storage)
                    && metadata.chip_select_gpio == Self::STORAGE_SD_CS_GPIO))
    }

    pub const fn display_user_registered() -> bool {
        let user = Self::display_user();
        matches!(user.user, VaachakSpiRuntimeUser::Display)
            && user.chip_select_gpio == Self::DISPLAY_CS_GPIO
            && user.shared_bus
    }

    pub const fn storage_user_registered() -> bool {
        let user = Self::storage_user();
        matches!(user.user, VaachakSpiRuntimeUser::Storage)
            && user.chip_select_gpio == Self::STORAGE_SD_CS_GPIO
            && user.shared_bus
    }

    pub const fn pin_map_ok() -> bool {
        let pins = Self::pin_map();
        pins.sclk_gpio == 8
            && pins.mosi_gpio == 10
            && pins.miso_gpio == 7
            && pins.display_cs_gpio == 21
            && pins.storage_cs_gpio == 12
    }

    pub const fn report() -> VaachakSpiRuntimeOwnershipReport {
        VaachakSpiRuntimeOwnershipReport {
            ownership_authority_moved_to_vaachak:
                Self::SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            active_backend_is_pulp_compatibility: matches!(
                Self::ACTIVE_BACKEND,
                VaachakSpiRuntimeBackend::PulpCompatibility
            ),
            display_user_registered: Self::display_user_registered(),
            storage_user_registered: Self::storage_user_registered(),
            pin_map_ok: Self::pin_map_ok(),
            backend_bridge_ok: VaachakSpiPulpBackend::bridge_ok(),
            arbitration_policy_moved_to_vaachak: Self::ARBITRATION_POLICY_MOVED_TO_VAACHAK,
            sd_probe_mount_moved_to_vaachak: Self::SD_PROBE_MOUNT_MOVED_TO_VAACHAK,
            sd_fat_moved_to_vaachak: Self::SD_FAT_MOVED_TO_VAACHAK,
            display_rendering_moved_to_vaachak: Self::DISPLAY_RENDERING_MOVED_TO_VAACHAK,
        }
    }

    pub const fn ownership_bridge_ok() -> bool {
        Self::report().ownership_bridge_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind};

    #[test]
    fn spi_runtime_ownership_entrypoint_is_active() {
        assert!(VaachakSpiBusRuntimeOwner::ownership_bridge_ok());
    }

    #[test]
    fn users_are_registered_on_expected_chip_selects() {
        assert_eq!(
            VaachakSpiBusRuntimeOwner::chip_select_gpio(VaachakSpiRuntimeUser::Display),
            21
        );
        assert_eq!(
            VaachakSpiBusRuntimeOwner::chip_select_gpio(VaachakSpiRuntimeUser::Storage),
            12
        );
    }

    #[test]
    fn transaction_metadata_stays_pulp_backed() {
        let display = VaachakSpiBusRuntimeOwner::transaction_ownership(
            VaachakSpiRuntimeUser::Display,
            VaachakSpiTransactionKind::DisplayRefreshMetadata,
        );
        let storage = VaachakSpiBusRuntimeOwner::transaction_ownership(
            VaachakSpiRuntimeUser::Storage,
            VaachakSpiTransactionKind::StorageFatIoMetadata,
        );

        assert!(VaachakSpiBusRuntimeOwner::transaction_metadata_is_safe(
            display
        ));
        assert!(VaachakSpiBusRuntimeOwner::transaction_metadata_is_safe(
            storage
        ));
    }
}
