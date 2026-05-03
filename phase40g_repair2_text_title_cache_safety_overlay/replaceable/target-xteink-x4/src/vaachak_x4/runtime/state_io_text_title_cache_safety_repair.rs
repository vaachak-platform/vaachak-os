//! Phase 40G Repair 2 — Text Title Cache Safety.
//!
//! Fixes the bad TXT display-title regression caused by scanning arbitrary body
//! text from `.txt` files.
//!
//! TXT/MD title extraction is strict:
//! - accept `Title: ...` metadata lines only
//! - do not save arbitrary body/license lines as titles
//! - rebuild `_X4/TITLES.BIN` after applying so bad cached entries disappear

#![allow(dead_code)]

pub const PHASE_40G_REPAIR2_MARKER: &str =
    "phase40g-repair2=x4-text-title-cache-safety-ok";

pub const PHASE_40G_REPAIR2_TEXT_TITLES_STRICT_TITLE_PREFIX_ONLY: bool = true;
pub const PHASE_40G_REPAIR2_REQUIRES_TITLE_CACHE_REBUILD: bool = true;
pub const PHASE_40G_REPAIR2_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40G_REPAIR2_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40G_REPAIR2_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40G_REPAIR2_TOUCHES_GEOMETRY: bool = false;
pub const PHASE_40G_REPAIR2_TOUCHES_READER_PAGINATION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gRepair2Status {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40gRepair2Report {
    pub status: Phase40gRepair2Status,
    pub strict_title_prefix_only: bool,
    pub requires_title_cache_rebuild: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_geometry: bool,
    pub touches_reader_pagination: bool,
}

impl Phase40gRepair2Report {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40gRepair2Status::Accepted)
            && self.strict_title_prefix_only
            && self.requires_title_cache_rebuild
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40G_REPAIR2_REPORT: Phase40gRepair2Report = Phase40gRepair2Report {
    status: Phase40gRepair2Status::Accepted,
    strict_title_prefix_only: PHASE_40G_REPAIR2_TEXT_TITLES_STRICT_TITLE_PREFIX_ONLY,
    requires_title_cache_rebuild: PHASE_40G_REPAIR2_REQUIRES_TITLE_CACHE_REBUILD,
    changes_footer_labels: PHASE_40G_REPAIR2_CHANGES_FOOTER_LABELS,
    changes_input_mapping: PHASE_40G_REPAIR2_CHANGES_INPUT_MAPPING,
    touches_write_lane: PHASE_40G_REPAIR2_TOUCHES_WRITE_LANE,
    touches_geometry: PHASE_40G_REPAIR2_TOUCHES_GEOMETRY,
    touches_reader_pagination: PHASE_40G_REPAIR2_TOUCHES_READER_PAGINATION,
};

pub fn phase40g_repair2_report() -> Phase40gRepair2Report {
    PHASE_40G_REPAIR2_REPORT
}

pub fn phase40g_repair2_marker() -> &'static str {
    PHASE_40G_REPAIR2_MARKER
}
