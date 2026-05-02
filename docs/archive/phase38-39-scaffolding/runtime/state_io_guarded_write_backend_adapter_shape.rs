//! Phase 38O — State I/O Guarded Write Backend Adapter Shape.
//!
//! This module defines the adapter shape that a future persistent mutation
//! backend must satisfy. The default adapter remains unavailable and no live
//! mutation path is invoked in this phase.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_write_backend_implementation_seam::{
    Phase38lBackendDispatch, Phase38lBookId, Phase38lGuardDecision, Phase38lMutationIntent,
    Phase38lMutationPlan, Phase38lMutationPolicy, Phase38lMutationRequest, Phase38lStateRecordKind,
    phase38l_plan_guarded_mutation,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_dry_run_acceptance::{
    Phase38nNextLane, phase38n_live_mutation_still_disabled,
};

pub const PHASE_38O_GUARDED_WRITE_BACKEND_ADAPTER_SHAPE_MARKER: &str =
    "phase38o=x4-state-io-guarded-write-backend-adapter-shape-ok";

pub const PHASE_38O_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38O_BACKEND_BOUND: bool = false;
pub const PHASE_38O_MAX_OPERATION_LABEL_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38oBackendCapability {
    ReadExistingRecord,
    ValidatePayload,
    PrepareMutation,
    CommitMutation,
    RollbackMutation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38oAdapterAvailability {
    Unbound,
    DryRunOnly,
    FuturePersistentAdapter,
}

impl Phase38oAdapterAvailability {
    pub const fn live_backend_available(self) -> bool {
        matches!(self, Self::FuturePersistentAdapter)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38oAdapterDecision {
    AdapterUnavailable,
    AcceptedForDryRun,
    AcceptedForFutureDispatch,
    RejectedByGuard,
    RejectedCapabilityMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38oAdapterOperation {
    PlanOnly,
    ValidateOnly,
    PrepareOnly,
    CommitThroughFutureBackend,
    RollbackThroughFutureBackend,
}

impl Phase38oAdapterOperation {
    pub const fn requires_live_backend(self) -> bool {
        matches!(
            self,
            Self::CommitThroughFutureBackend | Self::RollbackThroughFutureBackend
        )
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::PlanOnly => "plan-only",
            Self::ValidateOnly => "validate-only",
            Self::PrepareOnly => "prepare-only",
            Self::CommitThroughFutureBackend => "future-commit",
            Self::RollbackThroughFutureBackend => "future-rollback",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38oAdapterCapabilities {
    pub read_existing_record: bool,
    pub validate_payload: bool,
    pub prepare_mutation: bool,
    pub commit_mutation: bool,
    pub rollback_mutation: bool,
}

impl Phase38oAdapterCapabilities {
    pub const fn none() -> Self {
        Self {
            read_existing_record: false,
            validate_payload: false,
            prepare_mutation: false,
            commit_mutation: false,
            rollback_mutation: false,
        }
    }

    pub const fn dry_run() -> Self {
        Self {
            read_existing_record: false,
            validate_payload: true,
            prepare_mutation: true,
            commit_mutation: false,
            rollback_mutation: false,
        }
    }

    pub const fn has(self, capability: Phase38oBackendCapability) -> bool {
        match capability {
            Phase38oBackendCapability::ReadExistingRecord => self.read_existing_record,
            Phase38oBackendCapability::ValidatePayload => self.validate_payload,
            Phase38oBackendCapability::PrepareMutation => self.prepare_mutation,
            Phase38oBackendCapability::CommitMutation => self.commit_mutation,
            Phase38oBackendCapability::RollbackMutation => self.rollback_mutation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38oAdapterRequest {
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub book_id: Option<Phase38lBookId>,
    pub payload_len: usize,
    pub operation: Phase38oAdapterOperation,
    pub availability: Phase38oAdapterAvailability,
    pub capabilities: Phase38oAdapterCapabilities,
}

impl Phase38oAdapterRequest {
    pub const fn new(
        kind: Phase38lStateRecordKind,
        intent: Phase38lMutationIntent,
        book_id: Option<Phase38lBookId>,
        payload_len: usize,
        operation: Phase38oAdapterOperation,
        availability: Phase38oAdapterAvailability,
        capabilities: Phase38oAdapterCapabilities,
    ) -> Self {
        Self {
            kind,
            intent,
            book_id,
            payload_len,
            operation,
            availability,
            capabilities,
        }
    }

    pub const fn phase38l_policy(self) -> Phase38lMutationPolicy {
        match self.availability {
            Phase38oAdapterAvailability::Unbound => Phase38lMutationPolicy::DenyAll,
            Phase38oAdapterAvailability::DryRunOnly => Phase38lMutationPolicy::DryRunOnly,
            Phase38oAdapterAvailability::FuturePersistentAdapter => {
                Phase38lMutationPolicy::AllowLiveMutation
            }
        }
    }

    pub const fn phase38l_request(self) -> Phase38lMutationRequest {
        Phase38lMutationRequest::new(
            self.kind,
            self.intent,
            self.book_id,
            self.payload_len,
            self.phase38l_policy(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38oAdapterPlan {
    pub decision: Phase38oAdapterDecision,
    pub operation: Phase38oAdapterOperation,
    pub seam_decision: Phase38lGuardDecision,
    pub seam_dispatch: Phase38lBackendDispatch,
    pub next_lane: Phase38nNextLane,
    pub live_mutation_enabled: bool,
}

impl Phase38oAdapterPlan {
    pub const fn permits_live_mutation(self) -> bool {
        self.live_mutation_enabled
    }
}

pub trait Phase38oGuardedBackendAdapterShape {
    fn plan_adapter_operation(&self, request: Phase38oAdapterRequest) -> Phase38oAdapterPlan;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase38oDefaultUnboundAdapter;

impl Phase38oGuardedBackendAdapterShape for Phase38oDefaultUnboundAdapter {
    fn plan_adapter_operation(&self, request: Phase38oAdapterRequest) -> Phase38oAdapterPlan {
        phase38o_plan_adapter_operation(request)
    }
}

pub fn phase38o_plan_adapter_operation(request: Phase38oAdapterRequest) -> Phase38oAdapterPlan {
    let seam_plan = phase38l_plan_guarded_mutation(request.phase38l_request());
    let decision = phase38o_decision_from_request(request, seam_plan);

    Phase38oAdapterPlan {
        decision,
        operation: request.operation,
        seam_decision: seam_plan.decision,
        seam_dispatch: seam_plan.dispatch,
        next_lane: Phase38nNextLane::GuardedBackendAdapterShape,
        live_mutation_enabled: PHASE_38O_LIVE_MUTATION_ENABLED,
    }
}

pub const fn phase38o_decision_from_request(
    request: Phase38oAdapterRequest,
    seam_plan: Phase38lMutationPlan,
) -> Phase38oAdapterDecision {
    if !phase38o_operation_supported(request.operation, request.capabilities) {
        return Phase38oAdapterDecision::RejectedCapabilityMissing;
    }

    match seam_plan.decision {
        Phase38lGuardDecision::AcceptedAsDryRun => Phase38oAdapterDecision::AcceptedForDryRun,
        Phase38lGuardDecision::AcceptedForBackendDispatch => {
            if request.operation.requires_live_backend()
                && !request.availability.live_backend_available()
            {
                Phase38oAdapterDecision::AdapterUnavailable
            } else {
                Phase38oAdapterDecision::AcceptedForFutureDispatch
            }
        }
        Phase38lGuardDecision::DeniedByDefault => Phase38oAdapterDecision::AdapterUnavailable,
        Phase38lGuardDecision::RejectedMissingBookId
        | Phase38lGuardDecision::RejectedUnexpectedBookId
        | Phase38lGuardDecision::RejectedInvalidIntentForKind
        | Phase38lGuardDecision::RejectedEmptyPayload => Phase38oAdapterDecision::RejectedByGuard,
    }
}

pub const fn phase38o_operation_supported(
    operation: Phase38oAdapterOperation,
    capabilities: Phase38oAdapterCapabilities,
) -> bool {
    match operation {
        Phase38oAdapterOperation::PlanOnly => true,
        Phase38oAdapterOperation::ValidateOnly => {
            capabilities.has(Phase38oBackendCapability::ValidatePayload)
        }
        Phase38oAdapterOperation::PrepareOnly => {
            capabilities.has(Phase38oBackendCapability::PrepareMutation)
        }
        Phase38oAdapterOperation::CommitThroughFutureBackend => {
            capabilities.has(Phase38oBackendCapability::CommitMutation)
        }
        Phase38oAdapterOperation::RollbackThroughFutureBackend => {
            capabilities.has(Phase38oBackendCapability::RollbackMutation)
        }
    }
}

pub fn phase38o_live_mutation_still_disabled() -> bool {
    !PHASE_38O_LIVE_MUTATION_ENABLED
        && !PHASE_38O_BACKEND_BOUND
        && phase38n_live_mutation_still_disabled()
}

pub fn phase38o_marker() -> &'static str {
    PHASE_38O_GUARDED_WRITE_BACKEND_ADAPTER_SHAPE_MARKER
}
