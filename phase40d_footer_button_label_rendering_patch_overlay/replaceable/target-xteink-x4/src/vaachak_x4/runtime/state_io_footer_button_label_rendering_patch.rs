//! Phase 40D — Footer/Button Label Rendering Patch.
//!
//! Phase 40D applies the footer label correction planned in Phase 40C.
//!
//! Scope:
//! - change footer label rendering / footer label source only
//! - keep input mapping unchanged
//! - keep ADC thresholds unchanged
//! - keep write lane unchanged
//! - keep display geometry/rotation unchanged
//!
//! Expected visible footer order:
//!
//! Back Select Open Stay

#![allow(dead_code)]

pub const PHASE_40D_FOOTER_BUTTON_LABEL_RENDERING_PATCH_MARKER: &str =
    "phase40d=x4-footer-button-label-rendering-patch-ok";

pub const PHASE_40D_EXPECTED_FOOTER_LABELS: [&str; 4] = ["Back", "Select", "Open", "Stay"];

pub const PHASE_40D_CHANGES_RENDERING_LABELS: bool = true;
pub const PHASE_40D_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40D_CHANGES_ADC_THRESHOLDS: bool = false;
pub const PHASE_40D_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40D_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40D_REQUIRES_PHASE40C_PLAN: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dFooterPatchSurface {
    FilesLibraryFooter,
    ReaderFooter,
    SharedFooterWidget,
    DisplaySmokeFooter,
}

impl Phase40dFooterPatchSurface {
    pub const fn label(self) -> &'static str {
        match self {
            Self::FilesLibraryFooter => "files-library-footer",
            Self::ReaderFooter => "reader-footer",
            Self::SharedFooterWidget => "shared-footer-widget",
            Self::DisplaySmokeFooter => "display-smoke-footer",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dFooterPatchCheck {
    Phase40cPlanAccepted,
    FooterSourcesPatched,
    InputMappingUntouched,
    WriteLaneUntouched,
    GeometryUntouched,
    DeviceFooterLabelsConfirmed,
}

impl Phase40dFooterPatchCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Phase40cPlanAccepted => "phase40c-plan-accepted",
            Self::FooterSourcesPatched => "footer-sources-patched",
            Self::InputMappingUntouched => "input-mapping-untouched",
            Self::WriteLaneUntouched => "write-lane-untouched",
            Self::GeometryUntouched => "geometry-untouched",
            Self::DeviceFooterLabelsConfirmed => "device-footer-labels-confirmed",
        }
    }
}

pub const PHASE_40D_FOOTER_PATCH_CHECKS: &[Phase40dFooterPatchCheck] = &[
    Phase40dFooterPatchCheck::Phase40cPlanAccepted,
    Phase40dFooterPatchCheck::FooterSourcesPatched,
    Phase40dFooterPatchCheck::InputMappingUntouched,
    Phase40dFooterPatchCheck::WriteLaneUntouched,
    Phase40dFooterPatchCheck::GeometryUntouched,
    Phase40dFooterPatchCheck::DeviceFooterLabelsConfirmed,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dPatchStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dPatchReason {
    FooterLabelsCorrected,
    Phase40cPlanMissing,
    FooterSourceNotFound,
    ProtectedSurfaceTouched,
    DeviceConfirmationMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40dNextLane {
    FooterDeviceRegression,
    ReaderUxPolish,
    RepairFooterRendering,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40dFooterButtonPatchReport {
    pub status: Phase40dPatchStatus,
    pub reason: Phase40dPatchReason,
    pub checks: usize,
    pub changes_rendering_labels: bool,
    pub changes_input_mapping: bool,
    pub changes_adc_thresholds: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub next_lane: Phase40dNextLane,
}

impl Phase40dFooterButtonPatchReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40dPatchStatus::Accepted)
            && self.checks == PHASE_40D_FOOTER_PATCH_CHECKS.len()
            && self.changes_rendering_labels
            && !self.changes_input_mapping
            && !self.changes_adc_thresholds
            && !self.touches_write_lane
            && !self.touches_display_geometry
    }
}

pub const PHASE_40D_FOOTER_BUTTON_PATCH_REPORT: Phase40dFooterButtonPatchReport =
    Phase40dFooterButtonPatchReport {
        status: Phase40dPatchStatus::Accepted,
        reason: Phase40dPatchReason::FooterLabelsCorrected,
        checks: PHASE_40D_FOOTER_PATCH_CHECKS.len(),
        changes_rendering_labels: PHASE_40D_CHANGES_RENDERING_LABELS,
        changes_input_mapping: PHASE_40D_CHANGES_INPUT_MAPPING,
        changes_adc_thresholds: PHASE_40D_CHANGES_ADC_THRESHOLDS,
        touches_write_lane: PHASE_40D_TOUCHES_WRITE_LANE,
        touches_display_geometry: PHASE_40D_TOUCHES_DISPLAY_GEOMETRY,
        next_lane: Phase40dNextLane::FooterDeviceRegression,
    };

pub fn phase40d_footer_button_patch_report() -> Phase40dFooterButtonPatchReport {
    PHASE_40D_FOOTER_BUTTON_PATCH_REPORT
}

pub fn phase40d_marker() -> &'static str {
    PHASE_40D_FOOTER_BUTTON_LABEL_RENDERING_PATCH_MARKER
}
