//! Phase 39H — Typed State Runtime Callsite Wiring Acceptance.
//!
//! Acceptance/report layer for the all-at-once typed-state runtime write facade.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_typed_state_runtime_callsite_wiring::{
    Phase39hAllTypedStateWriteSummary, Phase39hTypedStateWriteReport, Phase39hTypedStateWriteStatus,
};

pub const PHASE_39H_TYPED_STATE_RUNTIME_CALLSITE_WIRING_ACCEPTANCE_MARKER: &str =
    "phase39h-acceptance=x4-typed-state-runtime-callsite-wiring-acceptance-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hAcceptanceStatus {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hAcceptanceReason {
    DryRunAccepted,
    RuntimeWriteCommitted,
    GateDisabled,
    RuntimeBackendRequired,
    RuntimeRejected,
    PartialBundleAccepted,
    FullBundleAccepted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hAcceptanceNext {
    WireExistingReaderSaveCallsites,
    EnableRuntimeGate,
    RepairRuntimeWriter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39hAcceptanceReport {
    pub status: Phase39hAcceptanceStatus,
    pub reason: Phase39hAcceptanceReason,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next: Phase39hAcceptanceNext,
}

impl Phase39hAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase39hAcceptanceStatus::Accepted)
    }

    pub const fn committed(self) -> bool {
        matches!(self.reason, Phase39hAcceptanceReason::RuntimeWriteCommitted)
    }
}

pub const fn phase39h_accept_typed_state_write_report(
    report: Phase39hTypedStateWriteReport,
) -> Phase39hAcceptanceReport {
    let (status, reason, next) = phase39h_status_reason_next(report.status);

    Phase39hAcceptanceReport {
        status,
        reason,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        next,
    }
}

pub const fn phase39h_accept_all_typed_state_summary(
    summary: Phase39hAllTypedStateWriteSummary,
) -> Phase39hAcceptanceReport {
    if summary.all_accepted() {
        Phase39hAcceptanceReport {
            status: Phase39hAcceptanceStatus::Accepted,
            reason: Phase39hAcceptanceReason::FullBundleAccepted,
            payload_len: summary.attempted,
            bytes_written: summary.committed,
            next: Phase39hAcceptanceNext::WireExistingReaderSaveCallsites,
        }
    } else if summary.accepted_or_committed > 0 {
        Phase39hAcceptanceReport {
            status: Phase39hAcceptanceStatus::Deferred,
            reason: Phase39hAcceptanceReason::PartialBundleAccepted,
            payload_len: summary.attempted,
            bytes_written: summary.committed,
            next: Phase39hAcceptanceNext::EnableRuntimeGate,
        }
    } else {
        Phase39hAcceptanceReport {
            status: Phase39hAcceptanceStatus::Rejected,
            reason: Phase39hAcceptanceReason::RuntimeRejected,
            payload_len: summary.attempted,
            bytes_written: 0,
            next: Phase39hAcceptanceNext::RepairRuntimeWriter,
        }
    }
}

pub const fn phase39h_status_reason_next(
    status: Phase39hTypedStateWriteStatus,
) -> (Phase39hAcceptanceStatus, Phase39hAcceptanceReason, Phase39hAcceptanceNext) {
    match status {
        Phase39hTypedStateWriteStatus::DryRunAccepted => (
            Phase39hAcceptanceStatus::Accepted,
            Phase39hAcceptanceReason::DryRunAccepted,
            Phase39hAcceptanceNext::WireExistingReaderSaveCallsites,
        ),
        Phase39hTypedStateWriteStatus::RuntimeWriteCommitted => (
            Phase39hAcceptanceStatus::Accepted,
            Phase39hAcceptanceReason::RuntimeWriteCommitted,
            Phase39hAcceptanceNext::WireExistingReaderSaveCallsites,
        ),
        Phase39hTypedStateWriteStatus::GateDisabled => (
            Phase39hAcceptanceStatus::Deferred,
            Phase39hAcceptanceReason::GateDisabled,
            Phase39hAcceptanceNext::EnableRuntimeGate,
        ),
        Phase39hTypedStateWriteStatus::Deferred => (
            Phase39hAcceptanceStatus::Deferred,
            Phase39hAcceptanceReason::RuntimeBackendRequired,
            Phase39hAcceptanceNext::EnableRuntimeGate,
        ),
        Phase39hTypedStateWriteStatus::Rejected => (
            Phase39hAcceptanceStatus::Rejected,
            Phase39hAcceptanceReason::RuntimeRejected,
            Phase39hAcceptanceNext::RepairRuntimeWriter,
        ),
    }
}

pub fn phase39h_acceptance_marker() -> &'static str {
    PHASE_39H_TYPED_STATE_RUNTIME_CALLSITE_WIRING_ACCEPTANCE_MARKER
}
