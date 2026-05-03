//! Phase 40H host-title-map acceptance.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_host_title_map_txt_display_names::{
    phase40h_report, Phase40hStatus,
};

pub const PHASE_40H_HOST_TITLE_MAP_ACCEPTANCE_MARKER: &str =
    "phase40h-acceptance=x4-host-title-map-txt-display-names-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40hAcceptanceStatus {
    Accepted,
    Rejected,
}

pub fn phase40h_acceptance_status() -> Phase40hAcceptanceStatus {
    let report = phase40h_report();
    if report.accepted() && matches!(report.status, Phase40hStatus::Accepted) {
        Phase40hAcceptanceStatus::Accepted
    } else {
        Phase40hAcceptanceStatus::Rejected
    }
}

pub fn phase40h_acceptance_marker() -> &'static str {
    PHASE_40H_HOST_TITLE_MAP_ACCEPTANCE_MARKER
}
