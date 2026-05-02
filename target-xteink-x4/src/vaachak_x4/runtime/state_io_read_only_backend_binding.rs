//! Phase 37A — State I/O read-only backend binding scaffold.
//!
//! This module is the first Phase 37 binding layer after the Phase 36Z
//! read-only probe acceptance gate. It defines how typed state records may be
//! mapped into a future read-only backend lane, but it deliberately performs no
//! filesystem, SD/FAT, SPI, display, input, power, or boot-flow operation.

#![allow(clippy::struct_excessive_bools)]

/// Phase 37A acceptance marker.
pub const PHASE_37A_STATE_IO_READ_ONLY_BACKEND_BINDING_MARKER: &str =
    "phase37a=x4-state-io-read-only-backend-binding-ok";

/// Prior accepted lane required before this binding scaffold is valid.
pub const PHASE_37A_REQUIRED_PREVIOUS_MARKER: &str =
    "phase36z=x4-state-io-read-only-backend-probe-acceptance-ok";

/// Next intended lane after this binding scaffold.
pub const PHASE_37A_NEXT_LANE: &str = "read-only-backend-binding-acceptance";

/// State records covered by the first read-only backend binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBindingRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl StateIoReadOnlyBindingRecordKind {
    /// Returns the 8.3-safe suffix or fixed index filename.
    pub const fn storage_name(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns the logical state family name.
    pub const fn family(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Theme => "theme",
            Self::Metadata => "metadata",
            Self::Bookmark => "bookmark",
            Self::BookmarkIndex => "bookmark-index",
        }
    }

    /// Returns true when the record is shared rather than book-scoped.
    pub const fn is_shared_index(self) -> bool {
        matches!(self, Self::BookmarkIndex)
    }

    /// Returns true when the record requires a book id.
    pub const fn requires_book_id(self) -> bool {
        !self.is_shared_index()
    }
}

/// Candidate role that the future backend may choose for read-only lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBindingCandidate {
    Primary,
    Backup,
    DefaultFallback,
}

impl StateIoReadOnlyBindingCandidate {
    /// Stable label for diagnostics and boot/runtime reports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Backup => "backup",
            Self::DefaultFallback => "default-fallback",
        }
    }
}

/// Operation intent allowed through the first binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBindingIntent {
    Probe,
    Load,
}

impl StateIoReadOnlyBindingIntent {
    /// Stable label for diagnostics and boot/runtime reports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Probe => "probe",
            Self::Load => "load",
        }
    }

    /// Phase 37A permits read-only intents only.
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::Probe | Self::Load)
    }
}

/// Route selected by the binding scaffold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBindingRoute {
    BookScopedPrimary,
    BookScopedBackup,
    SharedIndexPrimary,
    SharedIndexBackup,
    DefaultState,
    Rejected,
}

impl StateIoReadOnlyBindingRoute {
    /// Stable label for diagnostics and boot/runtime reports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BookScopedPrimary => "book-scoped-primary",
            Self::BookScopedBackup => "book-scoped-backup",
            Self::SharedIndexPrimary => "shared-index-primary",
            Self::SharedIndexBackup => "shared-index-backup",
            Self::DefaultState => "default-state",
            Self::Rejected => "rejected",
        }
    }

    /// Returns true when this route may be used by a read-only backend later.
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

/// Compile-only request envelope for the read-only backend binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyBindingRequest<'a> {
    pub intent: StateIoReadOnlyBindingIntent,
    pub record_kind: StateIoReadOnlyBindingRecordKind,
    pub candidate: StateIoReadOnlyBindingCandidate,
    pub book_id: Option<&'a str>,
}

impl<'a> StateIoReadOnlyBindingRequest<'a> {
    /// Builds a book-scoped read-only request.
    pub const fn book_scoped(
        intent: StateIoReadOnlyBindingIntent,
        record_kind: StateIoReadOnlyBindingRecordKind,
        candidate: StateIoReadOnlyBindingCandidate,
        book_id: &'a str,
    ) -> Self {
        Self {
            intent,
            record_kind,
            candidate,
            book_id: Some(book_id),
        }
    }

    /// Builds a shared-index read-only request.
    pub const fn shared_index(
        intent: StateIoReadOnlyBindingIntent,
        candidate: StateIoReadOnlyBindingCandidate,
    ) -> Self {
        Self {
            intent,
            record_kind: StateIoReadOnlyBindingRecordKind::BookmarkIndex,
            candidate,
            book_id: None,
        }
    }

    /// Returns true when the request is shape-valid for read-only binding.
    pub const fn is_shape_valid(self) -> bool {
        self.intent.is_read_only()
            && (!self.record_kind.requires_book_id() || self.book_id.is_some())
    }
}

/// Compile-only response envelope for the read-only backend binding lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyBindingOutcome {
    pub record_kind: StateIoReadOnlyBindingRecordKind,
    pub candidate: StateIoReadOnlyBindingCandidate,
    pub route: StateIoReadOnlyBindingRoute,
    pub accepted: bool,
}

impl StateIoReadOnlyBindingOutcome {
    /// Returns true when the outcome is accepted and read-only safe.
    pub const fn is_read_only_safe(self) -> bool {
        self.accepted && self.route.is_read_only_safe()
    }
}

/// Trait shape for a future read-only backend binding implementation.
pub trait StateIoReadOnlyBackendBinding {
    fn bind_read_only<'a>(
        &self,
        request: StateIoReadOnlyBindingRequest<'a>,
    ) -> StateIoReadOnlyBindingOutcome;
}

/// Side-effect-free planner for the first read-only backend binding lane.
pub const fn phase37a_plan_read_only_binding(
    request: StateIoReadOnlyBindingRequest<'_>,
) -> StateIoReadOnlyBindingOutcome {
    let route = if !request.is_shape_valid() {
        StateIoReadOnlyBindingRoute::Rejected
    } else if matches!(
        request.candidate,
        StateIoReadOnlyBindingCandidate::DefaultFallback
    ) {
        StateIoReadOnlyBindingRoute::DefaultState
    } else if request.record_kind.is_shared_index() {
        match request.candidate {
            StateIoReadOnlyBindingCandidate::Primary => {
                StateIoReadOnlyBindingRoute::SharedIndexPrimary
            }
            StateIoReadOnlyBindingCandidate::Backup => {
                StateIoReadOnlyBindingRoute::SharedIndexBackup
            }
            StateIoReadOnlyBindingCandidate::DefaultFallback => {
                StateIoReadOnlyBindingRoute::DefaultState
            }
        }
    } else {
        match request.candidate {
            StateIoReadOnlyBindingCandidate::Primary => {
                StateIoReadOnlyBindingRoute::BookScopedPrimary
            }
            StateIoReadOnlyBindingCandidate::Backup => {
                StateIoReadOnlyBindingRoute::BookScopedBackup
            }
            StateIoReadOnlyBindingCandidate::DefaultFallback => {
                StateIoReadOnlyBindingRoute::DefaultState
            }
        }
    };

    StateIoReadOnlyBindingOutcome {
        record_kind: request.record_kind,
        candidate: request.candidate,
        route,
        accepted: route.is_read_only_safe(),
    }
}

/// Records included in the first read-only backend binding lane.
pub const PHASE_37A_READ_ONLY_BINDING_RECORDS: [StateIoReadOnlyBindingRecordKind; 5] = [
    StateIoReadOnlyBindingRecordKind::Progress,
    StateIoReadOnlyBindingRecordKind::Theme,
    StateIoReadOnlyBindingRecordKind::Metadata,
    StateIoReadOnlyBindingRecordKind::Bookmark,
    StateIoReadOnlyBindingRecordKind::BookmarkIndex,
];

/// Binding guardrails that must remain true in Phase 37A.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyBindingGuardrail {
    PreviousProbeAccepted,
    ReadOnlyIntentsOnly,
    NoWriteRoutes,
    NoBackendCalls,
    NoHardwareBehaviorMoved,
    ExistingRuntimeAuthoritative,
    RollbackSafe,
}

impl StateIoReadOnlyBindingGuardrail {
    /// Stable label for diagnostics and boot/runtime reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviousProbeAccepted => "previous-probe-accepted",
            Self::ReadOnlyIntentsOnly => "read-only-intents-only",
            Self::NoWriteRoutes => "no-write-routes",
            Self::NoBackendCalls => "no-backend-calls",
            Self::NoHardwareBehaviorMoved => "no-hardware-behavior-moved",
            Self::ExistingRuntimeAuthoritative => "existing-runtime-authoritative",
            Self::RollbackSafe => "rollback-safe",
        }
    }
}

/// Guardrails included in the first read-only backend binding lane.
pub const PHASE_37A_READ_ONLY_BINDING_GUARDRAILS: [StateIoReadOnlyBindingGuardrail; 7] = [
    StateIoReadOnlyBindingGuardrail::PreviousProbeAccepted,
    StateIoReadOnlyBindingGuardrail::ReadOnlyIntentsOnly,
    StateIoReadOnlyBindingGuardrail::NoWriteRoutes,
    StateIoReadOnlyBindingGuardrail::NoBackendCalls,
    StateIoReadOnlyBindingGuardrail::NoHardwareBehaviorMoved,
    StateIoReadOnlyBindingGuardrail::ExistingRuntimeAuthoritative,
    StateIoReadOnlyBindingGuardrail::RollbackSafe,
];

/// Summary of the Phase 37A binding scaffold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyBindingSummary {
    pub marker: &'static str,
    pub required_previous_marker: &'static str,
    pub next_lane: &'static str,
    pub record_count: usize,
    pub guardrail_count: usize,
    pub read_only: bool,
    pub backend_calls_enabled: bool,
    pub write_routes_enabled: bool,
    pub hardware_behavior_moved: bool,
}

/// Returns the static Phase 37A binding summary.
pub const fn phase37a_state_io_read_only_backend_binding_summary() -> StateIoReadOnlyBindingSummary
{
    StateIoReadOnlyBindingSummary {
        marker: PHASE_37A_STATE_IO_READ_ONLY_BACKEND_BINDING_MARKER,
        required_previous_marker: PHASE_37A_REQUIRED_PREVIOUS_MARKER,
        next_lane: PHASE_37A_NEXT_LANE,
        record_count: PHASE_37A_READ_ONLY_BINDING_RECORDS.len(),
        guardrail_count: PHASE_37A_READ_ONLY_BINDING_GUARDRAILS.len(),
        read_only: true,
        backend_calls_enabled: false,
        write_routes_enabled: false,
        hardware_behavior_moved: false,
    }
}

/// Returns true when a record participates in Phase 37A.
pub fn phase37a_has_record_kind(kind: StateIoReadOnlyBindingRecordKind) -> bool {
    PHASE_37A_READ_ONLY_BINDING_RECORDS.contains(&kind)
}

/// Returns true when a guardrail is included in Phase 37A.
pub fn phase37a_has_guardrail(guardrail: StateIoReadOnlyBindingGuardrail) -> bool {
    PHASE_37A_READ_ONLY_BINDING_GUARDRAILS.contains(&guardrail)
}

/// Returns true when the Phase 37A binding lane remains side-effect free.
pub const fn phase37a_is_side_effect_free() -> bool {
    let summary = phase37a_state_io_read_only_backend_binding_summary();
    summary.read_only
        && !summary.backend_calls_enabled
        && !summary.write_routes_enabled
        && !summary.hardware_behavior_moved
}
