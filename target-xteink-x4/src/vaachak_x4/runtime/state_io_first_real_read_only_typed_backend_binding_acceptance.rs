// Phase 37G — State I/O First Real Read-Only Typed Backend Binding Acceptance.
//
// This module accepts the Phase 37F typed-state read-only binding seam as the
// controlled entry point for later backend work. It is intentionally a report
// layer only: it reads no files, mutates no files, and invokes no board-owned
// behavior.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_first_real_read_only_typed_backend_binding::{
    PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER,
    Phase37fReadOnlyBackendStatus, Phase37fTypedStateRecordKind,
    phase37f_is_backend_status_read_only, phase37f_supported_record_kinds,
};

pub const PHASE_37G_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_ACCEPTANCE_MARKER: &str =
    "phase37g=x4-state-io-first-real-read-only-typed-backend-binding-acceptance-ok";

pub const PHASE_37G_REQUIRED_RECORD_KIND_COUNT: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37gAcceptanceItem {
    Phase37fMarkerReachable,
    ProgressRecordCovered,
    ThemeRecordCovered,
    MetadataRecordCovered,
    BookmarkRecordCovered,
    BookmarkIndexRecordCovered,
    BackendOutcomesRemainReadOnly,
    MutationOperationsRemainAbsent,
    ReadyForFirstReadOnlyBackendAdapter,
}

pub const PHASE_37G_ACCEPTANCE_ITEMS: &[Phase37gAcceptanceItem] = &[
    Phase37gAcceptanceItem::Phase37fMarkerReachable,
    Phase37gAcceptanceItem::ProgressRecordCovered,
    Phase37gAcceptanceItem::ThemeRecordCovered,
    Phase37gAcceptanceItem::MetadataRecordCovered,
    Phase37gAcceptanceItem::BookmarkRecordCovered,
    Phase37gAcceptanceItem::BookmarkIndexRecordCovered,
    Phase37gAcceptanceItem::BackendOutcomesRemainReadOnly,
    Phase37gAcceptanceItem::MutationOperationsRemainAbsent,
    Phase37gAcceptanceItem::ReadyForFirstReadOnlyBackendAdapter,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37gAcceptanceReport {
    pub marker: &'static str,
    pub accepted_items: &'static [Phase37gAcceptanceItem],
    pub accepted_item_count: usize,
    pub required_record_kind_count: usize,
    pub phase37f_marker: &'static str,
    pub phase37f_record_kinds_complete: bool,
    pub phase37f_statuses_are_read_only: bool,
    pub mutation_operations_enabled: bool,
    pub ready_for_first_read_only_backend_adapter: bool,
}

impl Phase37gAcceptanceReport {
    pub const fn is_accepted(self) -> bool {
        self.phase37f_record_kinds_complete
            && self.phase37f_statuses_are_read_only
            && !self.mutation_operations_enabled
            && self.ready_for_first_read_only_backend_adapter
    }
}

pub fn phase37g_acceptance_report() -> Phase37gAcceptanceReport {
    let phase37f_record_kinds_complete = phase37g_phase37f_record_kinds_complete();
    let phase37f_statuses_are_read_only = phase37g_phase37f_statuses_are_read_only();

    Phase37gAcceptanceReport {
        marker: PHASE_37G_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_ACCEPTANCE_MARKER,
        accepted_items: PHASE_37G_ACCEPTANCE_ITEMS,
        accepted_item_count: PHASE_37G_ACCEPTANCE_ITEMS.len(),
        required_record_kind_count: PHASE_37G_REQUIRED_RECORD_KIND_COUNT,
        phase37f_marker: PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER,
        phase37f_record_kinds_complete,
        phase37f_statuses_are_read_only,
        mutation_operations_enabled: false,
        ready_for_first_read_only_backend_adapter: phase37f_record_kinds_complete
            && phase37f_statuses_are_read_only,
    }
}

pub fn phase37g_has_acceptance_item(item: Phase37gAcceptanceItem) -> bool {
    PHASE_37G_ACCEPTANCE_ITEMS.contains(&item)
}

pub fn phase37g_phase37f_record_kinds_complete() -> bool {
    let kinds = phase37f_supported_record_kinds();
    kinds.len() == PHASE_37G_REQUIRED_RECORD_KIND_COUNT
        && kinds.contains(&Phase37fTypedStateRecordKind::Progress)
        && kinds.contains(&Phase37fTypedStateRecordKind::Theme)
        && kinds.contains(&Phase37fTypedStateRecordKind::Metadata)
        && kinds.contains(&Phase37fTypedStateRecordKind::Bookmark)
        && kinds.contains(&Phase37fTypedStateRecordKind::BookmarkIndex)
}

pub fn phase37g_phase37f_statuses_are_read_only() -> bool {
    let statuses = [
        Phase37fReadOnlyBackendStatus::Found { bytes_read: 0 },
        Phase37fReadOnlyBackendStatus::NotFound,
        Phase37fReadOnlyBackendStatus::BufferTooSmall {
            required_capacity: 1,
        },
        Phase37fReadOnlyBackendStatus::BackendUnavailable,
        Phase37fReadOnlyBackendStatus::UnsupportedRecordKind,
        Phase37fReadOnlyBackendStatus::InvalidRequest,
        Phase37fReadOnlyBackendStatus::CorruptRecord,
    ];

    statuses
        .iter()
        .copied()
        .all(phase37f_is_backend_status_read_only)
}

pub fn phase37g_next_lane() -> &'static str {
    "first-read-only-backend-adapter"
}
