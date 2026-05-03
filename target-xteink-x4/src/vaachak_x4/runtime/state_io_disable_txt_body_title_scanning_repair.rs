//! Phase 40G Repair 3 — Disable TXT Body-Title Scanning.
//!
//! The TXT scanner still produced unsafe display titles on device. This repair
//! disables TXT/MD body-title scanning entirely while keeping EPUB/EPU metadata
//! title scanning enabled.
//!
//! Result:
//! - EPUB/EPU titles continue to use metadata/cache.
//! - TXT/MD entries no longer get body/license text cached as titles.
//! - Existing bad title cache must be moved away before retesting.
//!
//! Full TXT long-name support should be handled by a separate FAT LFN/title-map
//! lane, not by guessing from file body text.

#![allow(dead_code)]

pub const PHASE_40G_REPAIR3_MARKER: &str = "phase40g-repair3=x4-disable-txt-body-title-scanning-ok";

pub const PHASE_40G_REPAIR3_SCANS_EPUB_EPU: bool = true;
pub const PHASE_40G_REPAIR3_SCANS_TXT_MD_BODY_TITLES: bool = false;
pub const PHASE_40G_REPAIR3_REQUIRES_TITLE_CACHE_REBUILD: bool = true;
pub const PHASE_40G_REPAIR3_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40G_REPAIR3_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40G_REPAIR3_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40G_REPAIR3_TOUCHES_GEOMETRY: bool = false;
pub const PHASE_40G_REPAIR3_TOUCHES_READER_PAGINATION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gRepair3Status {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40gRepair3Report {
    pub status: Phase40gRepair3Status,
    pub scans_epub_epu: bool,
    pub scans_txt_md_body_titles: bool,
    pub requires_title_cache_rebuild: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_geometry: bool,
    pub touches_reader_pagination: bool,
}

impl Phase40gRepair3Report {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40gRepair3Status::Accepted)
            && self.scans_epub_epu
            && !self.scans_txt_md_body_titles
            && self.requires_title_cache_rebuild
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40G_REPAIR3_REPORT: Phase40gRepair3Report = Phase40gRepair3Report {
    status: Phase40gRepair3Status::Accepted,
    scans_epub_epu: PHASE_40G_REPAIR3_SCANS_EPUB_EPU,
    scans_txt_md_body_titles: PHASE_40G_REPAIR3_SCANS_TXT_MD_BODY_TITLES,
    requires_title_cache_rebuild: PHASE_40G_REPAIR3_REQUIRES_TITLE_CACHE_REBUILD,
    changes_footer_labels: PHASE_40G_REPAIR3_CHANGES_FOOTER_LABELS,
    changes_input_mapping: PHASE_40G_REPAIR3_CHANGES_INPUT_MAPPING,
    touches_write_lane: PHASE_40G_REPAIR3_TOUCHES_WRITE_LANE,
    touches_geometry: PHASE_40G_REPAIR3_TOUCHES_GEOMETRY,
    touches_reader_pagination: PHASE_40G_REPAIR3_TOUCHES_READER_PAGINATION,
};

pub fn phase40g_repair3_report() -> Phase40gRepair3Report {
    PHASE_40G_REPAIR3_REPORT
}

pub fn phase40g_repair3_marker() -> &'static str {
    PHASE_40G_REPAIR3_MARKER
}
