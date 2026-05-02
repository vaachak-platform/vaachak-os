//! Phase 38R — State I/O Guarded Read-Before-Write Stub.
//!
//! This module models the required read-before-write preflight step for future
//! typed-state mutation. It is still a stub: it does not read storage, does not
//! write storage, and does not bind a persistent backend.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_persistent_backend_stub::{
    PHASE_38Q_LIVE_MUTATION_ENABLED, PHASE_38Q_PERSISTENT_BACKEND_BOUND,
    Phase38qBackendStubNextLane, Phase38qBackendStubReport, Phase38qBackendStubRequest,
    Phase38qBackendStubStatus, phase38q_live_mutation_still_disabled,
    phase38q_plan_persistent_backend_stub,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_adapter_shape::Phase38oAdapterOperation;
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_implementation_seam::{
    Phase38lBookId, Phase38lMutationIntent, Phase38lStateRecordKind,
};

pub const PHASE_38R_GUARDED_READ_BEFORE_WRITE_STUB_MARKER: &str =
    "phase38r=x4-state-io-guarded-read-before-write-stub-ok";

pub const PHASE_38R_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38R_PERSISTENT_BACKEND_BOUND: bool = false;
pub const PHASE_38R_PREWRITE_READ_AVAILABLE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38rReadRequirement {
    Required,
    Optional,
    SkippedForDelete,
    SkippedForIndexAppend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38rObservedRecordState {
    Unknown,
    WouldBeMissing,
    WouldBePresent,
    ReadUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38rPreflightDecision {
    PlanAccepted,
    DryRunAccepted,
    WaitForReadBackend,
    GuardRejected,
    PayloadRejected,
    BackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38rConflictPolicy {
    AcceptUnknown,
    RequireExistingMatch,
    RequireMissingRecord,
    DenyIfUnknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38rNextLane {
    GuardedWritePrepareStub,
    ReadBackendNeeded,
    LiveMutationStillBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38rReadBeforeWriteRequest {
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub book_id: Option<Phase38lBookId>,
    pub payload_len: usize,
    pub conflict_policy: Phase38rConflictPolicy,
}

impl Phase38rReadBeforeWriteRequest {
    pub const fn new(
        kind: Phase38lStateRecordKind,
        intent: Phase38lMutationIntent,
        book_id: Option<Phase38lBookId>,
        payload_len: usize,
        conflict_policy: Phase38rConflictPolicy,
    ) -> Self {
        Self {
            kind,
            intent,
            book_id,
            payload_len,
            conflict_policy,
        }
    }

    pub const fn read_requirement(self) -> Phase38rReadRequirement {
        match self.intent {
            Phase38lMutationIntent::RemoveRecord => Phase38rReadRequirement::SkippedForDelete,
            Phase38lMutationIntent::AppendIndex => Phase38rReadRequirement::SkippedForIndexAppend,
            Phase38lMutationIntent::CompactIndex => Phase38rReadRequirement::Required,
            Phase38lMutationIntent::ReplaceRecord => Phase38rReadRequirement::Required,
            Phase38lMutationIntent::UpsertRecord => Phase38rReadRequirement::Optional,
        }
    }

    pub const fn adapter_operation(self) -> Phase38oAdapterOperation {
        match self.read_requirement() {
            Phase38rReadRequirement::Required | Phase38rReadRequirement::Optional => {
                Phase38oAdapterOperation::PrepareOnly
            }
            Phase38rReadRequirement::SkippedForDelete
            | Phase38rReadRequirement::SkippedForIndexAppend => Phase38oAdapterOperation::PlanOnly,
        }
    }

    pub const fn as_phase38q_request(self) -> Phase38qBackendStubRequest {
        Phase38qBackendStubRequest::new(
            self.kind,
            self.intent,
            self.book_id,
            self.payload_len,
            self.adapter_operation(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38rReadBeforeWriteReport {
    pub decision: Phase38rPreflightDecision,
    pub requirement: Phase38rReadRequirement,
    pub observed_state: Phase38rObservedRecordState,
    pub conflict_policy: Phase38rConflictPolicy,
    pub stub_status: Phase38qBackendStubStatus,
    pub stub_next_lane: Phase38qBackendStubNextLane,
    pub next_lane: Phase38rNextLane,
    pub backend_bound: bool,
    pub live_mutation_enabled: bool,
}

impl Phase38rReadBeforeWriteReport {
    pub const fn accepted(self) -> bool {
        matches!(
            self.decision,
            Phase38rPreflightDecision::PlanAccepted | Phase38rPreflightDecision::DryRunAccepted
        )
    }

    pub const fn permits_live_mutation(self) -> bool {
        self.live_mutation_enabled
    }

    pub const fn persistent_backend_bound(self) -> bool {
        self.backend_bound
    }
}

pub trait Phase38rGuardedReadBeforeWriteStub {
    fn plan_read_before_write(
        &self,
        request: Phase38rReadBeforeWriteRequest,
    ) -> Phase38rReadBeforeWriteReport;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase38rDefaultReadBeforeWriteStub;

impl Phase38rGuardedReadBeforeWriteStub for Phase38rDefaultReadBeforeWriteStub {
    fn plan_read_before_write(
        &self,
        request: Phase38rReadBeforeWriteRequest,
    ) -> Phase38rReadBeforeWriteReport {
        phase38r_plan_read_before_write(request)
    }
}

pub fn phase38r_plan_read_before_write(
    request: Phase38rReadBeforeWriteRequest,
) -> Phase38rReadBeforeWriteReport {
    let requirement = request.read_requirement();
    let observed_state = phase38r_observed_state_for_requirement(requirement);
    let stub_report = phase38q_plan_persistent_backend_stub(request.as_phase38q_request());
    phase38r_report_from_stub(request, requirement, observed_state, stub_report)
}

pub const fn phase38r_report_from_stub(
    request: Phase38rReadBeforeWriteRequest,
    requirement: Phase38rReadRequirement,
    observed_state: Phase38rObservedRecordState,
    stub_report: Phase38qBackendStubReport,
) -> Phase38rReadBeforeWriteReport {
    let decision = phase38r_decision_from_stub(request, requirement, observed_state, stub_report);
    let next_lane = phase38r_next_lane_for_decision(decision);

    Phase38rReadBeforeWriteReport {
        decision,
        requirement,
        observed_state,
        conflict_policy: request.conflict_policy,
        stub_status: stub_report.status,
        stub_next_lane: stub_report.next_lane,
        next_lane,
        backend_bound: PHASE_38R_PERSISTENT_BACKEND_BOUND,
        live_mutation_enabled: PHASE_38R_LIVE_MUTATION_ENABLED,
    }
}

pub const fn phase38r_decision_from_stub(
    request: Phase38rReadBeforeWriteRequest,
    requirement: Phase38rReadRequirement,
    observed_state: Phase38rObservedRecordState,
    stub_report: Phase38qBackendStubReport,
) -> Phase38rPreflightDecision {
    match stub_report.status {
        Phase38qBackendStubStatus::AcceptedForPlan => {
            phase38r_decision_for_accepted_plan(request, requirement, observed_state)
        }
        Phase38qBackendStubStatus::AcceptedForDryRun => Phase38rPreflightDecision::DryRunAccepted,
        Phase38qBackendStubStatus::DeferredUntilBackendBound => {
            Phase38rPreflightDecision::WaitForReadBackend
        }
        Phase38qBackendStubStatus::RejectedByGuard => Phase38rPreflightDecision::GuardRejected,
        Phase38qBackendStubStatus::RejectedPayloadTooLarge => {
            Phase38rPreflightDecision::PayloadRejected
        }
        Phase38qBackendStubStatus::RejectedBackendUnavailable => {
            Phase38rPreflightDecision::BackendUnavailable
        }
    }
}

pub const fn phase38r_decision_for_accepted_plan(
    request: Phase38rReadBeforeWriteRequest,
    requirement: Phase38rReadRequirement,
    observed_state: Phase38rObservedRecordState,
) -> Phase38rPreflightDecision {
    match request.conflict_policy {
        Phase38rConflictPolicy::AcceptUnknown => Phase38rPreflightDecision::PlanAccepted,
        Phase38rConflictPolicy::DenyIfUnknown => match observed_state {
            Phase38rObservedRecordState::Unknown | Phase38rObservedRecordState::ReadUnavailable => {
                Phase38rPreflightDecision::WaitForReadBackend
            }
            Phase38rObservedRecordState::WouldBeMissing
            | Phase38rObservedRecordState::WouldBePresent => {
                Phase38rPreflightDecision::PlanAccepted
            }
        },
        Phase38rConflictPolicy::RequireExistingMatch => match requirement {
            Phase38rReadRequirement::Required => Phase38rPreflightDecision::WaitForReadBackend,
            Phase38rReadRequirement::Optional
            | Phase38rReadRequirement::SkippedForDelete
            | Phase38rReadRequirement::SkippedForIndexAppend => {
                Phase38rPreflightDecision::PlanAccepted
            }
        },
        Phase38rConflictPolicy::RequireMissingRecord => match observed_state {
            Phase38rObservedRecordState::WouldBeMissing => Phase38rPreflightDecision::PlanAccepted,
            Phase38rObservedRecordState::Unknown
            | Phase38rObservedRecordState::WouldBePresent
            | Phase38rObservedRecordState::ReadUnavailable => {
                Phase38rPreflightDecision::WaitForReadBackend
            }
        },
    }
}

pub const fn phase38r_observed_state_for_requirement(
    requirement: Phase38rReadRequirement,
) -> Phase38rObservedRecordState {
    match requirement {
        Phase38rReadRequirement::Required | Phase38rReadRequirement::Optional => {
            Phase38rObservedRecordState::ReadUnavailable
        }
        Phase38rReadRequirement::SkippedForDelete
        | Phase38rReadRequirement::SkippedForIndexAppend => Phase38rObservedRecordState::Unknown,
    }
}

pub const fn phase38r_next_lane_for_decision(
    decision: Phase38rPreflightDecision,
) -> Phase38rNextLane {
    match decision {
        Phase38rPreflightDecision::PlanAccepted | Phase38rPreflightDecision::DryRunAccepted => {
            Phase38rNextLane::GuardedWritePrepareStub
        }
        Phase38rPreflightDecision::WaitForReadBackend => Phase38rNextLane::ReadBackendNeeded,
        Phase38rPreflightDecision::GuardRejected
        | Phase38rPreflightDecision::PayloadRejected
        | Phase38rPreflightDecision::BackendUnavailable => {
            Phase38rNextLane::LiveMutationStillBlocked
        }
    }
}

pub fn phase38r_live_mutation_still_disabled() -> bool {
    !PHASE_38R_LIVE_MUTATION_ENABLED
        && !PHASE_38R_PERSISTENT_BACKEND_BOUND
        && !PHASE_38R_PREWRITE_READ_AVAILABLE
        && !PHASE_38Q_LIVE_MUTATION_ENABLED
        && !PHASE_38Q_PERSISTENT_BACKEND_BOUND
        && phase38q_live_mutation_still_disabled()
}

pub const fn phase38r_next_lane() -> Phase38rNextLane {
    Phase38rNextLane::GuardedWritePrepareStub
}

pub fn phase38r_marker() -> &'static str {
    PHASE_38R_GUARDED_READ_BEFORE_WRITE_STUB_MARKER
}
