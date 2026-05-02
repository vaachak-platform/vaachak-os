//! Phase 36F state I/O runtime boundary.
//!
//! This module is intentionally contract/facade-only. It names the typed state
//! records and runtime capabilities that will be used when the first real
//! behavior path is extracted behind a Vaachak-owned boundary.
//!
//! No SD/FAT, SPI, display, input, power, or imported Pulp runtime behavior is
//! moved or called by this module.

#![allow(dead_code)]

/// Phase marker emitted by the Phase 36F overlay and optional diagnostics.
pub const PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_MARKER: &str =
    "phase36f=x4-state-io-runtime-boundary-ok";

/// Stable name for the Phase 36F runtime artifact.
pub const PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_NAME: &str = "x4-state-io-runtime-boundary";

/// Accepted dependency marker from Phase 36E.
pub const PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_MARKER: &str =
    "phase36e=x4-boot-runtime-handoff-summary-ok";

/// Accepted dependency marker from Phase 35G.
pub const PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER: &str = "phase35g=x4-state-registry-adapter-ok";

/// Typed state records represented by the runtime boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoRecordKind {
    /// Per-book progress record: `state/<BOOKID>.PRG`.
    Progress,
    /// Per-book theme/layout record: `state/<BOOKID>.THM`.
    Theme,
    /// Per-book metadata record: `state/<BOOKID>.MTA`.
    Metadata,
    /// Per-book bookmark record: `state/<BOOKID>.BKM`.
    Bookmark,
    /// State registry/index record for typed-state discovery.
    Registry,
}

impl StateIoRecordKind {
    /// Returns a stable lowercase record label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Theme => "theme",
            Self::Metadata => "metadata",
            Self::Bookmark => "bookmark",
            Self::Registry => "registry",
        }
    }

    /// Returns the 8.3-safe extension or registry label used by the X4 layout.
    pub const fn x4_suffix(self) -> &'static str {
        match self {
            Self::Progress => "PRG",
            Self::Theme => "THM",
            Self::Metadata => "MTA",
            Self::Bookmark => "BKM",
            Self::Registry => "IDX",
        }
    }

    /// Returns true when this record is per-book state.
    pub const fn is_per_book(self) -> bool {
        !matches!(self, Self::Registry)
    }
}

/// Runtime capabilities represented by this boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoRuntimeCapability {
    /// Compute or accept a typed-state record identity.
    ResolveRecordIdentity,
    /// Read typed-state bytes through a future backend binding.
    ReadTypedState,
    /// Write typed-state bytes through a future backend binding.
    WriteTypedState,
    /// Validate that a typed-state record path remains 8.3-safe for X4.
    ValidateX4StateName,
    /// Enumerate known typed-state surfaces without probing hardware.
    EnumerateKnownStateSurfaces,
}

impl StateIoRuntimeCapability {
    /// Returns a stable capability label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveRecordIdentity => "resolve-record-identity",
            Self::ReadTypedState => "read-typed-state",
            Self::WriteTypedState => "write-typed-state",
            Self::ValidateX4StateName => "validate-x4-state-name",
            Self::EnumerateKnownStateSurfaces => "enumerate-known-state-surfaces",
        }
    }
}

/// First behavior path that will be extracted behind a Vaachak-owned facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoBehaviorPath {
    /// No backend has been bound yet.
    BackendUnbound,
    /// The next phase may bind typed state I/O to the existing storage path.
    TypedStateBackendBinding,
}

impl StateIoBehaviorPath {
    /// Returns a stable behavior-path label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackendUnbound => "backend-unbound",
            Self::TypedStateBackendBinding => "typed-state-backend-binding",
        }
    }
}

/// Static record kinds represented by the boundary.
pub const X4_STATE_IO_RECORD_KINDS: [StateIoRecordKind; 5] = [
    StateIoRecordKind::Progress,
    StateIoRecordKind::Theme,
    StateIoRecordKind::Metadata,
    StateIoRecordKind::Bookmark,
    StateIoRecordKind::Registry,
];

/// Static capabilities represented by the boundary.
pub const X4_STATE_IO_RUNTIME_CAPABILITIES: [StateIoRuntimeCapability; 5] = [
    StateIoRuntimeCapability::ResolveRecordIdentity,
    StateIoRuntimeCapability::ReadTypedState,
    StateIoRuntimeCapability::WriteTypedState,
    StateIoRuntimeCapability::ValidateX4StateName,
    StateIoRuntimeCapability::EnumerateKnownStateSurfaces,
];

/// Contract/facade summary for typed state I/O runtime work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoRuntimeBoundary {
    /// Phase marker for this overlay.
    pub phase_marker: &'static str,
    /// Stable artifact name.
    pub name: &'static str,
    /// Accepted boot/runtime handoff marker.
    pub handoff_marker: &'static str,
    /// Accepted typed state registry marker.
    pub registry_marker: &'static str,
    /// First behavior path represented by this boundary.
    pub first_behavior_path: StateIoBehaviorPath,
    /// Known typed state record kinds.
    pub record_kinds: &'static [StateIoRecordKind],
    /// Known runtime capabilities.
    pub capabilities: &'static [StateIoRuntimeCapability],
    /// Whether a real storage backend has been bound in this phase.
    pub backend_bound: bool,
    /// Whether a real behavior path has been moved in this phase.
    pub behavior_moved: bool,
    /// Current physical/runtime owner for live behavior.
    pub live_behavior_owner: &'static str,
}

impl StateIoRuntimeBoundary {
    /// Returns true when this boundary is still contract/facade-only.
    pub const fn is_contract_only(self) -> bool {
        !self.backend_bound && !self.behavior_moved
    }

    /// Returns true when all expected typed-state records are represented.
    pub const fn has_expected_records(self) -> bool {
        self.record_kinds.len() == 5
    }

    /// Returns true when all expected runtime capabilities are represented.
    pub const fn has_expected_capabilities(self) -> bool {
        self.capabilities.len() == 5
    }

    /// Returns true when the next phase may safely bind a backend adapter.
    pub const fn ready_for_backend_binding(self) -> bool {
        self.is_contract_only() && self.has_expected_records() && self.has_expected_capabilities()
    }
}

/// Static Phase 36F state I/O runtime boundary.
pub const X4_STATE_IO_RUNTIME_BOUNDARY: StateIoRuntimeBoundary = StateIoRuntimeBoundary {
    phase_marker: PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_MARKER,
    name: PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_NAME,
    handoff_marker: PHASE_36E_BOOT_RUNTIME_HANDOFF_SUMMARY_MARKER,
    registry_marker: PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
    first_behavior_path: StateIoBehaviorPath::TypedStateBackendBinding,
    record_kinds: &X4_STATE_IO_RECORD_KINDS,
    capabilities: &X4_STATE_IO_RUNTIME_CAPABILITIES,
    backend_bound: false,
    behavior_moved: false,
    live_behavior_owner: "imported-pulp-runtime",
};

/// Returns the Phase 36F marker.
pub const fn state_io_runtime_boundary_marker() -> &'static str {
    PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_MARKER
}

/// Returns the Phase 36F artifact name.
pub const fn state_io_runtime_boundary_name() -> &'static str {
    PHASE_36F_STATE_IO_RUNTIME_BOUNDARY_NAME
}

/// Returns the static Phase 36F state I/O runtime boundary.
pub const fn state_io_runtime_boundary() -> &'static StateIoRuntimeBoundary {
    &X4_STATE_IO_RUNTIME_BOUNDARY
}

/// Returns the typed state record kinds represented by this boundary.
pub fn state_io_record_kinds() -> &'static [StateIoRecordKind] {
    &X4_STATE_IO_RECORD_KINDS
}

/// Returns the runtime capabilities represented by this boundary.
pub fn state_io_runtime_capabilities() -> &'static [StateIoRuntimeCapability] {
    &X4_STATE_IO_RUNTIME_CAPABILITIES
}

/// Returns true when the next phase may bind a real typed-state backend.
pub const fn state_io_runtime_ready_for_backend_binding() -> bool {
    X4_STATE_IO_RUNTIME_BOUNDARY.ready_for_backend_binding()
}
