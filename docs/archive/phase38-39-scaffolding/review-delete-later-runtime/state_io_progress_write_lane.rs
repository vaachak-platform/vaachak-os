//! Phase 39C — Progress Write Lane Integrated Facade.
//!
//! This module consolidates the Phase 39A/39B `.PRG` write lane into a larger,
//! usable facade. It keeps the scope intentionally narrow while providing an
//! end-to-end call path:
//!
//! - validate `.PRG` request
//! - render `STATE/<BOOKID>.PRG`
//! - require preflight evidence
//! - require explicit commit mode for backend dispatch
//! - support callback-backed backend dispatch
//! - support a recording backend for validation/smoke checks
//!
//! It does not add `.THM`, `.MTA`, `.BKM`, or `BMIDX.TXT` writes.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_progress_write_backend_binding::{
    PHASE_39A_MAX_PROGRESS_PATH_LEN, PHASE_39A_MAX_PROGRESS_PAYLOAD_LEN,
    Phase39aBackendWriteResult, Phase39aBackendWriteStatus, Phase39aProgressBookId,
    Phase39aProgressStatePath, Phase39aProgressWriteBackend, Phase39aProgressWriteDecision,
    Phase39aProgressWriteMode, Phase39aProgressWriteOutcome, Phase39aProgressWritePreflight,
    Phase39aProgressWriteReport, Phase39aProgressWriteRequest, phase39a_execute_progress_write,
};
use crate::vaachak_x4::runtime::state_io_progress_write_callback_backend::{
    PHASE_39B_CALLBACK_BACKEND_SUPPORTED, Phase39bCallbackBackendOutcome,
    Phase39bCallbackProgressWriteReport, Phase39bProgressWriteCallback,
    phase39b_execute_callback_progress_write,
};

pub const PHASE_39C_PROGRESS_WRITE_LANE_INTEGRATION_MARKER: &str =
    "phase39c=x4-progress-write-lane-integration-bundle-ok";

pub const PHASE_39C_PROGRESS_WRITE_ONLY: bool = true;
pub const PHASE_39C_CALLBACK_BACKEND_SUPPORTED: bool = true;
pub const PHASE_39C_RECORDING_BACKEND_SUPPORTED: bool = true;
pub const PHASE_39C_MAX_PROGRESS_PAYLOAD_LEN: usize = PHASE_39A_MAX_PROGRESS_PAYLOAD_LEN;
pub const PHASE_39C_MAX_PROGRESS_PATH_LEN: usize = PHASE_39A_MAX_PROGRESS_PATH_LEN;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cProgressWriteBackendKind {
    None,
    Callback,
    Recording,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cProgressWriteLaneMode {
    DryRun,
    CommitViaCallback,
    CommitViaRecordingBackend,
}

impl Phase39cProgressWriteLaneMode {
    pub const fn phase39a_mode(self) -> Phase39aProgressWriteMode {
        match self {
            Self::DryRun => Phase39aProgressWriteMode::DryRunOnly,
            Self::CommitViaCallback | Self::CommitViaRecordingBackend => {
                Phase39aProgressWriteMode::CommitToBoundBackend
            }
        }
    }

    pub const fn backend_kind(self) -> Phase39cProgressWriteBackendKind {
        match self {
            Self::DryRun => Phase39cProgressWriteBackendKind::None,
            Self::CommitViaCallback => Phase39cProgressWriteBackendKind::Callback,
            Self::CommitViaRecordingBackend => Phase39cProgressWriteBackendKind::Recording,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cProgressWriteLaneStatus {
    DryRunAccepted,
    CallbackCommitted,
    RecordingCommitted,
    RejectedInvalidRequest,
    RejectedMissingCallback,
    BackendRejected,
    BackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39cProgressWriteLaneReason {
    DryRunOnly,
    CommitViaCallback,
    CommitViaRecordingBackend,
    InvalidPhase39aRequest,
    CallbackNotBound,
    BackendRejected,
    BackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39cProgressWriteLaneRequest<'a> {
    pub book_id: Phase39aProgressBookId,
    pub payload: &'a [u8],
    pub preflight: Phase39aProgressWritePreflight,
    pub mode: Phase39cProgressWriteLaneMode,
}

impl<'a> Phase39cProgressWriteLaneRequest<'a> {
    pub const fn new(
        book_id: Phase39aProgressBookId,
        payload: &'a [u8],
        preflight: Phase39aProgressWritePreflight,
        mode: Phase39cProgressWriteLaneMode,
    ) -> Self {
        Self {
            book_id,
            payload,
            preflight,
            mode,
        }
    }

    pub const fn as_phase39a_request(self) -> Phase39aProgressWriteRequest<'a> {
        Phase39aProgressWriteRequest::new(
            self.book_id,
            self.payload,
            self.mode.phase39a_mode(),
            self.preflight,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39cProgressWriteLaneReport {
    pub status: Phase39cProgressWriteLaneStatus,
    pub reason: Phase39cProgressWriteLaneReason,
    pub backend_kind: Phase39cProgressWriteBackendKind,
    pub phase39a_decision: Phase39aProgressWriteDecision,
    pub phase39a_outcome: Phase39aProgressWriteOutcome,
    pub backend_status: Phase39aBackendWriteStatus,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub path: Phase39aProgressStatePath,
}

impl Phase39cProgressWriteLaneReport {
    pub const fn committed(self) -> bool {
        matches!(
            self.status,
            Phase39cProgressWriteLaneStatus::CallbackCommitted
                | Phase39cProgressWriteLaneStatus::RecordingCommitted
        )
    }

    pub const fn dry_run(self) -> bool {
        matches!(self.status, Phase39cProgressWriteLaneStatus::DryRunAccepted)
    }

    pub const fn valid_progress_write_only(self) -> bool {
        PHASE_39C_PROGRESS_WRITE_ONLY
    }
}

/// Recording backend for smoke tests and future integration tests.
///
/// It does not persist anything; it records the last write request in memory so
/// the guarded write lane can be validated without binding SD/FAT yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39cRecordingProgressBackend {
    pub calls: usize,
    pub last_path: Phase39aProgressStatePath,
    pub last_payload_len: usize,
    pub last_status: Phase39aBackendWriteStatus,
    pub accept_writes: bool,
}

impl Phase39cRecordingProgressBackend {
    pub const fn new_accepting() -> Self {
        Self {
            calls: 0,
            last_path: Phase39aProgressStatePath::empty(),
            last_payload_len: 0,
            last_status: Phase39aBackendWriteStatus::NotCalled,
            accept_writes: true,
        }
    }

    pub const fn new_rejecting() -> Self {
        Self {
            calls: 0,
            last_path: Phase39aProgressStatePath::empty(),
            last_payload_len: 0,
            last_status: Phase39aBackendWriteStatus::NotCalled,
            accept_writes: false,
        }
    }

    pub const fn wrote_once(self) -> bool {
        self.calls == 1 && matches!(self.last_status, Phase39aBackendWriteStatus::Committed)
    }
}

impl Phase39aProgressWriteBackend for Phase39cRecordingProgressBackend {
    fn write_progress_record(
        &mut self,
        path: Phase39aProgressStatePath,
        request: &Phase39aProgressWriteRequest<'_>,
    ) -> Phase39aBackendWriteResult {
        self.calls = self.calls.saturating_add(1);
        self.last_path = path;
        self.last_payload_len = request.payload.len();

        if self.accept_writes {
            self.last_status = Phase39aBackendWriteStatus::Committed;
            Phase39aBackendWriteResult::committed(request.payload.len())
        } else {
            self.last_status = Phase39aBackendWriteStatus::BackendRejected;
            Phase39aBackendWriteResult::rejected()
        }
    }
}

pub fn phase39c_execute_progress_write_dry_run(
    request: Phase39cProgressWriteLaneRequest<'_>,
) -> Phase39cProgressWriteLaneReport {
    let phase39a_report = phase39a_execute_progress_write::<Phase39cRecordingProgressBackend>(
        None,
        Phase39aProgressWriteRequest::new(
            request.book_id,
            request.payload,
            Phase39aProgressWriteMode::DryRunOnly,
            request.preflight,
        ),
    );

    phase39c_report_from_phase39a(Phase39cProgressWriteBackendKind::None, phase39a_report)
}

pub fn phase39c_execute_progress_write_with_callback(
    request: Phase39cProgressWriteLaneRequest<'_>,
    callback: Option<Phase39bProgressWriteCallback>,
) -> Phase39cProgressWriteLaneReport {
    let callback_report =
        phase39b_execute_callback_progress_write(callback, request.as_phase39a_request());

    phase39c_report_from_phase39b(callback_report)
}

pub fn phase39c_execute_progress_write_with_recording_backend(
    request: Phase39cProgressWriteLaneRequest<'_>,
    backend: &mut Phase39cRecordingProgressBackend,
) -> Phase39cProgressWriteLaneReport {
    let phase39a_report =
        phase39a_execute_progress_write(Some(backend), request.as_phase39a_request());

    phase39c_report_from_phase39a(Phase39cProgressWriteBackendKind::Recording, phase39a_report)
}

pub fn phase39c_execute_progress_write_lane(
    request: Phase39cProgressWriteLaneRequest<'_>,
    callback: Option<Phase39bProgressWriteCallback>,
    recording_backend: Option<&mut Phase39cRecordingProgressBackend>,
) -> Phase39cProgressWriteLaneReport {
    match request.mode {
        Phase39cProgressWriteLaneMode::DryRun => phase39c_execute_progress_write_dry_run(request),
        Phase39cProgressWriteLaneMode::CommitViaCallback => {
            phase39c_execute_progress_write_with_callback(request, callback)
        }
        Phase39cProgressWriteLaneMode::CommitViaRecordingBackend => {
            if let Some(backend) = recording_backend {
                phase39c_execute_progress_write_with_recording_backend(request, backend)
            } else {
                let phase39a_report = phase39a_execute_progress_write::<
                    Phase39cRecordingProgressBackend,
                >(None, request.as_phase39a_request());
                phase39c_report_from_phase39a(
                    Phase39cProgressWriteBackendKind::Recording,
                    phase39a_report,
                )
            }
        }
    }
}

pub const fn phase39c_report_from_phase39b(
    report: Phase39bCallbackProgressWriteReport,
) -> Phase39cProgressWriteLaneReport {
    let (status, reason) = match report.outcome {
        Phase39bCallbackBackendOutcome::DryRunAccepted => (
            Phase39cProgressWriteLaneStatus::DryRunAccepted,
            Phase39cProgressWriteLaneReason::DryRunOnly,
        ),
        Phase39bCallbackBackendOutcome::WriteCommitted => (
            Phase39cProgressWriteLaneStatus::CallbackCommitted,
            Phase39cProgressWriteLaneReason::CommitViaCallback,
        ),
        Phase39bCallbackBackendOutcome::WriteRejected => (
            Phase39cProgressWriteLaneStatus::BackendRejected,
            Phase39cProgressWriteLaneReason::BackendRejected,
        ),
        Phase39bCallbackBackendOutcome::BackendUnavailable => (
            Phase39cProgressWriteLaneStatus::RejectedMissingCallback,
            Phase39cProgressWriteLaneReason::CallbackNotBound,
        ),
        Phase39bCallbackBackendOutcome::InvalidRequest => (
            Phase39cProgressWriteLaneStatus::RejectedInvalidRequest,
            Phase39cProgressWriteLaneReason::InvalidPhase39aRequest,
        ),
    };

    Phase39cProgressWriteLaneReport {
        status,
        reason,
        backend_kind: Phase39cProgressWriteBackendKind::Callback,
        phase39a_decision: report.phase39a_decision,
        phase39a_outcome: report.phase39a_outcome,
        backend_status: report.backend_status,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        path: Phase39aProgressStatePath::empty(),
    }
}

pub const fn phase39c_report_from_phase39a(
    backend_kind: Phase39cProgressWriteBackendKind,
    report: Phase39aProgressWriteReport,
) -> Phase39cProgressWriteLaneReport {
    let (status, reason) = match report.outcome {
        Phase39aProgressWriteOutcome::DryRunAccepted => (
            Phase39cProgressWriteLaneStatus::DryRunAccepted,
            Phase39cProgressWriteLaneReason::DryRunOnly,
        ),
        Phase39aProgressWriteOutcome::WriteCommitted => {
            if matches!(backend_kind, Phase39cProgressWriteBackendKind::Recording) {
                (
                    Phase39cProgressWriteLaneStatus::RecordingCommitted,
                    Phase39cProgressWriteLaneReason::CommitViaRecordingBackend,
                )
            } else {
                (
                    Phase39cProgressWriteLaneStatus::CallbackCommitted,
                    Phase39cProgressWriteLaneReason::CommitViaCallback,
                )
            }
        }
        Phase39aProgressWriteOutcome::WriteRejected => (
            Phase39cProgressWriteLaneStatus::BackendRejected,
            Phase39cProgressWriteLaneReason::BackendRejected,
        ),
        Phase39aProgressWriteOutcome::BackendUnavailable => (
            Phase39cProgressWriteLaneStatus::BackendUnavailable,
            Phase39cProgressWriteLaneReason::BackendUnavailable,
        ),
        Phase39aProgressWriteOutcome::InvalidRequest => (
            Phase39cProgressWriteLaneStatus::RejectedInvalidRequest,
            Phase39cProgressWriteLaneReason::InvalidPhase39aRequest,
        ),
    };

    Phase39cProgressWriteLaneReport {
        status,
        reason,
        backend_kind,
        phase39a_decision: report.decision,
        phase39a_outcome: report.outcome,
        backend_status: report.backend_status,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        path: report.path,
    }
}

pub const fn phase39c_progress_write_lane_ready() -> bool {
    PHASE_39C_PROGRESS_WRITE_ONLY
        && PHASE_39C_CALLBACK_BACKEND_SUPPORTED
        && PHASE_39C_RECORDING_BACKEND_SUPPORTED
        && PHASE_39B_CALLBACK_BACKEND_SUPPORTED
}

pub fn phase39c_marker() -> &'static str {
    PHASE_39C_PROGRESS_WRITE_LANE_INTEGRATION_MARKER
}
