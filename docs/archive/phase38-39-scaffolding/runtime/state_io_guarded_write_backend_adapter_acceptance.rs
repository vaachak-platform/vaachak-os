//! Phase 38P — State I/O Guarded Write Backend Adapter Acceptance.
//!
//! This module accepts the Phase 38O guarded backend adapter shape as the
//! current write-lane boundary. It is an acceptance/report layer only:
//! live mutation remains disabled and no persistent backend is bound here.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_write_backend_adapter_shape::{
    PHASE_38O_BACKEND_BOUND, PHASE_38O_LIVE_MUTATION_ENABLED, Phase38oAdapterDecision,
    Phase38oAdapterOperation, Phase38oAdapterPlan, phase38o_live_mutation_still_disabled,
};

pub const PHASE_38P_GUARDED_WRITE_BACKEND_ADAPTER_ACCEPTANCE_MARKER: &str =
    "phase38p=x4-state-io-guarded-write-backend-adapter-acceptance-ok";

pub const PHASE_38P_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38P_BACKEND_BOUND: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38pAcceptanceItem {
    AdapterShapePresent,
    CapabilityGatePresent,
    DefaultAdapterUnbound,
    DryRunPathAccepted,
    FutureDispatchStillGated,
    LiveMutationStillDisabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38pAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38pAcceptanceReason {
    AdapterDryRunAccepted,
    AdapterFutureDispatchGated,
    AdapterUnavailable,
    AdapterRejectedByGuard,
    AdapterCapabilityMissing,
    LiveMutationUnexpectedlyEnabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38pNextLane {
    GuardedPersistentBackendStub,
    LiveMutationStillBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38pAcceptanceReport {
    pub status: Phase38pAcceptanceStatus,
    pub reason: Phase38pAcceptanceReason,
    pub operation: Phase38oAdapterOperation,
    pub next_lane: Phase38pNextLane,
    pub backend_bound: bool,
    pub live_mutation_enabled: bool,
}

impl Phase38pAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase38pAcceptanceStatus::Accepted)
    }

    pub const fn permits_live_mutation(self) -> bool {
        self.live_mutation_enabled
    }

    pub const fn persistent_backend_bound(self) -> bool {
        self.backend_bound
    }
}

pub const PHASE_38P_ACCEPTANCE_ITEMS: &[Phase38pAcceptanceItem] = &[
    Phase38pAcceptanceItem::AdapterShapePresent,
    Phase38pAcceptanceItem::CapabilityGatePresent,
    Phase38pAcceptanceItem::DefaultAdapterUnbound,
    Phase38pAcceptanceItem::DryRunPathAccepted,
    Phase38pAcceptanceItem::FutureDispatchStillGated,
    Phase38pAcceptanceItem::LiveMutationStillDisabled,
];

pub fn phase38p_has_acceptance_item(item: Phase38pAcceptanceItem) -> bool {
    PHASE_38P_ACCEPTANCE_ITEMS.contains(&item)
}

pub fn phase38p_accept_adapter_plan(plan: Phase38oAdapterPlan) -> Phase38pAcceptanceReport {
    let (status, reason) = phase38p_status_and_reason(plan.decision);

    Phase38pAcceptanceReport {
        status,
        reason,
        operation: plan.operation,
        next_lane: phase38p_next_lane_for_status(status),
        backend_bound: PHASE_38P_BACKEND_BOUND,
        live_mutation_enabled: PHASE_38P_LIVE_MUTATION_ENABLED,
    }
}

pub const fn phase38p_status_and_reason(
    decision: Phase38oAdapterDecision,
) -> (Phase38pAcceptanceStatus, Phase38pAcceptanceReason) {
    match decision {
        Phase38oAdapterDecision::AcceptedForDryRun => (
            Phase38pAcceptanceStatus::Accepted,
            Phase38pAcceptanceReason::AdapterDryRunAccepted,
        ),
        Phase38oAdapterDecision::AcceptedForFutureDispatch => (
            Phase38pAcceptanceStatus::Deferred,
            Phase38pAcceptanceReason::AdapterFutureDispatchGated,
        ),
        Phase38oAdapterDecision::AdapterUnavailable => (
            Phase38pAcceptanceStatus::Deferred,
            Phase38pAcceptanceReason::AdapterUnavailable,
        ),
        Phase38oAdapterDecision::RejectedByGuard => (
            Phase38pAcceptanceStatus::Rejected,
            Phase38pAcceptanceReason::AdapterRejectedByGuard,
        ),
        Phase38oAdapterDecision::RejectedCapabilityMissing => (
            Phase38pAcceptanceStatus::Rejected,
            Phase38pAcceptanceReason::AdapterCapabilityMissing,
        ),
    }
}

pub const fn phase38p_next_lane_for_status(status: Phase38pAcceptanceStatus) -> Phase38pNextLane {
    match status {
        Phase38pAcceptanceStatus::Accepted | Phase38pAcceptanceStatus::Deferred => {
            Phase38pNextLane::GuardedPersistentBackendStub
        }
        Phase38pAcceptanceStatus::Rejected => Phase38pNextLane::LiveMutationStillBlocked,
    }
}

pub fn phase38p_live_mutation_still_disabled() -> bool {
    !PHASE_38P_LIVE_MUTATION_ENABLED
        && !PHASE_38P_BACKEND_BOUND
        && !PHASE_38O_LIVE_MUTATION_ENABLED
        && !PHASE_38O_BACKEND_BOUND
        && phase38o_live_mutation_still_disabled()
}

pub fn phase38p_acceptance_summary() -> Phase38pAcceptanceReport {
    Phase38pAcceptanceReport {
        status: Phase38pAcceptanceStatus::Deferred,
        reason: Phase38pAcceptanceReason::AdapterFutureDispatchGated,
        operation: Phase38oAdapterOperation::PrepareOnly,
        next_lane: Phase38pNextLane::GuardedPersistentBackendStub,
        backend_bound: PHASE_38P_BACKEND_BOUND,
        live_mutation_enabled: PHASE_38P_LIVE_MUTATION_ENABLED,
    }
}

pub fn phase38p_marker() -> &'static str {
    PHASE_38P_GUARDED_WRITE_BACKEND_ADAPTER_ACCEPTANCE_MARKER
}
