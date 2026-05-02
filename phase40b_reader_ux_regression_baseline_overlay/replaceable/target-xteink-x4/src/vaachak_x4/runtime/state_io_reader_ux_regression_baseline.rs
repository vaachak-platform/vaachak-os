//! Phase 40B — Reader UX Regression Baseline.
//!
//! Phase 40B freezes the current reader UX behavior after Phase 40A closed the
//! write lane.
//!
//! This phase is baseline/acceptance only:
//! - no new feature
//! - no Home UI change
//! - no Files/Library UI change
//! - no Reader UI change
//! - no footer/button-label change
//! - no title-display change
//! - no write-lane change
//! - no SD/FAT/display/input/power behavior change
//!
//! Baseline path:
//!
//! Home
//!   -> Files/Library
//!   -> Reader
//!   -> Back to Files/Library
//!   -> Reopen Reader with restored state

#![allow(dead_code)]

pub const PHASE_40B_READER_UX_REGRESSION_BASELINE_MARKER: &str =
    "phase40b=x4-reader-ux-regression-baseline-ok";

pub const PHASE_40B_ADDS_FEATURES: bool = false;
pub const PHASE_40B_TOUCHES_ACTIVE_READER_PATH: bool = false;
pub const PHASE_40B_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40B_TOUCHES_DISPLAY_INPUT_POWER: bool = false;
pub const PHASE_40B_REQUIRES_PHASE40A_CLOSEOUT: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bReaderUxBaselineCheck {
    HomeOpens,
    FilesLibraryOpens,
    ReaderOpens,
    BackReturnsToLibrary,
    FooterLabelsCaptured,
    EpubTitlesCaptured,
    ReaderRestoreCaptured,
    NoCrashReboot,
    WriteLaneStillClosed,
}

impl Phase40bReaderUxBaselineCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::HomeOpens => "home-opens",
            Self::FilesLibraryOpens => "files-library-opens",
            Self::ReaderOpens => "reader-opens",
            Self::BackReturnsToLibrary => "back-returns-to-library",
            Self::FooterLabelsCaptured => "footer-labels-captured",
            Self::EpubTitlesCaptured => "epub-titles-captured",
            Self::ReaderRestoreCaptured => "reader-restore-captured",
            Self::NoCrashReboot => "no-crash-reboot",
            Self::WriteLaneStillClosed => "write-lane-still-closed",
        }
    }
}

pub const PHASE_40B_READER_UX_BASELINE_CHECKS: &[Phase40bReaderUxBaselineCheck] = &[
    Phase40bReaderUxBaselineCheck::HomeOpens,
    Phase40bReaderUxBaselineCheck::FilesLibraryOpens,
    Phase40bReaderUxBaselineCheck::ReaderOpens,
    Phase40bReaderUxBaselineCheck::BackReturnsToLibrary,
    Phase40bReaderUxBaselineCheck::FooterLabelsCaptured,
    Phase40bReaderUxBaselineCheck::EpubTitlesCaptured,
    Phase40bReaderUxBaselineCheck::ReaderRestoreCaptured,
    Phase40bReaderUxBaselineCheck::NoCrashReboot,
    Phase40bReaderUxBaselineCheck::WriteLaneStillClosed,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bReaderUxSurface {
    Home,
    FilesLibrary,
    Reader,
    Footer,
    EpubTitleDisplay,
    ReaderRestore,
}

impl Phase40bReaderUxSurface {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::FilesLibrary => "files-library",
            Self::Reader => "reader",
            Self::Footer => "footer",
            Self::EpubTitleDisplay => "epub-title-display",
            Self::ReaderRestore => "reader-restore",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bBaselineStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bBaselineReason {
    ReaderUxBaselineCaptured,
    Phase40aCloseoutMissing,
    ManualDeviceConfirmationMissing,
    RuntimeSurfaceRegression,
    SdPersistenceRegression,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40bNextLane {
    StartReaderUxPolish,
    RepairReaderUxRegression,
    RepairWriteLaneRegression,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40bReaderUxBaselineReport {
    pub status: Phase40bBaselineStatus,
    pub reason: Phase40bBaselineReason,
    pub checks: usize,
    pub adds_features: bool,
    pub touches_active_reader_path: bool,
    pub touches_write_lane: bool,
    pub touches_display_input_power: bool,
    pub next_lane: Phase40bNextLane,
}

impl Phase40bReaderUxBaselineReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40bBaselineStatus::Accepted)
            && self.checks == PHASE_40B_READER_UX_BASELINE_CHECKS.len()
            && !self.adds_features
            && !self.touches_active_reader_path
            && !self.touches_write_lane
            && !self.touches_display_input_power
    }
}

pub const PHASE_40B_READER_UX_BASELINE_REPORT: Phase40bReaderUxBaselineReport =
    Phase40bReaderUxBaselineReport {
        status: Phase40bBaselineStatus::Accepted,
        reason: Phase40bBaselineReason::ReaderUxBaselineCaptured,
        checks: PHASE_40B_READER_UX_BASELINE_CHECKS.len(),
        adds_features: PHASE_40B_ADDS_FEATURES,
        touches_active_reader_path: PHASE_40B_TOUCHES_ACTIVE_READER_PATH,
        touches_write_lane: PHASE_40B_TOUCHES_WRITE_LANE,
        touches_display_input_power: PHASE_40B_TOUCHES_DISPLAY_INPUT_POWER,
        next_lane: Phase40bNextLane::StartReaderUxPolish,
    };

pub fn phase40b_reader_ux_baseline_report() -> Phase40bReaderUxBaselineReport {
    PHASE_40B_READER_UX_BASELINE_REPORT
}

pub fn phase40b_marker() -> &'static str {
    PHASE_40B_READER_UX_REGRESSION_BASELINE_MARKER
}
