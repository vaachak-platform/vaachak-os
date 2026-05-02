//! Phase 40D — Footer/Button Label Rendering Patch Acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_footer_button_label_rendering_patch::{
    Phase40dNextLane, Phase40dPatchReason, Phase40dPatchStatus, phase40d_footer_button_patch_report,
};

pub const PHASE_40D_FOOTER_BUTTON_LABEL_RENDERING_PATCH_ACCEPTANCE_MARKER: &str =
    "phase40d-acceptance=x4-footer-button-label-rendering-patch-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dAcceptanceReason {
    RenderingPatchAccepted,
    RenderingPatchBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40dAcceptanceReport {
    pub status: Phase40dAcceptanceStatus,
    pub reason: Phase40dAcceptanceReason,
    pub patch_reason: Phase40dPatchReason,
    pub next_lane: Phase40dNextLane,
}

impl Phase40dAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40dAcceptanceStatus::Accepted)
    }
}

pub fn phase40d_acceptance_report() -> Phase40dAcceptanceReport {
    let report = phase40d_footer_button_patch_report();
    let accepted = report.accepted();

    Phase40dAcceptanceReport {
        status: if accepted {
            Phase40dAcceptanceStatus::Accepted
        } else {
            Phase40dAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40dPatchStatus::Accepted) {
            Phase40dAcceptanceReason::RenderingPatchAccepted
        } else {
            Phase40dAcceptanceReason::RenderingPatchBlocked
        },
        patch_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase40d_acceptance_marker() -> &'static str {
    PHASE_40D_FOOTER_BUTTON_LABEL_RENDERING_PATCH_ACCEPTANCE_MARKER
}
