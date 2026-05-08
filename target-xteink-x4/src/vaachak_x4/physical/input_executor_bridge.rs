#![allow(dead_code)]

use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::input_runtime_owner::{VaachakInputRuntimeOperation, VaachakInputRuntimeOwner};

/// Vaachak-owned input executor bridge.
///
/// This bridge owns the input executor intent entrypoint for button scan, ADC
/// ladder ownership metadata, debounce/repeat handoff, and navigation handoff.
/// The active button/ADC/input executor remains Pulp-compatible.
pub struct VaachakInputExecutorBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputExecutorIntent {
    ButtonScan,
    AdcLadderSample,
    DebounceRepeatHandoff,
    NavigationHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputExecutorDecision {
    RoutedToPulpCompatibilityExecutor,
    RejectedBeforeInputExecution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputExecutorRoute {
    pub intent: VaachakInputExecutorIntent,
    pub operation: VaachakInputRuntimeOperation,
    pub decision: VaachakInputExecutorDecision,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub input_runtime_owner_ready: bool,
    pub adc_sampling_rewritten: bool,
    pub button_scan_rewritten: bool,
    pub debounce_navigation_rewritten: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakInputExecutorBridge {
    pub const INPUT_EXECUTOR_BRIDGE_MARKER: &'static str = "x4-input-executor-bridge-ok";
    pub const INPUT_EXECUTOR_BRIDGE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";

    pub const ADC_SAMPLING_REWRITTEN: bool = false;
    pub const BUTTON_SCAN_REWRITTEN: bool = false;
    pub const DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub const fn operation_for(intent: VaachakInputExecutorIntent) -> VaachakInputRuntimeOperation {
        match intent {
            VaachakInputExecutorIntent::ButtonScan => {
                VaachakInputRuntimeOperation::ButtonGpioMetadata
            }
            VaachakInputExecutorIntent::AdcLadderSample => {
                VaachakInputRuntimeOperation::AdcLadderMetadata
            }
            VaachakInputExecutorIntent::DebounceRepeatHandoff => {
                VaachakInputRuntimeOperation::TimingPolicyMetadata
            }
            VaachakInputExecutorIntent::NavigationHandoff => {
                VaachakInputRuntimeOperation::ShellInputBoundaryMetadata
            }
        }
    }

    pub const fn route_intent(intent: VaachakInputExecutorIntent) -> VaachakInputExecutorRoute {
        let backend_route =
            VaachakHardwareExecutorPulpBackend::route_for(VaachakHardwareExecutorDomain::Input);
        let owner_ready = VaachakInputRuntimeOwner::ownership_ok();
        let decision = if owner_ready
            && VaachakHardwareExecutorPulpBackend::route_is_pulp_compatible(backend_route)
        {
            VaachakInputExecutorDecision::RoutedToPulpCompatibilityExecutor
        } else {
            VaachakInputExecutorDecision::RejectedBeforeInputExecution
        };

        VaachakInputExecutorRoute {
            intent,
            operation: Self::operation_for(intent),
            decision,
            backend: backend_route.backend,
            backend_name: backend_route.backend_name,
            active_executor_owner: backend_route.active_executor_owner,
            input_runtime_owner_ready: owner_ready,
            adc_sampling_rewritten: Self::ADC_SAMPLING_REWRITTEN,
            button_scan_rewritten: Self::BUTTON_SCAN_REWRITTEN,
            debounce_navigation_rewritten: Self::DEBOUNCE_NAVIGATION_REWRITTEN,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub const fn route_is_safe(route: VaachakInputExecutorRoute) -> bool {
        matches!(
            route.decision,
            VaachakInputExecutorDecision::RoutedToPulpCompatibilityExecutor
        ) && matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && route.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && route.input_runtime_owner_ready
            && !route.adc_sampling_rewritten
            && !route.button_scan_rewritten
            && !route.debounce_navigation_rewritten
            && !route.app_navigation_behavior_changed
    }

    pub const fn input_routes_ready() -> bool {
        Self::route_is_safe(Self::route_intent(VaachakInputExecutorIntent::ButtonScan))
            && Self::route_is_safe(Self::route_intent(
                VaachakInputExecutorIntent::AdcLadderSample,
            ))
            && Self::route_is_safe(Self::route_intent(
                VaachakInputExecutorIntent::DebounceRepeatHandoff,
            ))
            && Self::route_is_safe(Self::route_intent(
                VaachakInputExecutorIntent::NavigationHandoff,
            ))
    }

    pub const fn bridge_ok() -> bool {
        VaachakInputRuntimeOwner::ownership_ok()
            && VaachakHardwareExecutorPulpBackend::input_route_ok()
            && Self::input_routes_ready()
    }
}
