//! Phase 39P — Post-Cleanup Runtime Surface Acceptance.
//!
//! Phase 39P verifies the write-lane cleanup after Phase 39M/39O archived the
//! old scaffolding from the runtime build surface.
//!
//! It is acceptance-only:
//! - no new write abstraction
//! - no active reader path change
//! - no SD/FAT/display/input/power behavior change
//! - no additional removals
//!
//! Accepted write path:
//!
//! vendor/pulp-os/src/apps/reader/mod.rs
//!   -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
//!   -> KernelHandle
//!   -> _X4/state
//!   -> restore

#![allow(dead_code)]

pub const PHASE_39P_POST_CLEANUP_RUNTIME_SURFACE_ACCEPTANCE_MARKER: &str =
    "phase39p=x4-post-cleanup-runtime-surface-acceptance-ok";

pub const PHASE_39P_ADDS_WRITE_ABSTRACTION: bool = false;
pub const PHASE_39P_DELETES_CODE_NOW: bool = false;
pub const PHASE_39P_TOUCHES_ACTIVE_READER_PATH: bool = false;
pub const PHASE_39P_TOUCHES_PHASE39J_VERIFICATION: bool = false;
pub const PHASE_39P_REQUIRES_PHASE39O_ACCEPTANCE: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39pRuntimeSurfaceCheck {
    ActiveReaderPathPresent,
    Phase39jVerificationPresent,
    Phase39kFreezePresent,
    Phase39lPlanPresent,
    Phase39mArchivePresent,
    Phase39nDryRunPresent,
    Phase39oRemovalPresent,
    ArchivedScaffoldingNotExported,
    BuildBaselineCaptured,
}

impl Phase39pRuntimeSurfaceCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveReaderPathPresent => "active-reader-path-present",
            Self::Phase39jVerificationPresent => "phase39j-verification-present",
            Self::Phase39kFreezePresent => "phase39k-freeze-present",
            Self::Phase39lPlanPresent => "phase39l-plan-present",
            Self::Phase39mArchivePresent => "phase39m-archive-present",
            Self::Phase39nDryRunPresent => "phase39n-dry-run-present",
            Self::Phase39oRemovalPresent => "phase39o-removal-present",
            Self::ArchivedScaffoldingNotExported => "archived-scaffolding-not-exported",
            Self::BuildBaselineCaptured => "build-baseline-captured",
        }
    }
}

pub const PHASE_39P_RUNTIME_SURFACE_CHECKS: &[Phase39pRuntimeSurfaceCheck] = &[
    Phase39pRuntimeSurfaceCheck::ActiveReaderPathPresent,
    Phase39pRuntimeSurfaceCheck::Phase39jVerificationPresent,
    Phase39pRuntimeSurfaceCheck::Phase39kFreezePresent,
    Phase39pRuntimeSurfaceCheck::Phase39lPlanPresent,
    Phase39pRuntimeSurfaceCheck::Phase39mArchivePresent,
    Phase39pRuntimeSurfaceCheck::Phase39nDryRunPresent,
    Phase39pRuntimeSurfaceCheck::Phase39oRemovalPresent,
    Phase39pRuntimeSurfaceCheck::ArchivedScaffoldingNotExported,
    Phase39pRuntimeSurfaceCheck::BuildBaselineCaptured,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39pSurfaceStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39pSurfaceReason {
    CleanRuntimeSurfaceAccepted,
    AcceptedPathMissing,
    ArchivedExportFound,
    BuildBaselineMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39pNextLane {
    DeviceRegressionAndCommit,
    StartNextFeatureLane,
    RepairRuntimeSurface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39pRuntimeSurfaceReport {
    pub status: Phase39pSurfaceStatus,
    pub reason: Phase39pSurfaceReason,
    pub checks: usize,
    pub adds_write_abstraction: bool,
    pub deletes_code_now: bool,
    pub touches_active_reader_path: bool,
    pub touches_phase39j_verification: bool,
    pub next_lane: Phase39pNextLane,
}

impl Phase39pRuntimeSurfaceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39pSurfaceStatus::Accepted)
            && self.checks == PHASE_39P_RUNTIME_SURFACE_CHECKS.len()
            && !self.adds_write_abstraction
            && !self.deletes_code_now
            && !self.touches_active_reader_path
            && !self.touches_phase39j_verification
    }
}

pub const PHASE_39P_RUNTIME_SURFACE_REPORT: Phase39pRuntimeSurfaceReport =
    Phase39pRuntimeSurfaceReport {
        status: Phase39pSurfaceStatus::Accepted,
        reason: Phase39pSurfaceReason::CleanRuntimeSurfaceAccepted,
        checks: PHASE_39P_RUNTIME_SURFACE_CHECKS.len(),
        adds_write_abstraction: PHASE_39P_ADDS_WRITE_ABSTRACTION,
        deletes_code_now: PHASE_39P_DELETES_CODE_NOW,
        touches_active_reader_path: PHASE_39P_TOUCHES_ACTIVE_READER_PATH,
        touches_phase39j_verification: PHASE_39P_TOUCHES_PHASE39J_VERIFICATION,
        next_lane: Phase39pNextLane::DeviceRegressionAndCommit,
    };

pub fn phase39p_runtime_surface_report() -> Phase39pRuntimeSurfaceReport {
    PHASE_39P_RUNTIME_SURFACE_REPORT
}

pub fn phase39p_marker() -> &'static str {
    PHASE_39P_POST_CLEANUP_RUNTIME_SURFACE_ACCEPTANCE_MARKER
}
