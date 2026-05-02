//! Phase 38N — State I/O Guarded Write Dry-Run Acceptance.
//!
//! This module accepts the Phase 38M dry-run executor as the current safe
//! write-lane state. It summarizes whether a dry-run report is acceptable for
//! future backend work while keeping live mutation disabled.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_write_backend_dry_run_executor::{
    PHASE_38M_LIVE_MUTATION_ENABLED, Phase38mDryRunReport, Phase38mExecutionStatus,
    Phase38mRejectionReason,
};

pub const PHASE_38N_GUARDED_WRITE_DRY_RUN_ACCEPTANCE_MARKER: &str =
    "phase38n=x4-state-io-guarded-write-dry-run-acceptance-ok";

pub const PHASE_38N_LIVE_MUTATION_ENABLED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38nAcceptanceItem {
    GuardedSeamPresent,
    DryRunExecutorPresent,
    LiveMutationDisabled,
    PayloadPreviewBounded,
    RejectionMapped,
    FutureBackendStillGated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38nAcceptanceStatus {
    Accepted,
    Rejected,
    Deferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38nAcceptanceReason {
    DryRunAccepted,
    DryRunRejectedByGuard,
    PolicyDenied,
    InvalidRequest,
    FutureBackendDispatchNotAllowedHere,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38nNextLane {
    GuardedBackendAdapterShape,
    LiveWriteStillBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38nAcceptanceReport {
    pub status: Phase38nAcceptanceStatus,
    pub reason: Phase38nAcceptanceReason,
    pub next_lane: Phase38nNextLane,
    pub live_mutation_enabled: bool,
}

impl Phase38nAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase38nAcceptanceStatus::Accepted)
    }

    pub const fn permits_live_mutation(self) -> bool {
        self.live_mutation_enabled
    }
}

pub const PHASE_38N_ACCEPTANCE_ITEMS: &[Phase38nAcceptanceItem] = &[
    Phase38nAcceptanceItem::GuardedSeamPresent,
    Phase38nAcceptanceItem::DryRunExecutorPresent,
    Phase38nAcceptanceItem::LiveMutationDisabled,
    Phase38nAcceptanceItem::PayloadPreviewBounded,
    Phase38nAcceptanceItem::RejectionMapped,
    Phase38nAcceptanceItem::FutureBackendStillGated,
];

pub fn phase38n_has_acceptance_item(item: Phase38nAcceptanceItem) -> bool {
    PHASE_38N_ACCEPTANCE_ITEMS.contains(&item)
}

pub fn phase38n_accept_dry_run_report(report: Phase38mDryRunReport) -> Phase38nAcceptanceReport {
    let (status, reason) = phase38n_status_and_reason(report);

    Phase38nAcceptanceReport {
        status,
        reason,
        next_lane: Phase38nNextLane::GuardedBackendAdapterShape,
        live_mutation_enabled: PHASE_38N_LIVE_MUTATION_ENABLED,
    }
}

pub const fn phase38n_status_and_reason(
    report: Phase38mDryRunReport,
) -> (Phase38nAcceptanceStatus, Phase38nAcceptanceReason) {
    match report.status {
        Phase38mExecutionStatus::Validated
        | Phase38mExecutionStatus::Planned
        | Phase38mExecutionStatus::Previewed => (
            Phase38nAcceptanceStatus::Accepted,
            Phase38nAcceptanceReason::DryRunAccepted,
        ),
        Phase38mExecutionStatus::Denied => (
            Phase38nAcceptanceStatus::Deferred,
            Phase38nAcceptanceReason::PolicyDenied,
        ),
        Phase38mExecutionStatus::Rejected => (
            Phase38nAcceptanceStatus::Rejected,
            phase38n_reason_from_rejection(report.rejection),
        ),
    }
}

pub const fn phase38n_reason_from_rejection(
    rejection: Phase38mRejectionReason,
) -> Phase38nAcceptanceReason {
    match rejection {
        Phase38mRejectionReason::None => {
            Phase38nAcceptanceReason::FutureBackendDispatchNotAllowedHere
        }
        Phase38mRejectionReason::PolicyDenied => Phase38nAcceptanceReason::PolicyDenied,
        Phase38mRejectionReason::MissingBookId
        | Phase38mRejectionReason::UnexpectedBookId
        | Phase38mRejectionReason::InvalidIntentForKind
        | Phase38mRejectionReason::EmptyPayload => Phase38nAcceptanceReason::InvalidRequest,
    }
}

pub const fn phase38n_live_mutation_still_disabled() -> bool {
    !PHASE_38N_LIVE_MUTATION_ENABLED && !PHASE_38M_LIVE_MUTATION_ENABLED
}

pub const fn phase38n_next_lane() -> Phase38nNextLane {
    Phase38nNextLane::GuardedBackendAdapterShape
}

pub fn phase38n_marker() -> &'static str {
    PHASE_38N_GUARDED_WRITE_DRY_RUN_ACCEPTANCE_MARKER
}
