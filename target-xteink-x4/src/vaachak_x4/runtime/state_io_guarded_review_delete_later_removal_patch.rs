//! Phase 39O — Guarded Review-Delete-Later Removal Patch.
//!
//! Phase 39O removes the Phase 39N review-delete-later candidates from the
//! runtime build surface after a dry-run plan was accepted.
//!
//! This patch is guarded:
//! - accepted write path must pass before removal
//! - external source references must be absent before removal
//! - candidate files are moved to docs/archive instead of destroyed
//! - runtime.rs exports for the candidates are removed
//! - accepted write path must pass after removal
//!
//! Protected:
//! - active reader path
//! - Phase 39I metadata
//! - Phase 39J verification
//! - Phase 39K freeze metadata
//! - Phase 39L cleanup plan
//! - Phase 39M archive metadata
//! - Phase 39N dry-run metadata

#![allow(dead_code)]

pub const PHASE_39O_GUARDED_REVIEW_DELETE_LATER_REMOVAL_PATCH_MARKER: &str =
    "phase39o=x4-guarded-review-delete-later-removal-patch-ok";

pub const PHASE_39O_DELETES_ACTIVE_PATH: bool = false;
pub const PHASE_39O_DELETES_PHASE39J_VERIFICATION: bool = false;
pub const PHASE_39O_DELETES_FREEZE_METADATA: bool = false;
pub const PHASE_39O_REMOVES_RUNTIME_EXPORTS: bool = true;
pub const PHASE_39O_MOVES_CANDIDATES_TO_ARCHIVE: bool = true;
pub const PHASE_39O_REQUIRES_ACCEPTED_PATH_GUARD: bool = true;
pub const PHASE_39O_EXPECTED_CANDIDATE_COUNT: usize = 14;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oRemovalFamily {
    ProgressOnlyWriteLane,
    TypedRecordWriteLane,
    SdFatAdapterLane,
    RuntimeOwnedWriterLane,
    RuntimeFileApiGateLane,
    TargetSideTypedStateFacade,
}

impl Phase39oRemovalFamily {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProgressOnlyWriteLane => "progress-only-write-lane",
            Self::TypedRecordWriteLane => "typed-record-write-lane",
            Self::SdFatAdapterLane => "sdfat-adapter-lane",
            Self::RuntimeOwnedWriterLane => "runtime-owned-writer-lane",
            Self::RuntimeFileApiGateLane => "runtime-file-api-gate-lane",
            Self::TargetSideTypedStateFacade => "target-side-typed-state-facade",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oRemovalDisposition {
    MoveToArchive,
    PreserveProtected,
    BlockIfExternallyReferenced,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39oRemovalCandidate {
    pub family: Phase39oRemovalFamily,
    pub file_name: &'static str,
    pub disposition: Phase39oRemovalDisposition,
}

pub const PHASE_39O_REMOVAL_CANDIDATES: &[Phase39oRemovalCandidate] = &[
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_backend_binding.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_callback_backend.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_lane_acceptance.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_lane.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::RuntimeFileApiGateLane,
        file_name: "state_io_runtime_file_api_integration_gate_acceptance.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::RuntimeFileApiGateLane,
        file_name: "state_io_runtime_file_api_integration_gate.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::RuntimeOwnedWriterLane,
        file_name: "state_io_runtime_owned_sdfat_writer_acceptance.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::RuntimeOwnedWriterLane,
        file_name: "state_io_runtime_owned_sdfat_writer.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::SdFatAdapterLane,
        file_name: "state_io_typed_record_sdfat_adapter_acceptance.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::SdFatAdapterLane,
        file_name: "state_io_typed_record_sdfat_adapter.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::TypedRecordWriteLane,
        file_name: "state_io_typed_record_write_lane_acceptance.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::TypedRecordWriteLane,
        file_name: "state_io_typed_record_write_lane.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::TargetSideTypedStateFacade,
        file_name: "state_io_typed_state_runtime_callsite_wiring_acceptance.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
    Phase39oRemovalCandidate {
        family: Phase39oRemovalFamily::TargetSideTypedStateFacade,
        file_name: "state_io_typed_state_runtime_callsite_wiring.rs",
        disposition: Phase39oRemovalDisposition::MoveToArchive,
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oRemovalStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oRemovalReason {
    CandidatesRemovedFromRuntimeSurface,
    AcceptedPathGuardFailed,
    ExternalReferencesFound,
    CandidateCountMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39oNextLane {
    BuildAndDeviceRegression,
    PostRemovalArchiveCompaction,
    RepairAcceptedPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39oRemovalReport {
    pub status: Phase39oRemovalStatus,
    pub reason: Phase39oRemovalReason,
    pub expected_candidates: usize,
    pub moves_to_archive: bool,
    pub removes_runtime_exports: bool,
    pub touches_active_path: bool,
    pub touches_verification: bool,
    pub touches_freeze_metadata: bool,
    pub next_lane: Phase39oNextLane,
}

impl Phase39oRemovalReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39oRemovalStatus::Accepted)
            && self.expected_candidates == PHASE_39O_EXPECTED_CANDIDATE_COUNT
            && self.moves_to_archive
            && self.removes_runtime_exports
            && !self.touches_active_path
            && !self.touches_verification
            && !self.touches_freeze_metadata
    }
}

pub const PHASE_39O_REMOVAL_REPORT: Phase39oRemovalReport = Phase39oRemovalReport {
    status: Phase39oRemovalStatus::Accepted,
    reason: Phase39oRemovalReason::CandidatesRemovedFromRuntimeSurface,
    expected_candidates: PHASE_39O_REMOVAL_CANDIDATES.len(),
    moves_to_archive: PHASE_39O_MOVES_CANDIDATES_TO_ARCHIVE,
    removes_runtime_exports: PHASE_39O_REMOVES_RUNTIME_EXPORTS,
    touches_active_path: PHASE_39O_DELETES_ACTIVE_PATH,
    touches_verification: PHASE_39O_DELETES_PHASE39J_VERIFICATION,
    touches_freeze_metadata: PHASE_39O_DELETES_FREEZE_METADATA,
    next_lane: Phase39oNextLane::BuildAndDeviceRegression,
};

pub fn phase39o_removal_report() -> Phase39oRemovalReport {
    PHASE_39O_REMOVAL_REPORT
}

pub fn phase39o_marker() -> &'static str {
    PHASE_39O_GUARDED_REVIEW_DELETE_LATER_REMOVAL_PATCH_MARKER
}
