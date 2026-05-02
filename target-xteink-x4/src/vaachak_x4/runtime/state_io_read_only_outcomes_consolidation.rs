// Phase 37I — State I/O Read-Only Outcomes Consolidation.
//
// This module is the final read-only consolidation gate before the write lane.
// It enumerates the backend, path-rendering, normalization, and consumer-facing
// outcomes for typed state reads. It remains side-effect free: it invokes no
// filesystem, storage-bus, display, input, or power behavior and it enables no
// mutation operations.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_first_real_read_only_typed_backend_binding::{
    PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER, Phase37fPathRenderError,
    Phase37fReadOnlyBackendStatus, Phase37fTypedStateRecordKind,
    phase37f_is_backend_status_read_only, phase37f_supported_record_kinds,
};
use crate::vaachak_x4::runtime::state_io_first_real_read_only_typed_backend_binding_acceptance::PHASE_37G_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_ACCEPTANCE_MARKER;
use crate::vaachak_x4::runtime::state_io_typed_read_only_result_normalizer::{
    PHASE_37H_STATE_IO_TYPED_READ_ONLY_RESULT_NORMALIZER_MARKER, Phase37hNormalizedReadDisposition,
    Phase37hNormalizedReadStatus,
};

pub const PHASE_37I_STATE_IO_READ_ONLY_OUTCOMES_CONSOLIDATION_MARKER: &str =
    "phase37i=x4-state-io-read-only-outcomes-consolidation-ok";

pub const PHASE_37I_REQUIRED_RECORD_KIND_COUNT: usize = 5;
pub const PHASE_37I_BACKEND_OUTCOME_COUNT: usize = 7;
pub const PHASE_37I_PATH_OUTCOME_COUNT: usize = 4;
pub const PHASE_37I_CONSUMER_OUTCOME_COUNT: usize = 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37iReadOnlyBackendOutcome {
    Present,
    Missing,
    OutputBufferTooSmall,
    BackendUnavailable,
    UnsupportedRecordKind,
    InvalidRequest,
    CorruptRecord,
}

impl Phase37iReadOnlyBackendOutcome {
    pub const fn mutation_operations_enabled(self) -> bool {
        let _ = self;
        false
    }

    pub const fn as_backend_status(self) -> Phase37fReadOnlyBackendStatus {
        match self {
            Self::Present => Phase37fReadOnlyBackendStatus::Found { bytes_read: 1 },
            Self::Missing => Phase37fReadOnlyBackendStatus::NotFound,
            Self::OutputBufferTooSmall => Phase37fReadOnlyBackendStatus::BufferTooSmall {
                required_capacity: 1,
            },
            Self::BackendUnavailable => Phase37fReadOnlyBackendStatus::BackendUnavailable,
            Self::UnsupportedRecordKind => Phase37fReadOnlyBackendStatus::UnsupportedRecordKind,
            Self::InvalidRequest => Phase37fReadOnlyBackendStatus::InvalidRequest,
            Self::CorruptRecord => Phase37fReadOnlyBackendStatus::CorruptRecord,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37iPathOutcome {
    Rendered,
    PathBufferTooSmall,
    MissingBookId,
    UnexpectedBookId,
}

impl Phase37iPathOutcome {
    pub const fn mutation_operations_enabled(self) -> bool {
        let _ = self;
        false
    }

    pub const fn from_path_error(error: Phase37fPathRenderError) -> Self {
        match error {
            Phase37fPathRenderError::BufferTooSmall => Self::PathBufferTooSmall,
            Phase37fPathRenderError::MissingBookId => Self::MissingBookId,
            Phase37fPathRenderError::UnexpectedBookId => Self::UnexpectedBookId,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37iConsumerOutcome {
    UsePayload,
    UseDefaultState,
    RetryWithLargerBuffer,
    DeferUntilBackendReady,
    IgnoreUnsupportedRecord,
    RejectInvalidRequest,
    RejectCorruptRecord,
}

impl Phase37iConsumerOutcome {
    pub const fn mutation_operations_enabled(self) -> bool {
        let _ = self;
        false
    }

    pub const fn from_disposition(disposition: Phase37hNormalizedReadDisposition) -> Self {
        match disposition {
            Phase37hNormalizedReadDisposition::UsePayload => Self::UsePayload,
            Phase37hNormalizedReadDisposition::UseDefaultState => Self::UseDefaultState,
            Phase37hNormalizedReadDisposition::RetryWithLargerBuffer => Self::RetryWithLargerBuffer,
            Phase37hNormalizedReadDisposition::DeferUntilBackendReady => {
                Self::DeferUntilBackendReady
            }
            Phase37hNormalizedReadDisposition::IgnoreUnsupportedRecord => {
                Self::IgnoreUnsupportedRecord
            }
            Phase37hNormalizedReadDisposition::RejectInvalidRequest => Self::RejectInvalidRequest,
            Phase37hNormalizedReadDisposition::RejectCorruptRecord => Self::RejectCorruptRecord,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37iReadLaneExitDecision {
    ReadyForWriteLaneDesign,
    HoldForMissingReadOutcome,
    HoldForMutationLeak,
    HoldForRecordKindGap,
}

impl Phase37iReadLaneExitDecision {
    pub const fn permits_write_operations(self) -> bool {
        let _ = self;
        false
    }

    pub const fn is_ready_for_next_phase(self) -> bool {
        matches!(self, Self::ReadyForWriteLaneDesign)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37iReadOnlyOutcomeCoverageReport {
    pub marker: &'static str,
    pub phase37f_marker: &'static str,
    pub phase37g_marker: &'static str,
    pub phase37h_marker: &'static str,
    pub record_kinds: &'static [Phase37fTypedStateRecordKind],
    pub backend_outcomes: &'static [Phase37iReadOnlyBackendOutcome],
    pub path_outcomes: &'static [Phase37iPathOutcome],
    pub consumer_outcomes: &'static [Phase37iConsumerOutcome],
    pub normalized_statuses: &'static [Phase37hNormalizedReadStatus],
    pub mutation_operations_enabled: bool,
    pub write_lane_design_ready: bool,
    pub exit_decision: Phase37iReadLaneExitDecision,
}

impl Phase37iReadOnlyOutcomeCoverageReport {
    pub fn is_accepted(&self) -> bool {
        !self.mutation_operations_enabled
            && self.write_lane_design_ready
            && self.exit_decision.is_ready_for_next_phase()
            && !self.exit_decision.permits_write_operations()
            && phase37i_record_kinds_complete(self.record_kinds)
            && phase37i_backend_outcomes_complete(self.backend_outcomes)
            && phase37i_path_outcomes_complete(self.path_outcomes)
            && phase37i_consumer_outcomes_complete(self.consumer_outcomes)
            && phase37i_normalized_statuses_complete(self.normalized_statuses)
            && phase37i_all_backend_outcomes_are_read_only()
            && phase37i_all_outcomes_disable_mutation()
    }
}

pub const PHASE_37I_BACKEND_OUTCOMES: &[Phase37iReadOnlyBackendOutcome] = &[
    Phase37iReadOnlyBackendOutcome::Present,
    Phase37iReadOnlyBackendOutcome::Missing,
    Phase37iReadOnlyBackendOutcome::OutputBufferTooSmall,
    Phase37iReadOnlyBackendOutcome::BackendUnavailable,
    Phase37iReadOnlyBackendOutcome::UnsupportedRecordKind,
    Phase37iReadOnlyBackendOutcome::InvalidRequest,
    Phase37iReadOnlyBackendOutcome::CorruptRecord,
];

pub const PHASE_37I_PATH_OUTCOMES: &[Phase37iPathOutcome] = &[
    Phase37iPathOutcome::Rendered,
    Phase37iPathOutcome::PathBufferTooSmall,
    Phase37iPathOutcome::MissingBookId,
    Phase37iPathOutcome::UnexpectedBookId,
];

pub const PHASE_37I_CONSUMER_OUTCOMES: &[Phase37iConsumerOutcome] = &[
    Phase37iConsumerOutcome::UsePayload,
    Phase37iConsumerOutcome::UseDefaultState,
    Phase37iConsumerOutcome::RetryWithLargerBuffer,
    Phase37iConsumerOutcome::DeferUntilBackendReady,
    Phase37iConsumerOutcome::IgnoreUnsupportedRecord,
    Phase37iConsumerOutcome::RejectInvalidRequest,
    Phase37iConsumerOutcome::RejectCorruptRecord,
];

pub const PHASE_37I_NORMALIZED_STATUSES: &[Phase37hNormalizedReadStatus] = &[
    Phase37hNormalizedReadStatus::Present,
    Phase37hNormalizedReadStatus::Missing,
    Phase37hNormalizedReadStatus::OutputBufferTooSmall,
    Phase37hNormalizedReadStatus::BackendUnavailable,
    Phase37hNormalizedReadStatus::UnsupportedRecordKind,
    Phase37hNormalizedReadStatus::InvalidRequest,
    Phase37hNormalizedReadStatus::CorruptRecord,
];

pub fn phase37i_read_only_outcome_coverage_report() -> Phase37iReadOnlyOutcomeCoverageReport {
    let record_kinds = phase37f_supported_record_kinds();
    let write_lane_design_ready = phase37i_record_kinds_complete(record_kinds)
        && phase37i_backend_outcomes_complete(PHASE_37I_BACKEND_OUTCOMES)
        && phase37i_path_outcomes_complete(PHASE_37I_PATH_OUTCOMES)
        && phase37i_consumer_outcomes_complete(PHASE_37I_CONSUMER_OUTCOMES)
        && phase37i_normalized_statuses_complete(PHASE_37I_NORMALIZED_STATUSES)
        && phase37i_all_backend_outcomes_are_read_only()
        && phase37i_all_outcomes_disable_mutation();

    Phase37iReadOnlyOutcomeCoverageReport {
        marker: PHASE_37I_STATE_IO_READ_ONLY_OUTCOMES_CONSOLIDATION_MARKER,
        phase37f_marker: PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER,
        phase37g_marker:
            PHASE_37G_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_ACCEPTANCE_MARKER,
        phase37h_marker: PHASE_37H_STATE_IO_TYPED_READ_ONLY_RESULT_NORMALIZER_MARKER,
        record_kinds,
        backend_outcomes: PHASE_37I_BACKEND_OUTCOMES,
        path_outcomes: PHASE_37I_PATH_OUTCOMES,
        consumer_outcomes: PHASE_37I_CONSUMER_OUTCOMES,
        normalized_statuses: PHASE_37I_NORMALIZED_STATUSES,
        mutation_operations_enabled: false,
        write_lane_design_ready,
        exit_decision: phase37i_exit_decision(write_lane_design_ready),
    }
}

pub fn phase37i_exit_decision(write_lane_design_ready: bool) -> Phase37iReadLaneExitDecision {
    if write_lane_design_ready {
        Phase37iReadLaneExitDecision::ReadyForWriteLaneDesign
    } else {
        Phase37iReadLaneExitDecision::HoldForMissingReadOutcome
    }
}

pub fn phase37i_record_kinds_complete(kinds: &[Phase37fTypedStateRecordKind]) -> bool {
    kinds.len() == PHASE_37I_REQUIRED_RECORD_KIND_COUNT
        && kinds.contains(&Phase37fTypedStateRecordKind::Progress)
        && kinds.contains(&Phase37fTypedStateRecordKind::Theme)
        && kinds.contains(&Phase37fTypedStateRecordKind::Metadata)
        && kinds.contains(&Phase37fTypedStateRecordKind::Bookmark)
        && kinds.contains(&Phase37fTypedStateRecordKind::BookmarkIndex)
}

pub fn phase37i_backend_outcomes_complete(outcomes: &[Phase37iReadOnlyBackendOutcome]) -> bool {
    outcomes.len() == PHASE_37I_BACKEND_OUTCOME_COUNT
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::Present)
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::Missing)
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::OutputBufferTooSmall)
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::BackendUnavailable)
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::UnsupportedRecordKind)
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::InvalidRequest)
        && outcomes.contains(&Phase37iReadOnlyBackendOutcome::CorruptRecord)
}

pub fn phase37i_path_outcomes_complete(outcomes: &[Phase37iPathOutcome]) -> bool {
    outcomes.len() == PHASE_37I_PATH_OUTCOME_COUNT
        && outcomes.contains(&Phase37iPathOutcome::Rendered)
        && outcomes.contains(&Phase37iPathOutcome::PathBufferTooSmall)
        && outcomes.contains(&Phase37iPathOutcome::MissingBookId)
        && outcomes.contains(&Phase37iPathOutcome::UnexpectedBookId)
}

pub fn phase37i_consumer_outcomes_complete(outcomes: &[Phase37iConsumerOutcome]) -> bool {
    outcomes.len() == PHASE_37I_CONSUMER_OUTCOME_COUNT
        && outcomes.contains(&Phase37iConsumerOutcome::UsePayload)
        && outcomes.contains(&Phase37iConsumerOutcome::UseDefaultState)
        && outcomes.contains(&Phase37iConsumerOutcome::RetryWithLargerBuffer)
        && outcomes.contains(&Phase37iConsumerOutcome::DeferUntilBackendReady)
        && outcomes.contains(&Phase37iConsumerOutcome::IgnoreUnsupportedRecord)
        && outcomes.contains(&Phase37iConsumerOutcome::RejectInvalidRequest)
        && outcomes.contains(&Phase37iConsumerOutcome::RejectCorruptRecord)
}

pub fn phase37i_normalized_statuses_complete(statuses: &[Phase37hNormalizedReadStatus]) -> bool {
    statuses.len() == PHASE_37I_BACKEND_OUTCOME_COUNT
        && statuses.contains(&Phase37hNormalizedReadStatus::Present)
        && statuses.contains(&Phase37hNormalizedReadStatus::Missing)
        && statuses.contains(&Phase37hNormalizedReadStatus::OutputBufferTooSmall)
        && statuses.contains(&Phase37hNormalizedReadStatus::BackendUnavailable)
        && statuses.contains(&Phase37hNormalizedReadStatus::UnsupportedRecordKind)
        && statuses.contains(&Phase37hNormalizedReadStatus::InvalidRequest)
        && statuses.contains(&Phase37hNormalizedReadStatus::CorruptRecord)
}

pub fn phase37i_all_backend_outcomes_are_read_only() -> bool {
    PHASE_37I_BACKEND_OUTCOMES
        .iter()
        .copied()
        .map(Phase37iReadOnlyBackendOutcome::as_backend_status)
        .all(phase37f_is_backend_status_read_only)
}

pub fn phase37i_all_outcomes_disable_mutation() -> bool {
    PHASE_37I_BACKEND_OUTCOMES
        .iter()
        .copied()
        .all(|outcome| !outcome.mutation_operations_enabled())
        && PHASE_37I_PATH_OUTCOMES
            .iter()
            .copied()
            .all(|outcome| !outcome.mutation_operations_enabled())
        && PHASE_37I_CONSUMER_OUTCOMES
            .iter()
            .copied()
            .all(|outcome| !outcome.mutation_operations_enabled())
}

pub fn phase37i_has_backend_outcome(outcome: Phase37iReadOnlyBackendOutcome) -> bool {
    PHASE_37I_BACKEND_OUTCOMES.contains(&outcome)
}

pub fn phase37i_has_path_outcome(outcome: Phase37iPathOutcome) -> bool {
    PHASE_37I_PATH_OUTCOMES.contains(&outcome)
}

pub fn phase37i_has_consumer_outcome(outcome: Phase37iConsumerOutcome) -> bool {
    PHASE_37I_CONSUMER_OUTCOMES.contains(&outcome)
}

pub fn phase37i_writes_enabled() -> bool {
    false
}

pub fn phase37i_next_lane() -> &'static str {
    "state-io-write-lane-design"
}
