//! Phase 40B — Reader UX Regression Baseline Acceptance.
//!
//! Acceptance wrapper for the Phase 40B reader UX baseline metadata.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_reader_ux_regression_baseline::{
    Phase40bBaselineReason, Phase40bBaselineStatus, Phase40bNextLane,
    phase40b_reader_ux_baseline_report,
};

pub const PHASE_40B_READER_UX_REGRESSION_BASELINE_ACCEPTANCE_MARKER: &str =
    "phase40b-acceptance=x4-reader-ux-regression-baseline-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bAcceptanceReason {
    BaselineAccepted,
    BaselineBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40bAcceptanceReport {
    pub status: Phase40bAcceptanceStatus,
    pub reason: Phase40bAcceptanceReason,
    pub baseline_reason: Phase40bBaselineReason,
    pub next_lane: Phase40bNextLane,
}

impl Phase40bAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40bAcceptanceStatus::Accepted)
    }
}

pub fn phase40b_acceptance_report() -> Phase40bAcceptanceReport {
    let report = phase40b_reader_ux_baseline_report();
    let accepted = report.accepted();

    Phase40bAcceptanceReport {
        status: if accepted {
            Phase40bAcceptanceStatus::Accepted
        } else {
            Phase40bAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40bBaselineStatus::Accepted) {
            Phase40bAcceptanceReason::BaselineAccepted
        } else {
            Phase40bAcceptanceReason::BaselineBlocked
        },
        baseline_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase40b_acceptance_marker() -> &'static str {
    PHASE_40B_READER_UX_REGRESSION_BASELINE_ACCEPTANCE_MARKER
}
