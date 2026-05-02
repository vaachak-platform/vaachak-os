// Phase 37H — State I/O Typed Read-Only Result Normalizer.
//
// This module normalizes the Phase 37F read-only backend seam into a stable
// runtime-facing result shape. It is deliberately side-effect free: it does not
// call storage, does not expose mutation operations, and does not alter board
// behavior.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_first_real_read_only_typed_backend_binding::{
    PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER,
    Phase37fReadOnlyBackendOutcome, Phase37fReadOnlyBackendStatus, Phase37fTypedStateRecordKind,
    Phase37fTypedStateRecordRef,
};
use crate::vaachak_x4::runtime::state_io_first_real_read_only_typed_backend_binding_acceptance::PHASE_37G_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_ACCEPTANCE_MARKER;

pub const PHASE_37H_STATE_IO_TYPED_READ_ONLY_RESULT_NORMALIZER_MARKER: &str =
    "phase37h=x4-state-io-typed-read-only-result-normalizer-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37hNormalizedReadStatus {
    Present,
    Missing,
    OutputBufferTooSmall,
    BackendUnavailable,
    UnsupportedRecordKind,
    InvalidRequest,
    CorruptRecord,
}

impl Phase37hNormalizedReadStatus {
    pub const fn is_read_only(self) -> bool {
        matches!(
            self,
            Self::Present
                | Self::Missing
                | Self::OutputBufferTooSmall
                | Self::BackendUnavailable
                | Self::UnsupportedRecordKind
                | Self::InvalidRequest
                | Self::CorruptRecord
        )
    }

    pub const fn is_payload_available(self) -> bool {
        matches!(self, Self::Present)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37hNormalizedReadDisposition {
    UsePayload,
    UseDefaultState,
    RetryWithLargerBuffer,
    DeferUntilBackendReady,
    IgnoreUnsupportedRecord,
    RejectInvalidRequest,
    RejectCorruptRecord,
}

impl Phase37hNormalizedReadDisposition {
    pub const fn is_write_enabled(self) -> bool {
        let _ = self;
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37hNormalizedReadResult {
    pub record_ref: Phase37fTypedStateRecordRef,
    pub record_kind: Phase37fTypedStateRecordKind,
    pub rendered_path_len: usize,
    pub payload_len: usize,
    pub required_capacity: usize,
    pub status: Phase37hNormalizedReadStatus,
    pub disposition: Phase37hNormalizedReadDisposition,
    pub mutation_operations_enabled: bool,
}

impl Phase37hNormalizedReadResult {
    pub const fn is_read_only(self) -> bool {
        !self.mutation_operations_enabled
            && self.status.is_read_only()
            && !self.disposition.is_write_enabled()
    }

    pub const fn has_payload(self) -> bool {
        self.status.is_payload_available() && self.payload_len > 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37hNormalizerReport {
    pub marker: &'static str,
    pub phase37f_marker: &'static str,
    pub phase37g_marker: &'static str,
    pub supported_record_kinds: &'static [Phase37fTypedStateRecordKind],
    pub normalized_statuses: &'static [Phase37hNormalizedReadStatus],
    pub mutation_operations_enabled: bool,
    pub ready_for_read_only_state_consumer: bool,
}

impl Phase37hNormalizerReport {
    pub fn is_accepted(&self) -> bool {
        !self.mutation_operations_enabled
            && self.ready_for_read_only_state_consumer
            && self
                .normalized_statuses
                .iter()
                .copied()
                .all(|status| status.is_read_only())
    }
}

pub const PHASE_37H_SUPPORTED_RECORD_KINDS: &[Phase37fTypedStateRecordKind] = &[
    Phase37fTypedStateRecordKind::Progress,
    Phase37fTypedStateRecordKind::Theme,
    Phase37fTypedStateRecordKind::Metadata,
    Phase37fTypedStateRecordKind::Bookmark,
    Phase37fTypedStateRecordKind::BookmarkIndex,
];

pub const PHASE_37H_NORMALIZED_STATUSES: &[Phase37hNormalizedReadStatus] = &[
    Phase37hNormalizedReadStatus::Present,
    Phase37hNormalizedReadStatus::Missing,
    Phase37hNormalizedReadStatus::OutputBufferTooSmall,
    Phase37hNormalizedReadStatus::BackendUnavailable,
    Phase37hNormalizedReadStatus::UnsupportedRecordKind,
    Phase37hNormalizedReadStatus::InvalidRequest,
    Phase37hNormalizedReadStatus::CorruptRecord,
];

pub fn phase37h_normalizer_report() -> Phase37hNormalizerReport {
    Phase37hNormalizerReport {
        marker: PHASE_37H_STATE_IO_TYPED_READ_ONLY_RESULT_NORMALIZER_MARKER,
        phase37f_marker: PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER,
        phase37g_marker:
            PHASE_37G_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_ACCEPTANCE_MARKER,
        supported_record_kinds: PHASE_37H_SUPPORTED_RECORD_KINDS,
        normalized_statuses: PHASE_37H_NORMALIZED_STATUSES,
        mutation_operations_enabled: false,
        ready_for_read_only_state_consumer: true,
    }
}

pub fn phase37h_normalize_backend_outcome(
    outcome: Phase37fReadOnlyBackendOutcome,
) -> Phase37hNormalizedReadResult {
    let (status, payload_len, required_capacity, disposition) = match outcome.status {
        Phase37fReadOnlyBackendStatus::Found { bytes_read } => (
            Phase37hNormalizedReadStatus::Present,
            bytes_read,
            bytes_read,
            Phase37hNormalizedReadDisposition::UsePayload,
        ),
        Phase37fReadOnlyBackendStatus::NotFound => (
            Phase37hNormalizedReadStatus::Missing,
            0,
            0,
            Phase37hNormalizedReadDisposition::UseDefaultState,
        ),
        Phase37fReadOnlyBackendStatus::BufferTooSmall { required_capacity } => (
            Phase37hNormalizedReadStatus::OutputBufferTooSmall,
            0,
            required_capacity,
            Phase37hNormalizedReadDisposition::RetryWithLargerBuffer,
        ),
        Phase37fReadOnlyBackendStatus::BackendUnavailable => (
            Phase37hNormalizedReadStatus::BackendUnavailable,
            0,
            0,
            Phase37hNormalizedReadDisposition::DeferUntilBackendReady,
        ),
        Phase37fReadOnlyBackendStatus::UnsupportedRecordKind => (
            Phase37hNormalizedReadStatus::UnsupportedRecordKind,
            0,
            0,
            Phase37hNormalizedReadDisposition::IgnoreUnsupportedRecord,
        ),
        Phase37fReadOnlyBackendStatus::InvalidRequest => (
            Phase37hNormalizedReadStatus::InvalidRequest,
            0,
            0,
            Phase37hNormalizedReadDisposition::RejectInvalidRequest,
        ),
        Phase37fReadOnlyBackendStatus::CorruptRecord => (
            Phase37hNormalizedReadStatus::CorruptRecord,
            0,
            0,
            Phase37hNormalizedReadDisposition::RejectCorruptRecord,
        ),
    };

    Phase37hNormalizedReadResult {
        record_ref: outcome.record_ref,
        record_kind: outcome.record_ref.kind,
        rendered_path_len: outcome.rendered_path_len,
        payload_len,
        required_capacity,
        status,
        disposition,
        mutation_operations_enabled: false,
    }
}

pub fn phase37h_default_result_for_record(
    record_ref: Phase37fTypedStateRecordRef,
) -> Phase37hNormalizedReadResult {
    phase37h_normalize_backend_outcome(Phase37fReadOnlyBackendOutcome {
        record_ref,
        rendered_path_len: 0,
        status: Phase37fReadOnlyBackendStatus::NotFound,
    })
}

pub fn phase37h_status_for_backend_status(
    status: Phase37fReadOnlyBackendStatus,
) -> Phase37hNormalizedReadStatus {
    let record_ref = Phase37fTypedStateRecordRef::bookmark_index();
    phase37h_normalize_backend_outcome(Phase37fReadOnlyBackendOutcome {
        record_ref,
        rendered_path_len: 0,
        status,
    })
    .status
}

pub fn phase37h_is_status_read_only(status: Phase37hNormalizedReadStatus) -> bool {
    status.is_read_only()
}

pub fn phase37h_result_is_safe_for_consumer(result: Phase37hNormalizedReadResult) -> bool {
    result.is_read_only()
}

pub fn phase37h_next_lane() -> &'static str {
    "typed-state-read-only-consumer-seam"
}
