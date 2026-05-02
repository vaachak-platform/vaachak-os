//! Phase 36T — State I/O Null Backend Acceptance Overlay.
//!
//! This module accepts the Phase 36S null backend as a safe, side-effect-free
//! implementation shape. It deliberately performs no storage, display, input,
//! power, or SPI work.
//!
//! The purpose is to freeze the null-backend lane before a later phase decides
//! how a real X4 SD/FAT state backend should be introduced.

#![allow(dead_code)]

use super::state_io_null_backend::{
    STATE_IO_NULL_BACKEND_STATUS, StateIoNullBackendStatus, phase36s_is_accepted, phase36s_marker,
};

/// Phase 36T boot/build marker.
pub const PHASE_36T_STATE_IO_NULL_BACKEND_ACCEPTANCE_MARKER: &str =
    "phase36t=x4-state-io-null-backend-acceptance-ok";

/// Prior null-backend marker required before this acceptance layer is valid.
pub const REQUIRED_PHASE_36S_MARKER: &str = "phase36s=x4-state-io-null-backend-ok";

/// Minimum typed state records required for the null-backend lane.
pub const REQUIRED_ACCEPTED_NULL_BACKEND_RECORD_COUNT: usize = 5;

/// Minimum backend operations required for the null-backend lane.
pub const REQUIRED_ACCEPTED_NULL_BACKEND_OPERATION_COUNT: usize = 5;

/// Minimum guardrails required for the null-backend lane.
pub const REQUIRED_ACCEPTED_NULL_BACKEND_GUARDRAIL_COUNT: usize = 10;

/// Compile-time acceptance decision for the Phase 36S null backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullBackendAcceptanceDecision {
    Accepted,
    Rejected,
}

impl NullBackendAcceptanceDecision {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }

    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// Side-effect-free acceptance report for the Phase 36S null backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoNullBackendAcceptanceReport {
    pub marker: &'static str,
    pub required_null_backend_marker: &'static str,
    pub observed_null_backend_marker: &'static str,
    pub decision: NullBackendAcceptanceDecision,
    pub null_backend_accepted: bool,
    pub backend_name: &'static str,
    pub backend_default_enabled: bool,
    pub side_effects_enabled: bool,
    pub record_count: usize,
    pub operation_count: usize,
    pub guardrail_count: usize,
    pub required_record_count: usize,
    pub required_operation_count: usize,
    pub required_guardrail_count: usize,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
    pub spi_behavior_moved: bool,
    pub next_lane: &'static str,
}

impl StateIoNullBackendAcceptanceReport {
    pub const fn from_status(status: StateIoNullBackendStatus) -> Self {
        let null_backend_accepted = phase36s_is_accepted();
        let has_required_records =
            status.record_count >= REQUIRED_ACCEPTED_NULL_BACKEND_RECORD_COUNT;
        let has_required_operations =
            status.operation_count >= REQUIRED_ACCEPTED_NULL_BACKEND_OPERATION_COUNT;
        let has_required_guardrails =
            status.guardrail_count >= REQUIRED_ACCEPTED_NULL_BACKEND_GUARDRAIL_COUNT;
        let accepted = null_backend_accepted
            && has_required_records
            && has_required_operations
            && has_required_guardrails
            && !status.backend_default_enabled
            && !status.side_effects_enabled
            && !status.storage_behavior_moved
            && !status.display_behavior_moved
            && !status.input_behavior_moved
            && !status.power_behavior_moved
            && !status.spi_behavior_moved;

        Self {
            marker: PHASE_36T_STATE_IO_NULL_BACKEND_ACCEPTANCE_MARKER,
            required_null_backend_marker: REQUIRED_PHASE_36S_MARKER,
            observed_null_backend_marker: phase36s_marker(),
            decision: if accepted {
                NullBackendAcceptanceDecision::Accepted
            } else {
                NullBackendAcceptanceDecision::Rejected
            },
            null_backend_accepted,
            backend_name: status.backend_name,
            backend_default_enabled: status.backend_default_enabled,
            side_effects_enabled: status.side_effects_enabled,
            record_count: status.record_count,
            operation_count: status.operation_count,
            guardrail_count: status.guardrail_count,
            required_record_count: REQUIRED_ACCEPTED_NULL_BACKEND_RECORD_COUNT,
            required_operation_count: REQUIRED_ACCEPTED_NULL_BACKEND_OPERATION_COUNT,
            required_guardrail_count: REQUIRED_ACCEPTED_NULL_BACKEND_GUARDRAIL_COUNT,
            storage_behavior_moved: status.storage_behavior_moved,
            display_behavior_moved: status.display_behavior_moved,
            input_behavior_moved: status.input_behavior_moved,
            power_behavior_moved: status.power_behavior_moved,
            spi_behavior_moved: status.spi_behavior_moved,
            next_lane: "state-io-real-backend-read-probe-design",
        }
    }

    pub const fn accepted() -> Self {
        Self::from_status(STATE_IO_NULL_BACKEND_STATUS)
    }

    pub const fn is_accepted(self) -> bool {
        self.decision.is_accepted()
    }

    pub const fn decision_label(self) -> &'static str {
        self.decision.label()
    }
}

/// Compile-time acceptance report for the current null backend.
pub const STATE_IO_NULL_BACKEND_ACCEPTANCE_REPORT: StateIoNullBackendAcceptanceReport =
    StateIoNullBackendAcceptanceReport::accepted();

/// Return the accepted Phase 36T marker for boot/runtime status reporting.
pub const fn phase36t_marker() -> &'static str {
    PHASE_36T_STATE_IO_NULL_BACKEND_ACCEPTANCE_MARKER
}

/// Return the Phase 36S marker that this acceptance layer covers.
pub const fn phase36t_accepted_null_backend_marker() -> &'static str {
    STATE_IO_NULL_BACKEND_ACCEPTANCE_REPORT.observed_null_backend_marker
}

/// Return the side-effect-free acceptance report.
pub const fn phase36t_acceptance_report() -> StateIoNullBackendAcceptanceReport {
    STATE_IO_NULL_BACKEND_ACCEPTANCE_REPORT
}

/// Return whether the Phase 36S null backend is accepted by Phase 36T.
pub const fn phase36t_is_accepted() -> bool {
    STATE_IO_NULL_BACKEND_ACCEPTANCE_REPORT.is_accepted()
}
