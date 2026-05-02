//! Phase 36V — State I/O read-probe acceptance metadata.
//!
//! This module accepts the Phase 36U read-probe lane as bounded and
//! side-effect free. It intentionally does not call SD/FAT/SPI/display/input
//! or power code.

#![allow(dead_code)]

/// Phase 36V boot/runtime marker.
pub const PHASE_36V_STATE_IO_READ_PROBE_ACCEPTANCE_MARKER: &str =
    "phase36v=x4-state-io-read-probe-acceptance-ok";

/// Phase accepted by this report.
pub const ACCEPTED_READ_PROBE_PHASE: &str = "phase36u=x4-state-io-real-backend-read-probe-ok";

/// Next intended lane after this acceptance layer.
pub const NEXT_STATE_IO_LANE: &str = "real-backend-read-probe-backend-adapter";

/// Record kinds covered by the accepted read-probe lane.
pub const ACCEPTED_READ_PROBE_RECORDS: [&str; 5] = [".PRG", ".THM", ".MTA", ".BKM", "BMIDX.TXT"];

/// Safety assertions for the read-probe lane.
pub const READ_PROBE_ACCEPTANCE_ASSERTIONS: [&str; 6] = [
    "read-probe lane is metadata-only",
    "read-probe lane models candidate/fallback decisions only",
    "read-probe lane performs no file open",
    "read-probe lane performs no file read",
    "read-probe lane performs no SPI or SD transaction",
    "read-probe lane does not change boot flow",
];

/// Acceptance status for a read-probe lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadProbeAcceptanceStatus {
    /// The lane is accepted as side-effect free.
    Accepted,
    /// The lane should remain blocked.
    Blocked,
}

impl StateIoReadProbeAcceptanceStatus {
    /// Compact status string for boot/runtime reporting.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Blocked => "blocked",
        }
    }
}

/// Acceptance report for Phase 36V.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadProbeAcceptanceReport {
    /// Phase marker emitted by this module.
    pub marker: &'static str,
    /// Previously accepted read-probe phase.
    pub accepted_phase: &'static str,
    /// Acceptance status.
    pub status: StateIoReadProbeAcceptanceStatus,
    /// True when the accepted lane is side-effect free.
    pub side_effect_free: bool,
    /// True when no real backend calls are permitted by this phase.
    pub real_backend_calls_permitted: bool,
    /// Next intended lane.
    pub next_lane: &'static str,
}

impl StateIoReadProbeAcceptanceReport {
    /// Returns true when this report allows the project to proceed to the next
    /// design-only backend adapter phase.
    pub const fn permits_next_lane(self) -> bool {
        matches!(self.status, StateIoReadProbeAcceptanceStatus::Accepted)
            && self.side_effect_free
            && !self.real_backend_calls_permitted
    }
}

/// Canonical Phase 36V acceptance report.
pub const STATE_IO_READ_PROBE_ACCEPTANCE_REPORT: StateIoReadProbeAcceptanceReport =
    StateIoReadProbeAcceptanceReport {
        marker: PHASE_36V_STATE_IO_READ_PROBE_ACCEPTANCE_MARKER,
        accepted_phase: ACCEPTED_READ_PROBE_PHASE,
        status: StateIoReadProbeAcceptanceStatus::Accepted,
        side_effect_free: true,
        real_backend_calls_permitted: false,
        next_lane: NEXT_STATE_IO_LANE,
    };

/// Convenience marker accessor for boot/runtime logging.
pub const fn phase36v_marker() -> &'static str {
    PHASE_36V_STATE_IO_READ_PROBE_ACCEPTANCE_MARKER
}

/// Convenience report accessor.
pub const fn state_io_read_probe_acceptance_report() -> StateIoReadProbeAcceptanceReport {
    STATE_IO_READ_PROBE_ACCEPTANCE_REPORT
}
