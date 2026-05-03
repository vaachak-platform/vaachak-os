//! Phase 40G Repair 2 acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_text_title_cache_safety_repair::{
    Phase40gRepair2Status, phase40g_repair2_report,
};

pub const PHASE_40G_REPAIR2_ACCEPTANCE_MARKER: &str =
    "phase40g-repair2-acceptance=x4-text-title-cache-safety-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gRepair2AcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40g_repair2_acceptance_status() -> Phase40gRepair2AcceptanceStatus {
    let report = phase40g_repair2_report();
    if report.accepted() && matches!(report.status, Phase40gRepair2Status::Accepted) {
        Phase40gRepair2AcceptanceStatus::Accepted
    } else {
        Phase40gRepair2AcceptanceStatus::Rejected
    }
}

pub fn phase40g_repair2_acceptance_marker() -> &'static str {
    PHASE_40G_REPAIR2_ACCEPTANCE_MARKER
}
