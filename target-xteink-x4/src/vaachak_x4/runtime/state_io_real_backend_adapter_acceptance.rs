//! Phase 36X — State I/O real backend adapter acceptance.
//!
//! This module records acceptance metadata for the Phase 36W adapter contract.
//! It is intentionally side-effect free: no device, filesystem, SPI, display,
//! input, power, or boot-flow operation is performed here.

#![allow(clippy::struct_excessive_bools)]

/// Phase marker emitted by the overlay installer/check scripts.
pub const PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE_MARKER: &str =
    "phase36x=x4-state-io-real-backend-adapter-acceptance-ok";

/// Next intended implementation lane after this acceptance overlay.
pub const PHASE_36X_NEXT_LANE: &str = "typed-state-real-backend-read-only-probe-implementation";

/// Individual acceptance facts for the real-backend adapter contract lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoRealBackendAdapterAcceptanceItem {
    ContractDeclared,
    OperationPermissionsDeclared,
    ReadOnlyProbeDeclared,
    ShadowWriteDeclared,
    AtomicCommitGateDeclared,
    SideEffectFree,
    RealBackendCallsDisabled,
    HardwareBehaviorUnmoved,
}

impl StateIoRealBackendAdapterAcceptanceItem {
    /// Stable text form for boot/runtime reporting.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContractDeclared => "contract-declared",
            Self::OperationPermissionsDeclared => "operation-permissions-declared",
            Self::ReadOnlyProbeDeclared => "read-only-probe-declared",
            Self::ShadowWriteDeclared => "shadow-write-declared",
            Self::AtomicCommitGateDeclared => "atomic-commit-gate-declared",
            Self::SideEffectFree => "side-effect-free",
            Self::RealBackendCallsDisabled => "real-backend-calls-disabled",
            Self::HardwareBehaviorUnmoved => "hardware-behavior-unmoved",
        }
    }
}

/// Static acceptance report for the Phase 36W adapter-contract lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoRealBackendAdapterAcceptanceReport {
    pub phase_marker: &'static str,
    pub accepted: bool,
    pub side_effect_free: bool,
    pub real_backend_calls_enabled: bool,
    pub hardware_behavior_moved: bool,
    pub next_lane: &'static str,
    pub acceptance_items: &'static [StateIoRealBackendAdapterAcceptanceItem],
}

const PHASE_36X_ACCEPTANCE_ITEMS: &[StateIoRealBackendAdapterAcceptanceItem] = &[
    StateIoRealBackendAdapterAcceptanceItem::ContractDeclared,
    StateIoRealBackendAdapterAcceptanceItem::OperationPermissionsDeclared,
    StateIoRealBackendAdapterAcceptanceItem::ReadOnlyProbeDeclared,
    StateIoRealBackendAdapterAcceptanceItem::ShadowWriteDeclared,
    StateIoRealBackendAdapterAcceptanceItem::AtomicCommitGateDeclared,
    StateIoRealBackendAdapterAcceptanceItem::SideEffectFree,
    StateIoRealBackendAdapterAcceptanceItem::RealBackendCallsDisabled,
    StateIoRealBackendAdapterAcceptanceItem::HardwareBehaviorUnmoved,
];

/// Phase 36X acceptance report.
pub const PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE:
    StateIoRealBackendAdapterAcceptanceReport = StateIoRealBackendAdapterAcceptanceReport {
    phase_marker: PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE_MARKER,
    accepted: true,
    side_effect_free: true,
    real_backend_calls_enabled: false,
    hardware_behavior_moved: false,
    next_lane: PHASE_36X_NEXT_LANE,
    acceptance_items: PHASE_36X_ACCEPTANCE_ITEMS,
};

/// Returns the Phase 36X acceptance report.
pub const fn phase36x_state_io_real_backend_adapter_acceptance()
-> &'static StateIoRealBackendAdapterAcceptanceReport {
    &PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE
}

/// Reports whether the Phase 36W adapter contract has been accepted without side effects.
pub const fn phase36x_is_accepted() -> bool {
    PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE.accepted
        && PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE.side_effect_free
        && !PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE.real_backend_calls_enabled
        && !PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE.hardware_behavior_moved
}

/// Returns the next intended implementation lane.
pub const fn phase36x_next_lane() -> &'static str {
    PHASE_36X_STATE_IO_REAL_BACKEND_ADAPTER_ACCEPTANCE.next_lane
}

/// Returns whether the report includes a specific acceptance item.
pub fn phase36x_has_acceptance_item(item: StateIoRealBackendAdapterAcceptanceItem) -> bool {
    PHASE_36X_ACCEPTANCE_ITEMS.contains(&item)
}
