//! Phase 39C — Progress Write Lane Acceptance.
//!
//! Acceptance/report layer for the integrated Phase 39C progress write lane.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_progress_write_lane::{
    phase39c_progress_write_lane_ready, Phase39cProgressWriteBackendKind,
    Phase39cProgressWriteLaneReport, Phase39cProgressWriteLaneStatus,
};

pub const PHASE_39C_PROGRESS_WRITE_LANE_ACCEPTANCE_MARKER: &str =
    "phase39c-acceptance=x4-progress-write-lane-acceptance-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cAcceptanceReason {
    DryRunAccepted,
    ProgressWriteCommitted,
    BackendRequired,
    InvalidRequest,
    BackendRejected,
    BackendUnavailable,
    LaneNotReady,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cNextLane {
    SdFatProgressWriterBinding,
    ThemeWriteLane,
    KeepProgressOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39cAcceptanceReport {
    pub status: Phase39cAcceptanceStatus,
    pub reason: Phase39cAcceptanceReason,
    pub backend_kind: Phase39cProgressWriteBackendKind,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next_lane: Phase39cNextLane,
}

impl Phase39cAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39cAcceptanceStatus::Accepted)
    }

    pub const fn committed(self) -> bool {
        matches!(self.reason, Phase39cAcceptanceReason::ProgressWriteCommitted)
    }
}

pub fn phase39c_accept_progress_write_report(
    report: Phase39cProgressWriteLaneReport,
) -> Phase39cAcceptanceReport {
    if !phase39c_progress_write_lane_ready() {
        return Phase39cAcceptanceReport {
            status: Phase39cAcceptanceStatus::Rejected,
            reason: Phase39cAcceptanceReason::LaneNotReady,
            backend_kind: report.backend_kind,
            payload_len: report.payload_len,
            bytes_written: report.bytes_written,
            next_lane: Phase39cNextLane::KeepProgressOnly,
        };
    }

    let (status, reason, next_lane) = phase39c_status_reason_next_lane(report.status);

    Phase39cAcceptanceReport {
        status,
        reason,
        backend_kind: report.backend_kind,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        next_lane,
    }
}

pub const fn phase39c_status_reason_next_lane(
    status: Phase39cProgressWriteLaneStatus,
) -> (Phase39cAcceptanceStatus, Phase39cAcceptanceReason, Phase39cNextLane) {
    match status {
        Phase39cProgressWriteLaneStatus::DryRunAccepted => (
            Phase39cAcceptanceStatus::Accepted,
            Phase39cAcceptanceReason::DryRunAccepted,
            Phase39cNextLane::SdFatProgressWriterBinding,
        ),
        Phase39cProgressWriteLaneStatus::CallbackCommitted
        | Phase39cProgressWriteLaneStatus::RecordingCommitted => (
            Phase39cAcceptanceStatus::Accepted,
            Phase39cAcceptanceReason::ProgressWriteCommitted,
            Phase39cNextLane::SdFatProgressWriterBinding,
        ),
        Phase39cProgressWriteLaneStatus::RejectedMissingCallback => (
            Phase39cAcceptanceStatus::Deferred,
            Phase39cAcceptanceReason::BackendRequired,
            Phase39cNextLane::SdFatProgressWriterBinding,
        ),
        Phase39cProgressWriteLaneStatus::RejectedInvalidRequest => (
            Phase39cAcceptanceStatus::Rejected,
            Phase39cAcceptanceReason::InvalidRequest,
            Phase39cNextLane::KeepProgressOnly,
        ),
        Phase39cProgressWriteLaneStatus::BackendRejected => (
            Phase39cAcceptanceStatus::Rejected,
            Phase39cAcceptanceReason::BackendRejected,
            Phase39cNextLane::KeepProgressOnly,
        ),
        Phase39cProgressWriteLaneStatus::BackendUnavailable => (
            Phase39cAcceptanceStatus::Deferred,
            Phase39cAcceptanceReason::BackendUnavailable,
            Phase39cNextLane::SdFatProgressWriterBinding,
        ),
    }
}

pub fn phase39c_acceptance_marker() -> &'static str {
    PHASE_39C_PROGRESS_WRITE_LANE_ACCEPTANCE_MARKER
}
