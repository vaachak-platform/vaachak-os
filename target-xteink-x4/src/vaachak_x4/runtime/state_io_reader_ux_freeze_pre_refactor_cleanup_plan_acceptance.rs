//! Phase 40J reader UX freeze acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_reader_ux_freeze_pre_refactor_cleanup_plan::{
    Phase40jStatus, phase40j_reader_ux_freeze_report,
};

pub const PHASE_40J_READER_UX_FREEZE_PRE_REFACTOR_PLAN_ACCEPTANCE_MARKER: &str =
    "phase40j-acceptance=x4-reader-ux-freeze-pre-refactor-cleanup-plan-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40jAcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40j_acceptance_status() -> Phase40jAcceptanceStatus {
    let report = phase40j_reader_ux_freeze_report();
    if report.accepted() && matches!(report.status, Phase40jStatus::Accepted) {
        Phase40jAcceptanceStatus::Accepted
    } else {
        Phase40jAcceptanceStatus::Rejected
    }
}

pub fn phase40j_acceptance_marker() -> &'static str {
    PHASE_40J_READER_UX_FREEZE_PRE_REFACTOR_PLAN_ACCEPTANCE_MARKER
}
