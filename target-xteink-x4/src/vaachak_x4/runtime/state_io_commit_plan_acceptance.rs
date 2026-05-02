//! Phase 36M — State I/O Commit Plan Acceptance.
//!
//! This module is intentionally metadata-only. It accepts the Phase 36L
//! commit-plan contract as a side-effect-free plan before any real SD/FAT
//! backend is introduced. It does not perform filesystem, SD/FAT, SPI,
//! display, input, power, or boot-flow operations.

use crate::vaachak_x4::runtime::state_io_backend_commit_plan::{
    PHASE_36L_STATE_IO_COMMIT_PLAN_MARKER, state_io_commit_plan_is_side_effect_free,
};

/// Phase marker emitted by the Phase 36M overlay tooling.
pub const PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE_MARKER: &str =
    "phase36m=x4-state-io-commit-plan-acceptance-ok";

/// Accepted prior phase marker captured as metadata only.
pub const PHASE_36M_ACCEPTED_COMMIT_PLAN_MARKER: &str = PHASE_36L_STATE_IO_COMMIT_PLAN_MARKER;

/// Acceptance outcome for one commit-plan safety criterion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoCommitPlanAcceptanceCriterion {
    pub name: &'static str,
    pub accepted: bool,
    pub note: &'static str,
}

/// Compile-time acceptance report for the planned state I/O commit sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoCommitPlanAcceptanceReport {
    pub marker: &'static str,
    pub accepted_commit_plan_marker: &'static str,
    pub accepted: bool,
    pub behavior_moved: bool,
    pub hardware_required: bool,
    pub next_lane: &'static str,
    pub criteria: [StateIoCommitPlanAcceptanceCriterion; 5],
}

/// Phase 36M acceptance report. All criteria are metadata-only assertions.
pub const PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE: StateIoCommitPlanAcceptanceReport =
    StateIoCommitPlanAcceptanceReport {
        marker: PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE_MARKER,
        accepted_commit_plan_marker: PHASE_36M_ACCEPTED_COMMIT_PLAN_MARKER,
        accepted: true,
        behavior_moved: false,
        hardware_required: false,
        next_lane: "state-io-backend-write-shadow-plan",
        criteria: [
            StateIoCommitPlanAcceptanceCriterion {
                name: "side-effect-free-plan",
                accepted: true,
                note: "Phase 36L remains a plan and performs no writes",
            },
            StateIoCommitPlanAcceptanceCriterion {
                name: "typed-record-scope-bounded",
                accepted: true,
                note: "Scope remains .PRG, .THM, .MTA, .BKM, and BMIDX.TXT",
            },
            StateIoCommitPlanAcceptanceCriterion {
                name: "no-hardware-path-moved",
                accepted: true,
                note: "SD/FAT/SPI/display/input/power behavior remains unchanged",
            },
            StateIoCommitPlanAcceptanceCriterion {
                name: "runtime-layout-preserved",
                accepted: true,
                note: "runtime.rs remains authoritative; runtime/mod.rs is not created",
            },
            StateIoCommitPlanAcceptanceCriterion {
                name: "ready-for-shadow-write-design",
                accepted: true,
                note: "Next phase may define a shadow-write plan before any real backend call",
            },
        ],
    };

/// Return the Phase 36M metadata-only acceptance report.
pub const fn state_io_commit_plan_acceptance_report() -> &'static StateIoCommitPlanAcceptanceReport
{
    &PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE
}

/// Compact status line for boot/runtime diagnostic surfaces.
pub const fn state_io_commit_plan_acceptance_status() -> &'static str {
    PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE_MARKER
}

/// True only when the accepted commit plan and this acceptance report are both
/// side-effect free.
pub const fn state_io_commit_plan_acceptance_is_safe() -> bool {
    state_io_commit_plan_is_side_effect_free()
        && PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE.accepted
        && !PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE.behavior_moved
        && !PHASE_36M_STATE_IO_COMMIT_PLAN_ACCEPTANCE.hardware_required
}
