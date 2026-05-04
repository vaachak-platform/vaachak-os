//! Phase 38Q — State I/O Guarded Persistent Backend Stub.
//!
//! This module introduces the first persistent-backend-shaped stub for the
//! typed-state write lane. The stub is intentionally unavailable for live
//! mutation: it can validate and plan requests through the guarded adapter,
//! but it does not persist or mutate records.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_write_backend_adapter_acceptance::{
    phase38p_live_mutation_still_disabled, Phase38pNextLane, PHASE_38P_BACKEND_BOUND,
    PHASE_38P_LIVE_MUTATION_ENABLED,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_adapter_shape::{
    phase38o_plan_adapter_operation, Phase38oAdapterAvailability, Phase38oAdapterCapabilities,
    Phase38oAdapterDecision, Phase38oAdapterOperation, Phase38oAdapterPlan, Phase38oAdapterRequest,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_implementation_seam::{
    Phase38lBookId, Phase38lMutationIntent, Phase38lStateRecordKind,
};

pub const PHASE_38Q_GUARDED_PERSISTENT_BACKEND_STUB_MARKER: &str =
    "phase38q=x4-state-io-guarded-persistent-backend-stub-ok";

pub const PHASE_38Q_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38Q_PERSISTENT_BACKEND_BOUND: bool = false;
pub const PHASE_38Q_STUB_PAYLOAD_LIMIT: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38qBackendStubCapability {
    ValidateRequest,
    PlanOperation,
    PrepareFutureMutation,
    CommitFutureMutation,
    RollbackFutureMutation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38qBackendStubStatus {
    AcceptedForPlan,
    AcceptedForDryRun,
    DeferredUntilBackendBound,
    RejectedByGuard,
    RejectedPayloadTooLarge,
    RejectedBackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38qBackendStubReason {
    PlanOnly,
    DryRunOnly,
    BackendNotBound,
    GuardRejected,
    PayloadTooLarge,
    LiveMutationDisabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38qBackendStubNextLane {
    PersistentBackendReadBeforeWriteStub,
    LiveMutationStillBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38qBackendStubRequest {
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub book_id: Option<Phase38lBookId>,
    pub payload_len: usize,
    pub operation: Phase38oAdapterOperation,
}

impl Phase38qBackendStubRequest {
    pub const fn new(
        kind: Phase38lStateRecordKind,
        intent: Phase38lMutationIntent,
        book_id: Option<Phase38lBookId>,
        payload_len: usize,
        operation: Phase38oAdapterOperation,
    ) -> Self {
        Self {
            kind,
            intent,
            book_id,
            payload_len,
            operation,
        }
    }

    pub const fn as_adapter_request(self) -> Phase38oAdapterRequest {
        Phase38oAdapterRequest::new(
            self.kind,
            self.intent,
            self.book_id,
            self.payload_len,
            self.operation,
            Phase38oAdapterAvailability::DryRunOnly,
            Phase38oAdapterCapabilities::dry_run(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38qBackendStubReport {
    pub status: Phase38qBackendStubStatus,
    pub reason: Phase38qBackendStubReason,
    pub operation: Phase38oAdapterOperation,
    pub adapter_decision: Phase38oAdapterDecision,
    pub next_lane: Phase38qBackendStubNextLane,
    pub backend_bound: bool,
    pub live_mutation_enabled: bool,
}

impl Phase38qBackendStubReport {
    pub const fn accepted(self) -> bool {
        matches!(
            self.status,
            Phase38qBackendStubStatus::AcceptedForPlan
                | Phase38qBackendStubStatus::AcceptedForDryRun
        )
    }

    pub const fn permits_live_mutation(self) -> bool {
        self.live_mutation_enabled
    }

    pub const fn persistent_backend_bound(self) -> bool {
        self.backend_bound
    }
}

pub trait Phase38qGuardedPersistentBackendStub {
    fn plan_stub_operation(&self, request: Phase38qBackendStubRequest) -> Phase38qBackendStubReport;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase38qDefaultPersistentBackendStub;

impl Phase38qGuardedPersistentBackendStub for Phase38qDefaultPersistentBackendStub {
    fn plan_stub_operation(&self, request: Phase38qBackendStubRequest) -> Phase38qBackendStubReport {
        phase38q_plan_persistent_backend_stub(request)
    }
}

pub fn phase38q_plan_persistent_backend_stub(
    request: Phase38qBackendStubRequest,
) -> Phase38qBackendStubReport {
    if request.payload_len > PHASE_38Q_STUB_PAYLOAD_LIMIT {
        return Phase38qBackendStubReport {
            status: Phase38qBackendStubStatus::RejectedPayloadTooLarge,
            reason: Phase38qBackendStubReason::PayloadTooLarge,
            operation: request.operation,
            adapter_decision: Phase38oAdapterDecision::RejectedCapabilityMissing,
            next_lane: Phase38qBackendStubNextLane::LiveMutationStillBlocked,
            backend_bound: PHASE_38Q_PERSISTENT_BACKEND_BOUND,
            live_mutation_enabled: PHASE_38Q_LIVE_MUTATION_ENABLED,
        };
    }

    let adapter_plan = phase38o_plan_adapter_operation(request.as_adapter_request());
    phase38q_report_from_adapter_plan(request, adapter_plan)
}

pub const fn phase38q_report_from_adapter_plan(
    request: Phase38qBackendStubRequest,
    adapter_plan: Phase38oAdapterPlan,
) -> Phase38qBackendStubReport {
    let (status, reason, next_lane) =
        phase38q_status_reason_next_lane(adapter_plan.decision, request.operation);

    Phase38qBackendStubReport {
        status,
        reason,
        operation: request.operation,
        adapter_decision: adapter_plan.decision,
        next_lane,
        backend_bound: PHASE_38Q_PERSISTENT_BACKEND_BOUND,
        live_mutation_enabled: PHASE_38Q_LIVE_MUTATION_ENABLED,
    }
}

pub const fn phase38q_status_reason_next_lane(
    decision: Phase38oAdapterDecision,
    operation: Phase38oAdapterOperation,
) -> (
    Phase38qBackendStubStatus,
    Phase38qBackendStubReason,
    Phase38qBackendStubNextLane,
) {
    match decision {
        Phase38oAdapterDecision::AcceptedForDryRun => {
            if matches!(
                operation,
                Phase38oAdapterOperation::PlanOnly | Phase38oAdapterOperation::ValidateOnly
            ) {
                (
                    Phase38qBackendStubStatus::AcceptedForPlan,
                    Phase38qBackendStubReason::PlanOnly,
                    Phase38qBackendStubNextLane::PersistentBackendReadBeforeWriteStub,
                )
            } else {
                (
                    Phase38qBackendStubStatus::AcceptedForDryRun,
                    Phase38qBackendStubReason::DryRunOnly,
                    Phase38qBackendStubNextLane::PersistentBackendReadBeforeWriteStub,
                )
            }
        }
        Phase38oAdapterDecision::AcceptedForFutureDispatch => (
            Phase38qBackendStubStatus::DeferredUntilBackendBound,
            Phase38qBackendStubReason::BackendNotBound,
            Phase38qBackendStubNextLane::PersistentBackendReadBeforeWriteStub,
        ),
        Phase38oAdapterDecision::AdapterUnavailable => (
            Phase38qBackendStubStatus::RejectedBackendUnavailable,
            Phase38qBackendStubReason::BackendNotBound,
            Phase38qBackendStubNextLane::LiveMutationStillBlocked,
        ),
        Phase38oAdapterDecision::RejectedByGuard => (
            Phase38qBackendStubStatus::RejectedByGuard,
            Phase38qBackendStubReason::GuardRejected,
            Phase38qBackendStubNextLane::LiveMutationStillBlocked,
        ),
        Phase38oAdapterDecision::RejectedCapabilityMissing => (
            Phase38qBackendStubStatus::RejectedBackendUnavailable,
            Phase38qBackendStubReason::LiveMutationDisabled,
            Phase38qBackendStubNextLane::LiveMutationStillBlocked,
        ),
    }
}

pub const PHASE_38Q_STUB_CAPABILITIES: &[Phase38qBackendStubCapability] = &[
    Phase38qBackendStubCapability::ValidateRequest,
    Phase38qBackendStubCapability::PlanOperation,
    Phase38qBackendStubCapability::PrepareFutureMutation,
];

pub fn phase38q_has_stub_capability(capability: Phase38qBackendStubCapability) -> bool {
    PHASE_38Q_STUB_CAPABILITIES.contains(&capability)
}

pub const fn phase38q_next_lane() -> Phase38qBackendStubNextLane {
    Phase38qBackendStubNextLane::PersistentBackendReadBeforeWriteStub
}

pub fn phase38q_live_mutation_still_disabled() -> bool {
    !PHASE_38Q_LIVE_MUTATION_ENABLED
        && !PHASE_38Q_PERSISTENT_BACKEND_BOUND
        && !PHASE_38P_LIVE_MUTATION_ENABLED
        && !PHASE_38P_BACKEND_BOUND
        && phase38p_live_mutation_still_disabled()
}

pub const fn phase38q_phase38p_next_lane_bridge() -> Phase38pNextLane {
    Phase38pNextLane::GuardedPersistentBackendStub
}

pub fn phase38q_marker() -> &'static str {
    PHASE_38Q_GUARDED_PERSISTENT_BACKEND_STUB_MARKER
}
