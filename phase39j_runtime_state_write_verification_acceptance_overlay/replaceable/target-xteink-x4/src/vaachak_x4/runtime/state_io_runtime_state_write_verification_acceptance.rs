//! Phase 39J — Runtime State Write Verification Acceptance.
//!
//! Acceptance metadata for SD persistence verification. The concrete SD-card
//! inspection is performed by the Phase 39J shell scripts.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_runtime_state_write_verification::{
    Phase39jNextLane, Phase39jPersistenceAcceptance, Phase39jPersistenceReason,
    Phase39jRuntimeStateWriteVerificationReport,
};

pub const PHASE_39J_RUNTIME_STATE_WRITE_VERIFICATION_ACCEPTANCE_MARKER: &str =
    "phase39j-acceptance=x4-runtime-state-write-verification-acceptance-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jAcceptanceStatus {
    Accepted,
    Partial,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39jAcceptanceNext {
    CleanupWriteLaneScaffolding,
    AddCrashRecoveryForAtomicWrites,
    ExpandRestoreRegressionTests,
    RepairReaderPersistence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39jAcceptanceReport {
    pub status: Phase39jAcceptanceStatus,
    pub reason: Phase39jPersistenceReason,
    pub accepted_records: usize,
    pub restore_verified: bool,
    pub next: Phase39jAcceptanceNext,
}

impl Phase39jAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39jAcceptanceStatus::Accepted)
    }
}

pub fn phase39j_accept_runtime_state_write_verification(
    report: Phase39jRuntimeStateWriteVerificationReport,
) -> Phase39jAcceptanceReport {
    Phase39jAcceptanceReport {
        status: phase39j_acceptance_status(report.acceptance()),
        reason: report.reason(),
        accepted_records: report.accepted_records(),
        restore_verified: report.restore_verified,
        next: phase39j_map_next_lane(report.next_lane()),
    }
}

pub const fn phase39j_acceptance_status(
    acceptance: Phase39jPersistenceAcceptance,
) -> Phase39jAcceptanceStatus {
    match acceptance {
        Phase39jPersistenceAcceptance::Accepted => Phase39jAcceptanceStatus::Accepted,
        Phase39jPersistenceAcceptance::Partial => Phase39jAcceptanceStatus::Partial,
        Phase39jPersistenceAcceptance::Rejected => Phase39jAcceptanceStatus::Rejected,
    }
}

pub const fn phase39j_map_next_lane(next_lane: Phase39jNextLane) -> Phase39jAcceptanceNext {
    match next_lane {
        Phase39jNextLane::CleanupWriteLaneScaffolding => {
            Phase39jAcceptanceNext::CleanupWriteLaneScaffolding
        }
        Phase39jNextLane::AddCrashRecoveryForAtomicWrites => {
            Phase39jAcceptanceNext::AddCrashRecoveryForAtomicWrites
        }
        Phase39jNextLane::ExpandRestoreRegressionTests => {
            Phase39jAcceptanceNext::ExpandRestoreRegressionTests
        }
        Phase39jNextLane::RepairReaderPersistence => Phase39jAcceptanceNext::RepairReaderPersistence,
    }
}

pub fn phase39j_acceptance_marker() -> &'static str {
    PHASE_39J_RUNTIME_STATE_WRITE_VERIFICATION_ACCEPTANCE_MARKER
}
