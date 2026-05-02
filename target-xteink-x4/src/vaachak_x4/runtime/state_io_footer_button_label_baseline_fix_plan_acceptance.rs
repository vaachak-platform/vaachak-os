//! Phase 40C — Footer/Button Label Baseline and Fix Plan Acceptance.
//!
//! Acceptance wrapper for the Phase 40C plan-only baseline.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_footer_button_label_baseline_fix_plan::{
    Phase40cNextLane, Phase40cPlanReason, Phase40cPlanStatus, phase40c_footer_button_plan_report,
};

pub const PHASE_40C_FOOTER_BUTTON_LABEL_BASELINE_FIX_PLAN_ACCEPTANCE_MARKER: &str =
    "phase40c-acceptance=x4-footer-button-label-baseline-fix-plan-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cAcceptanceReason {
    FixPlanAccepted,
    FixPlanBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40cAcceptanceReport {
    pub status: Phase40cAcceptanceStatus,
    pub reason: Phase40cAcceptanceReason,
    pub plan_reason: Phase40cPlanReason,
    pub next_lane: Phase40cNextLane,
}

impl Phase40cAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40cAcceptanceStatus::Accepted)
    }
}

pub fn phase40c_acceptance_report() -> Phase40cAcceptanceReport {
    let report = phase40c_footer_button_plan_report();
    let accepted = report.accepted();

    Phase40cAcceptanceReport {
        status: if accepted {
            Phase40cAcceptanceStatus::Accepted
        } else {
            Phase40cAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40cPlanStatus::Ready) {
            Phase40cAcceptanceReason::FixPlanAccepted
        } else {
            Phase40cAcceptanceReason::FixPlanBlocked
        },
        plan_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase40c_acceptance_marker() -> &'static str {
    PHASE_40C_FOOTER_BUTTON_LABEL_BASELINE_FIX_PLAN_ACCEPTANCE_MARKER
}
