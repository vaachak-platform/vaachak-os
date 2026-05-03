//! Phase 40H Repair 1 acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_seed_txt_titlemap_into_titles_bin_repair::{
    Phase40hRepair1Status, phase40h_repair1_report,
};

pub const PHASE_40H_REPAIR1_ACCEPTANCE_MARKER: &str =
    "phase40h-repair1-acceptance=x4-seed-txt-titlemap-into-titles-bin-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40hRepair1AcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40h_repair1_acceptance_status() -> Phase40hRepair1AcceptanceStatus {
    let report = phase40h_repair1_report();
    if report.accepted() && matches!(report.status, Phase40hRepair1Status::Accepted) {
        Phase40hRepair1AcceptanceStatus::Accepted
    } else {
        Phase40hRepair1AcceptanceStatus::Rejected
    }
}

pub fn phase40h_repair1_acceptance_marker() -> &'static str {
    PHASE_40H_REPAIR1_ACCEPTANCE_MARKER
}
