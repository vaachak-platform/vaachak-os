//! Phase 36G boot runtime contract catalog.
//!
//! This module is intentionally metadata-only. It gathers the accepted Phase
//! 35C-36F boundary/facade phases into one stable catalog that future real
//! backend bindings can reference before behavior is moved.
//!
//! No SD/FAT, SPI, display, input, power, or imported Pulp runtime behavior is
//! moved or called by this module.

#![allow(dead_code)]

/// Phase marker emitted by the Phase 36G overlay and optional diagnostics.
pub const PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_MARKER: &str =
    "phase36g=x4-boot-runtime-contract-catalog-ok";

/// Stable name for the Phase 36G runtime artifact.
pub const PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_NAME: &str = "x4-boot-runtime-contract-catalog";

/// Accepted Phase 36F dependency marker.
pub const PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_MARKER: &str =
    "phase36f=x4-state-io-runtime-boundary-ok";

/// A boundary/facade phase represented by the boot runtime contract catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootRuntimeContractPhase {
    /// Phase 35C: progress state I/O adapter, `state/<BOOKID>.PRG`.
    Phase35CProgressStateIoAdapter,
    /// Phase 35D: theme/layout state I/O adapter, `state/<BOOKID>.THM`.
    Phase35DThemeStateIoAdapter,
    /// Phase 35E: metadata state I/O adapter, `state/<BOOKID>.MTA`.
    Phase35EMetadataStateIoAdapter,
    /// Phase 35F: bookmark state I/O adapter, `state/<BOOKID>.BKM`.
    Phase35FBookmarkStateIoAdapter,
    /// Phase 35G: typed state registry adapter.
    Phase35GStateRegistryAdapter,
    /// Phase 35H: shared SPI bus arbitration facade.
    Phase35HSpiBusArbitrationFacade,
    /// Phase 36A: boot runtime manifest adapter.
    Phase36ABootRuntimeManifest,
    /// Phase 36B: boot runtime marker emitter.
    Phase36BBootRuntimeMarkerEmitter,
    /// Phase 36C: boot runtime readiness report.
    Phase36CBootRuntimeReadinessReport,
    /// Phase 36D: boot runtime acceptance gate.
    Phase36DBootRuntimeAcceptanceGate,
    /// Phase 36E: boot runtime handoff summary.
    Phase36EBootRuntimeHandoffSummary,
    /// Phase 36F: state I/O runtime boundary.
    Phase36FStateIoRuntimeBoundary,
}

impl BootRuntimeContractPhase {
    /// Returns the stable phase id.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Phase35CProgressStateIoAdapter => "35C",
            Self::Phase35DThemeStateIoAdapter => "35D",
            Self::Phase35EMetadataStateIoAdapter => "35E",
            Self::Phase35FBookmarkStateIoAdapter => "35F",
            Self::Phase35GStateRegistryAdapter => "35G",
            Self::Phase35HSpiBusArbitrationFacade => "35H",
            Self::Phase36ABootRuntimeManifest => "36A",
            Self::Phase36BBootRuntimeMarkerEmitter => "36B",
            Self::Phase36CBootRuntimeReadinessReport => "36C",
            Self::Phase36DBootRuntimeAcceptanceGate => "36D",
            Self::Phase36EBootRuntimeHandoffSummary => "36E",
            Self::Phase36FStateIoRuntimeBoundary => "36F",
        }
    }

    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Phase35CProgressStateIoAdapter => "progress-state-io-adapter",
            Self::Phase35DThemeStateIoAdapter => "theme-state-io-adapter",
            Self::Phase35EMetadataStateIoAdapter => "metadata-state-io-adapter",
            Self::Phase35FBookmarkStateIoAdapter => "bookmark-state-io-adapter",
            Self::Phase35GStateRegistryAdapter => "state-registry-adapter",
            Self::Phase35HSpiBusArbitrationFacade => "spi-bus-arbitration-facade",
            Self::Phase36ABootRuntimeManifest => "boot-runtime-manifest",
            Self::Phase36BBootRuntimeMarkerEmitter => "boot-runtime-marker-emitter",
            Self::Phase36CBootRuntimeReadinessReport => "boot-runtime-readiness-report",
            Self::Phase36DBootRuntimeAcceptanceGate => "boot-runtime-acceptance-gate",
            Self::Phase36EBootRuntimeHandoffSummary => "boot-runtime-handoff-summary",
            Self::Phase36FStateIoRuntimeBoundary => "state-io-runtime-boundary",
        }
    }

    /// Returns the accepted marker string for the represented phase.
    pub const fn marker(self) -> &'static str {
        match self {
            Self::Phase35CProgressStateIoAdapter => "phase35c=x4-progress-state-io-adapter-ok",
            Self::Phase35DThemeStateIoAdapter => "phase35d=x4-theme-state-io-adapter-ok",
            Self::Phase35EMetadataStateIoAdapter => "phase35e=x4-metadata-state-io-adapter-ok",
            Self::Phase35FBookmarkStateIoAdapter => "phase35f=x4-bookmark-state-io-adapter-ok",
            Self::Phase35GStateRegistryAdapter => "phase35g=x4-state-registry-adapter-ok",
            Self::Phase35HSpiBusArbitrationFacade => "phase35h=x4-spi-bus-arbitration-facade-ok",
            Self::Phase36ABootRuntimeManifest => "phase36a=x4-boot-runtime-manifest-ok",
            Self::Phase36BBootRuntimeMarkerEmitter => "phase36b=x4-boot-runtime-marker-emitter-ok",
            Self::Phase36CBootRuntimeReadinessReport => {
                "phase36c=x4-boot-runtime-readiness-report-ok"
            }
            Self::Phase36DBootRuntimeAcceptanceGate => {
                "phase36d=x4-boot-runtime-acceptance-gate-ok"
            }
            Self::Phase36EBootRuntimeHandoffSummary => {
                "phase36e=x4-boot-runtime-handoff-summary-ok"
            }
            Self::Phase36FStateIoRuntimeBoundary => PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_MARKER,
        }
    }

    /// Returns true for phases that describe typed state records or registry.
    pub const fn is_state_related(self) -> bool {
        matches!(
            self,
            Self::Phase35CProgressStateIoAdapter
                | Self::Phase35DThemeStateIoAdapter
                | Self::Phase35EMetadataStateIoAdapter
                | Self::Phase35FBookmarkStateIoAdapter
                | Self::Phase35GStateRegistryAdapter
                | Self::Phase36FStateIoRuntimeBoundary
        )
    }
}

/// Runtime surfaces represented by the catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootRuntimeContractSurface {
    /// Per-book typed state records.
    TypedStateRecords,
    /// Typed state registry/discovery metadata.
    StateRegistry,
    /// Shared SPI bus ownership and chip-select metadata.
    SharedSpiContract,
    /// Boot manifest metadata.
    BootManifest,
    /// Marker/status emission metadata.
    MarkerEmitter,
    /// Readiness reporting metadata.
    ReadinessReport,
    /// Acceptance gate metadata.
    AcceptanceGate,
    /// Handoff summary metadata.
    HandoffSummary,
    /// State I/O runtime boundary metadata.
    StateIoRuntimeBoundary,
}

impl BootRuntimeContractSurface {
    /// Returns a stable lowercase surface label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypedStateRecords => "typed-state-records",
            Self::StateRegistry => "state-registry",
            Self::SharedSpiContract => "shared-spi-contract",
            Self::BootManifest => "boot-manifest",
            Self::MarkerEmitter => "marker-emitter",
            Self::ReadinessReport => "readiness-report",
            Self::AcceptanceGate => "acceptance-gate",
            Self::HandoffSummary => "handoff-summary",
            Self::StateIoRuntimeBoundary => "state-io-runtime-boundary",
        }
    }
}

/// The next behavior-binding lane declared by this catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootRuntimeBindingLane {
    /// Keep all live behavior in the imported runtime.
    MetadataOnly,
    /// Future lane: bind typed state I/O behind a Vaachak-owned backend adapter.
    TypedStateIoBackend,
}

impl BootRuntimeBindingLane {
    /// Returns a stable lowercase lane label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata-only",
            Self::TypedStateIoBackend => "typed-state-io-backend",
        }
    }
}

/// Ordered list of accepted boundary/facade phases represented by the catalog.
pub const X4_BOOT_RUNTIME_CONTRACT_PHASES: [BootRuntimeContractPhase; 12] = [
    BootRuntimeContractPhase::Phase35CProgressStateIoAdapter,
    BootRuntimeContractPhase::Phase35DThemeStateIoAdapter,
    BootRuntimeContractPhase::Phase35EMetadataStateIoAdapter,
    BootRuntimeContractPhase::Phase35FBookmarkStateIoAdapter,
    BootRuntimeContractPhase::Phase35GStateRegistryAdapter,
    BootRuntimeContractPhase::Phase35HSpiBusArbitrationFacade,
    BootRuntimeContractPhase::Phase36ABootRuntimeManifest,
    BootRuntimeContractPhase::Phase36BBootRuntimeMarkerEmitter,
    BootRuntimeContractPhase::Phase36CBootRuntimeReadinessReport,
    BootRuntimeContractPhase::Phase36DBootRuntimeAcceptanceGate,
    BootRuntimeContractPhase::Phase36EBootRuntimeHandoffSummary,
    BootRuntimeContractPhase::Phase36FStateIoRuntimeBoundary,
];

/// Runtime surfaces represented by the Phase 36G catalog.
pub const X4_BOOT_RUNTIME_CONTRACT_SURFACES: [BootRuntimeContractSurface; 9] = [
    BootRuntimeContractSurface::TypedStateRecords,
    BootRuntimeContractSurface::StateRegistry,
    BootRuntimeContractSurface::SharedSpiContract,
    BootRuntimeContractSurface::BootManifest,
    BootRuntimeContractSurface::MarkerEmitter,
    BootRuntimeContractSurface::ReadinessReport,
    BootRuntimeContractSurface::AcceptanceGate,
    BootRuntimeContractSurface::HandoffSummary,
    BootRuntimeContractSurface::StateIoRuntimeBoundary,
];

/// Static boot runtime contract catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootRuntimeContractCatalog {
    /// Phase marker for this overlay.
    pub phase_marker: &'static str,
    /// Stable artifact name.
    pub name: &'static str,
    /// Accepted predecessor marker.
    pub predecessor_marker: &'static str,
    /// Ordered accepted boundary/facade phases.
    pub phases: &'static [BootRuntimeContractPhase],
    /// Runtime surfaces represented by the catalog.
    pub surfaces: &'static [BootRuntimeContractSurface],
    /// Current binding lane.
    pub current_lane: BootRuntimeBindingLane,
    /// Next intended binding lane.
    pub next_lane: BootRuntimeBindingLane,
    /// Whether a storage backend is bound by this phase.
    pub storage_backend_bound: bool,
    /// Whether any physical behavior is moved by this phase.
    pub physical_behavior_moved: bool,
    /// Current live behavior owner.
    pub live_behavior_owner: &'static str,
}

impl BootRuntimeContractCatalog {
    /// Returns true when the catalog is metadata-only.
    pub const fn is_metadata_only(self) -> bool {
        !self.storage_backend_bound && !self.physical_behavior_moved
    }

    /// Returns true when all expected boundary/facade phases are represented.
    pub const fn has_expected_phase_count(self) -> bool {
        self.phases.len() == 12
    }

    /// Returns true when all expected runtime surfaces are represented.
    pub const fn has_expected_surface_count(self) -> bool {
        self.surfaces.len() == 9
    }

    /// Returns true when the catalog is safe to use for planning backend binding.
    pub const fn ready_for_typed_state_backend_plan(self) -> bool {
        self.is_metadata_only()
            && self.has_expected_phase_count()
            && self.has_expected_surface_count()
            && matches!(self.next_lane, BootRuntimeBindingLane::TypedStateIoBackend)
    }
}

/// Static Phase 36G boot runtime contract catalog.
pub const X4_BOOT_RUNTIME_CONTRACT_CATALOG: BootRuntimeContractCatalog =
    BootRuntimeContractCatalog {
        phase_marker: PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_MARKER,
        name: PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_NAME,
        predecessor_marker: PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_MARKER,
        phases: &X4_BOOT_RUNTIME_CONTRACT_PHASES,
        surfaces: &X4_BOOT_RUNTIME_CONTRACT_SURFACES,
        current_lane: BootRuntimeBindingLane::MetadataOnly,
        next_lane: BootRuntimeBindingLane::TypedStateIoBackend,
        storage_backend_bound: false,
        physical_behavior_moved: false,
        live_behavior_owner: "imported-pulp-runtime",
    };

/// Returns the Phase 36G marker.
pub const fn boot_runtime_contract_catalog_marker() -> &'static str {
    PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_MARKER
}

/// Returns the Phase 36G artifact name.
pub const fn boot_runtime_contract_catalog_name() -> &'static str {
    PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_NAME
}

/// Returns the static Phase 36G boot runtime contract catalog.
pub const fn boot_runtime_contract_catalog() -> &'static BootRuntimeContractCatalog {
    &X4_BOOT_RUNTIME_CONTRACT_CATALOG
}

/// Returns the accepted boundary/facade phases represented by this catalog.
pub fn boot_runtime_contract_phases() -> &'static [BootRuntimeContractPhase] {
    &X4_BOOT_RUNTIME_CONTRACT_PHASES
}

/// Returns the runtime surfaces represented by this catalog.
pub fn boot_runtime_contract_surfaces() -> &'static [BootRuntimeContractSurface] {
    &X4_BOOT_RUNTIME_CONTRACT_SURFACES
}
