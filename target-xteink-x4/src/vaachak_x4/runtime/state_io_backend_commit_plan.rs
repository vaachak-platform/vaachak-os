//! Phase 36L — State I/O Backend Commit Plan.
//!
//! This module is intentionally metadata-only. It documents the future safe
//! commit sequence for typed Vaachak state records, but it does not perform
//! filesystem, SD/FAT, SPI, display, input, or power operations.

/// Phase marker emitted by the Phase 36L overlay tooling.
pub const PHASE_36L_STATE_IO_COMMIT_PLAN_MARKER: &str = "phase36l=x4-state-io-commit-plan-ok";

/// Prior dry-run acceptance marker captured as metadata only.
pub const PHASE_36K_STATE_IO_DRY_RUN_ACCEPTANCE_MARKER: &str =
    "phase36k=x4-state-io-dry-run-acceptance-ok";

/// Typed state records covered by the future commit protocol.
pub const PHASE_36L_TYPED_STATE_RECORDS: [&str; 5] = [".PRG", ".THM", ".MTA", ".BKM", "BMIDX.TXT"];

/// One planned commit step. `implemented_now` must remain false in Phase 36L.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoCommitPlanStep {
    pub order: u8,
    pub name: &'static str,
    pub purpose: &'static str,
    pub implemented_now: bool,
}

/// Compile-time description of the future state I/O commit sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoCommitPlan {
    pub marker: &'static str,
    pub prior_acceptance_marker: &'static str,
    pub records: [&'static str; 5],
    pub behavior_moved: bool,
    pub hardware_required: bool,
    pub implementation_status: &'static str,
    pub steps: [StateIoCommitPlanStep; 8],
}

/// Planned safe sequence for the later real backend binding.
pub const PHASE_36L_STATE_IO_COMMIT_PLAN: StateIoCommitPlan = StateIoCommitPlan {
    marker: PHASE_36L_STATE_IO_COMMIT_PLAN_MARKER,
    prior_acceptance_marker: PHASE_36K_STATE_IO_DRY_RUN_ACCEPTANCE_MARKER,
    records: PHASE_36L_TYPED_STATE_RECORDS,
    behavior_moved: false,
    hardware_required: false,
    implementation_status: "metadata-only-plan",
    steps: [
        StateIoCommitPlanStep {
            order: 1,
            name: "resolve-record-path",
            purpose: "map a typed state record to its 8.3-safe X4 path",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 2,
            name: "read-current-record",
            purpose: "load prior state before replacing it",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 3,
            name: "encode-next-record",
            purpose: "serialize the new typed state payload",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 4,
            name: "stage-next-record",
            purpose: "prepare a separate staged payload before final replacement",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 5,
            name: "verify-staged-record",
            purpose: "confirm the staged payload matches the expected record shape",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 6,
            name: "replace-final-record",
            purpose: "make the verified payload the active state record",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 7,
            name: "refresh-index-record",
            purpose: "update BMIDX.TXT only after the primary record is accepted",
            implemented_now: false,
        },
        StateIoCommitPlanStep {
            order: 8,
            name: "report-commit-result",
            purpose: "return a deterministic status to the reader runtime",
            implemented_now: false,
        },
    ],
};

/// Return the Phase 36L metadata-only commit plan.
pub const fn state_io_commit_plan() -> &'static StateIoCommitPlan {
    &PHASE_36L_STATE_IO_COMMIT_PLAN
}

/// Compact status line for boot/runtime diagnostic surfaces.
pub const fn state_io_commit_plan_status() -> &'static str {
    PHASE_36L_STATE_IO_COMMIT_PLAN_MARKER
}

/// True only when this phase remains a side-effect-free plan.
pub const fn state_io_commit_plan_is_side_effect_free() -> bool {
    !PHASE_36L_STATE_IO_COMMIT_PLAN.behavior_moved
        && !PHASE_36L_STATE_IO_COMMIT_PLAN.hardware_required
}
