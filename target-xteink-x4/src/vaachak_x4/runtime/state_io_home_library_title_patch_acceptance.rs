//! Phase 40G — Home/Library Title Patch Acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_home_library_title_patch::{
    Phase40gNextLane, Phase40gPatchReason, Phase40gPatchStatus,
    phase40g_home_library_title_patch_report,
};

pub const PHASE_40G_HOME_LIBRARY_TITLE_PATCH_ACCEPTANCE_MARKER: &str =
    "phase40g-acceptance=x4-home-library-title-patch-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gAcceptanceReason {
    PatchAccepted,
    PatchBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40gAcceptanceReport {
    pub status: Phase40gAcceptanceStatus,
    pub reason: Phase40gAcceptanceReason,
    pub patch_reason: Phase40gPatchReason,
    pub next_lane: Phase40gNextLane,
}

impl Phase40gAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40gAcceptanceStatus::Accepted)
    }
}

pub fn phase40g_acceptance_report() -> Phase40gAcceptanceReport {
    let report = phase40g_home_library_title_patch_report();
    Phase40gAcceptanceReport {
        status: if report.accepted() {
            Phase40gAcceptanceStatus::Accepted
        } else {
            Phase40gAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40gPatchStatus::Accepted) {
            Phase40gAcceptanceReason::PatchAccepted
        } else {
            Phase40gAcceptanceReason::PatchBlocked
        },
        patch_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase40g_acceptance_marker() -> &'static str {
    PHASE_40G_HOME_LIBRARY_TITLE_PATCH_ACCEPTANCE_MARKER
}
