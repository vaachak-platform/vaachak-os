//! Phase 40G — Home Full-Width Current Title and Library Long-Name/Title Patch.

#![allow(dead_code)]

pub const PHASE_40G_HOME_LIBRARY_TITLE_PATCH_MARKER: &str =
    "phase40g=x4-home-full-width-library-title-patch-ok";

pub const PHASE_40G_CHANGES_HOME_TITLE_LAYOUT: bool = true;
pub const PHASE_40G_CHANGES_LIBRARY_TITLE_RESOLUTION: bool = true;
pub const PHASE_40G_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40G_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40G_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40G_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40G_TOUCHES_READER_PAGINATION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gPatchStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gPatchReason {
    HomeAndLibraryTitlesFixed,
    ProtectedSurfaceTouched,
    DeviceConfirmationMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40gNextLane {
    DeviceRegression,
    RepairTitlePatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40gHomeLibraryTitlePatchReport {
    pub status: Phase40gPatchStatus,
    pub reason: Phase40gPatchReason,
    pub changes_home_title_layout: bool,
    pub changes_library_title_resolution: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub touches_reader_pagination: bool,
    pub next_lane: Phase40gNextLane,
}

impl Phase40gHomeLibraryTitlePatchReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40gPatchStatus::Accepted)
            && self.changes_home_title_layout
            && self.changes_library_title_resolution
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_display_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40G_HOME_LIBRARY_TITLE_PATCH_REPORT: Phase40gHomeLibraryTitlePatchReport =
    Phase40gHomeLibraryTitlePatchReport {
        status: Phase40gPatchStatus::Accepted,
        reason: Phase40gPatchReason::HomeAndLibraryTitlesFixed,
        changes_home_title_layout: PHASE_40G_CHANGES_HOME_TITLE_LAYOUT,
        changes_library_title_resolution: PHASE_40G_CHANGES_LIBRARY_TITLE_RESOLUTION,
        changes_footer_labels: PHASE_40G_CHANGES_FOOTER_LABELS,
        changes_input_mapping: PHASE_40G_CHANGES_INPUT_MAPPING,
        touches_write_lane: PHASE_40G_TOUCHES_WRITE_LANE,
        touches_display_geometry: PHASE_40G_TOUCHES_DISPLAY_GEOMETRY,
        touches_reader_pagination: PHASE_40G_TOUCHES_READER_PAGINATION,
        next_lane: Phase40gNextLane::DeviceRegression,
    };

pub fn phase40g_home_library_title_patch_report() -> Phase40gHomeLibraryTitlePatchReport {
    PHASE_40G_HOME_LIBRARY_TITLE_PATCH_REPORT
}

pub fn phase40g_marker() -> &'static str {
    PHASE_40G_HOME_LIBRARY_TITLE_PATCH_MARKER
}
