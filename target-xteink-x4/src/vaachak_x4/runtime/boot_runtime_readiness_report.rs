//! Phase 36C boot/runtime readiness report adapter.
//!
//! This module is intentionally a pure metadata/reporting facade. It summarizes
//! the accepted boot/runtime extraction state from Phase 35G through Phase 36B
//! without calling or moving SD/FAT, SPI, display, input, power, or imported
//! Pulp runtime behavior.

#![allow(dead_code)]

/// Phase 36C acceptance marker.
pub const PHASE_36C_BOOT_RUNTIME_READINESS_REPORT_MARKER: &str =
    "phase36c=x4-boot-runtime-readiness-report-ok";

/// Accepted dependency marker from Phase 36B.
pub const PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER: &str =
    "phase36b=x4-boot-runtime-marker-emitter-ok";

/// Accepted dependency marker from Phase 36A.
pub const PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER: &str = "phase36a=x4-boot-runtime-manifest-ok";

/// Accepted dependency marker from Phase 35G.
pub const PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER: &str = "phase35g=x4-state-registry-adapter-ok";

/// Accepted dependency marker from Phase 35H.
pub const PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER: &str =
    "phase35h=x4-spi-bus-arbitration-facade-ok";

/// Stable readiness lines suitable for boot-console/reporting integration later.
pub const X4_BOOT_RUNTIME_READINESS_LINES: [&str; 11] = [
    PHASE_36C_BOOT_RUNTIME_READINESS_REPORT_MARKER,
    PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER,
    PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER,
    PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
    PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER,
    "state-adapters=progress,theme,metadata,bookmark,registry",
    "runtime-facades=manifest,marker-emitter,readiness-report",
    "physical-behavior-owner=imported-pulp-runtime",
    "storage-behavior-owner=imported-pulp-runtime",
    "display-input-behavior-owner=imported-pulp-runtime",
    "ready-for-next-extraction=true",
];

/// Pure metadata readiness report for the current X4 extraction boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct X4BootRuntimeReadinessReport {
    pub phase_marker: &'static str,
    pub manifest_marker: &'static str,
    pub marker_emitter_marker: &'static str,
    pub state_registry_marker: &'static str,
    pub spi_bus_facade_marker: &'static str,
    pub state_adapter_count: usize,
    pub runtime_facade_count: usize,
    pub physical_behavior_owner: &'static str,
    pub storage_behavior_owner: &'static str,
    pub display_input_behavior_owner: &'static str,
    pub behavior_moved: bool,
    pub boot_flow_changed: bool,
    pub ready_for_next_extraction: bool,
}

/// Singleton readiness report for Phase 36C.
pub const X4_BOOT_RUNTIME_READINESS_REPORT: X4BootRuntimeReadinessReport =
    X4BootRuntimeReadinessReport {
        phase_marker: PHASE_36C_BOOT_RUNTIME_READINESS_REPORT_MARKER,
        manifest_marker: PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER,
        marker_emitter_marker: PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER,
        state_registry_marker: PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
        spi_bus_facade_marker: PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER,
        state_adapter_count: 5,
        runtime_facade_count: 3,
        physical_behavior_owner: "imported Pulp runtime",
        storage_behavior_owner: "imported Pulp runtime",
        display_input_behavior_owner: "imported Pulp runtime",
        behavior_moved: false,
        boot_flow_changed: false,
        ready_for_next_extraction: true,
    };

impl X4BootRuntimeReadinessReport {
    /// Returns true when this report is still metadata-only.
    pub const fn is_metadata_only(&self) -> bool {
        !self.behavior_moved && !self.boot_flow_changed
    }

    /// Returns true when all expected Phase 36 runtime facades are represented.
    pub const fn has_runtime_facade_set(&self) -> bool {
        self.runtime_facade_count == 3
    }

    /// Returns true when the typed state adapter sequence is represented.
    pub const fn has_state_adapter_set(&self) -> bool {
        self.state_adapter_count == 5
    }

    /// Returns true when the next extraction step can safely start from this boundary.
    pub const fn is_ready_for_next_extraction(&self) -> bool {
        self.ready_for_next_extraction && self.is_metadata_only()
    }
}

/// Returns the singleton readiness report.
pub const fn phase36c_boot_runtime_readiness_report() -> X4BootRuntimeReadinessReport {
    X4_BOOT_RUNTIME_READINESS_REPORT
}

/// Returns the stable readiness lines.
pub const fn phase36c_boot_runtime_readiness_lines() -> [&'static str; 11] {
    X4_BOOT_RUNTIME_READINESS_LINES
}

/// Returns the Phase 36C marker.
pub const fn phase36c_marker() -> &'static str {
    PHASE_36C_BOOT_RUNTIME_READINESS_REPORT_MARKER
}
