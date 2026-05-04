//! Phase 39L — Post-Freeze Scaffolding Cleanup Plan Acceptance.
//!
//! Acceptance wrapper for the Phase 39L review-only cleanup plan.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_post_freeze_scaffolding_cleanup_plan::{
    phase39l_cleanup_plan_report, Phase39lGuardReason, Phase39lGuardStatus, Phase39lNextLane,
};

pub const PHASE_39L_POST_FREEZE_SCAFFOLDING_CLEANUP_PLAN_ACCEPTANCE_MARKER: &str =
    "phase39l-acceptance=x4-post-freeze-scaffolding-cleanup-plan-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lAcceptanceReason {
    ReviewPlanAccepted,
    GuardBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39lAcceptanceReport {
    pub status: Phase39lAcceptanceStatus,
    pub reason: Phase39lAcceptanceReason,
    pub guard_reason: Phase39lGuardReason,
    pub next_lane: Phase39lNextLane,
}

impl Phase39lAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39lAcceptanceStatus::Accepted)
    }
}

pub fn phase39l_acceptance_report() -> Phase39lAcceptanceReport {
    let plan = phase39l_cleanup_plan_report();
    let accepted = plan.accepted();

    Phase39lAcceptanceReport {
        status: if accepted {
            Phase39lAcceptanceStatus::Accepted
        } else {
            Phase39lAcceptanceStatus::Rejected
        },
        reason: if matches!(plan.guard_status, Phase39lGuardStatus::SafeToPlan) {
            Phase39lAcceptanceReason::ReviewPlanAccepted
        } else {
            Phase39lAcceptanceReason::GuardBlocked
        },
        guard_reason: plan.guard_reason,
        next_lane: plan.next_lane,
    }
}

pub fn phase39l_acceptance_marker() -> &'static str {
    PHASE_39L_POST_FREEZE_SCAFFOLDING_CLEANUP_PLAN_ACCEPTANCE_MARKER
}
