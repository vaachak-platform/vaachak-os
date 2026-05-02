//! Phase 39F — Runtime-Owned SD/FAT Writer Binding Acceptance.
//!
//! Acceptance/report layer for Phase 39F.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_runtime_owned_sdfat_writer::{
    Phase39fRuntimeOwnedWriterReport, Phase39fRuntimeWriterNextLane, Phase39fRuntimeWriterStatus,
};

pub const PHASE_39F_RUNTIME_OWNED_SDFAT_WRITER_ACCEPTANCE_MARKER: &str =
    "phase39f-acceptance=x4-runtime-owned-sdfat-writer-acceptance-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39fAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39fAcceptanceReason {
    DryRunAccepted,
    RuntimeWriteCommitted,
    RuntimeBackendRequired,
    RuntimeBackendRejected,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39fAcceptanceNext {
    WireReaderRuntimeCallSites,
    AddRuntimeFeatureGate,
    RepairRuntimeBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39fAcceptanceReport {
    pub status: Phase39fAcceptanceStatus,
    pub reason: Phase39fAcceptanceReason,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next: Phase39fAcceptanceNext,
}

impl Phase39fAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39fAcceptanceStatus::Accepted)
    }

    pub const fn committed(self) -> bool {
        matches!(self.reason, Phase39fAcceptanceReason::RuntimeWriteCommitted)
    }
}

pub const fn phase39f_accept_runtime_writer_report(
    report: Phase39fRuntimeOwnedWriterReport,
) -> Phase39fAcceptanceReport {
    let (status, reason, next) = phase39f_status_reason_next(report.status, report.next_lane);

    Phase39fAcceptanceReport {
        status,
        reason,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        next,
    }
}

pub const fn phase39f_status_reason_next(
    status: Phase39fRuntimeWriterStatus,
    next_lane: Phase39fRuntimeWriterNextLane,
) -> (
    Phase39fAcceptanceStatus,
    Phase39fAcceptanceReason,
    Phase39fAcceptanceNext,
) {
    match status {
        Phase39fRuntimeWriterStatus::DryRunAccepted => (
            Phase39fAcceptanceStatus::Accepted,
            Phase39fAcceptanceReason::DryRunAccepted,
            phase39f_map_next_lane(next_lane),
        ),
        Phase39fRuntimeWriterStatus::RuntimeWriteCommitted => (
            Phase39fAcceptanceStatus::Accepted,
            Phase39fAcceptanceReason::RuntimeWriteCommitted,
            phase39f_map_next_lane(next_lane),
        ),
        Phase39fRuntimeWriterStatus::RuntimeBackendUnavailable => (
            Phase39fAcceptanceStatus::Deferred,
            Phase39fAcceptanceReason::RuntimeBackendRequired,
            phase39f_map_next_lane(next_lane),
        ),
        Phase39fRuntimeWriterStatus::RuntimeBackendRejected => (
            Phase39fAcceptanceStatus::Rejected,
            Phase39fAcceptanceReason::RuntimeBackendRejected,
            phase39f_map_next_lane(next_lane),
        ),
        Phase39fRuntimeWriterStatus::InvalidRequest => (
            Phase39fAcceptanceStatus::Rejected,
            Phase39fAcceptanceReason::InvalidRequest,
            phase39f_map_next_lane(next_lane),
        ),
    }
}

pub const fn phase39f_map_next_lane(
    next_lane: Phase39fRuntimeWriterNextLane,
) -> Phase39fAcceptanceNext {
    match next_lane {
        Phase39fRuntimeWriterNextLane::WireReaderRuntimeCallSites => {
            Phase39fAcceptanceNext::WireReaderRuntimeCallSites
        }
        Phase39fRuntimeWriterNextLane::AddRuntimeFeatureGate
        | Phase39fRuntimeWriterNextLane::AddCrashRecoveryForAtomicTempWrites => {
            Phase39fAcceptanceNext::AddRuntimeFeatureGate
        }
        Phase39fRuntimeWriterNextLane::RepairRuntimeBackend => {
            Phase39fAcceptanceNext::RepairRuntimeBackend
        }
    }
}

pub fn phase39f_acceptance_marker() -> &'static str {
    PHASE_39F_RUNTIME_OWNED_SDFAT_WRITER_ACCEPTANCE_MARKER
}
