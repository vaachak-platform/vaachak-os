//! Phase 36O — State I/O Shadow Write Acceptance Overlay.
//!
//! This module accepts the Phase 36N shadow-write plan as a side-effect-free
//! runtime contract. It intentionally performs no filesystem, SPI, display,
//! input, or power work.
//!
//! The purpose is to make the shadow-write lane auditable before a later phase
//! binds it to a real X4 SD/FAT backend.

#![allow(dead_code)]

use super::state_io_shadow_write_plan::{
    SHADOW_WRITE_RECORD_KINDS, SHADOW_WRITE_STEPS, STATE_IO_SHADOW_WRITE_PLAN_SUMMARY,
    ShadowWritePlanSummary, phase36n_is_side_effect_free, phase36n_marker,
};

/// Phase 36O boot/build marker.
pub const PHASE_36O_STATE_IO_SHADOW_WRITE_ACCEPTANCE_MARKER: &str =
    "phase36o=x4-state-io-shadow-write-acceptance-ok";

/// Minimum number of typed state records required before the plan can be accepted.
pub const REQUIRED_SHADOW_WRITE_RECORD_COUNT: usize = 5;

/// Minimum number of ordered plan steps required before the plan can be accepted.
pub const REQUIRED_SHADOW_WRITE_STEP_COUNT: usize = 7;

/// Compile-time acceptance decision for the shadow-write lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowWriteAcceptanceDecision {
    Accepted,
    Rejected,
}

impl ShadowWriteAcceptanceDecision {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }

    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// Side-effect-free acceptance report for the Phase 36N shadow-write plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShadowWriteAcceptanceReport {
    pub marker: &'static str,
    pub accepted_plan_marker: &'static str,
    pub decision: ShadowWriteAcceptanceDecision,
    pub record_count: usize,
    pub step_count: usize,
    pub required_record_count: usize,
    pub required_step_count: usize,
    pub side_effect_free: bool,
    pub backend_bound: bool,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
    pub next_lane: &'static str,
}

impl ShadowWriteAcceptanceReport {
    pub const fn from_plan(plan: ShadowWritePlanSummary) -> Self {
        let record_count = SHADOW_WRITE_RECORD_KINDS.len();
        let step_count = SHADOW_WRITE_STEPS.len();
        let has_required_records = record_count >= REQUIRED_SHADOW_WRITE_RECORD_COUNT;
        let has_required_steps = step_count >= REQUIRED_SHADOW_WRITE_STEP_COUNT;
        let side_effect_free = phase36n_is_side_effect_free();
        let backend_bound = plan.backend_bound;
        let storage_behavior_moved = plan.moves_storage_behavior;
        let display_behavior_moved = plan.moves_display_behavior;
        let input_behavior_moved = plan.moves_input_behavior;
        let power_behavior_moved = plan.moves_power_behavior;
        let accepted = has_required_records
            && has_required_steps
            && side_effect_free
            && !backend_bound
            && !storage_behavior_moved
            && !display_behavior_moved
            && !input_behavior_moved
            && !power_behavior_moved;

        Self {
            marker: PHASE_36O_STATE_IO_SHADOW_WRITE_ACCEPTANCE_MARKER,
            accepted_plan_marker: phase36n_marker(),
            decision: if accepted {
                ShadowWriteAcceptanceDecision::Accepted
            } else {
                ShadowWriteAcceptanceDecision::Rejected
            },
            record_count,
            step_count,
            required_record_count: REQUIRED_SHADOW_WRITE_RECORD_COUNT,
            required_step_count: REQUIRED_SHADOW_WRITE_STEP_COUNT,
            side_effect_free,
            backend_bound,
            storage_behavior_moved,
            display_behavior_moved,
            input_behavior_moved,
            power_behavior_moved,
            next_lane: "state-io-shadow-write-backend-design",
        }
    }

    pub const fn accepted() -> Self {
        Self::from_plan(STATE_IO_SHADOW_WRITE_PLAN_SUMMARY)
    }

    pub const fn is_accepted(self) -> bool {
        self.decision.is_accepted()
    }

    pub const fn decision_label(self) -> &'static str {
        self.decision.label()
    }
}

/// Compile-time acceptance report for the current shadow-write plan.
pub const STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT: ShadowWriteAcceptanceReport =
    ShadowWriteAcceptanceReport::accepted();

/// Return the accepted Phase 36O marker for boot/runtime status reporting.
pub const fn phase36o_marker() -> &'static str {
    PHASE_36O_STATE_IO_SHADOW_WRITE_ACCEPTANCE_MARKER
}

/// Return the Phase 36N marker that this acceptance layer covers.
pub const fn phase36o_accepted_plan_marker() -> &'static str {
    STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT.accepted_plan_marker
}

/// Return the side-effect-free acceptance report.
pub const fn phase36o_acceptance_report() -> ShadowWriteAcceptanceReport {
    STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT
}

/// Return whether the Phase 36N shadow-write plan is accepted.
pub const fn phase36o_is_accepted() -> bool {
    STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT.is_accepted()
}
