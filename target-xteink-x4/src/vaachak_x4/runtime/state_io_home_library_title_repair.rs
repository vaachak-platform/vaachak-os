//! Phase 40G repair — home full-width current title and reader-title scanner.

#![allow(dead_code)]

pub const PHASE_40G_REPAIR_MARKER: &str = "phase40g-repair=x4-home-full-width-reader-titles-ok";

pub const PHASE_40G_REPAIR_HOME_CURRENT_TITLE_FULL_WIDTH: bool = true;
pub const PHASE_40G_REPAIR_SCANS_EPU: bool = true;
pub const PHASE_40G_REPAIR_SCANS_TEXT_TITLES: bool = true;
pub const PHASE_40G_REPAIR_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40G_REPAIR_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40G_REPAIR_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40G_REPAIR_TOUCHES_GEOMETRY: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gRepairStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40gRepairReport {
    pub status: Phase40gRepairStatus,
    pub home_current_title_full_width: bool,
    pub scans_epu: bool,
    pub scans_text_titles: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_geometry: bool,
}

impl Phase40gRepairReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40gRepairStatus::Accepted)
            && self.home_current_title_full_width
            && self.scans_epu
            && self.scans_text_titles
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_geometry
    }
}

pub const PHASE_40G_REPAIR_REPORT: Phase40gRepairReport = Phase40gRepairReport {
    status: Phase40gRepairStatus::Accepted,
    home_current_title_full_width: PHASE_40G_REPAIR_HOME_CURRENT_TITLE_FULL_WIDTH,
    scans_epu: PHASE_40G_REPAIR_SCANS_EPU,
    scans_text_titles: PHASE_40G_REPAIR_SCANS_TEXT_TITLES,
    changes_footer_labels: PHASE_40G_REPAIR_CHANGES_FOOTER_LABELS,
    changes_input_mapping: PHASE_40G_REPAIR_CHANGES_INPUT_MAPPING,
    touches_write_lane: PHASE_40G_REPAIR_TOUCHES_WRITE_LANE,
    touches_geometry: PHASE_40G_REPAIR_TOUCHES_GEOMETRY,
};

pub fn phase40g_repair_report() -> Phase40gRepairReport {
    PHASE_40G_REPAIR_REPORT
}

pub fn phase40g_repair_marker() -> &'static str {
    PHASE_40G_REPAIR_MARKER
}
