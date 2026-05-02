//! Phase 39P — Post-Cleanup Runtime Surface Acceptance Report.
//!
//! Acceptance wrapper for the Phase 39P cleaned runtime surface.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_post_cleanup_runtime_surface_acceptance::{
    phase39p_runtime_surface_report, Phase39pNextLane, Phase39pSurfaceReason, Phase39pSurfaceStatus,
};

pub const PHASE_39P_POST_CLEANUP_RUNTIME_SURFACE_ACCEPTANCE_REPORT_MARKER: &str =
    "phase39p-acceptance=x4-post-cleanup-runtime-surface-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39pAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39pAcceptanceReason {
    RuntimeSurfaceAccepted,
    RuntimeSurfaceBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39pAcceptanceReport {
    pub status: Phase39pAcceptanceStatus,
    pub reason: Phase39pAcceptanceReason,
    pub surface_reason: Phase39pSurfaceReason,
    pub next_lane: Phase39pNextLane,
}

impl Phase39pAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39pAcceptanceStatus::Accepted)
    }
}

pub fn phase39p_acceptance_report() -> Phase39pAcceptanceReport {
    let report = phase39p_runtime_surface_report();
    let accepted = report.accepted();

    Phase39pAcceptanceReport {
        status: if accepted {
            Phase39pAcceptanceStatus::Accepted
        } else {
            Phase39pAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase39pSurfaceStatus::Accepted) {
            Phase39pAcceptanceReason::RuntimeSurfaceAccepted
        } else {
            Phase39pAcceptanceReason::RuntimeSurfaceBlocked
        },
        surface_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase39p_acceptance_marker() -> &'static str {
    PHASE_39P_POST_CLEANUP_RUNTIME_SURFACE_ACCEPTANCE_REPORT_MARKER
}
