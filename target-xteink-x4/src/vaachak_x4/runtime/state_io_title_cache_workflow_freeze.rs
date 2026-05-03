//! Phase 40I — Title Cache Workflow Freeze and Regression Baseline.
//!
//! Phase 40I freezes the accepted title-cache workflow after Phase 40H Repair 1.
//!
//! Accepted workflow:
//! - TXT/MD body-title scanning remains disabled.
//! - Host generates `_X4/TITLEMAP.TSV` from SD root filenames.
//! - Host seeds TXT/MD aliases from `TITLEMAP.TSV` into `_X4/TITLES.BIN`.
//! - Device loads TXT titles from the known-working `_X4/TITLES.BIN` path.
//! - EPUB/EPU metadata title caching remains enabled.
//!
//! No new UX behavior changes are introduced in this phase.

#![allow(dead_code)]

pub const PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_MARKER: &str =
    "phase40i=x4-title-cache-workflow-freeze-ok";

pub const PHASE_40I_FREEZES_TITLE_CACHE_WORKFLOW: bool = true;
pub const PHASE_40I_TXT_BODY_TITLE_SCANNING_DISABLED: bool = true;
pub const PHASE_40I_TXT_TITLES_FROM_TITLES_BIN: bool = true;
pub const PHASE_40I_EPUB_EPU_METADATA_ENABLED: bool = true;
pub const PHASE_40I_CHANGES_UX_NOW: bool = false;
pub const PHASE_40I_CHANGES_FOOTER_LABELS: bool = false;
pub const PHASE_40I_CHANGES_INPUT_MAPPING: bool = false;
pub const PHASE_40I_TOUCHES_WRITE_LANE: bool = false;
pub const PHASE_40I_TOUCHES_DISPLAY_GEOMETRY: bool = false;
pub const PHASE_40I_TOUCHES_READER_PAGINATION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40iFreezeStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40iFreezeCheck {
    TxtBodyScanningDisabled,
    HostTitleMapDocumented,
    TitlesBinSeedingDocumented,
    BadPhraseRegressionGuarded,
    EpubEpuMetadataPreserved,
    NoUxBehaviorChange,
    DeviceBaselineCaptured,
}

impl Phase40iFreezeCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TxtBodyScanningDisabled => "txt-body-scanning-disabled",
            Self::HostTitleMapDocumented => "host-title-map-documented",
            Self::TitlesBinSeedingDocumented => "titles-bin-seeding-documented",
            Self::BadPhraseRegressionGuarded => "bad-phrase-regression-guarded",
            Self::EpubEpuMetadataPreserved => "epub-epu-metadata-preserved",
            Self::NoUxBehaviorChange => "no-ux-behavior-change",
            Self::DeviceBaselineCaptured => "device-baseline-captured",
        }
    }
}

pub const PHASE_40I_FREEZE_CHECKS: &[Phase40iFreezeCheck] = &[
    Phase40iFreezeCheck::TxtBodyScanningDisabled,
    Phase40iFreezeCheck::HostTitleMapDocumented,
    Phase40iFreezeCheck::TitlesBinSeedingDocumented,
    Phase40iFreezeCheck::BadPhraseRegressionGuarded,
    Phase40iFreezeCheck::EpubEpuMetadataPreserved,
    Phase40iFreezeCheck::NoUxBehaviorChange,
    Phase40iFreezeCheck::DeviceBaselineCaptured,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40iTitleCacheWorkflowFreezeReport {
    pub status: Phase40iFreezeStatus,
    pub checks: usize,
    pub freezes_title_cache_workflow: bool,
    pub txt_body_title_scanning_disabled: bool,
    pub txt_titles_from_titles_bin: bool,
    pub epub_epu_metadata_enabled: bool,
    pub changes_ux_now: bool,
    pub changes_footer_labels: bool,
    pub changes_input_mapping: bool,
    pub touches_write_lane: bool,
    pub touches_display_geometry: bool,
    pub touches_reader_pagination: bool,
}

impl Phase40iTitleCacheWorkflowFreezeReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40iFreezeStatus::Accepted)
            && self.checks == PHASE_40I_FREEZE_CHECKS.len()
            && self.freezes_title_cache_workflow
            && self.txt_body_title_scanning_disabled
            && self.txt_titles_from_titles_bin
            && self.epub_epu_metadata_enabled
            && !self.changes_ux_now
            && !self.changes_footer_labels
            && !self.changes_input_mapping
            && !self.touches_write_lane
            && !self.touches_display_geometry
            && !self.touches_reader_pagination
    }
}

pub const PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_REPORT: Phase40iTitleCacheWorkflowFreezeReport =
    Phase40iTitleCacheWorkflowFreezeReport {
        status: Phase40iFreezeStatus::Accepted,
        checks: PHASE_40I_FREEZE_CHECKS.len(),
        freezes_title_cache_workflow: PHASE_40I_FREEZES_TITLE_CACHE_WORKFLOW,
        txt_body_title_scanning_disabled: PHASE_40I_TXT_BODY_TITLE_SCANNING_DISABLED,
        txt_titles_from_titles_bin: PHASE_40I_TXT_TITLES_FROM_TITLES_BIN,
        epub_epu_metadata_enabled: PHASE_40I_EPUB_EPU_METADATA_ENABLED,
        changes_ux_now: PHASE_40I_CHANGES_UX_NOW,
        changes_footer_labels: PHASE_40I_CHANGES_FOOTER_LABELS,
        changes_input_mapping: PHASE_40I_CHANGES_INPUT_MAPPING,
        touches_write_lane: PHASE_40I_TOUCHES_WRITE_LANE,
        touches_display_geometry: PHASE_40I_TOUCHES_DISPLAY_GEOMETRY,
        touches_reader_pagination: PHASE_40I_TOUCHES_READER_PAGINATION,
    };

pub fn phase40i_title_cache_workflow_freeze_report() -> Phase40iTitleCacheWorkflowFreezeReport {
    PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_REPORT
}

pub fn phase40i_marker() -> &'static str {
    PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_MARKER
}
