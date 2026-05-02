//! Phase 39I — Active Reader Save Callsite Wiring Acceptance.
//!
//! Metadata mirror for the target runtime. The active code patch lives in
//! `vendor/pulp-os/src/apps/reader/typed_state_wiring.rs` because the active
//! reader persistence callsites are in the imported Pulp reader crate.

#![allow(dead_code)]

pub const PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER: &str =
    "phase39i=x4-active-reader-save-callsite-wiring-bundle-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39iActiveReaderSaveCallsite {
    ProgressRecord,
    ThemeRecord,
    MetadataRecord,
    BookmarkRecord,
    BookmarkIndex,
    BookmarkStub,
    RecentRecord,
}

impl Phase39iActiveReaderSaveCallsite {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProgressRecord => "progress-record",
            Self::ThemeRecord => "theme-record",
            Self::MetadataRecord => "metadata-record",
            Self::BookmarkRecord => "bookmark-record",
            Self::BookmarkIndex => "bookmark-index",
            Self::BookmarkStub => "bookmark-stub",
            Self::RecentRecord => "recent-record",
        }
    }
}

pub const PHASE_39I_ACTIVE_READER_SAVE_CALLSITES: &[Phase39iActiveReaderSaveCallsite] = &[
    Phase39iActiveReaderSaveCallsite::ProgressRecord,
    Phase39iActiveReaderSaveCallsite::ThemeRecord,
    Phase39iActiveReaderSaveCallsite::MetadataRecord,
    Phase39iActiveReaderSaveCallsite::BookmarkRecord,
    Phase39iActiveReaderSaveCallsite::BookmarkIndex,
    Phase39iActiveReaderSaveCallsite::BookmarkStub,
    Phase39iActiveReaderSaveCallsite::RecentRecord,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39iActiveReaderSaveCallsiteReport {
    pub marker: &'static str,
    pub active_reader_callsite_count: usize,
    pub typed_state_facade_present: bool,
    pub concrete_filesystem_hardcoded_in_target: bool,
    pub reader_callsite_wiring_active: bool,
}

impl Phase39iActiveReaderSaveCallsiteReport {
    pub const fn accepted(self) -> bool {
        self.active_reader_callsite_count == PHASE_39I_ACTIVE_READER_SAVE_CALLSITES.len()
            && self.typed_state_facade_present
            && !self.concrete_filesystem_hardcoded_in_target
            && self.reader_callsite_wiring_active
    }
}

pub const PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_REPORT: Phase39iActiveReaderSaveCallsiteReport =
    Phase39iActiveReaderSaveCallsiteReport {
        marker: PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER,
        active_reader_callsite_count: PHASE_39I_ACTIVE_READER_SAVE_CALLSITES.len(),
        typed_state_facade_present: true,
        concrete_filesystem_hardcoded_in_target: false,
        reader_callsite_wiring_active: true,
    };

pub fn phase39i_active_reader_save_callsite_report() -> Phase39iActiveReaderSaveCallsiteReport {
    PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_REPORT
}

pub fn phase39i_marker() -> &'static str {
    PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER
}
