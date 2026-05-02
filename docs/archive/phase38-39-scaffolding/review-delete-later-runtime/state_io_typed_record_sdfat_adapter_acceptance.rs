//! Phase 39E — Typed Record SD/FAT Adapter Acceptance.
//!
//! Acceptance/report layer for the Phase 39E SD/FAT-shaped adapter binding.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_typed_record_sdfat_adapter::{
    Phase39eAdapterDecision, Phase39eAdapterNextLane, Phase39eSdFatAdapterReport,
};

pub const PHASE_39E_TYPED_RECORD_SDFAT_ADAPTER_ACCEPTANCE_MARKER: &str =
    "phase39e-acceptance=x4-typed-record-sdfat-adapter-acceptance-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eAcceptanceReason {
    DryRunAccepted,
    DirectWriteCommitted,
    AtomicWriteCommitted,
    BackendRequired,
    BackendError,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eAcceptanceNext {
    WireReaderRuntimeCallSites,
    AddRuntimeFeatureGate,
    RepairBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eAcceptanceReport {
    pub status: Phase39eAcceptanceStatus,
    pub reason: Phase39eAcceptanceReason,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next: Phase39eAcceptanceNext,
}

impl Phase39eAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39eAcceptanceStatus::Accepted)
    }

    pub const fn committed(self) -> bool {
        matches!(
            self.reason,
            Phase39eAcceptanceReason::DirectWriteCommitted
                | Phase39eAcceptanceReason::AtomicWriteCommitted
        )
    }
}

pub const fn phase39e_accept_sdfat_adapter_report(
    report: Phase39eSdFatAdapterReport,
) -> Phase39eAcceptanceReport {
    let (status, reason, next) = phase39e_status_reason_next(report.decision, report.next_lane);

    Phase39eAcceptanceReport {
        status,
        reason,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        next,
    }
}

pub const fn phase39e_status_reason_next(
    decision: Phase39eAdapterDecision,
    next_lane: Phase39eAdapterNextLane,
) -> (
    Phase39eAcceptanceStatus,
    Phase39eAcceptanceReason,
    Phase39eAcceptanceNext,
) {
    match decision {
        Phase39eAdapterDecision::DryRunAccepted => (
            Phase39eAcceptanceStatus::Accepted,
            Phase39eAcceptanceReason::DryRunAccepted,
            phase39e_map_next_lane(next_lane),
        ),
        Phase39eAdapterDecision::DirectWriteCommitted => (
            Phase39eAcceptanceStatus::Accepted,
            Phase39eAcceptanceReason::DirectWriteCommitted,
            phase39e_map_next_lane(next_lane),
        ),
        Phase39eAdapterDecision::AtomicWriteCommitted => (
            Phase39eAcceptanceStatus::Accepted,
            Phase39eAcceptanceReason::AtomicWriteCommitted,
            phase39e_map_next_lane(next_lane),
        ),
        Phase39eAdapterDecision::RejectedBackendUnavailable => (
            Phase39eAcceptanceStatus::Deferred,
            Phase39eAcceptanceReason::BackendRequired,
            Phase39eAcceptanceNext::AddRuntimeFeatureGate,
        ),
        Phase39eAdapterDecision::RejectedInvalidRequest => (
            Phase39eAcceptanceStatus::Rejected,
            Phase39eAcceptanceReason::InvalidRequest,
            Phase39eAcceptanceNext::RepairBackend,
        ),
        Phase39eAdapterDecision::RejectedDirectoryUnavailable
        | Phase39eAdapterDecision::RejectedBackendError => (
            Phase39eAcceptanceStatus::Rejected,
            Phase39eAcceptanceReason::BackendError,
            Phase39eAcceptanceNext::RepairBackend,
        ),
    }
}

pub const fn phase39e_map_next_lane(next_lane: Phase39eAdapterNextLane) -> Phase39eAcceptanceNext {
    match next_lane {
        Phase39eAdapterNextLane::WireReaderRuntimeCallSites => {
            Phase39eAcceptanceNext::WireReaderRuntimeCallSites
        }
        Phase39eAdapterNextLane::AddThemeMetadataBookmarkRuntimeWrites => {
            Phase39eAcceptanceNext::WireReaderRuntimeCallSites
        }
        Phase39eAdapterNextLane::KeepAdapterBehindFeatureGate => {
            Phase39eAcceptanceNext::AddRuntimeFeatureGate
        }
    }
}

pub fn phase39e_acceptance_marker() -> &'static str {
    PHASE_39E_TYPED_RECORD_SDFAT_ADAPTER_ACCEPTANCE_MARKER
}
