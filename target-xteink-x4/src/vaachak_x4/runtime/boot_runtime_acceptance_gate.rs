//! Phase 36D boot runtime acceptance gate.
//!
//! This module is intentionally metadata-only. It records whether the current
//! Phase 35G/35H and Phase 36A/36B/36C runtime layers are accepted as a safe
//! handoff surface, without touching SD/FAT/SPI/display/input behavior.

/// Phase marker emitted by the Phase 36D overlay and optional boot diagnostics.
pub const PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_MARKER: &str =
    "phase36d=x4-boot-runtime-acceptance-gate-ok";

/// Stable name for the Phase 36D runtime artifact.
pub const PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_NAME: &str = "x4-boot-runtime-acceptance-gate";

/// Stable status for the current metadata-only runtime handoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootRuntimeGateStatus {
    /// The metadata/facade layers are accepted as safe to report.
    Accepted,
    /// The runtime should not be treated as accepted.
    Blocked,
}

impl BootRuntimeGateStatus {
    /// Returns a compact status string suitable for boot logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Blocked => "blocked",
        }
    }
}

/// One acceptance check in the metadata-only boot-runtime gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootRuntimeAcceptanceCheck {
    /// Stable check id/name.
    pub name: &'static str,
    /// Whether this check is accepted for the current handoff stage.
    pub accepted: bool,
    /// Human-readable note for diagnostics and docs.
    pub note: &'static str,
}

impl BootRuntimeAcceptanceCheck {
    /// Returns `ok` or `blocked` for compact boot diagnostics.
    pub const fn result_str(self) -> &'static str {
        if self.accepted { "ok" } else { "blocked" }
    }
}

/// The Phase 36D acceptance gate summary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootRuntimeAcceptanceGate {
    /// Phase marker for this overlay.
    pub phase_marker: &'static str,
    /// Runtime handoff status.
    pub status: BootRuntimeGateStatus,
    /// Static acceptance checks.
    pub checks: &'static [BootRuntimeAcceptanceCheck],
    /// Compact summary for docs/logging.
    pub summary: &'static str,
}

impl BootRuntimeAcceptanceGate {
    /// Returns true when the gate is accepted and every check is accepted.
    pub fn is_accepted(&self) -> bool {
        if !matches!(self.status, BootRuntimeGateStatus::Accepted) {
            return false;
        }

        let mut i = 0;
        while i < self.checks.len() {
            if !self.checks[i].accepted {
                return false;
            }
            i += 1;
        }

        true
    }
}

/// Checks intentionally reference phase markers as strings only. This avoids
/// coupling Phase 36D to exact symbol names from prior overlay files.
pub const X4_BOOT_RUNTIME_ACCEPTANCE_CHECKS: [BootRuntimeAcceptanceCheck; 7] = [
    BootRuntimeAcceptanceCheck {
        name: "phase35g-state-registry",
        accepted: true,
        note: "Phase 35G typed state registry adapter is accepted as metadata-only.",
    },
    BootRuntimeAcceptanceCheck {
        name: "phase35h-spi-bus-facade",
        accepted: true,
        note: "Phase 35H SPI bus arbitration facade is accepted as metadata-only.",
    },
    BootRuntimeAcceptanceCheck {
        name: "phase36a-runtime-manifest",
        accepted: true,
        note: "Phase 36A boot runtime manifest is accepted as metadata-only.",
    },
    BootRuntimeAcceptanceCheck {
        name: "phase36b-marker-emitter",
        accepted: true,
        note: "Phase 36B marker emitter is accepted as metadata-only.",
    },
    BootRuntimeAcceptanceCheck {
        name: "phase36c-readiness-report",
        accepted: true,
        note: "Phase 36C readiness report is accepted as metadata-only.",
    },
    BootRuntimeAcceptanceCheck {
        name: "module-layout",
        accepted: true,
        note: "Existing vaachak_x4/runtime.rs remains authoritative; no runtime/mod.rs is created.",
    },
    BootRuntimeAcceptanceCheck {
        name: "behavior-movement",
        accepted: true,
        note: "No SD/FAT/SPI/display/input/runtime behavior is moved by Phase 36D.",
    },
];

/// Static Phase 36D acceptance gate.
pub const X4_BOOT_RUNTIME_ACCEPTANCE_GATE: BootRuntimeAcceptanceGate = BootRuntimeAcceptanceGate {
    phase_marker: PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_MARKER,
    status: BootRuntimeGateStatus::Accepted,
    checks: &X4_BOOT_RUNTIME_ACCEPTANCE_CHECKS,
    summary: "metadata-only boot runtime handoff accepted",
};

/// Returns the Phase 36D acceptance marker.
pub const fn boot_runtime_acceptance_gate_marker() -> &'static str {
    PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_MARKER
}

/// Returns the Phase 36D gate name.
pub const fn boot_runtime_acceptance_gate_name() -> &'static str {
    PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_NAME
}

/// Returns the static Phase 36D acceptance gate.
pub const fn boot_runtime_acceptance_gate() -> &'static BootRuntimeAcceptanceGate {
    &X4_BOOT_RUNTIME_ACCEPTANCE_GATE
}

/// Returns the static Phase 36D acceptance checks.
pub fn boot_runtime_acceptance_checks() -> &'static [BootRuntimeAcceptanceCheck] {
    &X4_BOOT_RUNTIME_ACCEPTANCE_CHECKS
}

/// Returns true when the current metadata-only handoff is accepted.
pub fn boot_runtime_is_metadata_only_accepted() -> bool {
    X4_BOOT_RUNTIME_ACCEPTANCE_GATE.is_accepted()
}
