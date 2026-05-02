//! Phase 40C — Footer/Button Label Baseline and Fix Plan.
//!
//! Phase 40C is a plan-only phase after Phase 40B captured the Reader UX
//! regression baseline.
//!
//! It does not change rendering.
//! It does not change input mapping.
//! It does not change display/SPI/power behavior.
//! It does not change the write lane.
//!
//! Scope:
//! - inspect current footer rendering code
//! - inspect current button mapping label candidates
//! - capture expected labels per screen
//! - identify mismatch between label order and physical button behavior
//! - produce exact patch plan for footer label correction

#![allow(dead_code)]

pub const PHASE_40C_FOOTER_BUTTON_LABEL_BASELINE_FIX_PLAN_MARKER: &str =
    "phase40c=x4-footer-button-label-baseline-fix-plan-ok";

pub const PHASE_40C_PLAN_ONLY: bool = true;
pub const PHASE_40C_CHANGES_RENDERING_NOW: bool = false;
pub const PHASE_40C_CHANGES_INPUT_NOW: bool = false;
pub const PHASE_40C_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40C_TOUCHES_SD_FAT_SPI_POWER: bool = false;
pub const PHASE_40C_REQUIRES_PHASE40B_BASELINE: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cFooterSurface {
    Home,
    FilesLibrary,
    Reader,
    Dialog,
    Unknown,
}

impl Phase40cFooterSurface {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::FilesLibrary => "files-library",
            Self::Reader => "reader",
            Self::Dialog => "dialog",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cButtonSlot {
    LeftMost,
    LeftCenter,
    RightCenter,
    RightMost,
    MenuOrAux,
    Unknown,
}

impl Phase40cButtonSlot {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LeftMost => "left-most",
            Self::LeftCenter => "left-center",
            Self::RightCenter => "right-center",
            Self::RightMost => "right-most",
            Self::MenuOrAux => "menu-or-aux",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cFooterPlanCheck {
    ReaderBaselineAvailable,
    FooterSourcesScanned,
    ButtonMappingScanned,
    ExpectedLabelsCaptured,
    MismatchHypothesisRecorded,
    PatchPlanGenerated,
    NoBehaviorChanged,
}

impl Phase40cFooterPlanCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReaderBaselineAvailable => "reader-baseline-available",
            Self::FooterSourcesScanned => "footer-sources-scanned",
            Self::ButtonMappingScanned => "button-mapping-scanned",
            Self::ExpectedLabelsCaptured => "expected-labels-captured",
            Self::MismatchHypothesisRecorded => "mismatch-hypothesis-recorded",
            Self::PatchPlanGenerated => "patch-plan-generated",
            Self::NoBehaviorChanged => "no-behavior-changed",
        }
    }
}

pub const PHASE_40C_FOOTER_PLAN_CHECKS: &[Phase40cFooterPlanCheck] = &[
    Phase40cFooterPlanCheck::ReaderBaselineAvailable,
    Phase40cFooterPlanCheck::FooterSourcesScanned,
    Phase40cFooterPlanCheck::ButtonMappingScanned,
    Phase40cFooterPlanCheck::ExpectedLabelsCaptured,
    Phase40cFooterPlanCheck::MismatchHypothesisRecorded,
    Phase40cFooterPlanCheck::PatchPlanGenerated,
    Phase40cFooterPlanCheck::NoBehaviorChanged,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cPlanStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cPlanReason {
    FooterFixPlanCaptured,
    ReaderBaselineMissing,
    SourceInspectionMissing,
    ExpectedLabelsMissing,
    BehaviorChangeDetected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40cNextLane {
    ApplyFooterLabelPatch,
    RepairBaselineCapture,
    ReinspectInputMapping,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40cFooterButtonPlanReport {
    pub status: Phase40cPlanStatus,
    pub reason: Phase40cPlanReason,
    pub checks: usize,
    pub plan_only: bool,
    pub changes_rendering_now: bool,
    pub changes_input_now: bool,
    pub touches_write_lane: bool,
    pub touches_sd_fat_spi_power: bool,
    pub next_lane: Phase40cNextLane,
}

impl Phase40cFooterButtonPlanReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40cPlanStatus::Ready)
            && self.checks == PHASE_40C_FOOTER_PLAN_CHECKS.len()
            && self.plan_only
            && !self.changes_rendering_now
            && !self.changes_input_now
            && !self.touches_write_lane
            && !self.touches_sd_fat_spi_power
    }
}

pub const PHASE_40C_FOOTER_BUTTON_PLAN_REPORT: Phase40cFooterButtonPlanReport =
    Phase40cFooterButtonPlanReport {
        status: Phase40cPlanStatus::Ready,
        reason: Phase40cPlanReason::FooterFixPlanCaptured,
        checks: PHASE_40C_FOOTER_PLAN_CHECKS.len(),
        plan_only: PHASE_40C_PLAN_ONLY,
        changes_rendering_now: PHASE_40C_CHANGES_RENDERING_NOW,
        changes_input_now: PHASE_40C_CHANGES_INPUT_NOW,
        touches_write_lane: PHASE_40C_TOUCHES_WRITE_LANE,
        touches_sd_fat_spi_power: PHASE_40C_TOUCHES_SD_FAT_SPI_POWER,
        next_lane: Phase40cNextLane::ApplyFooterLabelPatch,
    };

pub fn phase40c_footer_button_plan_report() -> Phase40cFooterButtonPlanReport {
    PHASE_40C_FOOTER_BUTTON_PLAN_REPORT
}

pub fn phase40c_marker() -> &'static str {
    PHASE_40C_FOOTER_BUTTON_LABEL_BASELINE_FIX_PLAN_MARKER
}
