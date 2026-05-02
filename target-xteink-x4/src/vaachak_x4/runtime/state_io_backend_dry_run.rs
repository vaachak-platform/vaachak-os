//! Phase 36J — State I/O backend dry-run facade.
//!
//! This module is intentionally compile-only and metadata-only. It models the
//! typed state I/O backend operations that will later bind to real persistent
//! storage, but it does not call or move any physical storage, SPI, display,
//! input, power, or boot-flow behavior.

#![allow(dead_code)]

/// Phase 36J acceptance marker emitted by the overlay/check scripts.
pub const PHASE_36J_STATE_IO_BACKEND_DRY_RUN_MARKER: &str =
    "phase36j=x4-state-io-backend-dry-run-ok";

/// Human-readable phase name for diagnostics and docs.
pub const PHASE_36J_NAME: &str = "Phase 36J — State I/O Backend Dry-Run Facade";

/// Typed state record extensions covered by the dry-run facade.
pub const PHASE_36J_TYPED_STATE_RECORDS: [&str; 5] = [".PRG", ".THM", ".MTA", ".BKM", "BMIDX.TXT"];

/// Canonical root used by the X4 flat 8.3-safe state layout.
pub const PHASE_36J_X4_STATE_ROOT: &str = "state";

/// Next intended implementation lane after this dry-run facade is accepted.
pub const PHASE_36J_NEXT_LANE: &str = "typed-state-backend-adapter-wiring";

/// Logical state record kind used by the dry-run backend facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DryRunStateRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmarks,
    BookmarkIndex,
}

impl DryRunStateRecordKind {
    /// Returns the canonical flat state suffix for this record kind.
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmarks => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns true when this record is per-book rather than a shared index.
    pub const fn is_per_book(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }
}

/// Logical dry-run operation modeled by this facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DryRunStateOperationKind {
    Read,
    Write,
    Upsert,
    Delete,
}

/// Outcome for a dry-run state operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DryRunStateOutcome {
    Planned,
    SkippedNoBackend,
    RejectedInvalidBookId,
}

/// Compile-only operation record. It deliberately stores metadata only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DryRunStateOperation {
    pub operation: DryRunStateOperationKind,
    pub record_kind: DryRunStateRecordKind,
    pub book_id: Option<&'static str>,
    pub outcome: DryRunStateOutcome,
}

impl DryRunStateOperation {
    /// Builds a planned operation for a per-book record kind.
    pub const fn planned_for_book(
        operation: DryRunStateOperationKind,
        record_kind: DryRunStateRecordKind,
        book_id: &'static str,
    ) -> Self {
        Self {
            operation,
            record_kind,
            book_id: Some(book_id),
            outcome: DryRunStateOutcome::Planned,
        }
    }

    /// Builds a planned shared-index operation.
    pub const fn planned_index(operation: DryRunStateOperationKind) -> Self {
        Self {
            operation,
            record_kind: DryRunStateRecordKind::BookmarkIndex,
            book_id: None,
            outcome: DryRunStateOutcome::Planned,
        }
    }

    /// Returns true when the operation is a pure dry-run and has no real backend.
    pub const fn is_dry_run_only(self) -> bool {
        matches!(
            self.outcome,
            DryRunStateOutcome::Planned | DryRunStateOutcome::SkippedNoBackend
        )
    }
}

/// Metadata-only dry-run backend status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoBackendDryRunStatus {
    pub marker: &'static str,
    pub state_root: &'static str,
    pub record_count: usize,
    pub moves_storage_backend: bool,
    pub moves_spi_backend: bool,
    pub changes_display_behavior: bool,
    pub changes_input_behavior: bool,
    pub changes_power_behavior: bool,
    pub next_lane: &'static str,
}

impl StateIoBackendDryRunStatus {
    /// Returns true only when this dry-run facade is metadata-only.
    pub const fn is_metadata_only(self) -> bool {
        !self.moves_storage_backend
            && !self.moves_spi_backend
            && !self.changes_display_behavior
            && !self.changes_input_behavior
            && !self.changes_power_behavior
    }

    /// Returns true when all typed state records are represented.
    pub const fn covers_all_typed_records(self) -> bool {
        self.record_count == PHASE_36J_TYPED_STATE_RECORDS.len()
    }

    /// Returns true when the dry-run facade is ready for later adapter wiring.
    pub const fn ready_for_adapter_wiring(self) -> bool {
        self.is_metadata_only() && self.covers_all_typed_records()
    }
}

/// Canonical dry-run status used by boot/runtime diagnostics.
pub const PHASE_36J_STATE_IO_BACKEND_DRY_RUN_STATUS: StateIoBackendDryRunStatus =
    StateIoBackendDryRunStatus {
        marker: PHASE_36J_STATE_IO_BACKEND_DRY_RUN_MARKER,
        state_root: PHASE_36J_X4_STATE_ROOT,
        record_count: PHASE_36J_TYPED_STATE_RECORDS.len(),
        moves_storage_backend: false,
        moves_spi_backend: false,
        changes_display_behavior: false,
        changes_input_behavior: false,
        changes_power_behavior: false,
        next_lane: PHASE_36J_NEXT_LANE,
    };

/// Compact boot/runtime status for logs or future diagnostics.
pub const fn phase36j_backend_dry_run_status() -> StateIoBackendDryRunStatus {
    PHASE_36J_STATE_IO_BACKEND_DRY_RUN_STATUS
}

/// Returns the accepted marker for this phase.
pub const fn phase36j_marker() -> &'static str {
    PHASE_36J_STATE_IO_BACKEND_DRY_RUN_MARKER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_run_status_is_metadata_only() {
        assert!(PHASE_36J_STATE_IO_BACKEND_DRY_RUN_STATUS.is_metadata_only());
    }

    #[test]
    fn dry_run_status_covers_all_records() {
        assert!(PHASE_36J_STATE_IO_BACKEND_DRY_RUN_STATUS.covers_all_typed_records());
    }

    #[test]
    fn per_book_suffixes_are_stable() {
        assert_eq!(DryRunStateRecordKind::Progress.suffix(), ".PRG");
        assert_eq!(DryRunStateRecordKind::Theme.suffix(), ".THM");
        assert_eq!(DryRunStateRecordKind::Metadata.suffix(), ".MTA");
        assert_eq!(DryRunStateRecordKind::Bookmarks.suffix(), ".BKM");
        assert_eq!(DryRunStateRecordKind::BookmarkIndex.suffix(), "BMIDX.TXT");
    }

    #[test]
    fn marker_is_stable() {
        assert_eq!(phase36j_marker(), "phase36j=x4-state-io-backend-dry-run-ok");
    }
}
