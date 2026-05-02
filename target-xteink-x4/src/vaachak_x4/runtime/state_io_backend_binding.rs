//! Phase 36H state I/O backend binding facade.
//!
//! This module is intentionally compile-only metadata. It defines the contract
//! a future Vaachak-owned typed-state backend must satisfy before any live
//! behavior is moved away from the imported Pulp runtime.
//!
//! No SD/FAT, SPI, display, input, power, or imported runtime behavior is moved
//! or called by this module.

#![allow(dead_code)]

/// Phase marker emitted by the Phase 36H overlay and optional diagnostics.
pub const PHASE_36H_STATE_IO_BACKEND_BINDING_MARKER: &str =
    "phase36h=x4-state-io-backend-binding-ok";

/// Stable name for the Phase 36H runtime artifact.
pub const PHASE_36H_STATE_IO_BACKEND_BINDING_NAME: &str = "x4-state-io-backend-binding";

/// Accepted predecessor marker from Phase 36G.
pub const PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_MARKER: &str =
    "phase36g=x4-boot-runtime-contract-catalog-ok";

/// The binding mode for typed state I/O.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoBackendBindingMode {
    /// Existing imported runtime behavior remains authoritative.
    MetadataOnly,
    /// A future Vaachak-owned backend may be wired behind the same record contract.
    VaachakBackendReady,
}

impl StateIoBackendBindingMode {
    /// Returns the stable lowercase binding-mode label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata-only",
            Self::VaachakBackendReady => "vaachak-backend-ready",
        }
    }
}

/// Typed state record families covered by the backend binding contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoRecordFamily {
    /// Reader progress record, `state/<BOOKID>.PRG`.
    Progress,
    /// Reader theme/layout record, `state/<BOOKID>.THM`.
    Theme,
    /// Per-book metadata record, `state/<BOOKID>.MTA`.
    Metadata,
    /// Per-book bookmark record, `state/<BOOKID>.BKM`.
    Bookmark,
    /// Shared bookmark index, `state/BMIDX.TXT`.
    BookmarkIndex,
}

impl StateIoRecordFamily {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Theme => "theme",
            Self::Metadata => "metadata",
            Self::Bookmark => "bookmark",
            Self::BookmarkIndex => "bookmark-index",
        }
    }

    /// Returns the 8.3-safe extension or file name used by the X4 state layout.
    pub const fn state_suffix(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns true when the record is per-book and keyed by a book id.
    pub const fn is_per_book(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }
}

/// Backend capabilities required before behavior can be moved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoBackendCapability {
    /// Resolve a record family to its canonical 8.3-safe state path.
    ResolveStatePath,
    /// Load a typed state record as bytes or an empty default.
    LoadRecord,
    /// Persist a typed state record atomically enough for X4 state files.
    PersistRecord,
    /// Keep imported-runtime fallbacks available during transition.
    PreserveFallback,
    /// Refuse to touch display/input/SPI behavior from state I/O code.
    StayStorageOnly,
}

impl StateIoBackendCapability {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveStatePath => "resolve-state-path",
            Self::LoadRecord => "load-record",
            Self::PersistRecord => "persist-record",
            Self::PreserveFallback => "preserve-fallback",
            Self::StayStorageOnly => "stay-storage-only",
        }
    }
}

/// Ordered typed state record families covered by Phase 36H.
pub const X4_STATE_IO_BACKEND_RECORD_FAMILIES: [StateIoRecordFamily; 5] = [
    StateIoRecordFamily::Progress,
    StateIoRecordFamily::Theme,
    StateIoRecordFamily::Metadata,
    StateIoRecordFamily::Bookmark,
    StateIoRecordFamily::BookmarkIndex,
];

/// Required future backend capabilities.
pub const X4_STATE_IO_BACKEND_REQUIRED_CAPABILITIES: [StateIoBackendCapability; 5] = [
    StateIoBackendCapability::ResolveStatePath,
    StateIoBackendCapability::LoadRecord,
    StateIoBackendCapability::PersistRecord,
    StateIoBackendCapability::PreserveFallback,
    StateIoBackendCapability::StayStorageOnly,
];

/// Phase 36H backend binding contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoBackendBindingContract {
    /// Phase marker for this overlay.
    pub phase_marker: &'static str,
    /// Stable artifact name.
    pub name: &'static str,
    /// Accepted predecessor marker.
    pub predecessor_marker: &'static str,
    /// Current binding mode.
    pub mode: StateIoBackendBindingMode,
    /// Typed record families covered by this binding.
    pub record_families: &'static [StateIoRecordFamily],
    /// Required future backend capabilities.
    pub required_capabilities: &'static [StateIoBackendCapability],
    /// Whether this phase moves live behavior.
    pub moves_live_behavior: bool,
    /// Whether imported runtime state behavior remains authoritative.
    pub imported_runtime_remains_authoritative: bool,
}

impl StateIoBackendBindingContract {
    /// Returns true when this phase is still a metadata/facade phase.
    pub const fn is_metadata_only(self) -> bool {
        matches!(self.mode, StateIoBackendBindingMode::MetadataOnly)
            && !self.moves_live_behavior
            && self.imported_runtime_remains_authoritative
    }

    /// Returns the number of covered state record families.
    pub const fn record_family_count(self) -> usize {
        self.record_families.len()
    }

    /// Returns the number of required future backend capabilities.
    pub const fn required_capability_count(self) -> usize {
        self.required_capabilities.len()
    }

    /// Returns true when the minimum set of state record families is represented.
    pub const fn covers_minimum_state_records(self) -> bool {
        self.record_families.len() >= 4
    }
}

/// Static Phase 36H contract instance.
pub const X4_STATE_IO_BACKEND_BINDING_CONTRACT: StateIoBackendBindingContract =
    StateIoBackendBindingContract {
        phase_marker: PHASE_36H_STATE_IO_BACKEND_BINDING_MARKER,
        name: PHASE_36H_STATE_IO_BACKEND_BINDING_NAME,
        predecessor_marker: PHASE_36G_BOOT_RUNTIME_CONTRACT_CATALOG_MARKER,
        mode: StateIoBackendBindingMode::MetadataOnly,
        record_families: &X4_STATE_IO_BACKEND_RECORD_FAMILIES,
        required_capabilities: &X4_STATE_IO_BACKEND_REQUIRED_CAPABILITIES,
        moves_live_behavior: false,
        imported_runtime_remains_authoritative: true,
    };

/// Returns the static Phase 36H binding contract.
pub const fn x4_state_io_backend_binding_contract() -> StateIoBackendBindingContract {
    X4_STATE_IO_BACKEND_BINDING_CONTRACT
}

/// Returns the Phase 36H marker string.
pub const fn phase36h_marker() -> &'static str {
    PHASE_36H_STATE_IO_BACKEND_BINDING_MARKER
}

/// Returns true when Phase 36H is safe to accept as a compile-only phase.
pub const fn phase36h_acceptance_ok() -> bool {
    X4_STATE_IO_BACKEND_BINDING_CONTRACT.is_metadata_only()
        && X4_STATE_IO_BACKEND_BINDING_CONTRACT.covers_minimum_state_records()
        && X4_STATE_IO_BACKEND_BINDING_CONTRACT.required_capability_count() == 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase36h_marker_is_stable() {
        assert_eq!(phase36h_marker(), "phase36h=x4-state-io-backend-binding-ok");
    }

    #[test]
    fn phase36h_contract_remains_metadata_only() {
        let contract = x4_state_io_backend_binding_contract();
        assert!(contract.is_metadata_only());
        assert!(!contract.moves_live_behavior);
        assert!(contract.imported_runtime_remains_authoritative);
    }

    #[test]
    fn state_record_suffixes_are_83_safe() {
        assert_eq!(StateIoRecordFamily::Progress.state_suffix(), ".PRG");
        assert_eq!(StateIoRecordFamily::Theme.state_suffix(), ".THM");
        assert_eq!(StateIoRecordFamily::Metadata.state_suffix(), ".MTA");
        assert_eq!(StateIoRecordFamily::Bookmark.state_suffix(), ".BKM");
        assert_eq!(
            StateIoRecordFamily::BookmarkIndex.state_suffix(),
            "BMIDX.TXT"
        );
    }
}
