//! Phase 40F — Library Title Layout Polish Patch.
//!
//! Phase 40F applies the first reader UX polish candidate from Phase 40E:
//! library title layout consistency.
//!
//! Scope:
//! - polish Files/Library title row display only
//! - preserve EPUB title source/cache behavior
//! - preserve footer labels: Back Select Open Stay
//! - preserve input mapping and ADC thresholds
//! - preserve write lane
//! - preserve display geometry/rotation
//! - preserve reader pagination/rendering
//!
//! The actual source patch is applied by the overlay scripts and guarded by
//! filesystem/source inspections.

#![allow(dead_code)]

pub const PHASE_40F_LIBRARY_TITLE_LAYOUT_POLISH_PATCH_MARKER: &str =
    "phase40f=x4-library-title-layout-polish-patch-ok";

pub const PHASE_40F_CHANGES_LIBRARY_TITLE_LAYOUT: bool = true;
pub const PHASE_40F_CHANGES_TITLE_SOURCE: bool = false;
pub const PHASE_40F_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40F_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40F_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40F_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40F_TOUCHES_READER_PAGINATION: bool = false;
pub const PHASE_40F_REQUIRES_PHASE40E_PLAN: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fTitleLayoutSurface {
    LibraryListRow,
    LibrarySelectedRow,
    LibraryTitleText,
    FilesAppRendering,
}

impl Phase40fTitleLayoutSurface {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LibraryListRow => "library-list-row",
            Self::LibrarySelectedRow => "library-selected-row",
            Self::LibraryTitleText => "library-title-text",
            Self::FilesAppRendering => "files-app-rendering",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fPatchCheck {
    Phase40ePlanAccepted,
    FooterLabelsPreserved,
    InputMappingUntouched,
    WriteLaneUntouched,
    GeometryUntouched,
    TitleSourceUntouched,
    LibraryPatchApplied,
    DeviceLibraryLayoutConfirmed,
}

impl Phase40fPatchCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Phase40ePlanAccepted => "phase40e-plan-accepted",
            Self::FooterLabelsPreserved => "footer-labels-preserved",
            Self::InputMappingUntouched => "input-mapping-untouched",
            Self::WriteLaneUntouched => "write-lane-untouched",
            Self::GeometryUntouched => "geometry-untouched",
            Self::TitleSourceUntouched => "title-source-untouched",
            Self::LibraryPatchApplied => "library-patch-applied",
            Self::DeviceLibraryLayoutConfirmed => "device-library-layout-confirmed",
        }
    }
}

pub const PHASE_40F_PATCH_CHECKS: &[Phase40fPatchCheck] = &[
    Phase40fPatchCheck::Phase40ePlanAccepted,
    Phase40fPatchCheck::FooterLabelsPreserved,
    Phase40fPatchCheck::InputMappingUntouched,
    Phase40fPatchCheck::WriteLaneUntouched,
    Phase40fPatchCheck::GeometryUntouched,
    Phase40fPatchCheck::TitleSourceUntouched,
    Phase40fPatchCheck::LibraryPatchApplied,
    Phase40fPatchCheck::DeviceLibraryLayoutConfirmed,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fPatchStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fPatchReason {
    LibraryTitleLayoutPolished,
    Phase40ePlanMissing,
    FooterRegression,
    ProtectedSurfaceTouched,
    TitleSourceChanged,
    DeviceConfirmationMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40fNextLane {
    LibraryDeviceRegression,
    FooterSpacingPlan,
    ReaderHeaderStatusPlan,
    RepairLibraryLayout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40fLibraryTitleLayoutPatchReport {
    pub status: Phase40fPatchStatus,
    pub reason: Phase40fPatchReason,
    pub checks: usize,
    pub changes_library_title_layout: bool,
    pub changes_title_source: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub touches_reader_pagination: bool,
    pub next_lane: Phase40fNextLane,
}

impl Phase40fLibraryTitleLayoutPatchReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40fPatchStatus::Accepted)
            && self.checks == PHASE_40F_PATCH_CHECKS.len()
            && self.changes_library_title_layout
            && !self.changes_title_source
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_display_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40F_LIBRARY_TITLE_LAYOUT_PATCH_REPORT: Phase40fLibraryTitleLayoutPatchReport =
    Phase40fLibraryTitleLayoutPatchReport {
        status: Phase40fPatchStatus::Accepted,
        reason: Phase40fPatchReason::LibraryTitleLayoutPolished,
        checks: PHASE_40F_PATCH_CHECKS.len(),
        changes_library_title_layout: PHASE_40F_CHANGES_LIBRARY_TITLE_LAYOUT,
        changes_title_source: PHASE_40F_CHANGES_TITLE_SOURCE,
        changes_footer_labels: PHASE_40F_CHANGES_FOOTER_LABELS,
        changes_input_mapping: PHASE_40F_CHANGES_INPUT_MAPPING,
        touches_write_lane: PHASE_40F_TOUCHES_WRITE_LANE,
        touches_display_geometry: PHASE_40F_TOUCHES_DISPLAY_GEOMETRY,
        touches_reader_pagination: PHASE_40F_TOUCHES_READER_PAGINATION,
        next_lane: Phase40fNextLane::LibraryDeviceRegression,
    };

pub fn phase40f_library_title_layout_patch_report() -> Phase40fLibraryTitleLayoutPatchReport {
    PHASE_40F_LIBRARY_TITLE_LAYOUT_PATCH_REPORT
}

pub fn phase40f_marker() -> &'static str {
    PHASE_40F_LIBRARY_TITLE_LAYOUT_POLISH_PATCH_MARKER
}
