//! Phase 39N — Review-Delete-Later Candidate Removal Dry Run.
//!
//! Phase 39N is a dry-run planning phase. It does not delete, move, or archive
//! any files.
//!
//! It targets only the Phase 39L `REVIEW DELETE LATER` candidates that remained
//! after Phase 39M archived the older review-archive scaffolding.
//!
//! Protected paths:
//! - active reader path
//! - Phase 39I active reader metadata
//! - Phase 39J verification
//! - Phase 39K freeze metadata
//! - Phase 39L cleanup plan metadata
//! - Phase 39M archive metadata
//!
//! This module records the dry-run policy and candidate list. The shell scripts
//! produce the concrete file/export/reference report.

#![allow(dead_code)]

pub const PHASE_39N_REVIEW_DELETE_LATER_REMOVAL_DRY_RUN_MARKER: &str =
    "phase39n=x4-review-delete-later-candidate-removal-dry-run-ok";

pub const PHASE_39N_DRY_RUN_ONLY: bool = true;
pub const PHASE_39N_DELETES_CODE_NOW: bool = false;
pub const PHASE_39N_MOVES_CODE_NOW: bool = false;
pub const PHASE_39N_TOUCHES_ACTIVE_READER_PATH: bool = false;
pub const PHASE_39N_TOUCHES_PHASE39J_VERIFICATION: bool = false;
pub const PHASE_39N_TOUCHES_FREEZE_METADATA: bool = false;
pub const PHASE_39N_REQUIRES_ACCEPTED_PATH_GUARD: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39nRemovalCandidateFamily {
    ProgressOnlyWriteLane,
    TypedRecordWriteLane,
    SdFatAdapterLane,
    RuntimeOwnedWriterLane,
    RuntimeFileApiGateLane,
    TargetSideTypedStateFacade,
}

impl Phase39nRemovalCandidateFamily {
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
pub enum Phase39nRemovalDryRunDisposition {
    Candidate,
    KeepProtected,
    BlockedByReference,
    BlockedByExport,
}

impl Phase39nRemovalDryRunDisposition {
    pub const fn deletes_now(self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39nRemovalCandidate {
    pub family: Phase39nRemovalCandidateFamily,
    pub file_name: &'static str,
    pub disposition: Phase39nRemovalDryRunDisposition,
    pub note: &'static str,
}

impl Phase39nRemovalCandidate {
    pub const fn module_name(self) -> &'static str {
        let file = self.file_name;
        let bytes = file.as_bytes();
        if bytes.len() > 3 {
            // module name is encoded in the scripts for concrete use; Rust keeps
            // this as a metadata-only placeholder.
            file
        } else {
            file
        }
    }
}

pub const PHASE_39N_REVIEW_DELETE_LATER_CANDIDATES: &[Phase39nRemovalCandidate] = &[
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_backend_binding.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "early progress-only backend binding; superseded by active reader path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_callback_backend.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "early progress-only callback backend; superseded by active reader path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_lane.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "progress-only lane; no longer active write path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::ProgressOnlyWriteLane,
        file_name: "state_io_progress_write_lane_acceptance.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "progress-only lane acceptance; no longer active write path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::TypedRecordWriteLane,
        file_name: "state_io_typed_record_write_lane.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "typed-record experiment; not used by active Pulp-local writer",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::TypedRecordWriteLane,
        file_name: "state_io_typed_record_write_lane_acceptance.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "typed-record experiment acceptance; not active path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::SdFatAdapterLane,
        file_name: "state_io_typed_record_sdfat_adapter.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "SD/FAT-shaped adapter experiment; not active Pulp-local writer",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::SdFatAdapterLane,
        file_name: "state_io_typed_record_sdfat_adapter_acceptance.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "SD/FAT-shaped adapter acceptance; not active path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::RuntimeOwnedWriterLane,
        file_name: "state_io_runtime_owned_sdfat_writer.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "runtime-owned writer experiment; active path delegates through KernelHandle directly",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::RuntimeOwnedWriterLane,
        file_name: "state_io_runtime_owned_sdfat_writer_acceptance.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "runtime-owned writer acceptance; not active path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::RuntimeFileApiGateLane,
        file_name: "state_io_runtime_file_api_integration_gate.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "runtime file API gate experiment; active path is Pulp-local typed_state_wiring",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::RuntimeFileApiGateLane,
        file_name: "state_io_runtime_file_api_integration_gate_acceptance.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "runtime file API gate acceptance; not active path",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::TargetSideTypedStateFacade,
        file_name: "state_io_typed_state_runtime_callsite_wiring.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "target-side facade superseded by Pulp-local active facade",
    },
    Phase39nRemovalCandidate {
        family: Phase39nRemovalCandidateFamily::TargetSideTypedStateFacade,
        file_name: "state_io_typed_state_runtime_callsite_wiring_acceptance.rs",
        disposition: Phase39nRemovalDryRunDisposition::Candidate,
        note: "target-side facade acceptance; not active path",
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39nDryRunStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39nDryRunReason {
    DryRunPlanGenerated,
    AcceptedPathGuardFailed,
    ProtectedPathIncluded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39nNextLane {
    RemovalPatchAfterRegression,
    RepairAcceptedPath,
    KeepCandidatesForAnotherCycle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39nRemovalDryRunReport {
    pub status: Phase39nDryRunStatus,
    pub reason: Phase39nDryRunReason,
    pub candidate_count: usize,
    pub dry_run_only: bool,
    pub deletes_code_now: bool,
    pub moves_code_now: bool,
    pub touches_active_reader_path: bool,
    pub next_lane: Phase39nNextLane,
}

impl Phase39nRemovalDryRunReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39nDryRunStatus::Accepted)
            && self.dry_run_only
            && !self.deletes_code_now
            && !self.moves_code_now
            && !self.touches_active_reader_path
            && self.candidate_count == PHASE_39N_REVIEW_DELETE_LATER_CANDIDATES.len()
    }
}

pub const PHASE_39N_REMOVAL_DRY_RUN_REPORT: Phase39nRemovalDryRunReport =
    Phase39nRemovalDryRunReport {
        status: Phase39nDryRunStatus::Accepted,
        reason: Phase39nDryRunReason::DryRunPlanGenerated,
        candidate_count: PHASE_39N_REVIEW_DELETE_LATER_CANDIDATES.len(),
        dry_run_only: PHASE_39N_DRY_RUN_ONLY,
        deletes_code_now: PHASE_39N_DELETES_CODE_NOW,
        moves_code_now: PHASE_39N_MOVES_CODE_NOW,
        touches_active_reader_path: PHASE_39N_TOUCHES_ACTIVE_READER_PATH,
        next_lane: Phase39nNextLane::RemovalPatchAfterRegression,
    };

pub fn phase39n_removal_dry_run_report() -> Phase39nRemovalDryRunReport {
    PHASE_39N_REMOVAL_DRY_RUN_REPORT
}

pub fn phase39n_marker() -> &'static str {
    PHASE_39N_REVIEW_DELETE_LATER_REMOVAL_DRY_RUN_MARKER
}
