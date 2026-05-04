//! Phase 38S — State I/O Write Lane Handoff Consolidation.
//!
//! This is the final Phase 38 write-lane design/handoff module.
//!
//! Phase 38 intentionally stops before live mutation. This module consolidates
//! the guarded write lane and declares Phase 39 as the first phase allowed to
//! bind a real write backend or perform a tightly-scoped guarded write.
//!
//! This module does not perform storage, bus, display, input, or power actions.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_guarded_read_before_write_stub::{
    phase38r_live_mutation_still_disabled, Phase38rNextLane,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_adapter_acceptance::{
    phase38p_live_mutation_still_disabled,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_dry_run_executor::{
    PHASE_38M_LIVE_MUTATION_ENABLED,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_backend_implementation_seam::{
    Phase38lMutationIntent, Phase38lStateRecordKind,
};
use crate::vaachak_x4::runtime::state_io_guarded_write_dry_run_acceptance::{
    phase38n_live_mutation_still_disabled,
};
use crate::vaachak_x4::runtime::state_io_guarded_persistent_backend_stub::{
    phase38q_live_mutation_still_disabled,
};
use crate::vaachak_x4::runtime::state_io_write_design_consolidation::{
    phase38c_live_writes_enabled,
};

pub const PHASE_38S_WRITE_LANE_HANDOFF_CONSOLIDATION_MARKER: &str =
    "phase38s=x4-state-io-write-lane-handoff-consolidation-ok";

pub const PHASE_38S_IS_FINAL_PHASE_38: bool = true;
pub const PHASE_38S_PHASE39_WRITE_LANE_ALLOWED_NEXT: bool = true;
pub const PHASE_38S_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38S_PERSISTENT_BACKEND_BOUND: bool = false;
pub const PHASE_38S_COMMIT_ENABLED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38sConsolidatedLane {
    ReadOnlyOutcomes,
    WriteDesign,
    GuardedDryRun,
    AdapterShape,
    PersistentBackendStub,
    ReadBeforeWritePreflight,
    Phase39WriteLaneEntry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38sExitGateStatus {
    ReadyForPhase39,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38sExitGateReason {
    AllDesignGuardsAccepted,
    LiveMutationAlreadyEnabledUnexpectedly,
    PersistentBackendAlreadyBoundUnexpectedly,
    Phase39NotAllowed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38sPhase39FirstWriteScope {
    ProgressOnly,
    ThemeOnly,
    MetadataOnly,
    BookmarkOnly,
    BookmarkIndexOnly,
}

impl Phase38sPhase39FirstWriteScope {
    pub const fn record_kind(self) -> Phase38lStateRecordKind {
        match self {
            Self::ProgressOnly => Phase38lStateRecordKind::Progress,
            Self::ThemeOnly => Phase38lStateRecordKind::Theme,
            Self::MetadataOnly => Phase38lStateRecordKind::Metadata,
            Self::BookmarkOnly => Phase38lStateRecordKind::Bookmark,
            Self::BookmarkIndexOnly => Phase38lStateRecordKind::BookmarkIndex,
        }
    }

    pub const fn allowed_intent(self) -> Phase38lMutationIntent {
        match self {
            Self::BookmarkIndexOnly => Phase38lMutationIntent::AppendIndex,
            Self::ProgressOnly
            | Self::ThemeOnly
            | Self::MetadataOnly
            | Self::BookmarkOnly => Phase38lMutationIntent::UpsertRecord,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ProgressOnly => "progress-only",
            Self::ThemeOnly => "theme-only",
            Self::MetadataOnly => "metadata-only",
            Self::BookmarkOnly => "bookmark-only",
            Self::BookmarkIndexOnly => "bookmark-index-only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38sPhase39Permission {
    MayBindBackend,
    MayWriteOneRecordKind,
    MustKeepRollbackPath,
    MustKeepReadBeforeWrite,
    MustKeepHardwareBehaviorUnchanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38sWriteLaneHandoffReport {
    pub status: Phase38sExitGateStatus,
    pub reason: Phase38sExitGateReason,
    pub first_write_scope: Phase38sPhase39FirstWriteScope,
    pub phase39_write_lane_allowed: bool,
    pub live_mutation_enabled_now: bool,
    pub persistent_backend_bound_now: bool,
    pub commit_enabled_now: bool,
}

impl Phase38sWriteLaneHandoffReport {
    pub const fn ready_for_phase39(self) -> bool {
        matches!(self.status, Phase38sExitGateStatus::ReadyForPhase39)
            && self.phase39_write_lane_allowed
            && !self.live_mutation_enabled_now
            && !self.persistent_backend_bound_now
            && !self.commit_enabled_now
    }

    pub const fn permits_phase38_live_mutation(self) -> bool {
        false
    }
}

pub const PHASE_38S_CONSOLIDATED_LANES: &[Phase38sConsolidatedLane] = &[
    Phase38sConsolidatedLane::ReadOnlyOutcomes,
    Phase38sConsolidatedLane::WriteDesign,
    Phase38sConsolidatedLane::GuardedDryRun,
    Phase38sConsolidatedLane::AdapterShape,
    Phase38sConsolidatedLane::PersistentBackendStub,
    Phase38sConsolidatedLane::ReadBeforeWritePreflight,
    Phase38sConsolidatedLane::Phase39WriteLaneEntry,
];

pub const PHASE_38S_PHASE39_PERMISSIONS: &[Phase38sPhase39Permission] = &[
    Phase38sPhase39Permission::MayBindBackend,
    Phase38sPhase39Permission::MayWriteOneRecordKind,
    Phase38sPhase39Permission::MustKeepRollbackPath,
    Phase38sPhase39Permission::MustKeepReadBeforeWrite,
    Phase38sPhase39Permission::MustKeepHardwareBehaviorUnchanged,
];

pub fn phase38s_has_consolidated_lane(lane: Phase38sConsolidatedLane) -> bool {
    PHASE_38S_CONSOLIDATED_LANES.contains(&lane)
}

pub fn phase38s_has_phase39_permission(permission: Phase38sPhase39Permission) -> bool {
    PHASE_38S_PHASE39_PERMISSIONS.contains(&permission)
}

pub fn phase38s_write_lane_handoff_report() -> Phase38sWriteLaneHandoffReport {
    let live_now = phase38s_live_mutation_enabled_now();
    let backend_bound_now = PHASE_38S_PERSISTENT_BACKEND_BOUND;
    let commit_enabled_now = PHASE_38S_COMMIT_ENABLED;

    let (status, reason) = if live_now {
        (
            Phase38sExitGateStatus::Blocked,
            Phase38sExitGateReason::LiveMutationAlreadyEnabledUnexpectedly,
        )
    } else if backend_bound_now {
        (
            Phase38sExitGateStatus::Blocked,
            Phase38sExitGateReason::PersistentBackendAlreadyBoundUnexpectedly,
        )
    } else if !PHASE_38S_PHASE39_WRITE_LANE_ALLOWED_NEXT {
        (
            Phase38sExitGateStatus::Blocked,
            Phase38sExitGateReason::Phase39NotAllowed,
        )
    } else {
        (
            Phase38sExitGateStatus::ReadyForPhase39,
            Phase38sExitGateReason::AllDesignGuardsAccepted,
        )
    };

    Phase38sWriteLaneHandoffReport {
        status,
        reason,
        first_write_scope: Phase38sPhase39FirstWriteScope::ProgressOnly,
        phase39_write_lane_allowed: PHASE_38S_PHASE39_WRITE_LANE_ALLOWED_NEXT,
        live_mutation_enabled_now: live_now,
        persistent_backend_bound_now: backend_bound_now,
        commit_enabled_now,
    }
}

pub fn phase38s_live_mutation_enabled_now() -> bool {
    PHASE_38S_LIVE_MUTATION_ENABLED
        || PHASE_38M_LIVE_MUTATION_ENABLED
        || phase38c_live_writes_enabled()
}

pub fn phase38s_all_live_mutation_guards_still_disabled() -> bool {
    !phase38s_live_mutation_enabled_now()
        && !PHASE_38S_PERSISTENT_BACKEND_BOUND
        && !PHASE_38S_COMMIT_ENABLED
        && phase38p_live_mutation_still_disabled()
        && phase38n_live_mutation_still_disabled()
        && phase38q_live_mutation_still_disabled()
        && phase38r_live_mutation_still_disabled()
}

pub const fn phase38s_phase38r_next_lane_bridge() -> Phase38rNextLane {
    Phase38rNextLane::GuardedWritePrepareStub
}

pub const fn phase38s_recommended_phase39_first_scope() -> Phase38sPhase39FirstWriteScope {
    Phase38sPhase39FirstWriteScope::ProgressOnly
}

pub const fn phase38s_next_phase_name() -> &'static str {
    "Phase 39A — Guarded Progress State Write Backend Binding"
}

pub fn phase38s_marker() -> &'static str {
    PHASE_38S_WRITE_LANE_HANDOFF_CONSOLIDATION_MARKER
}
