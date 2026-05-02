//! Phase 36R — State I/O Real Backend Scaffold Overlay.
//!
//! This module defines the first compile-only scaffold for a future typed state
//! backend. It declares the backend trait shape, request/response envelopes, and
//! operation catalog, but it deliberately performs no storage, display, input,
//! power, or SPI work.

#![allow(dead_code)]

use super::state_io_real_backend_entry_contract::{
    RealBackendEntryContract, phase36q_entry_contract, phase36q_is_ready_for_backend_scaffold,
    phase36q_marker,
};

/// Phase 36R boot/build marker.
pub const PHASE_36R_STATE_IO_REAL_BACKEND_SCAFFOLD_MARKER: &str =
    "phase36r=x4-state-io-real-backend-scaffold-ok";

/// Prior entry-contract marker required before this scaffold is valid.
pub const REQUIRED_PHASE_36Q_MARKER: &str = "phase36q=x4-state-io-real-backend-entry-contract-ok";

/// Records included in the first typed-state backend scaffold.
pub const STATE_IO_REAL_BACKEND_SCAFFOLD_RECORDS: [StateRecordKind; 5] = [
    StateRecordKind::Progress,
    StateRecordKind::Theme,
    StateRecordKind::Metadata,
    StateRecordKind::Bookmark,
    StateRecordKind::BookmarkIndex,
];

/// Operations declared by the scaffold. These are interface shapes only.
pub const STATE_IO_REAL_BACKEND_SCAFFOLD_OPERATIONS: [StateBackendOperation; 5] = [
    StateBackendOperation::Probe,
    StateBackendOperation::Load,
    StateBackendOperation::Store,
    StateBackendOperation::Remove,
    StateBackendOperation::RebuildIndex,
];

/// Guardrails that must remain true while this scaffold is accepted.
pub const STATE_IO_REAL_BACKEND_SCAFFOLD_GUARDRAILS: [&str; 10] = [
    "phase36q-entry-contract-ready",
    "backend-trait-shape-only",
    "operation-envelope-only",
    "side-effects-disabled",
    "existing-pulp-runtime-remains-authoritative",
    "shadow-write-plan-remains-required",
    "commit-plan-remains-required",
    "reader-ui-unchanged",
    "hardware-behavior-unmoved",
    "rollback-path-preserved",
];

/// Minimum operation count expected for this scaffold.
pub const REQUIRED_SCAFFOLD_OPERATION_COUNT: usize = 5;

/// Minimum record kind count expected for this scaffold.
pub const REQUIRED_SCAFFOLD_RECORD_COUNT: usize = 5;

/// Minimum guardrail count expected for this scaffold.
pub const REQUIRED_SCAFFOLD_GUARDRAIL_COUNT: usize = 10;

/// Typed-state record categories covered by the first backend scaffold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl StateRecordKind {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Progress => "PRG",
            Self::Theme => "THM",
            Self::Metadata => "MTA",
            Self::Bookmark => "BKM",
            Self::BookmarkIndex => "TXT",
        }
    }

    pub const fn path_template(self) -> &'static str {
        match self {
            Self::Progress => "state/<BOOKID>.PRG",
            Self::Theme => "state/<BOOKID>.THM",
            Self::Metadata => "state/<BOOKID>.MTA",
            Self::Bookmark => "state/<BOOKID>.BKM",
            Self::BookmarkIndex => "state/BMIDX.TXT",
        }
    }

    pub const fn requires_book_id(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }

    pub const fn is_index_record(self) -> bool {
        matches!(self, Self::BookmarkIndex)
    }
}

/// Compile-only backend operations. These names define intent, not execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateBackendOperation {
    Probe,
    Load,
    Store,
    Remove,
    RebuildIndex,
}

impl StateBackendOperation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Probe => "probe",
            Self::Load => "load",
            Self::Store => "store",
            Self::Remove => "remove",
            Self::RebuildIndex => "rebuild-index",
        }
    }

    pub const fn may_change_state(self) -> bool {
        matches!(self, Self::Store | Self::Remove | Self::RebuildIndex)
    }
}

/// Request envelope for a future typed-state backend implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateBackendRequest<'a> {
    pub operation: StateBackendOperation,
    pub record_kind: StateRecordKind,
    pub book_id: Option<&'a str>,
    pub payload: Option<&'a [u8]>,
}

impl<'a> StateBackendRequest<'a> {
    pub const fn new(
        operation: StateBackendOperation,
        record_kind: StateRecordKind,
        book_id: Option<&'a str>,
        payload: Option<&'a [u8]>,
    ) -> Self {
        Self {
            operation,
            record_kind,
            book_id,
            payload,
        }
    }

    pub const fn is_shape_valid(self) -> bool {
        let has_required_book_id = !self.record_kind.requires_book_id() || self.book_id.is_some();
        let has_payload_when_required = match self.operation {
            StateBackendOperation::Store => self.payload.is_some(),
            _ => true,
        };

        has_required_book_id && has_payload_when_required
    }
}

/// Response envelope for a future typed-state backend implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateBackendResponse<'a> {
    pub operation: StateBackendOperation,
    pub record_kind: StateRecordKind,
    pub path_template: &'static str,
    pub payload: Option<&'a [u8]>,
    pub changed_state: bool,
}

impl<'a> StateBackendResponse<'a> {
    pub const fn dry_shape(
        operation: StateBackendOperation,
        record_kind: StateRecordKind,
        payload: Option<&'a [u8]>,
    ) -> Self {
        Self {
            operation,
            record_kind,
            path_template: record_kind.path_template(),
            payload,
            changed_state: false,
        }
    }
}

/// Probe information exposed by a future typed-state backend implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateBackendProbe {
    pub backend_name: &'static str,
    pub marker: &'static str,
    pub ready: bool,
    pub side_effects_enabled: bool,
    pub record_count: usize,
    pub operation_count: usize,
}

/// Trait shape for a future real typed-state backend.
///
/// Implementors must remain disabled until a later phase explicitly binds this
/// trait to the X4 storage runtime and updates the acceptance gate.
pub trait StateIoBackendScaffold {
    type Error;

    fn probe(&mut self) -> Result<StateBackendProbe, Self::Error>;

    fn load<'a>(
        &mut self,
        request: StateBackendRequest<'a>,
    ) -> Result<StateBackendResponse<'a>, Self::Error>;

    fn store<'a>(
        &mut self,
        request: StateBackendRequest<'a>,
    ) -> Result<StateBackendResponse<'a>, Self::Error>;

    fn remove<'a>(
        &mut self,
        request: StateBackendRequest<'a>,
    ) -> Result<StateBackendResponse<'a>, Self::Error>;
}

/// Compile-only operation plan for the future backend lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateBackendOperationPlan {
    pub record_kind: StateRecordKind,
    pub operation: StateBackendOperation,
    pub path_template: &'static str,
    pub requires_book_id: bool,
    pub may_change_state: bool,
    pub side_effects_enabled: bool,
    pub requires_shadow_plan: bool,
    pub requires_commit_plan: bool,
}

impl StateBackendOperationPlan {
    pub const fn new(record_kind: StateRecordKind, operation: StateBackendOperation) -> Self {
        let may_change_state = operation.may_change_state();
        Self {
            record_kind,
            operation,
            path_template: record_kind.path_template(),
            requires_book_id: record_kind.requires_book_id(),
            may_change_state,
            side_effects_enabled: false,
            requires_shadow_plan: may_change_state,
            requires_commit_plan: may_change_state,
        }
    }

    pub const fn is_side_effect_free(self) -> bool {
        !self.side_effects_enabled
    }
}

/// Side-effect-free scaffold report for boot/runtime status surfaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoRealBackendScaffold {
    pub marker: &'static str,
    pub required_entry_marker: &'static str,
    pub observed_entry_marker: &'static str,
    pub entry_ready: bool,
    pub record_count: usize,
    pub operation_count: usize,
    pub guardrail_count: usize,
    pub required_record_count: usize,
    pub required_operation_count: usize,
    pub required_guardrail_count: usize,
    pub backend_default_enabled: bool,
    pub side_effects_enabled: bool,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
    pub spi_behavior_moved: bool,
    pub next_phase: &'static str,
}

impl StateIoRealBackendScaffold {
    pub const fn from_entry(entry: RealBackendEntryContract) -> Self {
        let record_count = STATE_IO_REAL_BACKEND_SCAFFOLD_RECORDS.len();
        let operation_count = STATE_IO_REAL_BACKEND_SCAFFOLD_OPERATIONS.len();
        let guardrail_count = STATE_IO_REAL_BACKEND_SCAFFOLD_GUARDRAILS.len();
        let backend_default_enabled = false;
        let side_effects_enabled = false;
        let storage_behavior_moved = false;
        let display_behavior_moved = false;
        let input_behavior_moved = false;
        let power_behavior_moved = false;
        let spi_behavior_moved = false;

        Self {
            marker: PHASE_36R_STATE_IO_REAL_BACKEND_SCAFFOLD_MARKER,
            required_entry_marker: REQUIRED_PHASE_36Q_MARKER,
            observed_entry_marker: phase36q_marker(),
            entry_ready: entry.is_ready() && phase36q_is_ready_for_backend_scaffold(),
            record_count,
            operation_count,
            guardrail_count,
            required_record_count: REQUIRED_SCAFFOLD_RECORD_COUNT,
            required_operation_count: REQUIRED_SCAFFOLD_OPERATION_COUNT,
            required_guardrail_count: REQUIRED_SCAFFOLD_GUARDRAIL_COUNT,
            backend_default_enabled,
            side_effects_enabled,
            storage_behavior_moved,
            display_behavior_moved,
            input_behavior_moved,
            power_behavior_moved,
            spi_behavior_moved,
            next_phase: "phase36s-state-io-real-backend-scaffold-acceptance",
        }
    }

    pub const fn current() -> Self {
        Self::from_entry(phase36q_entry_contract())
    }

    pub const fn is_accepted(self) -> bool {
        self.entry_ready
            && self.record_count >= self.required_record_count
            && self.operation_count >= self.required_operation_count
            && self.guardrail_count >= self.required_guardrail_count
            && !self.backend_default_enabled
            && !self.side_effects_enabled
            && !self.storage_behavior_moved
            && !self.display_behavior_moved
            && !self.input_behavior_moved
            && !self.power_behavior_moved
            && !self.spi_behavior_moved
    }
}

/// Compile-time scaffold report for Phase 36R.
pub const STATE_IO_REAL_BACKEND_SCAFFOLD: StateIoRealBackendScaffold =
    StateIoRealBackendScaffold::current();

/// Return the accepted Phase 36R marker for boot/runtime status reporting.
pub const fn phase36r_marker() -> &'static str {
    PHASE_36R_STATE_IO_REAL_BACKEND_SCAFFOLD_MARKER
}

/// Return the side-effect-free real-backend scaffold report.
pub const fn phase36r_real_backend_scaffold() -> StateIoRealBackendScaffold {
    STATE_IO_REAL_BACKEND_SCAFFOLD
}

/// Return whether this scaffold is accepted and still side-effect free.
pub const fn phase36r_is_accepted() -> bool {
    STATE_IO_REAL_BACKEND_SCAFFOLD.is_accepted()
}

/// Return a representative operation plan for compile-time contract checks.
pub const fn phase36r_operation_plan(
    record_kind: StateRecordKind,
    operation: StateBackendOperation,
) -> StateBackendOperationPlan {
    StateBackendOperationPlan::new(record_kind, operation)
}
