//! Phase 36I — State I/O backend readiness gate.
//!
//! This module is intentionally metadata-only. It records whether the typed
//! state I/O backend lane is ready to be bound in a later phase without moving
//! SD/FAT/SPI/display/input/power behavior in this phase.

#![allow(dead_code)]

/// Phase 36I acceptance marker emitted by the overlay/check scripts.
pub const PHASE_36I_STATE_IO_BACKEND_READINESS_GATE_MARKER: &str =
    "phase36i=x4-state-io-backend-readiness-gate-ok";

/// Human-readable phase name for diagnostics and docs.
pub const PHASE_36I_NAME: &str = "Phase 36I — State I/O Backend Readiness Gate";

/// State record kinds covered by the backend readiness gate.
pub const PHASE_36I_TYPED_STATE_RECORDS: [&str; 5] = [".PRG", ".THM", ".MTA", ".BKM", "BMIDX.TXT"];

/// Next intended implementation lane after the readiness gate is accepted.
pub const PHASE_36I_NEXT_LANE: &str = "typed-state-backend-dry-run";

/// Metadata-only readiness record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoBackendReadinessGate {
    /// Phase marker for boot/runtime diagnostics.
    pub marker: &'static str,
    /// Whether the backend binding contract exists and is accepted.
    pub binding_contract_present: bool,
    /// Whether this phase moves the real SD/FAT backend.
    pub moves_storage_backend: bool,
    /// Whether this phase moves shared SPI arbitration.
    pub moves_spi_backend: bool,
    /// Whether this phase changes display behavior.
    pub changes_display_behavior: bool,
    /// Whether this phase changes input behavior.
    pub changes_input_behavior: bool,
    /// Whether this phase changes power behavior.
    pub changes_power_behavior: bool,
    /// Number of typed state records in scope.
    pub typed_state_record_count: usize,
    /// Next implementation lane after this gate.
    pub next_lane: &'static str,
}

impl StateIoBackendReadinessGate {
    /// Returns true only when this gate is a pure metadata/readiness layer.
    pub const fn is_metadata_only(self) -> bool {
        !self.moves_storage_backend
            && !self.moves_spi_backend
            && !self.changes_display_behavior
            && !self.changes_input_behavior
            && !self.changes_power_behavior
    }

    /// Returns true when the typed-state backend lane is ready for a dry-run
    /// backend implementation in a later phase.
    pub const fn ready_for_dry_run_backend(self) -> bool {
        self.binding_contract_present
            && self.is_metadata_only()
            && self.typed_state_record_count == PHASE_36I_TYPED_STATE_RECORDS.len()
    }
}

/// Canonical readiness gate instance used by boot/runtime diagnostics.
pub const PHASE_36I_STATE_IO_BACKEND_READINESS_GATE: StateIoBackendReadinessGate =
    StateIoBackendReadinessGate {
        marker: PHASE_36I_STATE_IO_BACKEND_READINESS_GATE_MARKER,
        binding_contract_present: true,
        moves_storage_backend: false,
        moves_spi_backend: false,
        changes_display_behavior: false,
        changes_input_behavior: false,
        changes_power_behavior: false,
        typed_state_record_count: PHASE_36I_TYPED_STATE_RECORDS.len(),
        next_lane: PHASE_36I_NEXT_LANE,
    };

/// Compact boot/runtime status for logs or future diagnostics.
pub const fn phase36i_backend_readiness_status() -> StateIoBackendReadinessGate {
    PHASE_36I_STATE_IO_BACKEND_READINESS_GATE
}

/// Returns the accepted marker for this phase.
pub const fn phase36i_marker() -> &'static str {
    PHASE_36I_STATE_IO_BACKEND_READINESS_GATE_MARKER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_gate_is_metadata_only() {
        assert!(PHASE_36I_STATE_IO_BACKEND_READINESS_GATE.is_metadata_only());
    }

    #[test]
    fn readiness_gate_allows_dry_run_backend_lane() {
        assert!(PHASE_36I_STATE_IO_BACKEND_READINESS_GATE.ready_for_dry_run_backend());
    }

    #[test]
    fn marker_is_stable() {
        assert_eq!(
            phase36i_marker(),
            "phase36i=x4-state-io-backend-readiness-gate-ok"
        );
    }
}
