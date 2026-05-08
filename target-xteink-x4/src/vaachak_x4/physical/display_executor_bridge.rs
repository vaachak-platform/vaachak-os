#![allow(dead_code)]

use super::display_runtime_owner::{VaachakDisplayRuntimeOperation, VaachakDisplayRuntimeOwner};
use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::spi_bus_runtime_owner::{VaachakSpiRuntimeUser, VaachakSpiTransactionKind};
use super::spi_executor_bridge::{VaachakSpiExecutorBridge, VaachakSpiExecutorIntent};

/// Vaachak-owned display executor bridge.
///
/// This bridge owns display executor intent routing for full refresh, partial
/// refresh, clear, sleep, and render metadata. The SSD1677 drawing and refresh
/// executor remains Pulp-compatible; no draw algorithm is rewritten here.
pub struct VaachakDisplayExecutorBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayExecutorIntent {
    FullRefresh,
    PartialRefresh,
    ClearFrame,
    SleepFrame,
    RenderFrameMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayExecutorDecision {
    RoutedToPulpCompatibilityExecutor,
    RejectedBeforeDisplayExecution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayExecutorRoute {
    pub intent: VaachakDisplayExecutorIntent,
    pub operation: VaachakDisplayRuntimeOperation,
    pub decision: VaachakDisplayExecutorDecision,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub spi_handoff_ready: bool,
    pub display_runtime_owner_ready: bool,
    pub draw_algorithm_rewritten: bool,
    pub full_refresh_rewritten: bool,
    pub partial_refresh_rewritten: bool,
}

impl VaachakDisplayExecutorBridge {
    pub const DISPLAY_EXECUTOR_BRIDGE_MARKER: &'static str = "x4-display-executor-bridge-ok";
    pub const DISPLAY_EXECUTOR_BRIDGE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";

    pub const DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const FULL_REFRESH_REWRITTEN: bool = false;
    pub const PARTIAL_REFRESH_REWRITTEN: bool = false;

    pub const fn operation_for(
        intent: VaachakDisplayExecutorIntent,
    ) -> VaachakDisplayRuntimeOperation {
        match intent {
            VaachakDisplayExecutorIntent::FullRefresh => {
                VaachakDisplayRuntimeOperation::FullRefreshMetadata
            }
            VaachakDisplayExecutorIntent::PartialRefresh => {
                VaachakDisplayRuntimeOperation::PartialRefreshMetadata
            }
            VaachakDisplayExecutorIntent::ClearFrame
            | VaachakDisplayExecutorIntent::SleepFrame
            | VaachakDisplayExecutorIntent::RenderFrameMetadata => {
                VaachakDisplayRuntimeOperation::SurfaceRenderMetadata
            }
        }
    }

    pub const fn route_intent(intent: VaachakDisplayExecutorIntent) -> VaachakDisplayExecutorRoute {
        let backend_route =
            VaachakHardwareExecutorPulpBackend::route_for(VaachakHardwareExecutorDomain::Display);
        let spi_route = VaachakSpiExecutorBridge::route_transaction_intent(
            VaachakSpiExecutorIntent::DisplayTransaction,
            VaachakSpiRuntimeUser::Display,
            VaachakSpiTransactionKind::DisplayRefreshMetadata,
        );
        let display_owner_ready = VaachakDisplayRuntimeOwner::ownership_ok();
        let spi_handoff_ready = VaachakSpiExecutorBridge::route_is_safe(spi_route);
        let decision = if display_owner_ready
            && spi_handoff_ready
            && VaachakHardwareExecutorPulpBackend::route_is_pulp_compatible(backend_route)
        {
            VaachakDisplayExecutorDecision::RoutedToPulpCompatibilityExecutor
        } else {
            VaachakDisplayExecutorDecision::RejectedBeforeDisplayExecution
        };

        VaachakDisplayExecutorRoute {
            intent,
            operation: Self::operation_for(intent),
            decision,
            backend: backend_route.backend,
            backend_name: backend_route.backend_name,
            active_executor_owner: backend_route.active_executor_owner,
            spi_handoff_ready,
            display_runtime_owner_ready: display_owner_ready,
            draw_algorithm_rewritten: Self::DRAW_ALGORITHM_REWRITTEN,
            full_refresh_rewritten: Self::FULL_REFRESH_REWRITTEN,
            partial_refresh_rewritten: Self::PARTIAL_REFRESH_REWRITTEN,
        }
    }

    pub const fn route_is_safe(route: VaachakDisplayExecutorRoute) -> bool {
        matches!(
            route.decision,
            VaachakDisplayExecutorDecision::RoutedToPulpCompatibilityExecutor
        ) && matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && route.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && route.spi_handoff_ready
            && route.display_runtime_owner_ready
            && !route.draw_algorithm_rewritten
            && !route.full_refresh_rewritten
            && !route.partial_refresh_rewritten
    }

    pub const fn display_routes_ready() -> bool {
        Self::route_is_safe(Self::route_intent(
            VaachakDisplayExecutorIntent::FullRefresh,
        )) && Self::route_is_safe(Self::route_intent(
            VaachakDisplayExecutorIntent::PartialRefresh,
        )) && Self::route_is_safe(Self::route_intent(VaachakDisplayExecutorIntent::ClearFrame))
            && Self::route_is_safe(Self::route_intent(VaachakDisplayExecutorIntent::SleepFrame))
            && Self::route_is_safe(Self::route_intent(
                VaachakDisplayExecutorIntent::RenderFrameMetadata,
            ))
    }

    pub const fn bridge_ok() -> bool {
        VaachakDisplayRuntimeOwner::ownership_ok()
            && VaachakSpiExecutorBridge::bridge_ok()
            && VaachakHardwareExecutorPulpBackend::display_route_ok()
            && Self::display_routes_ready()
    }
}
