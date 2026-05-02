//! Phase 39M — Safe Scaffolding Archive Patch.
//!
//! Phase 39M archives only the Phase 38/39 review-archive scaffolding after
//! Phase 39L accepted the cleanup plan.
//!
//! It does not touch:
//! - active reader path
//! - `vendor/pulp-os/src/apps/reader/typed_state_wiring.rs`
//! - Phase 39J verification modules/scripts
//! - Phase 39K freeze metadata
//! - Phase 39L cleanup plan metadata
//! - Phase 39L review-delete-later files
//!
//! Archive action is performed by the overlay script, not by this Rust module.

#![allow(dead_code)]

pub const PHASE_39M_SAFE_SCAFFOLDING_ARCHIVE_PATCH_MARKER: &str =
    "phase39m=x4-safe-scaffolding-archive-patch-ok";

pub const PHASE_39M_DELETES_ACTIVE_PATH: bool = false;
pub const PHASE_39M_DELETES_VERIFICATION: bool = false;
pub const PHASE_39M_DELETES_FREEZE_METADATA: bool = false;
pub const PHASE_39M_ARCHIVES_REVIEW_DELETE_LATER: bool = false;
pub const PHASE_39M_ARCHIVES_REVIEW_ARCHIVE_ONLY: bool = true;
pub const PHASE_39M_REQUIRES_ACCEPTED_PATH_GUARD: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mArchiveDisposition {
    Archived,
    Preserved,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mArchivedScaffoldFamily {
    GuardedWriteDesign,
    ShadowWritePrework,
    WriteDesignContract,
    WriteLaneHandoff,
    PreBehaviorWriteEnablement,
}

impl Phase39mArchivedScaffoldFamily {
    pub const fn label(self) -> &'static str {
        match self {
            Self::GuardedWriteDesign => "guarded-write-design",
            Self::ShadowWritePrework => "shadow-write-prework",
            Self::WriteDesignContract => "write-design-contract",
            Self::WriteLaneHandoff => "write-lane-handoff",
            Self::PreBehaviorWriteEnablement => "pre-behavior-write-enablement",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39mArchivePlanEntry {
    pub family: Phase39mArchivedScaffoldFamily,
    pub file_name: &'static str,
    pub disposition: Phase39mArchiveDisposition,
}

impl Phase39mArchivePlanEntry {
    pub const fn archived(self) -> bool {
        matches!(self.disposition, Phase39mArchiveDisposition::Archived)
    }
}

pub const PHASE_39M_ARCHIVE_PLAN: &[Phase39mArchivePlanEntry] = &[
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_persistent_backend_stub.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_read_before_write_stub.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_write_backend_adapter_acceptance.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_write_backend_adapter_shape.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_write_backend_binding.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_write_backend_dry_run_executor.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_write_backend_implementation_seam.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::GuardedWriteDesign,
        file_name: "state_io_guarded_write_dry_run_acceptance.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::PreBehaviorWriteEnablement,
        file_name: "state_io_pre_behavior_write_enablement_consolidation.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::ShadowWritePrework,
        file_name: "state_io_shadow_write_acceptance.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::ShadowWritePrework,
        file_name: "state_io_shadow_write_plan.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::WriteDesignContract,
        file_name: "state_io_write_design_consolidation.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::WriteDesignContract,
        file_name: "state_io_write_lane_entry_contract.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::WriteLaneHandoff,
        file_name: "state_io_write_lane_handoff_consolidation.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
    Phase39mArchivePlanEntry {
        family: Phase39mArchivedScaffoldFamily::WriteDesignContract,
        file_name: "state_io_write_plan_design.rs",
        disposition: Phase39mArchiveDisposition::Archived,
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mArchiveStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mArchiveReason {
    ReviewArchiveOnly,
    AcceptedPathGuardFailed,
    UnexpectedActivePathChange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mNextLane {
    BuildAndDeviceRegression,
    ReviewDeleteLaterCandidatePatch,
    RepairAcceptedPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39mArchiveReport {
    pub status: Phase39mArchiveStatus,
    pub reason: Phase39mArchiveReason,
    pub planned_archive_count: usize,
    pub touches_active_path: bool,
    pub touches_verification: bool,
    pub touches_freeze_metadata: bool,
    pub touches_delete_later_candidates: bool,
    pub next_lane: Phase39mNextLane,
}

impl Phase39mArchiveReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39mArchiveStatus::Accepted)
            && self.planned_archive_count == PHASE_39M_ARCHIVE_PLAN.len()
            && !self.touches_active_path
            && !self.touches_verification
            && !self.touches_freeze_metadata
            && !self.touches_delete_later_candidates
    }
}

pub const PHASE_39M_ARCHIVE_REPORT: Phase39mArchiveReport = Phase39mArchiveReport {
    status: Phase39mArchiveStatus::Accepted,
    reason: Phase39mArchiveReason::ReviewArchiveOnly,
    planned_archive_count: PHASE_39M_ARCHIVE_PLAN.len(),
    touches_active_path: PHASE_39M_DELETES_ACTIVE_PATH,
    touches_verification: PHASE_39M_DELETES_VERIFICATION,
    touches_freeze_metadata: PHASE_39M_DELETES_FREEZE_METADATA,
    touches_delete_later_candidates: PHASE_39M_ARCHIVES_REVIEW_DELETE_LATER,
    next_lane: Phase39mNextLane::BuildAndDeviceRegression,
};

pub fn phase39m_archive_report() -> Phase39mArchiveReport {
    PHASE_39M_ARCHIVE_REPORT
}

pub fn phase39m_marker() -> &'static str {
    PHASE_39M_SAFE_SCAFFOLDING_ARCHIVE_PATCH_MARKER
}
