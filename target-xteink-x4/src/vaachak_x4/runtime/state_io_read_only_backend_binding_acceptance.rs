//! Phase 37B — State I/O read-only backend binding acceptance.
//!
//! This module records acceptance metadata for the Phase 37A read-only backend
//! binding scaffold. It is intentionally side-effect free: no device,
//! filesystem, SPI, display, input, power, or boot-flow operation is performed
//! here.

#![allow(clippy::struct_excessive_bools)]

/// Phase marker emitted by the overlay installer/check scripts.
pub const PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE_MARKER: &str =
    "phase37b=x4-state-io-read-only-backend-binding-acceptance-ok";

/// Prior accepted lane required before this acceptance overlay is valid.
pub const PHASE_37B_REQUIRED_PREVIOUS_MARKER: &str =
    "phase37a=x4-state-io-read-only-backend-binding-ok";

/// Next intended implementation lane after this acceptance overlay.
pub const PHASE_37B_NEXT_LANE: &str = "typed-state-read-only-backend-first-adapter";

/// Record families covered by the accepted read-only backend binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBackendBindingAcceptedRecord {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl StateIoReadOnlyBackendBindingAcceptedRecord {
    /// Returns the 8.3-safe suffix or fixed index filename for reporting.
    pub const fn storage_name(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns true for the shared bookmark index record.
    pub const fn is_shared_index(self) -> bool {
        matches!(self, Self::BookmarkIndex)
    }

    /// Returns true when the record is book-scoped.
    pub const fn is_book_scoped(self) -> bool {
        !self.is_shared_index()
    }
}

/// Binding routes accepted by the read-only backend binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBackendBindingAcceptedRoute {
    BookScopedPrimary,
    BookScopedBackup,
    SharedIndexPrimary,
    SharedIndexBackup,
    DefaultState,
}

impl StateIoReadOnlyBackendBindingAcceptedRoute {
    /// Stable text form for boot/runtime reporting.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BookScopedPrimary => "book-scoped-primary",
            Self::BookScopedBackup => "book-scoped-backup",
            Self::SharedIndexPrimary => "shared-index-primary",
            Self::SharedIndexBackup => "shared-index-backup",
            Self::DefaultState => "default-state",
        }
    }

    /// Phase 37B accepts only read-only-safe routes.
    pub const fn is_read_only_safe(self) -> bool {
        matches!(
            self,
            Self::BookScopedPrimary
                | Self::BookScopedBackup
                | Self::SharedIndexPrimary
                | Self::SharedIndexBackup
                | Self::DefaultState
        )
    }
}

/// Acceptance facts for the read-only backend binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBackendBindingAcceptanceItem {
    BindingContractAccepted,
    BookScopedPrimaryAccepted,
    BookScopedBackupAccepted,
    SharedIndexPrimaryAccepted,
    SharedIndexBackupAccepted,
    DefaultFallbackAccepted,
    InvalidShapeRejected,
    WriteRoutesDisabled,
    BackendCallsDisabled,
    SideEffectFree,
    HardwareBehaviorUnmoved,
}

impl StateIoReadOnlyBackendBindingAcceptanceItem {
    /// Stable text form for boot/runtime reporting.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BindingContractAccepted => "binding-contract-accepted",
            Self::BookScopedPrimaryAccepted => "book-scoped-primary-accepted",
            Self::BookScopedBackupAccepted => "book-scoped-backup-accepted",
            Self::SharedIndexPrimaryAccepted => "shared-index-primary-accepted",
            Self::SharedIndexBackupAccepted => "shared-index-backup-accepted",
            Self::DefaultFallbackAccepted => "default-fallback-accepted",
            Self::InvalidShapeRejected => "invalid-shape-rejected",
            Self::WriteRoutesDisabled => "write-routes-disabled",
            Self::BackendCallsDisabled => "backend-calls-disabled",
            Self::SideEffectFree => "side-effect-free",
            Self::HardwareBehaviorUnmoved => "hardware-behavior-unmoved",
        }
    }
}

/// Static acceptance report for the Phase 37A binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyBackendBindingAcceptanceReport {
    pub phase_marker: &'static str,
    pub required_previous_marker: &'static str,
    pub accepted: bool,
    pub side_effect_free: bool,
    pub real_backend_calls_enabled: bool,
    pub write_routes_enabled: bool,
    pub hardware_behavior_moved: bool,
    pub next_lane: &'static str,
    pub records: &'static [StateIoReadOnlyBackendBindingAcceptedRecord],
    pub routes: &'static [StateIoReadOnlyBackendBindingAcceptedRoute],
    pub acceptance_items: &'static [StateIoReadOnlyBackendBindingAcceptanceItem],
}

const PHASE_37B_ACCEPTED_RECORDS: &[StateIoReadOnlyBackendBindingAcceptedRecord] = &[
    StateIoReadOnlyBackendBindingAcceptedRecord::Progress,
    StateIoReadOnlyBackendBindingAcceptedRecord::Theme,
    StateIoReadOnlyBackendBindingAcceptedRecord::Metadata,
    StateIoReadOnlyBackendBindingAcceptedRecord::Bookmark,
    StateIoReadOnlyBackendBindingAcceptedRecord::BookmarkIndex,
];

const PHASE_37B_ACCEPTED_ROUTES: &[StateIoReadOnlyBackendBindingAcceptedRoute] = &[
    StateIoReadOnlyBackendBindingAcceptedRoute::BookScopedPrimary,
    StateIoReadOnlyBackendBindingAcceptedRoute::BookScopedBackup,
    StateIoReadOnlyBackendBindingAcceptedRoute::SharedIndexPrimary,
    StateIoReadOnlyBackendBindingAcceptedRoute::SharedIndexBackup,
    StateIoReadOnlyBackendBindingAcceptedRoute::DefaultState,
];

const PHASE_37B_ACCEPTANCE_ITEMS: &[StateIoReadOnlyBackendBindingAcceptanceItem] = &[
    StateIoReadOnlyBackendBindingAcceptanceItem::BindingContractAccepted,
    StateIoReadOnlyBackendBindingAcceptanceItem::BookScopedPrimaryAccepted,
    StateIoReadOnlyBackendBindingAcceptanceItem::BookScopedBackupAccepted,
    StateIoReadOnlyBackendBindingAcceptanceItem::SharedIndexPrimaryAccepted,
    StateIoReadOnlyBackendBindingAcceptanceItem::SharedIndexBackupAccepted,
    StateIoReadOnlyBackendBindingAcceptanceItem::DefaultFallbackAccepted,
    StateIoReadOnlyBackendBindingAcceptanceItem::InvalidShapeRejected,
    StateIoReadOnlyBackendBindingAcceptanceItem::WriteRoutesDisabled,
    StateIoReadOnlyBackendBindingAcceptanceItem::BackendCallsDisabled,
    StateIoReadOnlyBackendBindingAcceptanceItem::SideEffectFree,
    StateIoReadOnlyBackendBindingAcceptanceItem::HardwareBehaviorUnmoved,
];

/// Phase 37B acceptance report.
pub const PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE:
    StateIoReadOnlyBackendBindingAcceptanceReport = StateIoReadOnlyBackendBindingAcceptanceReport {
    phase_marker: PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE_MARKER,
    required_previous_marker: PHASE_37B_REQUIRED_PREVIOUS_MARKER,
    accepted: true,
    side_effect_free: true,
    real_backend_calls_enabled: false,
    write_routes_enabled: false,
    hardware_behavior_moved: false,
    next_lane: PHASE_37B_NEXT_LANE,
    records: PHASE_37B_ACCEPTED_RECORDS,
    routes: PHASE_37B_ACCEPTED_ROUTES,
    acceptance_items: PHASE_37B_ACCEPTANCE_ITEMS,
};

/// Returns the Phase 37B acceptance report.
pub const fn phase37b_state_io_read_only_backend_binding_acceptance()
-> &'static StateIoReadOnlyBackendBindingAcceptanceReport {
    &PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE
}

/// Reports whether the Phase 37A binding lane has been accepted safely.
pub const fn phase37b_is_accepted() -> bool {
    PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE.accepted
        && PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE.side_effect_free
        && !PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE.real_backend_calls_enabled
        && !PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE.write_routes_enabled
        && !PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE.hardware_behavior_moved
}

/// Returns the next intended implementation lane.
pub const fn phase37b_next_lane() -> &'static str {
    PHASE_37B_STATE_IO_READ_ONLY_BACKEND_BINDING_ACCEPTANCE.next_lane
}

/// Returns whether the accepted binding lane covers a record family.
pub fn phase37b_has_record(record: StateIoReadOnlyBackendBindingAcceptedRecord) -> bool {
    PHASE_37B_ACCEPTED_RECORDS.contains(&record)
}

/// Returns whether the accepted binding lane covers a read-only route.
pub fn phase37b_has_route(route: StateIoReadOnlyBackendBindingAcceptedRoute) -> bool {
    PHASE_37B_ACCEPTED_ROUTES.contains(&route)
}

/// Returns whether the report includes a specific acceptance item.
pub fn phase37b_has_acceptance_item(item: StateIoReadOnlyBackendBindingAcceptanceItem) -> bool {
    PHASE_37B_ACCEPTANCE_ITEMS.contains(&item)
}
