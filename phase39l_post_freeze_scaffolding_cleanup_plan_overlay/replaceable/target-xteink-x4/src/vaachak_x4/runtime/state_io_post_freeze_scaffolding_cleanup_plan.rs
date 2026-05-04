//! Phase 39L — Post-Freeze Scaffolding Cleanup Plan.
//!
//! Phase 39L is review-only. It creates a cleanup plan after Phase 39K froze
//! the accepted write lane.
//!
//! It does not delete code.
//! It does not add another write abstraction.
//! It does not touch the active reader path.
//!
//! Accepted active path preserved:
//!
//! vendor/pulp-os/src/apps/reader/mod.rs
//!   -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
//!   -> KernelHandle
//!   -> _X4/state typed records
//!   -> reader restore flow

#![allow(dead_code)]

pub const PHASE_39L_POST_FREEZE_SCAFFOLDING_CLEANUP_PLAN_MARKER: &str =
    "phase39l=x4-post-freeze-scaffolding-cleanup-plan-ok";

pub const PHASE_39L_REVIEW_ONLY: bool = true;
pub const PHASE_39L_DELETES_CODE_NOW: bool = false;
pub const PHASE_39L_TOUCHES_ACTIVE_READER_PATH: bool = false;
pub const PHASE_39L_TOUCHES_PHASE39J_VERIFICATION: bool = false;
pub const PHASE_39L_ADDS_WRITE_ABSTRACTION: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lCleanupBucket {
    KeepActive,
    KeepVerification,
    KeepFreezeMetadata,
    ReviewArchive,
    ReviewDeleteLater,
    PreserveHistorical,
}

impl Phase39lCleanupBucket {
    pub const fn deletes_now(self) -> bool {
        false
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::KeepActive => "keep-active",
            Self::KeepVerification => "keep-verification",
            Self::KeepFreezeMetadata => "keep-freeze-metadata",
            Self::ReviewArchive => "review-archive",
            Self::ReviewDeleteLater => "review-delete-later",
            Self::PreserveHistorical => "preserve-historical",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lScaffoldFamily {
    AcceptedReaderPath,
    VerificationEvidence,
    FreezeMetadata,
    Phase38Design,
    Phase39ProgressLane,
    Phase39TypedRecordLane,
    Phase39RuntimeGateLane,
    FileExplorerDisplayNameFixes,
    ShadowWritePrework,
}

impl Phase39lScaffoldFamily {
    pub const fn label(self) -> &'static str {
        match self {
            Self::AcceptedReaderPath => "accepted-reader-path",
            Self::VerificationEvidence => "verification-evidence",
            Self::FreezeMetadata => "freeze-metadata",
            Self::Phase38Design => "phase38-design",
            Self::Phase39ProgressLane => "phase39-progress-lane",
            Self::Phase39TypedRecordLane => "phase39-typed-record-lane",
            Self::Phase39RuntimeGateLane => "phase39-runtime-gate-lane",
            Self::FileExplorerDisplayNameFixes => "file-explorer-display-name-fixes",
            Self::ShadowWritePrework => "shadow-write-prework",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39lCleanupPlanEntry {
    pub family: Phase39lScaffoldFamily,
    pub bucket: Phase39lCleanupBucket,
    pub pattern: &'static str,
    pub note: &'static str,
}

impl Phase39lCleanupPlanEntry {
    pub const fn deletes_now(self) -> bool {
        self.bucket.deletes_now()
    }
}

pub const PHASE_39L_CLEANUP_PLAN: &[Phase39lCleanupPlanEntry] = &[
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::AcceptedReaderPath,
        bucket: Phase39lCleanupBucket::KeepActive,
        pattern: "vendor/pulp-os/src/apps/reader/mod.rs",
        note: "active reader save callsites; do not delete or rewrite during cleanup planning",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::AcceptedReaderPath,
        bucket: Phase39lCleanupBucket::KeepActive,
        pattern: "vendor/pulp-os/src/apps/reader/typed_state_wiring.rs",
        note: "accepted active typed-state facade; keep",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::VerificationEvidence,
        bucket: Phase39lCleanupBucket::KeepVerification,
        pattern: "state_io_runtime_state_write_verification*.rs",
        note: "Phase 39J verification metadata; keep",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::VerificationEvidence,
        bucket: Phase39lCleanupBucket::KeepVerification,
        pattern: "phase39j_runtime_state_write_verification_acceptance_overlay/scripts/*.sh",
        note: "SD persistence verification scripts; keep",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::FreezeMetadata,
        bucket: Phase39lCleanupBucket::KeepFreezeMetadata,
        pattern: "state_io_write_lane_cleanup_acceptance_freeze*.rs",
        note: "Phase 39K freeze metadata; keep",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase38Design,
        bucket: Phase39lCleanupBucket::ReviewArchive,
        pattern: "state_io_*write*_design*.rs",
        note: "design-stage write lane files; archive candidate after review",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase38Design,
        bucket: Phase39lCleanupBucket::ReviewArchive,
        pattern: "state_io_guarded_*",
        note: "guarded write design/dry-run scaffolding; archive candidate after review",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase39ProgressLane,
        bucket: Phase39lCleanupBucket::ReviewDeleteLater,
        pattern: "state_io_progress_write_*.rs",
        note: "early PRG-only lane superseded by active reader path; delete-later candidate",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase39TypedRecordLane,
        bucket: Phase39lCleanupBucket::ReviewDeleteLater,
        pattern: "state_io_typed_record_*.rs",
        note: "typed-record adapter experiments not used by active Pulp reader path; delete-later candidate",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase39RuntimeGateLane,
        bucket: Phase39lCleanupBucket::ReviewDeleteLater,
        pattern: "state_io_runtime_owned_sdfat_writer*.rs",
        note: "runtime-owned adapter lane not used by active reader path; delete-later candidate",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase39RuntimeGateLane,
        bucket: Phase39lCleanupBucket::ReviewDeleteLater,
        pattern: "state_io_runtime_file_api_integration_gate*.rs",
        note: "runtime gate lane not used by accepted active path; delete-later candidate",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::Phase39RuntimeGateLane,
        bucket: Phase39lCleanupBucket::ReviewDeleteLater,
        pattern: "state_io_typed_state_runtime_callsite_wiring*.rs",
        note: "target-side wiring facade superseded by Pulp-local typed_state_wiring; delete-later candidate",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::FileExplorerDisplayNameFixes,
        bucket: Phase39lCleanupBucket::PreserveHistorical,
        pattern: "file_explorer_*display*.rs",
        note: "file display-title fixes are separate from write lane; preserve until UI cleanup",
    },
    Phase39lCleanupPlanEntry {
        family: Phase39lScaffoldFamily::ShadowWritePrework,
        bucket: Phase39lCleanupBucket::ReviewArchive,
        pattern: "state_io_shadow_write*.rs",
        note: "pre-write shadow planning; archive candidate after review",
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lGuardStatus {
    SafeToPlan,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lGuardReason {
    AcceptedPathPreserved,
    ActiveReaderPathMissing,
    VerificationPathMissing,
    FreezePathMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39lNextLane {
    GenerateDeletionPatch,
    ArchiveScaffoldingDocs,
    RepairAcceptedPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39lCleanupPlanReport {
    pub guard_status: Phase39lGuardStatus,
    pub guard_reason: Phase39lGuardReason,
    pub plan_entries: usize,
    pub deletes_code_now: bool,
    pub review_only: bool,
    pub next_lane: Phase39lNextLane,
}

impl Phase39lCleanupPlanReport {
    pub const fn accepted(self) -> bool {
        matches!(self.guard_status, Phase39lGuardStatus::SafeToPlan)
            && self.review_only
            && !self.deletes_code_now
            && self.plan_entries > 0
    }
}

pub const PHASE_39L_CLEANUP_PLAN_REPORT: Phase39lCleanupPlanReport =
    Phase39lCleanupPlanReport {
        guard_status: Phase39lGuardStatus::SafeToPlan,
        guard_reason: Phase39lGuardReason::AcceptedPathPreserved,
        plan_entries: PHASE_39L_CLEANUP_PLAN.len(),
        deletes_code_now: PHASE_39L_DELETES_CODE_NOW,
        review_only: PHASE_39L_REVIEW_ONLY,
        next_lane: Phase39lNextLane::GenerateDeletionPatch,
    };

pub fn phase39l_cleanup_plan_report() -> Phase39lCleanupPlanReport {
    PHASE_39L_CLEANUP_PLAN_REPORT
}

pub fn phase39l_marker() -> &'static str {
    PHASE_39L_POST_FREEZE_SCAFFOLDING_CLEANUP_PLAN_MARKER
}
