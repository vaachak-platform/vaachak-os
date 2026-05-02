//! Phase 39G — Runtime File API Integration Gate Acceptance.
//!
//! Acceptance/report layer for the runtime file API integration gate.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_runtime_file_api_integration_gate::{
    Phase39gIntegrationGateDecision, Phase39gIntegrationGateReport, Phase39gIntegrationNextLane,
    Phase39gRuntimeFileApiAvailability,
};

pub const PHASE_39G_RUNTIME_FILE_API_INTEGRATION_GATE_ACCEPTANCE_MARKER: &str =
    "phase39g-acceptance=x4-runtime-file-api-integration-gate-acceptance-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gAcceptanceReason {
    GateReady,
    RuntimeWriteCommitted,
    GateDisabled,
    CandidateNeedsAdapter,
    RuntimeBackendRequired,
    RuntimeRejected,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gAcceptanceNext {
    WireProgressSaveCallsite,
    WireAllTypedStateCallsites,
    KeepBehindFeatureGate,
    RepairRuntimeFileOps,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39gAcceptanceReport {
    pub status: Phase39gAcceptanceStatus,
    pub reason: Phase39gAcceptanceReason,
    pub availability: Phase39gRuntimeFileApiAvailability,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next: Phase39gAcceptanceNext,
}

impl Phase39gAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39gAcceptanceStatus::Accepted)
    }

    pub const fn committed(self) -> bool {
        matches!(self.reason, Phase39gAcceptanceReason::RuntimeWriteCommitted)
    }
}

pub const fn phase39g_accept_integration_gate_report(
    report: Phase39gIntegrationGateReport,
) -> Phase39gAcceptanceReport {
    let (status, reason, next) = phase39g_status_reason_next(report.decision, report.next_lane);

    Phase39gAcceptanceReport {
        status,
        reason,
        availability: report.availability,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        next,
    }
}

pub const fn phase39g_status_reason_next(
    decision: Phase39gIntegrationGateDecision,
    next_lane: Phase39gIntegrationNextLane,
) -> (
    Phase39gAcceptanceStatus,
    Phase39gAcceptanceReason,
    Phase39gAcceptanceNext,
) {
    match decision {
        Phase39gIntegrationGateDecision::DispatchToRuntimeOps => (
            Phase39gAcceptanceStatus::Accepted,
            Phase39gAcceptanceReason::GateReady,
            phase39g_map_next_lane(next_lane),
        ),
        Phase39gIntegrationGateDecision::RuntimeWriteCommitted => (
            Phase39gAcceptanceStatus::Accepted,
            Phase39gAcceptanceReason::RuntimeWriteCommitted,
            phase39g_map_next_lane(next_lane),
        ),
        Phase39gIntegrationGateDecision::DisabledByGate => (
            Phase39gAcceptanceStatus::Deferred,
            Phase39gAcceptanceReason::GateDisabled,
            Phase39gAcceptanceNext::KeepBehindFeatureGate,
        ),
        Phase39gIntegrationGateDecision::CandidateOnly => (
            Phase39gAcceptanceStatus::Deferred,
            Phase39gAcceptanceReason::CandidateNeedsAdapter,
            Phase39gAcceptanceNext::KeepBehindFeatureGate,
        ),
        Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => (
            Phase39gAcceptanceStatus::Deferred,
            Phase39gAcceptanceReason::RuntimeBackendRequired,
            Phase39gAcceptanceNext::KeepBehindFeatureGate,
        ),
        Phase39gIntegrationGateDecision::RuntimeWriteRejected => (
            Phase39gAcceptanceStatus::Rejected,
            Phase39gAcceptanceReason::RuntimeRejected,
            Phase39gAcceptanceNext::RepairRuntimeFileOps,
        ),
        Phase39gIntegrationGateDecision::InvalidRequest => (
            Phase39gAcceptanceStatus::Rejected,
            Phase39gAcceptanceReason::InvalidRequest,
            Phase39gAcceptanceNext::RepairRuntimeFileOps,
        ),
    }
}

pub const fn phase39g_map_next_lane(
    next_lane: Phase39gIntegrationNextLane,
) -> Phase39gAcceptanceNext {
    match next_lane {
        Phase39gIntegrationNextLane::WireProgressSaveCallsite => {
            Phase39gAcceptanceNext::WireProgressSaveCallsite
        }
        Phase39gIntegrationNextLane::WireAllTypedStateCallsites => {
            Phase39gAcceptanceNext::WireAllTypedStateCallsites
        }
        Phase39gIntegrationNextLane::KeepBehindFeatureGate => {
            Phase39gAcceptanceNext::KeepBehindFeatureGate
        }
        Phase39gIntegrationNextLane::RepairRuntimeFileOps => {
            Phase39gAcceptanceNext::RepairRuntimeFileOps
        }
    }
}

pub fn phase39g_acceptance_marker() -> &'static str {
    PHASE_39G_RUNTIME_FILE_API_INTEGRATION_GATE_ACCEPTANCE_MARKER
}
