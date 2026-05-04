//! Phase 40E — Reader UX Polish Candidate Plan.
//!
//! Phase 40E is plan-only. It creates a prioritized reader UX polish backlog
//! after Phase 40D accepted the footer/button label baseline.
//!
//! It does not change footer labels, input mapping, ADC thresholds, write lane,
//! SD/FAT behavior, display geometry/rotation, or reader rendering behavior.

#![allow(dead_code)]

pub const PHASE_40E_READER_UX_POLISH_CANDIDATE_PLAN_MARKER: &str =
    "phase40e=x4-reader-ux-polish-candidate-plan-ok";

pub const PHASE_40E_PLAN_ONLY: bool = true;
pub const PHASE_40E_CHANGES_UX_NOW: bool = false;
pub const PHASE_40E_CHANGES_FOOTER_LABELS_NOW: bool = false;
pub const PHASE_40E_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40E_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40E_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40E_REQUIRES_PHASE40D_ACCEPTANCE: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40ePolishArea {
    LibraryTitleLayout,
    FooterSpacing,
    HeaderStatus,
    BodyTypography,
    SelectionHighlight,
    RestoreCopy,
    EmptyState,
    NameWrapping,
}

impl Phase40ePolishArea {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LibraryTitleLayout => "library-title-layout",
            Self::FooterSpacing => "footer-spacing",
            Self::HeaderStatus => "header-status",
            Self::BodyTypography => "body-typography",
            Self::SelectionHighlight => "selection-highlight",
            Self::RestoreCopy => "restore-copy",
            Self::EmptyState => "empty-state",
            Self::NameWrapping => "name-wrapping",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40ePolishRisk {
    Low,
    Medium,
    High,
    Defer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40ePolishPriority {
    First,
    Second,
    Third,
    Later,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40ePolishCandidate {
    pub area: Phase40ePolishArea,
    pub priority: Phase40ePolishPriority,
    pub risk: Phase40ePolishRisk,
    pub note: &'static str,
}

pub const PHASE_40E_POLISH_CANDIDATES: &[Phase40ePolishCandidate] = &[
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::LibraryTitleLayout,
        priority: Phase40ePolishPriority::First,
        risk: Phase40ePolishRisk::Low,
        note: "make file/library title rows more consistent without changing title source",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::FooterSpacing,
        priority: Phase40ePolishPriority::Second,
        risk: Phase40ePolishRisk::Low,
        note: "preserve Back Select Open Stay while improving spacing/alignment only",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::HeaderStatus,
        priority: Phase40ePolishPriority::Third,
        risk: Phase40ePolishRisk::Medium,
        note: "capture and possibly polish page/progress status placement",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::BodyTypography,
        priority: Phase40ePolishPriority::Later,
        risk: Phase40ePolishRisk::Medium,
        note: "font or line spacing polish needs careful pagination regression",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::SelectionHighlight,
        priority: Phase40ePolishPriority::Later,
        risk: Phase40ePolishRisk::Medium,
        note: "selected-row treatment can affect refresh feel and should be isolated",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::RestoreCopy,
        priority: Phase40ePolishPriority::Later,
        risk: Phase40ePolishRisk::Low,
        note: "small text-copy polish only after layout baseline remains stable",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::EmptyState,
        priority: Phase40ePolishPriority::Later,
        risk: Phase40ePolishRisk::Low,
        note: "safe copy/layout polish for no-files/no-books states",
    },
    Phase40ePolishCandidate {
        area: Phase40ePolishArea::NameWrapping,
        priority: Phase40ePolishPriority::Later,
        risk: Phase40ePolishRisk::High,
        note: "long title wrapping can disturb row height and navigation",
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40ePlanCheck {
    BaselinePresent,
    FooterAccepted,
    SourcesScanned,
    BacklogWritten,
    PriorityChosen,
    GuardrailsRecorded,
    NoBehaviorChanged,
}

pub const PHASE_40E_PLAN_CHECKS: &[Phase40ePlanCheck] = &[
    Phase40ePlanCheck::BaselinePresent,
    Phase40ePlanCheck::FooterAccepted,
    Phase40ePlanCheck::SourcesScanned,
    Phase40ePlanCheck::BacklogWritten,
    Phase40ePlanCheck::PriorityChosen,
    Phase40ePlanCheck::GuardrailsRecorded,
    Phase40ePlanCheck::NoBehaviorChanged,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40eStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40eReason {
    CandidatePlanCaptured,
    FooterPatchMissing,
    SourceInspectionMissing,
    BacklogMissing,
    GuardrailFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40eNext {
    LibraryTitleLayoutPatch,
    FooterSpacingPatch,
    ReinspectBaseline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40eReaderUxPolishPlanReport {
    pub status: Phase40eStatus,
    pub reason: Phase40eReason,
    pub checks: usize,
    pub candidate_count: usize,
    pub plan_only: bool,
    pub changes_ux_now: bool,
    pub changes_footer_labels_now: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub next: Phase40eNext,
}

impl Phase40eReaderUxPolishPlanReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40eStatus::Ready)
            && self.checks == PHASE_40E_PLAN_CHECKS.len()
            && self.candidate_count == PHASE_40E_POLISH_CANDIDATES.len()
            && self.plan_only
            && !self.changes_ux_now
            && !self.changes_footer_labels_now
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_display_geometry
    }
}

pub const PHASE_40E_READER_UX_POLISH_PLAN_REPORT: Phase40eReaderUxPolishPlanReport =
    Phase40eReaderUxPolishPlanReport {
        status: Phase40eStatus::Ready,
        reason: Phase40eReason::CandidatePlanCaptured,
        checks: PHASE_40E_PLAN_CHECKS.len(),
        candidate_count: PHASE_40E_POLISH_CANDIDATES.len(),
        plan_only: PHASE_40E_PLAN_ONLY,
        changes_ux_now: PHASE_40E_CHANGES_UX_NOW,
        changes_footer_labels_now: PHASE_40E_CHANGES_FOOTER_LABELS_NOW,
        changes_input_mapping: PHASE_40E_CHANGES_INPUT_MAPPING,
        touches_write_lane: PHASE_40E_TOUCHES_WRITE_LANE,
        touches_display_geometry: PHASE_40E_TOUCHES_DISPLAY_GEOMETRY,
        next: Phase40eNext::LibraryTitleLayoutPatch,
    };

pub fn phase40e_reader_ux_polish_plan_report() -> Phase40eReaderUxPolishPlanReport {
    PHASE_40E_READER_UX_POLISH_PLAN_REPORT
}

pub fn phase40e_marker() -> &'static str {
    PHASE_40E_READER_UX_POLISH_CANDIDATE_PLAN_MARKER
}
