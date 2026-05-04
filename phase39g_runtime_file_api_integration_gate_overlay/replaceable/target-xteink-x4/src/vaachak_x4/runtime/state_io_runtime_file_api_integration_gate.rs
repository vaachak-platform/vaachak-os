//! Phase 39G — Runtime File API Integration Gate.
//!
//! This module introduces the integration gate between the typed-record writer
//! lane and the runtime/kernel code that owns the actual SD/FAT file APIs.
//!
//! It does not directly import or hard-code a concrete filesystem crate.
//! Instead, it gates any runtime-owned file operations implementation before
//! letting Phase 39F execute through it.
//!
//! Scope remains all typed state records:
//! - `.PRG`
//! - `.THM`
//! - `.MTA`
//! - `.BKM`
//! - `BMIDX.TXT`

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_runtime_owned_sdfat_writer::{
    phase39f_execute_with_runtime_owned_file_ops, Phase39fRuntimeFileError,
    Phase39fRuntimeOwnedFileOps, Phase39fRuntimeOwnedWriterReport,
    Phase39fRuntimeWriterNextLane, Phase39fRuntimeWriterStatus,
};
use crate::vaachak_x4::runtime::state_io_typed_record_sdfat_adapter::Phase39eSdFatAdapterConfig;
use crate::vaachak_x4::runtime::state_io_typed_record_write_lane::{
    Phase39dBackendWriteStatus, Phase39dTypedRecordKind, Phase39dTypedRecordPath,
    Phase39dTypedWriteRequest,
};

pub const PHASE_39G_RUNTIME_FILE_API_INTEGRATION_GATE_MARKER: &str =
    "phase39g=x4-runtime-file-api-integration-gate-ok";

pub const PHASE_39G_RUNTIME_FILE_API_GATE_PRESENT: bool = true;
pub const PHASE_39G_CONCRETE_FILE_API_HARDCODED: bool = false;
pub const PHASE_39G_READER_CALLSITES_WIRED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gRuntimeFileApiCandidate {
    Unknown,
    PulpKernelDirCache,
    PulpKernelFileHandle,
    PulpAppReaderState,
    X4RuntimeStateLayer,
    FutureVaachakStateLayer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gRuntimeFileApiAvailability {
    Disabled,
    CandidateLocated,
    OpsBound,
    VerifiedWritable,
}

impl Phase39gRuntimeFileApiAvailability {
    pub const fn ops_may_execute(self) -> bool {
        matches!(self, Self::OpsBound | Self::VerifiedWritable)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gIntegrationGateDecision {
    DisabledByGate,
    CandidateOnly,
    DispatchToRuntimeOps,
    RuntimeWriteCommitted,
    RuntimeWriteRejected,
    RuntimeBackendUnavailable,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gIntegrationGateReason {
    GateDisabled,
    CandidateNeedsAdapter,
    RuntimeOpsBound,
    RuntimeCommitted,
    RuntimeRejected,
    RuntimeUnavailable,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39gIntegrationNextLane {
    WireProgressSaveCallsite,
    WireAllTypedStateCallsites,
    KeepBehindFeatureGate,
    RepairRuntimeFileOps,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39gRuntimeFileApiProbe {
    pub candidate: Phase39gRuntimeFileApiCandidate,
    pub availability: Phase39gRuntimeFileApiAvailability,
    pub reader_callsites_wired: bool,
}

impl Phase39gRuntimeFileApiProbe {
    pub const fn disabled() -> Self {
        Self {
            candidate: Phase39gRuntimeFileApiCandidate::Unknown,
            availability: Phase39gRuntimeFileApiAvailability::Disabled,
            reader_callsites_wired: PHASE_39G_READER_CALLSITES_WIRED,
        }
    }

    pub const fn located(candidate: Phase39gRuntimeFileApiCandidate) -> Self {
        Self {
            candidate,
            availability: Phase39gRuntimeFileApiAvailability::CandidateLocated,
            reader_callsites_wired: PHASE_39G_READER_CALLSITES_WIRED,
        }
    }

    pub const fn ops_bound(candidate: Phase39gRuntimeFileApiCandidate) -> Self {
        Self {
            candidate,
            availability: Phase39gRuntimeFileApiAvailability::OpsBound,
            reader_callsites_wired: PHASE_39G_READER_CALLSITES_WIRED,
        }
    }

    pub const fn verified_writable(candidate: Phase39gRuntimeFileApiCandidate) -> Self {
        Self {
            candidate,
            availability: Phase39gRuntimeFileApiAvailability::VerifiedWritable,
            reader_callsites_wired: PHASE_39G_READER_CALLSITES_WIRED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39gIntegrationRequest<'a> {
    pub typed_request: Phase39dTypedWriteRequest<'a>,
    pub config: Phase39eSdFatAdapterConfig,
    pub probe: Phase39gRuntimeFileApiProbe,
}

impl<'a> Phase39gIntegrationRequest<'a> {
    pub const fn new(
        typed_request: Phase39dTypedWriteRequest<'a>,
        config: Phase39eSdFatAdapterConfig,
        probe: Phase39gRuntimeFileApiProbe,
    ) -> Self {
        Self {
            typed_request,
            config,
            probe,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39gIntegrationGateReport {
    pub decision: Phase39gIntegrationGateDecision,
    pub reason: Phase39gIntegrationGateReason,
    pub candidate: Phase39gRuntimeFileApiCandidate,
    pub availability: Phase39gRuntimeFileApiAvailability,
    pub kind: Phase39dTypedRecordKind,
    pub backend_status: Phase39dBackendWriteStatus,
    pub runtime_status: Phase39fRuntimeWriterStatus,
    pub runtime_error: Phase39fRuntimeFileError,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub target_path: Phase39dTypedRecordPath,
    pub next_lane: Phase39gIntegrationNextLane,
}

impl Phase39gIntegrationGateReport {
    pub const fn committed(self) -> bool {
        matches!(
            self.decision,
            Phase39gIntegrationGateDecision::RuntimeWriteCommitted
        )
    }

    pub const fn dispatch_attempted(self) -> bool {
        matches!(
            self.decision,
            Phase39gIntegrationGateDecision::DispatchToRuntimeOps
                | Phase39gIntegrationGateDecision::RuntimeWriteCommitted
                | Phase39gIntegrationGateDecision::RuntimeWriteRejected
                | Phase39gIntegrationGateDecision::RuntimeBackendUnavailable
        )
    }
}

pub fn phase39g_execute_runtime_file_api_gate<O: Phase39fRuntimeOwnedFileOps>(
    ops: Option<&mut O>,
    request: Phase39gIntegrationRequest<'_>,
) -> Phase39gIntegrationGateReport {
    if !request.probe.availability.ops_may_execute() {
        return phase39g_report_without_dispatch(
            request,
            phase39g_decision_for_unavailable_probe(request.probe.availability),
        );
    }

    let Some(runtime_ops) = ops else {
        return phase39g_report_without_dispatch(
            request,
            Phase39gIntegrationGateDecision::RuntimeBackendUnavailable,
        );
    };

    let runtime_report =
        phase39f_execute_with_runtime_owned_file_ops(runtime_ops, request.typed_request, request.config);

    phase39g_report_from_phase39f(request.probe, runtime_report)
}

pub const fn phase39g_decision_for_unavailable_probe(
    availability: Phase39gRuntimeFileApiAvailability,
) -> Phase39gIntegrationGateDecision {
    match availability {
        Phase39gRuntimeFileApiAvailability::Disabled => {
            Phase39gIntegrationGateDecision::DisabledByGate
        }
        Phase39gRuntimeFileApiAvailability::CandidateLocated => {
            Phase39gIntegrationGateDecision::CandidateOnly
        }
        Phase39gRuntimeFileApiAvailability::OpsBound
        | Phase39gRuntimeFileApiAvailability::VerifiedWritable => {
            Phase39gIntegrationGateDecision::RuntimeBackendUnavailable
        }
    }
}

pub fn phase39g_report_without_dispatch(
    request: Phase39gIntegrationRequest<'_>,
    decision: Phase39gIntegrationGateDecision,
) -> Phase39gIntegrationGateReport {
    let path = request
        .typed_request
        .path()
        .unwrap_or_else(Phase39dTypedRecordPath::empty);

    Phase39gIntegrationGateReport {
        decision,
        reason: phase39g_reason_from_decision(decision),
        candidate: request.probe.candidate,
        availability: request.probe.availability,
        kind: request.typed_request.kind,
        backend_status: Phase39dBackendWriteStatus::NotCalled,
        runtime_status: phase39g_runtime_status_from_decision(decision),
        runtime_error: phase39g_runtime_error_from_decision(decision),
        payload_len: request.typed_request.payload.len(),
        bytes_written: 0,
        target_path: path,
        next_lane: phase39g_next_lane_from_decision(decision),
    }
}

pub const fn phase39g_report_from_phase39f(
    probe: Phase39gRuntimeFileApiProbe,
    report: Phase39fRuntimeOwnedWriterReport,
) -> Phase39gIntegrationGateReport {
    let decision = phase39g_decision_from_phase39f_status(report.status);

    Phase39gIntegrationGateReport {
        decision,
        reason: phase39g_reason_from_decision(decision),
        candidate: probe.candidate,
        availability: probe.availability,
        kind: report.kind,
        backend_status: report.backend_status,
        runtime_status: report.status,
        runtime_error: report.runtime_error,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        target_path: report.target_path,
        next_lane: phase39g_next_lane_from_phase39f(report.next_lane),
    }
}

pub const fn phase39g_decision_from_phase39f_status(
    status: Phase39fRuntimeWriterStatus,
) -> Phase39gIntegrationGateDecision {
    match status {
        Phase39fRuntimeWriterStatus::DryRunAccepted => {
            Phase39gIntegrationGateDecision::DispatchToRuntimeOps
        }
        Phase39fRuntimeWriterStatus::RuntimeWriteCommitted => {
            Phase39gIntegrationGateDecision::RuntimeWriteCommitted
        }
        Phase39fRuntimeWriterStatus::RuntimeBackendRejected => {
            Phase39gIntegrationGateDecision::RuntimeWriteRejected
        }
        Phase39fRuntimeWriterStatus::RuntimeBackendUnavailable => {
            Phase39gIntegrationGateDecision::RuntimeBackendUnavailable
        }
        Phase39fRuntimeWriterStatus::InvalidRequest => {
            Phase39gIntegrationGateDecision::InvalidRequest
        }
    }
}

pub const fn phase39g_reason_from_decision(
    decision: Phase39gIntegrationGateDecision,
) -> Phase39gIntegrationGateReason {
    match decision {
        Phase39gIntegrationGateDecision::DisabledByGate => {
            Phase39gIntegrationGateReason::GateDisabled
        }
        Phase39gIntegrationGateDecision::CandidateOnly => {
            Phase39gIntegrationGateReason::CandidateNeedsAdapter
        }
        Phase39gIntegrationGateDecision::DispatchToRuntimeOps => {
            Phase39gIntegrationGateReason::RuntimeOpsBound
        }
        Phase39gIntegrationGateDecision::RuntimeWriteCommitted => {
            Phase39gIntegrationGateReason::RuntimeCommitted
        }
        Phase39gIntegrationGateDecision::RuntimeWriteRejected => {
            Phase39gIntegrationGateReason::RuntimeRejected
        }
        Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => {
            Phase39gIntegrationGateReason::RuntimeUnavailable
        }
        Phase39gIntegrationGateDecision::InvalidRequest => Phase39gIntegrationGateReason::InvalidRequest,
    }
}

pub const fn phase39g_runtime_status_from_decision(
    decision: Phase39gIntegrationGateDecision,
) -> Phase39fRuntimeWriterStatus {
    match decision {
        Phase39gIntegrationGateDecision::DisabledByGate
        | Phase39gIntegrationGateDecision::CandidateOnly
        | Phase39gIntegrationGateDecision::DispatchToRuntimeOps => {
            Phase39fRuntimeWriterStatus::DryRunAccepted
        }
        Phase39gIntegrationGateDecision::RuntimeWriteCommitted => {
            Phase39fRuntimeWriterStatus::RuntimeWriteCommitted
        }
        Phase39gIntegrationGateDecision::RuntimeWriteRejected => {
            Phase39fRuntimeWriterStatus::RuntimeBackendRejected
        }
        Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => {
            Phase39fRuntimeWriterStatus::RuntimeBackendUnavailable
        }
        Phase39gIntegrationGateDecision::InvalidRequest => Phase39fRuntimeWriterStatus::InvalidRequest,
    }
}

pub const fn phase39g_runtime_error_from_decision(
    decision: Phase39gIntegrationGateDecision,
) -> Phase39fRuntimeFileError {
    match decision {
        Phase39gIntegrationGateDecision::DisabledByGate
        | Phase39gIntegrationGateDecision::CandidateOnly
        | Phase39gIntegrationGateDecision::DispatchToRuntimeOps
        | Phase39gIntegrationGateDecision::RuntimeWriteCommitted => Phase39fRuntimeFileError::None,
        Phase39gIntegrationGateDecision::RuntimeWriteRejected => Phase39fRuntimeFileError::WriteFailed,
        Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => {
            Phase39fRuntimeFileError::Unsupported
        }
        Phase39gIntegrationGateDecision::InvalidRequest => Phase39fRuntimeFileError::Unsupported,
    }
}

pub const fn phase39g_next_lane_from_decision(
    decision: Phase39gIntegrationGateDecision,
) -> Phase39gIntegrationNextLane {
    match decision {
        Phase39gIntegrationGateDecision::RuntimeWriteCommitted => {
            Phase39gIntegrationNextLane::WireProgressSaveCallsite
        }
        Phase39gIntegrationGateDecision::DispatchToRuntimeOps => {
            Phase39gIntegrationNextLane::WireProgressSaveCallsite
        }
        Phase39gIntegrationGateDecision::DisabledByGate
        | Phase39gIntegrationGateDecision::CandidateOnly
        | Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => {
            Phase39gIntegrationNextLane::KeepBehindFeatureGate
        }
        Phase39gIntegrationGateDecision::RuntimeWriteRejected
        | Phase39gIntegrationGateDecision::InvalidRequest => {
            Phase39gIntegrationNextLane::RepairRuntimeFileOps
        }
    }
}

pub const fn phase39g_next_lane_from_phase39f(
    next_lane: Phase39fRuntimeWriterNextLane,
) -> Phase39gIntegrationNextLane {
    match next_lane {
        Phase39fRuntimeWriterNextLane::WireReaderRuntimeCallSites => {
            Phase39gIntegrationNextLane::WireProgressSaveCallsite
        }
        Phase39fRuntimeWriterNextLane::AddRuntimeFeatureGate
        | Phase39fRuntimeWriterNextLane::AddCrashRecoveryForAtomicTempWrites => {
            Phase39gIntegrationNextLane::KeepBehindFeatureGate
        }
        Phase39fRuntimeWriterNextLane::RepairRuntimeBackend => {
            Phase39gIntegrationNextLane::RepairRuntimeFileOps
        }
    }
}

pub fn phase39g_marker() -> &'static str {
    PHASE_39G_RUNTIME_FILE_API_INTEGRATION_GATE_MARKER
}
