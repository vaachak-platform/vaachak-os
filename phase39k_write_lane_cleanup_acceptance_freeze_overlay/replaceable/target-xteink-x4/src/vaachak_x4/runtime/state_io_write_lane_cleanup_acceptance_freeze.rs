//! Phase 39K — Write Lane Cleanup and Acceptance Freeze.
//!
//! This phase freezes the accepted write path after Phase 39I/39J proved real
//! SD persistence and restore behavior.
//!
//! Accepted final write path:
//!
//! reader/mod.rs
//!   -> reader/typed_state_wiring.rs
//!   -> KernelHandle
//!   -> SD state directory
//!   -> reader restore path
//!
//! Phase 39K is intentionally cleanup/freeze metadata only:
//! - no new write abstraction
//! - no hardware behavior change
//! - no deletion of old scaffolding yet
//! - inventory older Phase 38/39 scaffolding for later cleanup
//! - freeze the accepted callsite path and acceptance evidence

#![allow(dead_code)]

pub const PHASE_39K_WRITE_LANE_CLEANUP_ACCEPTANCE_FREEZE_MARKER: &str =
    "phase39k=x4-write-lane-cleanup-acceptance-freeze-ok";

pub const PHASE_39K_DELETES_CODE_NOW: bool = false;
pub const PHASE_39K_ADDS_NEW_WRITE_ABSTRACTION: bool = false;
pub const PHASE_39K_FREEZES_PHASE39I_PATH: bool = true;
pub const PHASE_39K_REQUIRES_PHASE39J_ACCEPTANCE: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kAcceptedWritePathStep {
    ReaderSaveCallsite,
    TypedStateWiringFacade,
    KernelHandleFileApi,
    SdStateDirectory,
    ReaderRestoreFlow,
}

impl Phase39kAcceptedWritePathStep {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReaderSaveCallsite => "vendor/pulp-os/src/apps/reader/mod.rs",
            Self::TypedStateWiringFacade => "vendor/pulp-os/src/apps/reader/typed_state_wiring.rs",
            Self::KernelHandleFileApi => "KernelHandle ensure_app_subdir/write_app_subdir",
            Self::SdStateDirectory => "_X4/state typed records",
            Self::ReaderRestoreFlow => "reader restore progress/theme/bookmarks",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kRecordEvidence {
    ProgressRecord,
    ThemeRecord,
    MetadataRecord,
    BookmarkRecord,
    BookmarkIndex,
}

impl Phase39kRecordEvidence {
    pub const fn file_pattern(self) -> &'static str {
        match self {
            Self::ProgressRecord => "_X4/state/*.PRG",
            Self::ThemeRecord => "_X4/state/*.THM",
            Self::MetadataRecord => "_X4/state/*.MTA",
            Self::BookmarkRecord => "_X4/state/*.BKM",
            Self::BookmarkIndex => "_X4/state/BMIDX.TXT",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kCleanupDisposition {
    KeepActivePath,
    KeepVerificationTooling,
    ReviewSupersededScaffold,
    PreserveForNow,
}

impl Phase39kCleanupDisposition {
    pub const fn deletes_now(self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kScaffoldArea {
    Phase38ReadOnlyDesign,
    Phase38WriteDesign,
    Phase39ProgressAdapters,
    Phase39TypedRecordAdapters,
    Phase39RuntimeGate,
    Phase39ActiveReaderWiring,
    Phase39VerificationScripts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39kScaffoldInventoryEntry {
    pub area: Phase39kScaffoldArea,
    pub disposition: Phase39kCleanupDisposition,
    pub note: &'static str,
}

pub const PHASE_39K_ACCEPTED_WRITE_PATH: &[Phase39kAcceptedWritePathStep] = &[
    Phase39kAcceptedWritePathStep::ReaderSaveCallsite,
    Phase39kAcceptedWritePathStep::TypedStateWiringFacade,
    Phase39kAcceptedWritePathStep::KernelHandleFileApi,
    Phase39kAcceptedWritePathStep::SdStateDirectory,
    Phase39kAcceptedWritePathStep::ReaderRestoreFlow,
];

pub const PHASE_39K_RECORD_EVIDENCE: &[Phase39kRecordEvidence] = &[
    Phase39kRecordEvidence::ProgressRecord,
    Phase39kRecordEvidence::ThemeRecord,
    Phase39kRecordEvidence::MetadataRecord,
    Phase39kRecordEvidence::BookmarkRecord,
    Phase39kRecordEvidence::BookmarkIndex,
];

pub const PHASE_39K_SCAFFOLD_INVENTORY: &[Phase39kScaffoldInventoryEntry] = &[
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase38ReadOnlyDesign,
        disposition: Phase39kCleanupDisposition::PreserveForNow,
        note: "historical read-only lane; do not delete during freeze",
    },
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase38WriteDesign,
        disposition: Phase39kCleanupDisposition::ReviewSupersededScaffold,
        note: "design scaffolding superseded by Phase 39I active path and Phase 39J evidence",
    },
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase39ProgressAdapters,
        disposition: Phase39kCleanupDisposition::ReviewSupersededScaffold,
        note: "early .PRG adapter experiments; keep until post-freeze cleanup",
    },
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase39TypedRecordAdapters,
        disposition: Phase39kCleanupDisposition::ReviewSupersededScaffold,
        note: "typed-record adapter lane; useful reference, not active reader path",
    },
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase39RuntimeGate,
        disposition: Phase39kCleanupDisposition::ReviewSupersededScaffold,
        note: "runtime-gate lane; active reader path uses Pulp-local facade today",
    },
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase39ActiveReaderWiring,
        disposition: Phase39kCleanupDisposition::KeepActivePath,
        note: "accepted active path; keep",
    },
    Phase39kScaffoldInventoryEntry {
        area: Phase39kScaffoldArea::Phase39VerificationScripts,
        disposition: Phase39kCleanupDisposition::KeepVerificationTooling,
        note: "accepted SD persistence proof tooling; keep",
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kFreezeStatus {
    Frozen,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kFreezeReason {
    ActivePathAndEvidenceAccepted,
    Phase39jEvidenceMissing,
    ActivePathNotWired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39kNextLane {
    PostFreezeCleanupPlan,
    CrashRecoveryForAtomicWrites,
    ReaderRestoreRegressionSuite,
    RepairAcceptedPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39kWriteLaneFreezeReport {
    pub status: Phase39kFreezeStatus,
    pub reason: Phase39kFreezeReason,
    pub accepted_path_steps: usize,
    pub accepted_record_evidence: usize,
    pub deletes_code_now: bool,
    pub adds_new_write_abstraction: bool,
    pub next_lane: Phase39kNextLane,
}

impl Phase39kWriteLaneFreezeReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39kFreezeStatus::Frozen)
            && self.accepted_path_steps == 5
            && self.accepted_record_evidence == 5
            && !self.deletes_code_now
            && !self.adds_new_write_abstraction
    }
}

pub const PHASE_39K_WRITE_LANE_FREEZE_REPORT: Phase39kWriteLaneFreezeReport =
    Phase39kWriteLaneFreezeReport {
        status: Phase39kFreezeStatus::Frozen,
        reason: Phase39kFreezeReason::ActivePathAndEvidenceAccepted,
        accepted_path_steps: PHASE_39K_ACCEPTED_WRITE_PATH.len(),
        accepted_record_evidence: PHASE_39K_RECORD_EVIDENCE.len(),
        deletes_code_now: PHASE_39K_DELETES_CODE_NOW,
        adds_new_write_abstraction: PHASE_39K_ADDS_NEW_WRITE_ABSTRACTION,
        next_lane: Phase39kNextLane::PostFreezeCleanupPlan,
    };

pub fn phase39k_write_lane_freeze_report() -> Phase39kWriteLaneFreezeReport {
    PHASE_39K_WRITE_LANE_FREEZE_REPORT
}

pub fn phase39k_marker() -> &'static str {
    PHASE_39K_WRITE_LANE_CLEANUP_ACCEPTANCE_FREEZE_MARKER
}
