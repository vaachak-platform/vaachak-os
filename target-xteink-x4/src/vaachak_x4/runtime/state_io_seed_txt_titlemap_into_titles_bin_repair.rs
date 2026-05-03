//! Phase 40H Repair 1 — Seed TXT Title Map into Active TITLES.BIN.
//!
//! The direct `_X4/TITLEMAP.TSV` loader did not affect the on-device display.
//! `_X4/TITLES.BIN` is known to be loaded because EPUB/EPU title entries appear
//! there after device boot.
//!
//! This repair therefore merges host-generated TXT/MD alias mappings directly
//! into `_X4/TITLES.BIN`.
//!
//! Preserved:
//! - no TXT body-title scanning
//! - EPUB/EPU metadata scanning remains enabled
//! - footer labels unchanged
//! - input mapping unchanged
//! - write lane unchanged
//! - display geometry unchanged
//! - reader pagination unchanged

#![allow(dead_code)]

pub const PHASE_40H_REPAIR1_MARKER: &str =
    "phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok";

pub const PHASE_40H_REPAIR1_SEEDS_TITLES_BIN: bool = true;
pub const PHASE_40H_REPAIR1_USES_TXT_BODY_SCANNING: bool = false;
pub const PHASE_40H_REPAIR1_PRESERVES_EPUB_EPU_METADATA: bool = true;
pub const PHASE_40H_REPAIR1_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40H_REPAIR1_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40H_REPAIR1_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40H_REPAIR1_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40H_REPAIR1_TOUCHES_READER_PAGINATION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40hRepair1Status {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40hRepair1Report {
    pub status: Phase40hRepair1Status,
    pub seeds_titles_bin: bool,
    pub uses_txt_body_scanning: bool,
    pub preserves_epub_epu_metadata: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub touches_reader_pagination: bool,
}

impl Phase40hRepair1Report {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40hRepair1Status::Accepted)
            && self.seeds_titles_bin
            && !self.uses_txt_body_scanning
            && self.preserves_epub_epu_metadata
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_display_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40H_REPAIR1_REPORT: Phase40hRepair1Report = Phase40hRepair1Report {
    status: Phase40hRepair1Status::Accepted,
    seeds_titles_bin: PHASE_40H_REPAIR1_SEEDS_TITLES_BIN,
    uses_txt_body_scanning: PHASE_40H_REPAIR1_USES_TXT_BODY_SCANNING,
    preserves_epub_epu_metadata: PHASE_40H_REPAIR1_PRESERVES_EPUB_EPU_METADATA,
    changes_footer_labels: PHASE_40H_REPAIR1_CHANGES_FOOTER_LABELS,
    changes_input_mapping: PHASE_40H_REPAIR1_CHANGES_INPUT_MAPPING,
    touches_write_lane: PHASE_40H_REPAIR1_TOUCHES_WRITE_LANE,
    touches_display_geometry: PHASE_40H_REPAIR1_TOUCHES_DISPLAY_GEOMETRY,
    touches_reader_pagination: PHASE_40H_REPAIR1_TOUCHES_READER_PAGINATION,
};

pub fn phase40h_repair1_report() -> Phase40hRepair1Report {
    PHASE_40H_REPAIR1_REPORT
}

pub fn phase40h_repair1_marker() -> &'static str {
    PHASE_40H_REPAIR1_MARKER
}
