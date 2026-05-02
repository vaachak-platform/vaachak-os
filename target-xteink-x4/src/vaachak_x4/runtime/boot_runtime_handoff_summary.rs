//! Phase 36E boot runtime handoff summary.
//!
//! This module is intentionally metadata-only. It provides one compact handoff
//! summary across Phase 35G/35H and Phase 36A/36B/36C/36D so the next
//! extraction phase can start from an explicit accepted boundary.
//!
//! No SD/FAT, SPI, display, input, power, or imported Pulp runtime behavior is
//! moved or called by this module.

#![allow(dead_code)]

/// Phase marker emitted by the Phase 36E overlay and optional diagnostics.
pub const PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_MARKER: &str =
    "phase36e=x4-boot-runtime-handoff-summary-ok";

/// Stable name for the Phase 36E runtime artifact.
pub const PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_NAME: &str = "x4-boot-runtime-handoff-summary";

/// Accepted dependency marker from Phase 36D.
pub const PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_MARKER: &str =
    "phase36d=x4-boot-runtime-acceptance-gate-ok";

/// Accepted dependency marker from Phase 36C.
pub const PHASE_36C_BOOT_RUNTIME_READINESS_REPORT_MARKER: &str =
    "phase36c=x4-boot-runtime-readiness-report-ok";

/// Accepted dependency marker from Phase 36B.
pub const PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER: &str =
    "phase36b=x4-boot-runtime-marker-emitter-ok";

/// Accepted dependency marker from Phase 36A.
pub const PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER: &str = "phase36a=x4-boot-runtime-manifest-ok";

/// Accepted dependency marker from Phase 35H.
pub const PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER: &str =
    "phase35h=x4-spi-bus-arbitration-facade-ok";

/// Accepted dependency marker from Phase 35G.
pub const PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER: &str = "phase35g=x4-state-registry-adapter-ok";

/// Stage represented by the current boot/runtime handoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootRuntimeHandoffStage {
    /// The runtime surface is accepted as metadata-only.
    MetadataOnly,
    /// The next phase may start moving one real behavior path behind a facade.
    ReadyForBehaviorExtraction,
}

impl BootRuntimeHandoffStage {
    /// Returns a compact stage label suitable for diagnostics.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata-only",
            Self::ReadyForBehaviorExtraction => "ready-for-behavior-extraction",
        }
    }
}

/// One line in the handoff summary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootRuntimeHandoffLine {
    /// Stable key/name.
    pub key: &'static str,
    /// Stable value for docs and optional boot diagnostics.
    pub value: &'static str,
}

impl BootRuntimeHandoffLine {
    /// Returns true when this handoff line is intentionally static metadata.
    pub const fn is_static_metadata(self) -> bool {
        !self.key.is_empty() && !self.value.is_empty()
    }
}

/// Stable handoff lines in deterministic order.
pub const X4_BOOT_RUNTIME_HANDOFF_LINES: [BootRuntimeHandoffLine; 12] = [
    BootRuntimeHandoffLine {
        key: "phase36e",
        value: PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "phase36d",
        value: PHASE_36D_BOOT_RUNTIME_ACCEPTANCE_GATE_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "phase36c",
        value: PHASE_36C_BOOT_RUNTIME_READINESS_REPORT_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "phase36b",
        value: PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "phase36a",
        value: PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "phase35h",
        value: PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "phase35g",
        value: PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
    },
    BootRuntimeHandoffLine {
        key: "state-surface",
        value: "progress,theme,metadata,bookmark,registry",
    },
    BootRuntimeHandoffLine {
        key: "runtime-surface",
        value: "manifest,marker-emitter,readiness-report,acceptance-gate,handoff-summary",
    },
    BootRuntimeHandoffLine {
        key: "hardware-owner",
        value: "imported-pulp-runtime",
    },
    BootRuntimeHandoffLine {
        key: "behavior-moved",
        value: "false",
    },
    BootRuntimeHandoffLine {
        key: "next-safe-phase",
        value: "single-behavior-facade-extraction",
    },
];

/// Summary for the current accepted boot/runtime handoff surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootRuntimeHandoffSummary {
    /// Phase marker for this overlay.
    pub phase_marker: &'static str,
    /// Stable artifact name.
    pub name: &'static str,
    /// Current handoff stage.
    pub stage: BootRuntimeHandoffStage,
    /// Whether the metadata-only boundary has been accepted.
    pub accepted: bool,
    /// Number of typed state adapters represented in the handoff.
    pub state_adapter_count: usize,
    /// Number of runtime facade modules represented in the handoff.
    pub runtime_facade_count: usize,
    /// Whether any real hardware/storage/display/input behavior has been moved.
    pub behavior_moved: bool,
    /// Stable handoff lines.
    pub lines: &'static [BootRuntimeHandoffLine],
    /// Recommended next class of work.
    pub next_safe_phase: &'static str,
}

impl BootRuntimeHandoffSummary {
    /// Returns true when this summary is still a metadata-only surface.
    pub const fn is_metadata_only(self) -> bool {
        self.accepted && !self.behavior_moved
    }

    /// Returns true when the summary contains the expected typed state surface.
    pub const fn has_state_surface(self) -> bool {
        self.state_adapter_count == 5
    }

    /// Returns true when the summary contains the expected runtime facade surface.
    pub const fn has_runtime_surface(self) -> bool {
        self.runtime_facade_count == 5
    }

    /// Returns true when the next phase may start from this handoff boundary.
    pub const fn ready_for_next_phase(self) -> bool {
        self.is_metadata_only() && self.has_state_surface() && self.has_runtime_surface()
    }
}

/// Static Phase 36E handoff summary.
pub const X4_BOOT_RUNTIME_HANDOFF_SUMMARY: BootRuntimeHandoffSummary = BootRuntimeHandoffSummary {
    phase_marker: PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_MARKER,
    name: PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_NAME,
    stage: BootRuntimeHandoffStage::ReadyForBehaviorExtraction,
    accepted: true,
    state_adapter_count: 5,
    runtime_facade_count: 5,
    behavior_moved: false,
    lines: &X4_BOOT_RUNTIME_HANDOFF_LINES,
    next_safe_phase: "extract one real behavior path behind a facade",
};

/// Returns the Phase 36E marker.
pub const fn boot_runtime_handoff_summary_marker() -> &'static str {
    PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_MARKER
}

/// Returns the Phase 36E artifact name.
pub const fn boot_runtime_handoff_summary_name() -> &'static str {
    PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_NAME
}

/// Returns the static Phase 36E handoff summary.
pub const fn boot_runtime_handoff_summary() -> &'static BootRuntimeHandoffSummary {
    &X4_BOOT_RUNTIME_HANDOFF_SUMMARY
}

/// Returns the stable Phase 36E handoff lines.
pub fn boot_runtime_handoff_lines() -> &'static [BootRuntimeHandoffLine] {
    &X4_BOOT_RUNTIME_HANDOFF_LINES
}

/// Returns true when the boot/runtime boundary is accepted for the next phase.
pub const fn boot_runtime_handoff_ready_for_next_phase() -> bool {
    X4_BOOT_RUNTIME_HANDOFF_SUMMARY.ready_for_next_phase()
}
