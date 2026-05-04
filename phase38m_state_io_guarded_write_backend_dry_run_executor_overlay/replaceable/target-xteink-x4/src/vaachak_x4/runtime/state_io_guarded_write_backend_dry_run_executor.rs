//! Phase 38M — State I/O Guarded Write Backend Dry-Run Executor.
//!
//! This module executes the Phase 38L write-backend seam in dry-run form only.
//! It creates an auditable execution report for a planned typed-state mutation,
//! but does not perform live mutation and does not bind the persistent backend.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_write_backend_implementation_seam::{
    phase38l_plan_guarded_mutation, Phase38lBackendDispatch, Phase38lBookId,
    Phase38lGuardDecision, Phase38lMutationIntent, Phase38lMutationPlan,
    Phase38lMutationPolicy, Phase38lMutationRequest, Phase38lStateRecordKind,
};

pub const PHASE_38M_GUARDED_WRITE_BACKEND_DRY_RUN_EXECUTOR_MARKER: &str =
    "phase38m=x4-state-io-guarded-write-backend-dry-run-executor-ok";

pub const PHASE_38M_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES: usize = 16;
pub const PHASE_38M_MAX_PATH_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38mDryRunMode {
    ValidateOnly,
    ProducePlan,
    ProducePlanWithPayloadPreview,
}

impl Phase38mDryRunMode {
    pub const fn includes_payload_preview(self) -> bool {
        matches!(self, Self::ProducePlanWithPayloadPreview)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38mExecutionStatus {
    Denied,
    Validated,
    Planned,
    Previewed,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38mRejectionReason {
    None,
    MissingBookId,
    UnexpectedBookId,
    InvalidIntentForKind,
    EmptyPayload,
    PolicyDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38mPayloadPreview {
    pub bytes: [u8; PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES],
    pub len: usize,
    pub truncated: bool,
}

impl Phase38mPayloadPreview {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES],
            len: 0,
            truncated: false,
        }
    }

    pub fn from_payload(payload: &[u8]) -> Self {
        let mut bytes = [0u8; PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES];
        let len = payload.len().min(PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES);
        bytes[..len].copy_from_slice(&payload[..len]);
        Self {
            bytes,
            len,
            truncated: payload.len() > PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES,
        }
    }

    pub fn as_slice<'a>(&self, backing: &'a [u8; PHASE_38M_MAX_PAYLOAD_PREVIEW_BYTES]) -> &'a [u8] {
        &backing[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38mDryRunInput<'a> {
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub book_id: Option<Phase38lBookId>,
    pub payload: &'a [u8],
    pub mode: Phase38mDryRunMode,
}

impl<'a> Phase38mDryRunInput<'a> {
    pub const fn new(
        kind: Phase38lStateRecordKind,
        intent: Phase38lMutationIntent,
        book_id: Option<Phase38lBookId>,
        payload: &'a [u8],
        mode: Phase38mDryRunMode,
    ) -> Self {
        Self {
            kind,
            intent,
            book_id,
            payload,
            mode,
        }
    }

    pub const fn as_phase38l_request(self) -> Phase38lMutationRequest {
        Phase38lMutationRequest::new(
            self.kind,
            self.intent,
            self.book_id,
            self.payload.len(),
            Phase38lMutationPolicy::DryRunOnly,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38mDryRunReport {
    pub status: Phase38mExecutionStatus,
    pub rejection: Phase38mRejectionReason,
    pub decision: Phase38lGuardDecision,
    pub dispatch: Phase38lBackendDispatch,
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub payload_len: usize,
    pub preview: Phase38mPayloadPreview,
}

impl Phase38mDryRunReport {
    pub const fn live_mutation_performed(self) -> bool {
        false
    }

    pub const fn accepted(self) -> bool {
        matches!(
            self.status,
            Phase38mExecutionStatus::Validated
                | Phase38mExecutionStatus::Planned
                | Phase38mExecutionStatus::Previewed
        )
    }
}

pub trait Phase38mGuardedDryRunExecutor {
    fn execute_dry_run(&self, input: Phase38mDryRunInput<'_>) -> Phase38mDryRunReport;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase38mDefaultDryRunExecutor;

impl Phase38mGuardedDryRunExecutor for Phase38mDefaultDryRunExecutor {
    fn execute_dry_run(&self, input: Phase38mDryRunInput<'_>) -> Phase38mDryRunReport {
        phase38m_execute_guarded_dry_run(input)
    }
}

pub fn phase38m_execute_guarded_dry_run(input: Phase38mDryRunInput<'_>) -> Phase38mDryRunReport {
    let plan = phase38l_plan_guarded_mutation(input.as_phase38l_request());
    let rejection = phase38m_rejection_from_decision(plan.decision);
    let status = phase38m_status_from_plan(input.mode, plan);
    let preview = if input.mode.includes_payload_preview() && status == Phase38mExecutionStatus::Previewed {
        Phase38mPayloadPreview::from_payload(input.payload)
    } else {
        Phase38mPayloadPreview::empty()
    };

    Phase38mDryRunReport {
        status,
        rejection,
        decision: plan.decision,
        dispatch: plan.dispatch,
        kind: plan.kind,
        intent: plan.intent,
        payload_len: plan.payload_len,
        preview,
    }
}

pub const fn phase38m_status_from_plan(
    mode: Phase38mDryRunMode,
    plan: Phase38lMutationPlan,
) -> Phase38mExecutionStatus {
    match plan.decision {
        Phase38lGuardDecision::AcceptedAsDryRun => match mode {
            Phase38mDryRunMode::ValidateOnly => Phase38mExecutionStatus::Validated,
            Phase38mDryRunMode::ProducePlan => Phase38mExecutionStatus::Planned,
            Phase38mDryRunMode::ProducePlanWithPayloadPreview => Phase38mExecutionStatus::Previewed,
        },
        Phase38lGuardDecision::DeniedByDefault => Phase38mExecutionStatus::Denied,
        Phase38lGuardDecision::AcceptedForBackendDispatch => Phase38mExecutionStatus::Rejected,
        Phase38lGuardDecision::RejectedMissingBookId
        | Phase38lGuardDecision::RejectedUnexpectedBookId
        | Phase38lGuardDecision::RejectedInvalidIntentForKind
        | Phase38lGuardDecision::RejectedEmptyPayload => Phase38mExecutionStatus::Rejected,
    }
}

pub const fn phase38m_rejection_from_decision(
    decision: Phase38lGuardDecision,
) -> Phase38mRejectionReason {
    match decision {
        Phase38lGuardDecision::AcceptedAsDryRun => Phase38mRejectionReason::None,
        Phase38lGuardDecision::AcceptedForBackendDispatch => Phase38mRejectionReason::None,
        Phase38lGuardDecision::DeniedByDefault => Phase38mRejectionReason::PolicyDenied,
        Phase38lGuardDecision::RejectedMissingBookId => Phase38mRejectionReason::MissingBookId,
        Phase38lGuardDecision::RejectedUnexpectedBookId => {
            Phase38mRejectionReason::UnexpectedBookId
        }
        Phase38lGuardDecision::RejectedInvalidIntentForKind => {
            Phase38mRejectionReason::InvalidIntentForKind
        }
        Phase38lGuardDecision::RejectedEmptyPayload => Phase38mRejectionReason::EmptyPayload,
    }
}

pub fn phase38m_known_record_kinds() -> &'static [Phase38lStateRecordKind] {
    const KINDS: &[Phase38lStateRecordKind] = &[
        Phase38lStateRecordKind::Progress,
        Phase38lStateRecordKind::Theme,
        Phase38lStateRecordKind::Metadata,
        Phase38lStateRecordKind::Bookmark,
        Phase38lStateRecordKind::BookmarkIndex,
    ];
    KINDS
}

pub fn phase38m_marker() -> &'static str {
    PHASE_38M_GUARDED_WRITE_BACKEND_DRY_RUN_EXECUTOR_MARKER
}
