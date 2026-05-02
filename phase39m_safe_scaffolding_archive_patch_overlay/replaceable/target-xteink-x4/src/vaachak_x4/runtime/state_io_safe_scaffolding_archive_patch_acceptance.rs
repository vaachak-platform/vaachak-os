//! Phase 39M — Safe Scaffolding Archive Patch Acceptance.
//!
//! Acceptance wrapper for the guarded archive patch.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_safe_scaffolding_archive_patch::{
    phase39m_archive_report, Phase39mArchiveReason, Phase39mArchiveStatus, Phase39mNextLane,
};

pub const PHASE_39M_SAFE_SCAFFOLDING_ARCHIVE_PATCH_ACCEPTANCE_MARKER: &str =
    "phase39m-acceptance=x4-safe-scaffolding-archive-patch-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39mAcceptanceReason {
    ArchivePatchAccepted,
    ArchivePatchBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39mAcceptanceReport {
    pub status: Phase39mAcceptanceStatus,
    pub reason: Phase39mAcceptanceReason,
    pub archive_reason: Phase39mArchiveReason,
    pub next_lane: Phase39mNextLane,
}

impl Phase39mAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39mAcceptanceStatus::Accepted)
    }
}

pub fn phase39m_acceptance_report() -> Phase39mAcceptanceReport {
    let report = phase39m_archive_report();
    let accepted = report.accepted();

    Phase39mAcceptanceReport {
        status: if accepted {
            Phase39mAcceptanceStatus::Accepted
        } else {
            Phase39mAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase39mArchiveStatus::Accepted) {
            Phase39mAcceptanceReason::ArchivePatchAccepted
        } else {
            Phase39mAcceptanceReason::ArchivePatchBlocked
        },
        archive_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase39m_acceptance_marker() -> &'static str {
    PHASE_39M_SAFE_SCAFFOLDING_ARCHIVE_PATCH_ACCEPTANCE_MARKER
}
