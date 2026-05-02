//! Phase 39K — Write Lane Cleanup and Acceptance Freeze Report.
//!
//! Acceptance wrapper for Phase 39K freeze metadata.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_write_lane_cleanup_acceptance_freeze::{
    phase39k_write_lane_freeze_report, Phase39kFreezeReason, Phase39kFreezeStatus,
    Phase39kNextLane,
};

pub const PHASE_39K_WRITE_LANE_CLEANUP_ACCEPTANCE_FREEZE_REPORT_MARKER: &str =
    "phase39k-acceptance=x4-write-lane-cleanup-freeze-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kAcceptanceReason {
    FreezeAccepted,
    FreezeBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39kAcceptanceReport {
    pub status: Phase39kAcceptanceStatus,
    pub reason: Phase39kAcceptanceReason,
    pub freeze_reason: Phase39kFreezeReason,
    pub next_lane: Phase39kNextLane,
}

impl Phase39kAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39kAcceptanceStatus::Accepted)
    }
}

pub fn phase39k_acceptance_report() -> Phase39kAcceptanceReport {
    let freeze = phase39k_write_lane_freeze_report();
    let accepted = freeze.accepted();

    Phase39kAcceptanceReport {
        status: if accepted {
            Phase39kAcceptanceStatus::Accepted
        } else {
            Phase39kAcceptanceStatus::Rejected
        },
        reason: if matches!(freeze.status, Phase39kFreezeStatus::Frozen) {
            Phase39kAcceptanceReason::FreezeAccepted
        } else {
            Phase39kAcceptanceReason::FreezeBlocked
        },
        freeze_reason: freeze.reason,
        next_lane: freeze.next_lane,
    }
}

pub fn phase39k_acceptance_marker() -> &'static str {
    PHASE_39K_WRITE_LANE_CLEANUP_ACCEPTANCE_FREEZE_REPORT_MARKER
}
