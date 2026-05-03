//! Phase 40I title-cache workflow freeze acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_title_cache_workflow_freeze::{
    Phase40iFreezeStatus, phase40i_title_cache_workflow_freeze_report,
};

pub const PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_ACCEPTANCE_MARKER: &str =
    "phase40i-acceptance=x4-title-cache-workflow-freeze-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40iAcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40i_acceptance_status() -> Phase40iAcceptanceStatus {
    let report = phase40i_title_cache_workflow_freeze_report();
    if report.accepted() && matches!(report.status, Phase40iFreezeStatus::Accepted) {
        Phase40iAcceptanceStatus::Accepted
    } else {
        Phase40iAcceptanceStatus::Rejected
    }
}

pub fn phase40i_acceptance_marker() -> &'static str {
    PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_ACCEPTANCE_MARKER
}
