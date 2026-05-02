//! Phase 39J — Runtime State Write Verification.
//!
//! Verification-focused phase for Phase 39I active reader persistence.
//!
//! This module intentionally does not add another write abstraction. It records
//! the acceptance model for proving real SD-card state persistence for:
//!
//! - `_X4/state/<BOOKID>.PRG`
//! - `_X4/state/<BOOKID>.THM`
//! - `_X4/state/<BOOKID>.MTA`
//! - `_X4/state/<BOOKID>.BKM`
//! - `_X4/state/BMIDX.TXT`
//!
//! The executable verification lives in the Phase 39J shell scripts because the
//! real proof is on the mounted SD card after device interaction.

#![allow(dead_code)]

pub const PHASE_39J_RUNTIME_STATE_WRITE_VERIFICATION_MARKER: &str =
    "phase39j=x4-runtime-state-write-verification-acceptance-ok";

pub const PHASE_39J_WRITES_NEW_ABSTRACTION: bool = false;
pub const PHASE_39J_VERIFIES_PHASE39I_WRITES: bool = true;
pub const PHASE_39J_EXPECTED_RECORD_COUNT: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jVerifiedStateRecord {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl Phase39jVerifiedStateRecord {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Theme => "theme",
            Self::Metadata => "metadata",
            Self::Bookmark => "bookmark",
            Self::BookmarkIndex => "bookmark-index",
        }
    }

    pub const fn extension_or_name(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    pub const fn requires_book_id(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }
}

pub const PHASE_39J_VERIFIED_RECORDS: &[Phase39jVerifiedStateRecord] = &[
    Phase39jVerifiedStateRecord::Progress,
    Phase39jVerifiedStateRecord::Theme,
    Phase39jVerifiedStateRecord::Metadata,
    Phase39jVerifiedStateRecord::Bookmark,
    Phase39jVerifiedStateRecord::BookmarkIndex,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jVerificationSignal {
    NotChecked,
    Missing,
    PresentEmpty,
    PresentNonEmpty,
    RestoredByReader,
}

impl Phase39jVerificationSignal {
    pub const fn present(self) -> bool {
        matches!(
            self,
            Self::PresentEmpty | Self::PresentNonEmpty | Self::RestoredByReader
        )
    }

    pub const fn accepted(self) -> bool {
        matches!(self, Self::PresentNonEmpty | Self::RestoredByReader)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jPersistenceAcceptance {
    Accepted,
    Partial,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jPersistenceReason {
    AllRecordsVerified,
    SomeRecordsMissing,
    NoRecordsVerified,
    RestoreNotVerified,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jNextLane {
    CleanupWriteLaneScaffolding,
    AddCrashRecoveryForAtomicWrites,
    ExpandRestoreRegressionTests,
    RepairReaderPersistence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39jRecordObservation {
    pub record: Phase39jVerifiedStateRecord,
    pub signal: Phase39jVerificationSignal,
    pub bytes: usize,
}

impl Phase39jRecordObservation {
    pub const fn missing(record: Phase39jVerifiedStateRecord) -> Self {
        Self {
            record,
            signal: Phase39jVerificationSignal::Missing,
            bytes: 0,
        }
    }

    pub const fn present(record: Phase39jVerifiedStateRecord, bytes: usize) -> Self {
        Self {
            record,
            signal: if bytes == 0 {
                Phase39jVerificationSignal::PresentEmpty
            } else {
                Phase39jVerificationSignal::PresentNonEmpty
            },
            bytes,
        }
    }

    pub const fn restored(record: Phase39jVerifiedStateRecord, bytes: usize) -> Self {
        Self {
            record,
            signal: Phase39jVerificationSignal::RestoredByReader,
            bytes,
        }
    }

    pub const fn accepted(self) -> bool {
        self.signal.accepted()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39jRuntimeStateWriteVerificationReport {
    pub progress: Phase39jRecordObservation,
    pub theme: Phase39jRecordObservation,
    pub metadata: Phase39jRecordObservation,
    pub bookmark: Phase39jRecordObservation,
    pub bookmark_index: Phase39jRecordObservation,
    pub restore_verified: bool,
}

impl Phase39jRuntimeStateWriteVerificationReport {
    pub const fn empty() -> Self {
        Self {
            progress: Phase39jRecordObservation::missing(Phase39jVerifiedStateRecord::Progress),
            theme: Phase39jRecordObservation::missing(Phase39jVerifiedStateRecord::Theme),
            metadata: Phase39jRecordObservation::missing(Phase39jVerifiedStateRecord::Metadata),
            bookmark: Phase39jRecordObservation::missing(Phase39jVerifiedStateRecord::Bookmark),
            bookmark_index: Phase39jRecordObservation::missing(
                Phase39jVerifiedStateRecord::BookmarkIndex,
            ),
            restore_verified: false,
        }
    }

    pub fn accepted_records(self) -> usize {
        let observations = [
            self.progress,
            self.theme,
            self.metadata,
            self.bookmark,
            self.bookmark_index,
        ];

        observations
            .iter()
            .filter(|observation| observation.accepted())
            .count()
    }

    pub fn acceptance(self) -> Phase39jPersistenceAcceptance {
        let accepted_records = self.accepted_records();

        if accepted_records == PHASE_39J_EXPECTED_RECORD_COUNT && self.restore_verified {
            Phase39jPersistenceAcceptance::Accepted
        } else if accepted_records > 0 {
            Phase39jPersistenceAcceptance::Partial
        } else {
            Phase39jPersistenceAcceptance::Rejected
        }
    }

    pub fn reason(self) -> Phase39jPersistenceReason {
        let accepted_records = self.accepted_records();

        if accepted_records == PHASE_39J_EXPECTED_RECORD_COUNT && self.restore_verified {
            Phase39jPersistenceReason::AllRecordsVerified
        } else if accepted_records == PHASE_39J_EXPECTED_RECORD_COUNT && !self.restore_verified {
            Phase39jPersistenceReason::RestoreNotVerified
        } else if accepted_records > 0 {
            Phase39jPersistenceReason::SomeRecordsMissing
        } else {
            Phase39jPersistenceReason::NoRecordsVerified
        }
    }

    pub fn next_lane(self) -> Phase39jNextLane {
        match self.acceptance() {
            Phase39jPersistenceAcceptance::Accepted => {
                Phase39jNextLane::CleanupWriteLaneScaffolding
            }
            Phase39jPersistenceAcceptance::Partial => Phase39jNextLane::RepairReaderPersistence,
            Phase39jPersistenceAcceptance::Rejected => Phase39jNextLane::RepairReaderPersistence,
        }
    }
}

pub const PHASE_39J_EMPTY_VERIFICATION_REPORT: Phase39jRuntimeStateWriteVerificationReport =
    Phase39jRuntimeStateWriteVerificationReport::empty();

pub fn phase39j_marker() -> &'static str {
    PHASE_39J_RUNTIME_STATE_WRITE_VERIFICATION_MARKER
}
