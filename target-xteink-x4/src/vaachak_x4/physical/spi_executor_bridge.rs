#![allow(dead_code)]

use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::spi_bus_arbitration_runtime_owner::{
    VaachakSpiArbitrationDecision, VaachakSpiBusArbitrationRuntimeOwner,
};
use super::spi_bus_runtime_owner::{
    VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind,
};

/// Vaachak-owned SPI executor bridge.
///
/// This bridge moves the executor entrypoint for SPI transaction intent into the
/// Vaachak layer. It performs intent classification, ownership lookup, and safe
/// arbitration handoff metadata. It does not perform physical SPI transfers or
/// chip-select toggling; those remain in the active Pulp-compatible backend.
pub struct VaachakSpiExecutorBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiExecutorIntent {
    DisplayTransaction,
    StorageTransaction,
    SafeArbitrationHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiExecutorRoute {
    pub intent: VaachakSpiExecutorIntent,
    pub user: VaachakSpiRuntimeUser,
    pub transaction_kind: VaachakSpiTransactionKind,
    pub chip_select_gpio: u8,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub arbitration_granted: bool,
    pub physical_transfer_executor_moved_to_vaachak: bool,
    pub chip_select_executor_moved_to_vaachak: bool,
}

impl VaachakSpiExecutorBridge {
    pub const SPI_EXECUTOR_BRIDGE_MARKER: &'static str = "x4-spi-executor-bridge-ok";
    pub const SPI_EXECUTOR_BRIDGE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";

    pub const PHYSICAL_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;

    pub const fn route_transaction_intent(
        intent: VaachakSpiExecutorIntent,
        user: VaachakSpiRuntimeUser,
        transaction_kind: VaachakSpiTransactionKind,
    ) -> VaachakSpiExecutorRoute {
        let request = VaachakSpiBusArbitrationRuntimeOwner::request_for(user, transaction_kind);
        let grant = VaachakSpiBusArbitrationRuntimeOwner::grant_for(request);
        let backend_route =
            VaachakHardwareExecutorPulpBackend::route_for(VaachakHardwareExecutorDomain::SpiBus);

        VaachakSpiExecutorRoute {
            intent,
            user,
            transaction_kind,
            chip_select_gpio: VaachakSpiBusRuntimeOwner::chip_select_gpio(user),
            backend: backend_route.backend,
            backend_name: backend_route.backend_name,
            active_executor_owner: backend_route.active_executor_owner,
            arbitration_granted: matches!(
                grant.decision,
                VaachakSpiArbitrationDecision::GrantMetadataOnly
            ) && VaachakSpiBusArbitrationRuntimeOwner::grant_is_safe(grant),
            physical_transfer_executor_moved_to_vaachak:
                Self::PHYSICAL_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK,
            chip_select_executor_moved_to_vaachak: Self::CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK,
        }
    }

    pub const fn route_is_safe(route: VaachakSpiExecutorRoute) -> bool {
        matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && route.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && route.arbitration_granted
            && !route.physical_transfer_executor_moved_to_vaachak
            && !route.chip_select_executor_moved_to_vaachak
            && ((matches!(route.user, VaachakSpiRuntimeUser::Display)
                && route.chip_select_gpio == VaachakSpiBusRuntimeOwner::DISPLAY_CS_GPIO)
                || (matches!(route.user, VaachakSpiRuntimeUser::Storage)
                    && route.chip_select_gpio == VaachakSpiBusRuntimeOwner::STORAGE_SD_CS_GPIO))
    }

    pub const fn display_transaction_ready() -> bool {
        Self::route_is_safe(Self::route_transaction_intent(
            VaachakSpiExecutorIntent::DisplayTransaction,
            VaachakSpiRuntimeUser::Display,
            VaachakSpiTransactionKind::DisplayRefreshMetadata,
        ))
    }

    pub const fn storage_transaction_ready() -> bool {
        Self::route_is_safe(Self::route_transaction_intent(
            VaachakSpiExecutorIntent::StorageTransaction,
            VaachakSpiRuntimeUser::Storage,
            VaachakSpiTransactionKind::StorageFatIoMetadata,
        ))
    }

    pub const fn safe_arbitration_handoff_ready() -> bool {
        Self::route_is_safe(Self::route_transaction_intent(
            VaachakSpiExecutorIntent::SafeArbitrationHandoff,
            VaachakSpiRuntimeUser::Storage,
            VaachakSpiTransactionKind::StorageProbeMetadata,
        ))
    }

    pub const fn bridge_ok() -> bool {
        VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
            && VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok()
            && VaachakHardwareExecutorPulpBackend::spi_route_ok()
            && Self::display_transaction_ready()
            && Self::storage_transaction_ready()
            && Self::safe_arbitration_handoff_ready()
    }
}
