//! Phase 39N — Review-Delete-Later Candidate Removal Dry Run Acceptance.
//!
//! Acceptance wrapper for the Phase 39N dry-run plan.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_review_delete_later_removal_dry_run::{
    Phase39nDryRunReason, Phase39nDryRunStatus, Phase39nNextLane, phase39n_removal_dry_run_report,
};

pub const PHASE_39N_REVIEW_DELETE_LATER_REMOVAL_DRY_RUN_ACCEPTANCE_MARKER: &str =
    "phase39n-acceptance=x4-review-delete-later-removal-dry-run-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39nAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39nAcceptanceReason {
    DryRunAccepted,
    DryRunBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39nAcceptanceReport {
    pub status: Phase39nAcceptanceStatus,
    pub reason: Phase39nAcceptanceReason,
    pub dry_run_reason: Phase39nDryRunReason,
    pub next_lane: Phase39nNextLane,
}

impl Phase39nAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39nAcceptanceStatus::Accepted)
    }
}

pub fn phase39n_acceptance_report() -> Phase39nAcceptanceReport {
    let report = phase39n_removal_dry_run_report();
    let accepted = report.accepted();

    Phase39nAcceptanceReport {
        status: if accepted {
            Phase39nAcceptanceStatus::Accepted
        } else {
            Phase39nAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase39nDryRunStatus::Accepted) {
            Phase39nAcceptanceReason::DryRunAccepted
        } else {
            Phase39nAcceptanceReason::DryRunBlocked
        },
        dry_run_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase39n_acceptance_marker() -> &'static str {
    PHASE_39N_REVIEW_DELETE_LATER_REMOVAL_DRY_RUN_ACCEPTANCE_MARKER
}
