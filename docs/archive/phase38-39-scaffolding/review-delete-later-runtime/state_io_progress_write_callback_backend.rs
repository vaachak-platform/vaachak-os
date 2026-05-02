//! Phase 39B — Guarded Progress Write Callback Backend.
//!
//! This module makes the Phase 39A progress-write binding usable by providing a
//! callback-backed backend adapter.
//!
//! Scope remains intentionally narrow:
//! - `.PRG` progress-state writes only
//! - explicit commit mode still required by Phase 39A
//! - preflight evidence still required by Phase 39A
//! - the actual persistent writer is supplied by the caller as a callback
//! - this module does not hard-code platform storage, display, input, or power logic

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_progress_write_backend_binding::{
    PHASE_39A_PROGRESS_WRITE_ONLY, PHASE_39A_WRITE_LANE_STARTED, Phase39aBackendWriteResult,
    Phase39aBackendWriteStatus, Phase39aProgressStatePath, Phase39aProgressWriteBackend,
    Phase39aProgressWriteDecision, Phase39aProgressWriteOutcome, Phase39aProgressWriteReport,
    Phase39aProgressWriteRequest, phase39a_execute_progress_write,
};

pub const PHASE_39B_GUARDED_PROGRESS_WRITE_CALLBACK_BACKEND_MARKER: &str =
    "phase39b=x4-guarded-progress-write-callback-backend-ok";

pub const PHASE_39B_PROGRESS_WRITE_ONLY: bool = true;
pub const PHASE_39B_CALLBACK_BACKEND_SUPPORTED: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39bCallbackBackendState {
    Unbound,
    Bound,
}

impl Phase39bCallbackBackendState {
    pub const fn bound(self) -> bool {
        matches!(self, Self::Bound)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39bCallbackBackendDecision {
    MissingCallback,
    CallbackReady,
    CallbackReturnedCommitted,
    CallbackReturnedRejected,
    CallbackReturnedUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39bCallbackBackendOutcome {
    DryRunAccepted,
    WriteCommitted,
    WriteRejected,
    BackendUnavailable,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39bProgressWriteCall<'a> {
    pub path: Phase39aProgressStatePath,
    pub request: &'a Phase39aProgressWriteRequest<'a>,
}

pub type Phase39bProgressWriteCallback =
    for<'a> fn(Phase39bProgressWriteCall<'a>) -> Phase39aBackendWriteResult;

#[derive(Clone, Copy)]
pub struct Phase39bCallbackProgressWriteBackend {
    callback: Option<Phase39bProgressWriteCallback>,
}

impl Phase39bCallbackProgressWriteBackend {
    pub const fn unbound() -> Self {
        Self { callback: None }
    }

    pub const fn new(callback: Phase39bProgressWriteCallback) -> Self {
        Self {
            callback: Some(callback),
        }
    }

    pub const fn state(self) -> Phase39bCallbackBackendState {
        if self.callback.is_some() {
            Phase39bCallbackBackendState::Bound
        } else {
            Phase39bCallbackBackendState::Unbound
        }
    }

    pub const fn callback_bound(self) -> bool {
        self.callback.is_some()
    }
}

impl Phase39aProgressWriteBackend for Phase39bCallbackProgressWriteBackend {
    fn write_progress_record(
        &mut self,
        path: Phase39aProgressStatePath,
        request: &Phase39aProgressWriteRequest<'_>,
    ) -> Phase39aBackendWriteResult {
        if let Some(callback) = self.callback {
            callback(Phase39bProgressWriteCall { path, request })
        } else {
            Phase39aBackendWriteResult::unavailable()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39bCallbackProgressWriteReport {
    pub backend_state: Phase39bCallbackBackendState,
    pub backend_decision: Phase39bCallbackBackendDecision,
    pub outcome: Phase39bCallbackBackendOutcome,
    pub phase39a_decision: Phase39aProgressWriteDecision,
    pub phase39a_outcome: Phase39aProgressWriteOutcome,
    pub backend_status: Phase39aBackendWriteStatus,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub progress_write_only: bool,
}

impl Phase39bCallbackProgressWriteReport {
    pub const fn write_committed(self) -> bool {
        matches!(self.outcome, Phase39bCallbackBackendOutcome::WriteCommitted)
    }

    pub const fn dry_run(self) -> bool {
        matches!(self.outcome, Phase39bCallbackBackendOutcome::DryRunAccepted)
    }

    pub const fn backend_bound(self) -> bool {
        self.backend_state.bound()
    }
}

pub fn phase39b_execute_callback_progress_write(
    callback: Option<Phase39bProgressWriteCallback>,
    request: Phase39aProgressWriteRequest<'_>,
) -> Phase39bCallbackProgressWriteReport {
    let backend_state = if callback.is_some() {
        Phase39bCallbackBackendState::Bound
    } else {
        Phase39bCallbackBackendState::Unbound
    };

    let mut backend = Phase39bCallbackProgressWriteBackend { callback };
    let phase39a_report = if backend.callback_bound() {
        phase39a_execute_progress_write(Some(&mut backend), request)
    } else {
        phase39a_execute_progress_write::<Phase39bCallbackProgressWriteBackend>(None, request)
    };

    phase39b_report_from_phase39a(backend_state, phase39a_report)
}

pub const fn phase39b_report_from_phase39a(
    backend_state: Phase39bCallbackBackendState,
    phase39a_report: Phase39aProgressWriteReport,
) -> Phase39bCallbackProgressWriteReport {
    let backend_decision = phase39b_decision_from_status(backend_state, phase39a_report);
    let outcome = phase39b_outcome_from_phase39a(phase39a_report);

    Phase39bCallbackProgressWriteReport {
        backend_state,
        backend_decision,
        outcome,
        phase39a_decision: phase39a_report.decision,
        phase39a_outcome: phase39a_report.outcome,
        backend_status: phase39a_report.backend_status,
        payload_len: phase39a_report.payload_len,
        bytes_written: phase39a_report.bytes_written,
        progress_write_only: PHASE_39B_PROGRESS_WRITE_ONLY,
    }
}

pub const fn phase39b_decision_from_status(
    backend_state: Phase39bCallbackBackendState,
    phase39a_report: Phase39aProgressWriteReport,
) -> Phase39bCallbackBackendDecision {
    match phase39a_report.backend_status {
        Phase39aBackendWriteStatus::Committed => {
            Phase39bCallbackBackendDecision::CallbackReturnedCommitted
        }
        Phase39aBackendWriteStatus::BackendRejected => {
            Phase39bCallbackBackendDecision::CallbackReturnedRejected
        }
        Phase39aBackendWriteStatus::BackendUnavailable => {
            if backend_state.bound() {
                Phase39bCallbackBackendDecision::CallbackReturnedUnavailable
            } else {
                Phase39bCallbackBackendDecision::MissingCallback
            }
        }
        Phase39aBackendWriteStatus::NotCalled => {
            if backend_state.bound() {
                Phase39bCallbackBackendDecision::CallbackReady
            } else {
                Phase39bCallbackBackendDecision::MissingCallback
            }
        }
    }
}

pub const fn phase39b_outcome_from_phase39a(
    phase39a_report: Phase39aProgressWriteReport,
) -> Phase39bCallbackBackendOutcome {
    match phase39a_report.outcome {
        Phase39aProgressWriteOutcome::DryRunAccepted => {
            Phase39bCallbackBackendOutcome::DryRunAccepted
        }
        Phase39aProgressWriteOutcome::WriteCommitted => {
            Phase39bCallbackBackendOutcome::WriteCommitted
        }
        Phase39aProgressWriteOutcome::WriteRejected => {
            Phase39bCallbackBackendOutcome::WriteRejected
        }
        Phase39aProgressWriteOutcome::BackendUnavailable => {
            Phase39bCallbackBackendOutcome::BackendUnavailable
        }
        Phase39aProgressWriteOutcome::InvalidRequest => {
            Phase39bCallbackBackendOutcome::InvalidRequest
        }
    }
}

pub const fn phase39b_write_lane_ready() -> bool {
    PHASE_39A_WRITE_LANE_STARTED && PHASE_39A_PROGRESS_WRITE_ONLY && PHASE_39B_PROGRESS_WRITE_ONLY
}

pub fn phase39b_marker() -> &'static str {
    PHASE_39B_GUARDED_PROGRESS_WRITE_CALLBACK_BACKEND_MARKER
}
