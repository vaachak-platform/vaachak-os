#![allow(dead_code)]

use super::spi_bus_arbitration_pulp_backend::VaachakSpiArbitrationPulpBackend;
use super::spi_bus_runtime_owner::{
    VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind,
};

/// Vaachak-owned SPI arbitration runtime owner for Xteink X4.
///
/// This is the first narrow SPI runtime behavior migration after the hardware
/// ownership consolidation. Vaachak now owns the safe logical arbitration policy
/// and transaction ownership metadata for the shared display/SD SPI bus. The
/// physical SPI transfer executor, chip-select toggling, SD probe/mount, FAT
/// behavior, and SSD1677 draw/refresh execution remain in the Pulp compatibility
/// backend.
pub struct VaachakSpiBusArbitrationRuntimeOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiArbitrationRuntimeBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiArbitrationPriority {
    DisplayRefresh,
    StorageProbeMount,
    StorageFatReadonly,
    Idle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiArbitrationDecision {
    GrantMetadataOnly,
    RejectInvalidChipSelect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiArbitrationRequest {
    pub user: VaachakSpiRuntimeUser,
    pub kind: VaachakSpiTransactionKind,
    pub priority: VaachakSpiArbitrationPriority,
    pub requested_chip_select_gpio: u8,
    pub shared_bus: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiArbitrationGrant {
    pub request: VaachakSpiArbitrationRequest,
    pub decision: VaachakSpiArbitrationDecision,
    pub ownership_authority: &'static str,
    pub active_backend: VaachakSpiArbitrationRuntimeBackend,
    pub active_backend_name: &'static str,
    pub active_physical_executor_owner: &'static str,
    pub requires_exclusive_chip_select: bool,
    pub physical_executor_remains_pulp: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiArbitrationRuntimeReport {
    pub spi_runtime_owner_ready: bool,
    pub arbitration_authority_moved_to_vaachak: bool,
    pub arbitration_policy_moved_to_vaachak: bool,
    pub display_user_supported: bool,
    pub storage_user_supported: bool,
    pub active_backend_is_pulp_compatibility: bool,
    pub backend_ok: bool,
    pub physical_spi_transfer_executor_moved_to_vaachak: bool,
    pub chip_select_executor_moved_to_vaachak: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakSpiArbitrationRuntimeReport {
    pub const fn runtime_owner_ok(self) -> bool {
        self.spi_runtime_owner_ready
            && self.arbitration_authority_moved_to_vaachak
            && self.arbitration_policy_moved_to_vaachak
            && self.display_user_supported
            && self.storage_user_supported
            && self.active_backend_is_pulp_compatibility
            && self.backend_ok
            && !self.physical_spi_transfer_executor_moved_to_vaachak
            && !self.chip_select_executor_moved_to_vaachak
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakSpiBusArbitrationRuntimeOwner {
    pub const SPI_BUS_ARBITRATION_RUNTIME_OWNER_MARKER: &'static str =
        "spi_bus_arbitration_runtime_owner=ok";
    pub const SPI_BUS_ARBITRATION_RUNTIME_IDENTITY: &'static str =
        "xteink-x4-shared-spi-arbitration-runtime";

    pub const SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = true;

    pub const ACTIVE_BACKEND: VaachakSpiArbitrationRuntimeBackend =
        VaachakSpiArbitrationRuntimeBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND_NAME: &'static str = VaachakSpiArbitrationPulpBackend::BACKEND_NAME;
    pub const ACTIVE_PHYSICAL_EXECUTOR_OWNER: &'static str =
        VaachakSpiArbitrationPulpBackend::ACTIVE_PHYSICAL_EXECUTOR_OWNER;

    pub const SPI_BUS_IDENTITY: &'static str = VaachakSpiBusRuntimeOwner::SPI_BUS_IDENTITY;
    pub const SPI_SCLK_GPIO: u8 = VaachakSpiBusRuntimeOwner::SPI_SCLK_GPIO;
    pub const SPI_MOSI_GPIO: u8 = VaachakSpiBusRuntimeOwner::SPI_MOSI_GPIO;
    pub const SPI_MISO_GPIO: u8 = VaachakSpiBusRuntimeOwner::SPI_MISO_GPIO;
    pub const DISPLAY_CS_GPIO: u8 = VaachakSpiBusRuntimeOwner::DISPLAY_CS_GPIO;
    pub const STORAGE_SD_CS_GPIO: u8 = VaachakSpiBusRuntimeOwner::STORAGE_SD_CS_GPIO;

    pub const PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const SD_PROBE_MOUNT_BEHAVIOR_CHANGED: bool = false;
    pub const SD_FAT_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn priority_for(
        user: VaachakSpiRuntimeUser,
        kind: VaachakSpiTransactionKind,
    ) -> VaachakSpiArbitrationPriority {
        match (user, kind) {
            (VaachakSpiRuntimeUser::Display, VaachakSpiTransactionKind::DisplayRefreshMetadata) => {
                VaachakSpiArbitrationPriority::DisplayRefresh
            }
            (VaachakSpiRuntimeUser::Storage, VaachakSpiTransactionKind::StorageProbeMetadata)
            | (VaachakSpiRuntimeUser::Storage, VaachakSpiTransactionKind::StorageMountMetadata) => {
                VaachakSpiArbitrationPriority::StorageProbeMount
            }
            (VaachakSpiRuntimeUser::Storage, VaachakSpiTransactionKind::StorageFatIoMetadata) => {
                VaachakSpiArbitrationPriority::StorageFatReadonly
            }
            _ => VaachakSpiArbitrationPriority::Idle,
        }
    }

    pub const fn request_for(
        user: VaachakSpiRuntimeUser,
        kind: VaachakSpiTransactionKind,
    ) -> VaachakSpiArbitrationRequest {
        VaachakSpiArbitrationRequest {
            user,
            kind,
            priority: Self::priority_for(user, kind),
            requested_chip_select_gpio: VaachakSpiBusRuntimeOwner::chip_select_gpio(user),
            shared_bus: true,
        }
    }

    pub const fn request_is_safe(request: VaachakSpiArbitrationRequest) -> bool {
        request.shared_bus
            && ((matches!(request.user, VaachakSpiRuntimeUser::Display)
                && request.requested_chip_select_gpio == Self::DISPLAY_CS_GPIO)
                || (matches!(request.user, VaachakSpiRuntimeUser::Storage)
                    && request.requested_chip_select_gpio == Self::STORAGE_SD_CS_GPIO))
    }

    pub const fn grant_for(request: VaachakSpiArbitrationRequest) -> VaachakSpiArbitrationGrant {
        let decision = if Self::request_is_safe(request) {
            VaachakSpiArbitrationDecision::GrantMetadataOnly
        } else {
            VaachakSpiArbitrationDecision::RejectInvalidChipSelect
        };

        VaachakSpiArbitrationGrant {
            request,
            decision,
            ownership_authority: Self::SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            active_physical_executor_owner: Self::ACTIVE_PHYSICAL_EXECUTOR_OWNER,
            requires_exclusive_chip_select: true,
            physical_executor_remains_pulp: true,
        }
    }

    pub const fn grant_is_safe(grant: VaachakSpiArbitrationGrant) -> bool {
        matches!(
            grant.decision,
            VaachakSpiArbitrationDecision::GrantMetadataOnly
        ) && Self::request_is_safe(grant.request)
            && grant.requires_exclusive_chip_select
            && grant.physical_executor_remains_pulp
            && matches!(
                grant.active_backend,
                VaachakSpiArbitrationRuntimeBackend::PulpCompatibility
            )
            && grant.active_backend_name.len() == Self::ACTIVE_BACKEND_NAME.len()
            && grant.ownership_authority.len()
                == Self::SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY.len()
    }

    pub const fn display_arbitration_grant_is_safe() -> bool {
        let request = Self::request_for(
            VaachakSpiRuntimeUser::Display,
            VaachakSpiTransactionKind::DisplayRefreshMetadata,
        );
        Self::grant_is_safe(Self::grant_for(request))
    }

    pub const fn storage_arbitration_grant_is_safe() -> bool {
        let request = Self::request_for(
            VaachakSpiRuntimeUser::Storage,
            VaachakSpiTransactionKind::StorageFatIoMetadata,
        );
        Self::grant_is_safe(Self::grant_for(request))
    }

    pub const fn report() -> VaachakSpiArbitrationRuntimeReport {
        VaachakSpiArbitrationRuntimeReport {
            spi_runtime_owner_ready: VaachakSpiBusRuntimeOwner::ownership_bridge_ok(),
            arbitration_authority_moved_to_vaachak:
                Self::SPI_BUS_ARBITRATION_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            arbitration_policy_moved_to_vaachak: Self::SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK,
            display_user_supported: Self::display_arbitration_grant_is_safe(),
            storage_user_supported: Self::storage_arbitration_grant_is_safe(),
            active_backend_is_pulp_compatibility: matches!(
                Self::ACTIVE_BACKEND,
                VaachakSpiArbitrationRuntimeBackend::PulpCompatibility
            ),
            backend_ok: VaachakSpiArbitrationPulpBackend::backend_ok(),
            physical_spi_transfer_executor_moved_to_vaachak:
                Self::PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK,
            chip_select_executor_moved_to_vaachak: Self::CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn runtime_owner_ok() -> bool {
        Self::report().runtime_owner_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSpiBusArbitrationRuntimeOwner;

    #[test]
    fn spi_bus_arbitration_runtime_owner_is_active() {
        assert!(VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok());
    }

    #[test]
    fn display_and_storage_arbitration_metadata_are_safe() {
        assert!(VaachakSpiBusArbitrationRuntimeOwner::display_arbitration_grant_is_safe());
        assert!(VaachakSpiBusArbitrationRuntimeOwner::storage_arbitration_grant_is_safe());
    }
}
