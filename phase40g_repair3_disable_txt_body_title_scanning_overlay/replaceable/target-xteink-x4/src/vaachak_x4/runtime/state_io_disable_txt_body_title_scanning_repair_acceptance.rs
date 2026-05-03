//! Phase 40G Repair 3 acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_disable_txt_body_title_scanning_repair::{
    phase40g_repair3_report, Phase40gRepair3Status,
};

pub const PHASE_40G_REPAIR3_ACCEPTANCE_MARKER: &str =
    "phase40g-repair3-acceptance=x4-disable-txt-body-title-scanning-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gRepair3AcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40g_repair3_acceptance_status() -> Phase40gRepair3AcceptanceStatus {
    let report = phase40g_repair3_report();
    if report.accepted() && matches!(report.status, Phase40gRepair3Status::Accepted) {
        Phase40gRepair3AcceptanceStatus::Accepted
    } else {
        Phase40gRepair3AcceptanceStatus::Rejected
    }
}

pub fn phase40g_repair3_acceptance_marker() -> &'static str {
    PHASE_40G_REPAIR3_ACCEPTANCE_MARKER
}
