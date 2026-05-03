//! Phase 40G repair acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_home_library_title_repair::{
    phase40g_repair_report, Phase40gRepairStatus,
};

pub const PHASE_40G_REPAIR_ACCEPTANCE_MARKER: &str =
    "phase40g-repair-acceptance=x4-home-full-width-reader-titles-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gRepairAcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40g_repair_acceptance_status() -> Phase40gRepairAcceptanceStatus {
    let report = phase40g_repair_report();
    if report.accepted() && matches!(report.status, Phase40gRepairStatus::Accepted) {
        Phase40gRepairAcceptanceStatus::Accepted
    } else {
        Phase40gRepairAcceptanceStatus::Rejected
    }
}

pub fn phase40g_repair_acceptance_marker() -> &'static str {
    PHASE_40G_REPAIR_ACCEPTANCE_MARKER
}
