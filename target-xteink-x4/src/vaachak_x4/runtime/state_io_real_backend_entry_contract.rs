//! Phase 36Q — State I/O Real Backend Entry Contract Overlay.
//!
//! This module defines the side-effect-free entry contract for the first future
//! typed state backend implementation phase. It does not perform backend work.
//!
//! The purpose is to make the boundary explicit before any later phase binds
//! Vaachak-owned typed state records to the X4 storage runtime.

#![allow(dead_code)]

use super::state_io_backend_handoff_checklist::{
    BackendHandoffChecklist, phase36p_handoff_checklist, phase36p_is_ready_for_backend_design,
    phase36p_marker,
};

/// Phase 36Q boot/build marker.
pub const PHASE_36Q_STATE_IO_REAL_BACKEND_ENTRY_CONTRACT_MARKER: &str =
    "phase36q=x4-state-io-real-backend-entry-contract-ok";

/// Prior handoff marker required before this entry contract is valid.
pub const REQUIRED_PHASE_36P_MARKER: &str = "phase36p=x4-state-io-backend-handoff-checklist-ok";

/// Typed state records covered by the first real backend lane.
pub const STATE_IO_REAL_BACKEND_ENTRY_RECORDS: [&str; 5] = [
    "state/<BOOKID>.PRG",
    "state/<BOOKID>.THM",
    "state/<BOOKID>.MTA",
    "state/<BOOKID>.BKM",
    "state/BMIDX.TXT",
];

/// Scope allowed for the next real-backend scaffold phase.
pub const STATE_IO_REAL_BACKEND_ALLOWED_SCOPE: [&str; 8] = [
    "typed-state-records-only",
    "progress-theme-metadata-bookmark-index-only",
    "backend-trait-or-adapter-shape-only",
    "no-reader-ui-routing-change",
    "no-display-refresh-change",
    "no-input-event-change",
    "no-power-policy-change",
    "no-spi-arbitration-change",
];

/// Guardrails required before any future backend implementation is enabled.
pub const STATE_IO_REAL_BACKEND_ENTRY_GUARDRAILS: [&str; 9] = [
    "phase36p-handoff-ready",
    "shadow-write-plan-remains-required",
    "commit-plan-remains-required",
    "dry-run-path-remains-available",
    "backend-default-disabled",
    "existing-pulp-runtime-remains-authoritative",
    "typed-state-format-remains-83-safe",
    "rollback-path-required",
    "hardware-behavior-unmoved",
];

/// Minimum number of covered typed state records for backend entry readiness.
pub const REQUIRED_BACKEND_ENTRY_RECORD_COUNT: usize = 5;

/// Minimum number of allowed-scope entries for backend entry readiness.
pub const REQUIRED_BACKEND_ENTRY_SCOPE_COUNT: usize = 8;

/// Minimum number of guardrail entries for backend entry readiness.
pub const REQUIRED_BACKEND_ENTRY_GUARDRAIL_COUNT: usize = 9;

/// Compile-time decision for the first real backend lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealBackendEntryDecision {
    ReadyForScaffold,
    Blocked,
}

impl RealBackendEntryDecision {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReadyForScaffold => "ready-for-scaffold",
            Self::Blocked => "blocked",
        }
    }

    pub const fn is_ready(self) -> bool {
        matches!(self, Self::ReadyForScaffold)
    }
}

/// Side-effect-free entry contract for a future typed state backend scaffold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealBackendEntryContract {
    pub marker: &'static str,
    pub required_handoff_marker: &'static str,
    pub observed_handoff_marker: &'static str,
    pub decision: RealBackendEntryDecision,
    pub handoff_ready: bool,
    pub record_count: usize,
    pub allowed_scope_count: usize,
    pub guardrail_count: usize,
    pub required_record_count: usize,
    pub required_scope_count: usize,
    pub required_guardrail_count: usize,
    pub backend_default_enabled: bool,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
    pub spi_behavior_moved: bool,
    pub next_phase: &'static str,
}

impl RealBackendEntryContract {
    pub const fn from_handoff(checklist: BackendHandoffChecklist) -> Self {
        let record_count = STATE_IO_REAL_BACKEND_ENTRY_RECORDS.len();
        let allowed_scope_count = STATE_IO_REAL_BACKEND_ALLOWED_SCOPE.len();
        let guardrail_count = STATE_IO_REAL_BACKEND_ENTRY_GUARDRAILS.len();
        let has_records = record_count >= REQUIRED_BACKEND_ENTRY_RECORD_COUNT;
        let has_scope = allowed_scope_count >= REQUIRED_BACKEND_ENTRY_SCOPE_COUNT;
        let has_guardrails = guardrail_count >= REQUIRED_BACKEND_ENTRY_GUARDRAIL_COUNT;
        let handoff_ready = checklist.is_ready() && phase36p_is_ready_for_backend_design();
        let backend_default_enabled = false;
        let storage_behavior_moved = false;
        let display_behavior_moved = false;
        let input_behavior_moved = false;
        let power_behavior_moved = false;
        let spi_behavior_moved = false;
        let ready = handoff_ready
            && has_records
            && has_scope
            && has_guardrails
            && !backend_default_enabled
            && !storage_behavior_moved
            && !display_behavior_moved
            && !input_behavior_moved
            && !power_behavior_moved
            && !spi_behavior_moved;

        Self {
            marker: PHASE_36Q_STATE_IO_REAL_BACKEND_ENTRY_CONTRACT_MARKER,
            required_handoff_marker: REQUIRED_PHASE_36P_MARKER,
            observed_handoff_marker: phase36p_marker(),
            decision: if ready {
                RealBackendEntryDecision::ReadyForScaffold
            } else {
                RealBackendEntryDecision::Blocked
            },
            handoff_ready,
            record_count,
            allowed_scope_count,
            guardrail_count,
            required_record_count: REQUIRED_BACKEND_ENTRY_RECORD_COUNT,
            required_scope_count: REQUIRED_BACKEND_ENTRY_SCOPE_COUNT,
            required_guardrail_count: REQUIRED_BACKEND_ENTRY_GUARDRAIL_COUNT,
            backend_default_enabled,
            storage_behavior_moved,
            display_behavior_moved,
            input_behavior_moved,
            power_behavior_moved,
            spi_behavior_moved,
            next_phase: "phase37a-state-io-real-backend-scaffold",
        }
    }

    pub const fn current() -> Self {
        Self::from_handoff(phase36p_handoff_checklist())
    }

    pub const fn is_ready(self) -> bool {
        self.decision.is_ready()
    }

    pub const fn decision_label(self) -> &'static str {
        self.decision.label()
    }
}

/// Compile-time entry contract for the next typed state backend lane.
pub const STATE_IO_REAL_BACKEND_ENTRY_CONTRACT: RealBackendEntryContract =
    RealBackendEntryContract::current();

/// Return the accepted Phase 36Q marker for boot/runtime status reporting.
pub const fn phase36q_marker() -> &'static str {
    PHASE_36Q_STATE_IO_REAL_BACKEND_ENTRY_CONTRACT_MARKER
}

/// Return the side-effect-free backend entry contract.
pub const fn phase36q_entry_contract() -> RealBackendEntryContract {
    STATE_IO_REAL_BACKEND_ENTRY_CONTRACT
}

/// Return whether the future backend lane is ready for the scaffold-only phase.
pub const fn phase36q_is_ready_for_backend_scaffold() -> bool {
    STATE_IO_REAL_BACKEND_ENTRY_CONTRACT.is_ready()
}
