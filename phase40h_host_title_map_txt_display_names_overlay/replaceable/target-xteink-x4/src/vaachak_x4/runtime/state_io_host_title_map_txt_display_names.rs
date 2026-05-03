//! Phase 40H — FAT Long Filename / Host Title Map for TXT Display Names.
//!
//! TXT display names come from a host-generated `_X4/TITLEMAP.TSV` instead of
//! unsafe TXT body-title scanning.

#![allow(dead_code)]

pub const PHASE_40H_HOST_TITLE_MAP_MARKER: &str =
    "phase40h=x4-host-title-map-txt-display-names-ok";

pub const PHASE_40H_TITLE_MAP_FILE: &str = "TITLEMAP.TSV";
pub const PHASE_40H_LOADS_HOST_TITLE_MAP: bool = true;
pub const PHASE_40H_SCANS_TXT_BODY_TITLES: bool = false;
pub const PHASE_40H_SCANS_EPUB_EPU_METADATA: bool = true;
pub const PHASE_40H_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40H_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40H_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40H_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40H_TOUCHES_READER_PAGINATION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40hStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40hReport {
    pub status: Phase40hStatus,
    pub loads_host_title_map: bool,
    pub scans_txt_body_titles: bool,
    pub scans_epub_epu_metadata: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub touches_reader_pagination: bool,
}

impl Phase40hReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40hStatus::Accepted)
            && self.loads_host_title_map
            && !self.scans_txt_body_titles
            && self.scans_epub_epu_metadata
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_display_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40H_REPORT: Phase40hReport = Phase40hReport {
    status: Phase40hStatus::Accepted,
    loads_host_title_map: PHASE_40H_LOADS_HOST_TITLE_MAP,
    scans_txt_body_titles: PHASE_40H_SCANS_TXT_BODY_TITLES,
    scans_epub_epu_metadata: PHASE_40H_SCANS_EPUB_EPU_METADATA,
    changes_footer_labels: PHASE_40H_CHANGES_FOOTER_LABELS,
    changes_input_mapping: PHASE_40H_CHANGES_INPUT_MAPPING,
    touches_write_lane: PHASE_40H_TOUCHES_WRITE_LANE,
    touches_display_geometry: PHASE_40H_TOUCHES_DISPLAY_GEOMETRY,
    touches_reader_pagination: PHASE_40H_TOUCHES_READER_PAGINATION,
};

pub fn phase40h_report() -> Phase40hReport {
    PHASE_40H_REPORT
}

pub fn phase40h_marker() -> &'static str {
    PHASE_40H_HOST_TITLE_MAP_MARKER
}
