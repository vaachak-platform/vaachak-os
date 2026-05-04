//! Phase 40E — Reader UX Polish Candidate Plan Acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_reader_ux_polish_candidate_plan::{
    phase40e_reader_ux_polish_plan_report, Phase40eNext, Phase40eReason, Phase40eStatus,
};

pub const PHASE_40E_READER_UX_POLISH_CANDIDATE_PLAN_ACCEPTANCE_MARKER: &str =
    "phase40e-acceptance=x4-reader-ux-polish-candidate-plan-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40eAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40eAcceptanceReason {
    PlanAccepted,
    PlanBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40eAcceptanceReport {
    pub status: Phase40eAcceptanceStatus,
    pub reason: Phase40eAcceptanceReason,
    pub plan_reason: Phase40eReason,
    pub next: Phase40eNext,
}

impl Phase40eAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40eAcceptanceStatus::Accepted)
    }
}

pub fn phase40e_acceptance_report() -> Phase40eAcceptanceReport {
    let report = phase40e_reader_ux_polish_plan_report();
    let accepted = report.accepted();

    Phase40eAcceptanceReport {
        status: if accepted {
            Phase40eAcceptanceStatus::Accepted
        } else {
            Phase40eAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40eStatus::Ready) {
            Phase40eAcceptanceReason::PlanAccepted
        } else {
            Phase40eAcceptanceReason::PlanBlocked
        },
        plan_reason: report.reason,
        next: report.next,
    }
}

pub fn phase40e_acceptance_marker() -> &'static str {
    PHASE_40E_READER_UX_POLISH_CANDIDATE_PLAN_ACCEPTANCE_MARKER
}
