//! Phase 39O — Guarded Review-Delete-Later Removal Patch Acceptance.
//!
//! Acceptance wrapper for the guarded candidate-removal patch.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_review_delete_later_removal_patch::{
    phase39o_removal_report, Phase39oNextLane, Phase39oRemovalReason, Phase39oRemovalStatus,
};

pub const PHASE_39O_GUARDED_REVIEW_DELETE_LATER_REMOVAL_PATCH_ACCEPTANCE_MARKER: &str =
    "phase39o-acceptance=x4-guarded-review-delete-later-removal-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oAcceptanceReason {
    RemovalPatchAccepted,
    RemovalPatchBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39oAcceptanceReport {
    pub status: Phase39oAcceptanceStatus,
    pub reason: Phase39oAcceptanceReason,
    pub removal_reason: Phase39oRemovalReason,
    pub next_lane: Phase39oNextLane,
}

impl Phase39oAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39oAcceptanceStatus::Accepted)
    }
}

pub fn phase39o_acceptance_report() -> Phase39oAcceptanceReport {
    let report = phase39o_removal_report();
    let accepted = report.accepted();

    Phase39oAcceptanceReport {
        status: if accepted {
            Phase39oAcceptanceStatus::Accepted
        } else {
            Phase39oAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase39oRemovalStatus::Accepted) {
            Phase39oAcceptanceReason::RemovalPatchAccepted
        } else {
            Phase39oAcceptanceReason::RemovalPatchBlocked
        },
        removal_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase39o_acceptance_marker() -> &'static str {
    PHASE_39O_GUARDED_REVIEW_DELETE_LATER_REMOVAL_PATCH_ACCEPTANCE_MARKER
}
