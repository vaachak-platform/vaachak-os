//! Phase 39D — Typed Record Write Lane Acceptance.
//!
//! Acceptance/report layer for the multi-record Phase 39D write lane.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_typed_record_write_lane::{
    phase39d_has_record_kind, Phase39dTypedRecordKind, Phase39dTypedWriteOutcome,
    Phase39dTypedWriteReport,
};

pub const PHASE_39D_TYPED_RECORD_WRITE_LANE_ACCEPTANCE_MARKER: &str =
    "phase39d-acceptance=x4-typed-record-write-lane-acceptance-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dAcceptanceReason {
    DryRunAccepted,
    TypedRecordWriteCommitted,
    BackendRequired,
    InvalidRequest,
    BackendRejected,
    BackendUnavailable,
    UnsupportedRecordKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dNextLane {
    RealSdFatTypedRecordWriter,
    ExtendReaderRuntimeCallSites,
    KeepCallbackBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dAcceptanceReport {
    pub status: Phase39dAcceptanceStatus,
    pub reason: Phase39dAcceptanceReason,
    pub kind: Phase39dTypedRecordKind,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next_lane: Phase39dNextLane,
}

impl Phase39dAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39dAcceptanceStatus::Accepted)
    }

    pub const fn committed(self) -> bool {
        matches!(self.reason, Phase39dAcceptanceReason::TypedRecordWriteCommitted)
    }
}

pub fn phase39d_accept_typed_record_write_report(
    report: Phase39dTypedWriteReport,
) -> Phase39dAcceptanceReport {
    if !phase39d_has_record_kind(report.kind) {
        return Phase39dAcceptanceReport {
            status: Phase39dAcceptanceStatus::Rejected,
            reason: Phase39dAcceptanceReason::UnsupportedRecordKind,
            kind: report.kind,
            payload_len: report.payload_len,
            bytes_written: report.bytes_written,
            next_lane: Phase39dNextLane::KeepCallbackBackend,
        };
    }

    let (status, reason, next_lane) = phase39d_status_reason_next_lane(report.outcome);

    Phase39dAcceptanceReport {
        status,
        reason,
        kind: report.kind,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        next_lane,
    }
}

pub const fn phase39d_status_reason_next_lane(
    outcome: Phase39dTypedWriteOutcome,
) -> (
    Phase39dAcceptanceStatus,
    Phase39dAcceptanceReason,
    Phase39dNextLane,
) {
    match outcome {
        Phase39dTypedWriteOutcome::DryRunAccepted => (
            Phase39dAcceptanceStatus::Accepted,
            Phase39dAcceptanceReason::DryRunAccepted,
            Phase39dNextLane::RealSdFatTypedRecordWriter,
        ),
        Phase39dTypedWriteOutcome::WriteCommitted => (
            Phase39dAcceptanceStatus::Accepted,
            Phase39dAcceptanceReason::TypedRecordWriteCommitted,
            Phase39dNextLane::RealSdFatTypedRecordWriter,
        ),
        Phase39dTypedWriteOutcome::WriteRejected => (
            Phase39dAcceptanceStatus::Rejected,
            Phase39dAcceptanceReason::BackendRejected,
            Phase39dNextLane::KeepCallbackBackend,
        ),
        Phase39dTypedWriteOutcome::BackendUnavailable => (
            Phase39dAcceptanceStatus::Deferred,
            Phase39dAcceptanceReason::BackendUnavailable,
            Phase39dNextLane::RealSdFatTypedRecordWriter,
        ),
        Phase39dTypedWriteOutcome::InvalidRequest => (
            Phase39dAcceptanceStatus::Rejected,
            Phase39dAcceptanceReason::InvalidRequest,
            Phase39dNextLane::KeepCallbackBackend,
        ),
    }
}

pub fn phase39d_acceptance_marker() -> &'static str {
    PHASE_39D_TYPED_RECORD_WRITE_LANE_ACCEPTANCE_MARKER
}
