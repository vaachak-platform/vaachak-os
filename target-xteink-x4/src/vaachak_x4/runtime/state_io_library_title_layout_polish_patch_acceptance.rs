//! Phase 40F — Library Title Layout Polish Patch Acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_library_title_layout_polish_patch::{
    Phase40fNextLane, Phase40fPatchReason, Phase40fPatchStatus,
    phase40f_library_title_layout_patch_report,
};

pub const PHASE_40F_LIBRARY_TITLE_LAYOUT_POLISH_PATCH_ACCEPTANCE_MARKER: &str =
    "phase40f-acceptance=x4-library-title-layout-polish-patch-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fAcceptanceReason {
    LibraryPatchAccepted,
    LibraryPatchBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40fAcceptanceReport {
    pub status: Phase40fAcceptanceStatus,
    pub reason: Phase40fAcceptanceReason,
    pub patch_reason: Phase40fPatchReason,
    pub next_lane: Phase40fNextLane,
}

impl Phase40fAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40fAcceptanceStatus::Accepted)
    }
}

pub fn phase40f_acceptance_report() -> Phase40fAcceptanceReport {
    let report = phase40f_library_title_layout_patch_report();
    let accepted = report.accepted();

    Phase40fAcceptanceReport {
        status: if accepted {
            Phase40fAcceptanceStatus::Accepted
        } else {
            Phase40fAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40fPatchStatus::Accepted) {
            Phase40fAcceptanceReason::LibraryPatchAccepted
        } else {
            Phase40fAcceptanceReason::LibraryPatchBlocked
        },
        patch_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase40f_acceptance_marker() -> &'static str {
    PHASE_40F_LIBRARY_TITLE_LAYOUT_POLISH_PATCH_ACCEPTANCE_MARKER
}
