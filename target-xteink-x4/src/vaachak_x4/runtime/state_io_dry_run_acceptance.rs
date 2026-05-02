//! Phase 36K — State I/O Dry-Run Acceptance.
//!
//! This module is intentionally metadata-only. It accepts the Phase 36J dry-run
//! backend lane as bounded and ready for a later real backend binding, without
//! performing SD/FAT/SPI/display/input/power behavior.

/// Boot/runtime marker emitted by the Phase 36K overlay checks.
pub const PHASE_36K_STATE_IO_DRY_RUN_ACCEPTANCE_MARKER: &str =
    "phase36k=x4-state-io-dry-run-acceptance-ok";

/// Phase name for diagnostics and future boot/runtime reports.
pub const PHASE_36K_NAME: &str = "Phase 36K — State I/O Dry-Run Acceptance";

/// Previous phase marker accepted by this dry-run acceptance layer.
pub const ACCEPTED_PHASE_36J_MARKER: &str = "phase36j=x4-state-io-backend-dry-run-ok";

/// State record classes covered by the dry-run acceptance lane.
pub const ACCEPTED_STATE_RECORD_CLASSES: [&str; 5] = [".PRG", ".THM", ".MTA", ".BKM", "BMIDX.TXT"];

/// Dry-run operations that must remain side-effect free in this phase.
pub const ACCEPTED_DRY_RUN_OPERATIONS: [&str; 5] = ["read", "write", "upsert", "delete", "index"];

/// Explicitly forbidden live behavior for this phase.
pub const FORBIDDEN_LIVE_BEHAVIOR: [&str; 6] = [
    "sd_fat_write",
    "sd_fat_read",
    "spi_transaction",
    "display_update",
    "input_poll",
    "power_state_change",
];

/// Compile-time summary of the dry-run acceptance boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoDryRunAcceptance {
    pub marker: &'static str,
    pub phase_name: &'static str,
    pub accepted_previous_marker: &'static str,
    pub record_classes: &'static [&'static str],
    pub dry_run_operations: &'static [&'static str],
    pub forbidden_live_behavior: &'static [&'static str],
    pub side_effect_free: bool,
    pub ready_for_real_backend_planning: bool,
}

/// Canonical Phase 36K dry-run acceptance report.
pub const STATE_IO_DRY_RUN_ACCEPTANCE: StateIoDryRunAcceptance = StateIoDryRunAcceptance {
    marker: PHASE_36K_STATE_IO_DRY_RUN_ACCEPTANCE_MARKER,
    phase_name: PHASE_36K_NAME,
    accepted_previous_marker: ACCEPTED_PHASE_36J_MARKER,
    record_classes: &ACCEPTED_STATE_RECORD_CLASSES,
    dry_run_operations: &ACCEPTED_DRY_RUN_OPERATIONS,
    forbidden_live_behavior: &FORBIDDEN_LIVE_BEHAVIOR,
    side_effect_free: true,
    ready_for_real_backend_planning: true,
};

/// Returns the canonical Phase 36K marker.
pub const fn phase36k_marker() -> &'static str {
    PHASE_36K_STATE_IO_DRY_RUN_ACCEPTANCE_MARKER
}

/// Returns the canonical Phase 36K acceptance report.
pub const fn state_io_dry_run_acceptance() -> StateIoDryRunAcceptance {
    STATE_IO_DRY_RUN_ACCEPTANCE
}

/// Returns whether this phase is intentionally side-effect free.
pub const fn state_io_dry_run_is_side_effect_free() -> bool {
    STATE_IO_DRY_RUN_ACCEPTANCE.side_effect_free
}

/// Returns whether the dry-run lane is ready for real backend planning.
pub const fn state_io_dry_run_ready_for_real_backend_planning() -> bool {
    STATE_IO_DRY_RUN_ACCEPTANCE.ready_for_real_backend_planning
}
