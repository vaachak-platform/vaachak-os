//! Phase 39H — Typed State Runtime Callsite Wiring Bundle.
//!
//! This phase wires all typed-state write entrypoints through the Phase 39G
//! runtime file API integration gate and Phase 39F runtime-owned writer.
//!
//! Scope wired at once:
//! - `.PRG` progress state
//! - `.THM` theme state
//! - `.MTA` metadata state
//! - `.BKM` bookmark state
//! - `BMIDX.TXT` bookmark index
//!
//! This is intentionally a runtime-facing wiring facade. Existing reader/app
//! callsites can call these functions without knowing the lower-level Phase
//! 39D/39E/39F/39G details.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_runtime_file_api_integration_gate::{
    phase39g_execute_runtime_file_api_gate, Phase39gIntegrationGateDecision,
    Phase39gIntegrationGateReport, Phase39gIntegrationRequest, Phase39gRuntimeFileApiCandidate,
    Phase39gRuntimeFileApiProbe,
};
use crate::vaachak_x4::runtime::state_io_runtime_owned_sdfat_writer::Phase39fRuntimeOwnedFileOps;
use crate::vaachak_x4::runtime::state_io_typed_record_sdfat_adapter::Phase39eSdFatAdapterConfig;
use crate::vaachak_x4::runtime::state_io_typed_record_write_lane::{
    Phase39dBookId, Phase39dTypedRecordKind, Phase39dTypedWriteIntent, Phase39dTypedWriteMode,
    Phase39dTypedWritePreflight, Phase39dTypedWriteRequest,
};

pub const PHASE_39H_TYPED_STATE_RUNTIME_CALLSITE_WIRING_MARKER: &str =
    "phase39h=x4-typed-state-runtime-callsite-wiring-bundle-ok";

pub const PHASE_39H_ALL_TYPED_STATE_ENTRYPOINTS_PRESENT: bool = true;
pub const PHASE_39H_READER_CALLSITES_CAN_USE_FACADE: bool = true;
pub const PHASE_39H_CONCRETE_FILESYSTEM_HARDCODED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hRuntimeWriteEnablement {
    Disabled,
    DryRun,
    Commit,
}

impl Phase39hRuntimeWriteEnablement {
    pub const fn write_mode(self) -> Phase39dTypedWriteMode {
        match self {
            Self::Disabled | Self::DryRun => Phase39dTypedWriteMode::DryRun,
            Self::Commit => Phase39dTypedWriteMode::CommitToRecordingBackend,
        }
    }

    pub const fn probe(self, candidate: Phase39gRuntimeFileApiCandidate) -> Phase39gRuntimeFileApiProbe {
        match self {
            Self::Disabled => Phase39gRuntimeFileApiProbe::disabled(),
            Self::DryRun => Phase39gRuntimeFileApiProbe::ops_bound(candidate),
            Self::Commit => Phase39gRuntimeFileApiProbe::verified_writable(candidate),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hTypedStateOperation {
    SaveProgress,
    SaveTheme,
    SaveMetadata,
    SaveBookmark,
    AppendBookmarkIndex,
    ReplaceBookmarkIndex,
    CompactBookmarkIndex,
}

impl Phase39hTypedStateOperation {
    pub const fn kind(self) -> Phase39dTypedRecordKind {
        match self {
            Self::SaveProgress => Phase39dTypedRecordKind::Progress,
            Self::SaveTheme => Phase39dTypedRecordKind::Theme,
            Self::SaveMetadata => Phase39dTypedRecordKind::Metadata,
            Self::SaveBookmark => Phase39dTypedRecordKind::Bookmark,
            Self::AppendBookmarkIndex | Self::ReplaceBookmarkIndex | Self::CompactBookmarkIndex => {
                Phase39dTypedRecordKind::BookmarkIndex
            }
        }
    }

    pub const fn intent(self) -> Phase39dTypedWriteIntent {
        match self {
            Self::SaveProgress | Self::SaveTheme | Self::SaveMetadata | Self::SaveBookmark => {
                Phase39dTypedWriteIntent::Upsert
            }
            Self::AppendBookmarkIndex => Phase39dTypedWriteIntent::AppendIndex,
            Self::ReplaceBookmarkIndex => Phase39dTypedWriteIntent::Replace,
            Self::CompactBookmarkIndex => Phase39dTypedWriteIntent::CompactIndex,
        }
    }

    pub const fn preflight(self) -> Phase39dTypedWritePreflight {
        match self {
            Self::AppendBookmarkIndex => Phase39dTypedWritePreflight::IndexAppendAccepted,
            Self::ReplaceBookmarkIndex | Self::CompactBookmarkIndex => {
                Phase39dTypedWritePreflight::Accepted
            }
            Self::SaveProgress | Self::SaveTheme | Self::SaveMetadata | Self::SaveBookmark => {
                Phase39dTypedWritePreflight::Accepted
            }
        }
    }

    pub const fn requires_book_id(self) -> bool {
        !matches!(
            self,
            Self::AppendBookmarkIndex | Self::ReplaceBookmarkIndex | Self::CompactBookmarkIndex
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hTypedStateWriteStatus {
    GateDisabled,
    DryRunAccepted,
    RuntimeWriteCommitted,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39hTypedStateNextLane {
    WireReaderCallsitesDirectly,
    KeepBehindRuntimeGate,
    RepairRuntimeWriter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39hTypedStateWriteReport {
    pub operation: Phase39hTypedStateOperation,
    pub kind: Phase39dTypedRecordKind,
    pub status: Phase39hTypedStateWriteStatus,
    pub gate_decision: Phase39gIntegrationGateDecision,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub next_lane: Phase39hTypedStateNextLane,
}

impl Phase39hTypedStateWriteReport {
    pub const fn committed(self) -> bool {
        matches!(self.status, Phase39hTypedStateWriteStatus::RuntimeWriteCommitted)
    }

    pub const fn accepted_or_committed(self) -> bool {
        matches!(
            self.status,
            Phase39hTypedStateWriteStatus::DryRunAccepted
                | Phase39hTypedStateWriteStatus::RuntimeWriteCommitted
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39hAllTypedStatePayloads<'a> {
    pub book_id: Phase39dBookId,
    pub progress_payload: &'a [u8],
    pub theme_payload: &'a [u8],
    pub metadata_payload: &'a [u8],
    pub bookmark_payload: &'a [u8],
    pub bookmark_index_payload: &'a [u8],
}

impl<'a> Phase39hAllTypedStatePayloads<'a> {
    pub const fn new(
        book_id: Phase39dBookId,
        progress_payload: &'a [u8],
        theme_payload: &'a [u8],
        metadata_payload: &'a [u8],
        bookmark_payload: &'a [u8],
        bookmark_index_payload: &'a [u8],
    ) -> Self {
        Self {
            book_id,
            progress_payload,
            theme_payload,
            metadata_payload,
            bookmark_payload,
            bookmark_index_payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39hAllTypedStateWriteSummary {
    pub attempted: usize,
    pub accepted_or_committed: usize,
    pub committed: usize,
    pub rejected_or_deferred: usize,
}

impl Phase39hAllTypedStateWriteSummary {
    pub const fn all_accepted(self) -> bool {
        self.attempted == self.accepted_or_committed
    }
}

pub fn phase39h_write_progress_state<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    book_id: Phase39dBookId,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::SaveProgress,
        Some(book_id),
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_write_theme_state<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    book_id: Phase39dBookId,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::SaveTheme,
        Some(book_id),
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_write_metadata_state<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    book_id: Phase39dBookId,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::SaveMetadata,
        Some(book_id),
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_write_bookmark_state<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    book_id: Phase39dBookId,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::SaveBookmark,
        Some(book_id),
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_append_bookmark_index<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::AppendBookmarkIndex,
        None,
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_replace_bookmark_index<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::ReplaceBookmarkIndex,
        None,
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_compact_bookmark_index<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    phase39h_execute_typed_state_operation(
        Some(ops),
        Phase39hTypedStateOperation::CompactBookmarkIndex,
        None,
        payload,
        enablement,
        config,
    )
}

pub fn phase39h_write_all_typed_state<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    payloads: Phase39hAllTypedStatePayloads<'_>,
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hAllTypedStateWriteSummary {
    let reports = [
        phase39h_write_progress_state(
            ops,
            payloads.book_id,
            payloads.progress_payload,
            enablement,
            config,
        ),
        phase39h_write_theme_state(
            ops,
            payloads.book_id,
            payloads.theme_payload,
            enablement,
            config,
        ),
        phase39h_write_metadata_state(
            ops,
            payloads.book_id,
            payloads.metadata_payload,
            enablement,
            config,
        ),
        phase39h_write_bookmark_state(
            ops,
            payloads.book_id,
            payloads.bookmark_payload,
            enablement,
            config,
        ),
        phase39h_append_bookmark_index(
            ops,
            payloads.bookmark_index_payload,
            enablement,
            config,
        ),
    ];

    let mut accepted_or_committed = 0usize;
    let mut committed = 0usize;

    for report in reports {
        if report.accepted_or_committed() {
            accepted_or_committed = accepted_or_committed.saturating_add(1);
        }
        if report.committed() {
            committed = committed.saturating_add(1);
        }
    }

    Phase39hAllTypedStateWriteSummary {
        attempted: reports.len(),
        accepted_or_committed,
        committed,
        rejected_or_deferred: reports.len().saturating_sub(accepted_or_committed),
    }
}

pub fn phase39h_execute_typed_state_operation<O: Phase39fRuntimeOwnedFileOps>(
    ops: Option<&mut O>,
    operation: Phase39hTypedStateOperation,
    book_id: Option<Phase39dBookId>,
    payload: &[u8],
    enablement: Phase39hRuntimeWriteEnablement,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39hTypedStateWriteReport {
    let request = Phase39dTypedWriteRequest::new(
        operation.kind(),
        operation.intent(),
        book_id,
        payload,
        enablement.write_mode(),
        operation.preflight(),
    );

    let probe = enablement.probe(Phase39gRuntimeFileApiCandidate::X4RuntimeStateLayer);
    let integration_request = Phase39gIntegrationRequest::new(request, config, probe);
    let gate_report = phase39g_execute_runtime_file_api_gate(ops, integration_request);

    phase39h_report_from_gate(operation, gate_report)
}

pub const fn phase39h_report_from_gate(
    operation: Phase39hTypedStateOperation,
    gate_report: Phase39gIntegrationGateReport,
) -> Phase39hTypedStateWriteReport {
    Phase39hTypedStateWriteReport {
        operation,
        kind: gate_report.kind,
        status: phase39h_status_from_gate_decision(gate_report.decision),
        gate_decision: gate_report.decision,
        payload_len: gate_report.payload_len,
        bytes_written: gate_report.bytes_written,
        next_lane: phase39h_next_lane_from_gate_decision(gate_report.decision),
    }
}

pub const fn phase39h_status_from_gate_decision(
    decision: Phase39gIntegrationGateDecision,
) -> Phase39hTypedStateWriteStatus {
    match decision {
        Phase39gIntegrationGateDecision::DisabledByGate => {
            Phase39hTypedStateWriteStatus::GateDisabled
        }
        Phase39gIntegrationGateDecision::DispatchToRuntimeOps => {
            Phase39hTypedStateWriteStatus::DryRunAccepted
        }
        Phase39gIntegrationGateDecision::RuntimeWriteCommitted => {
            Phase39hTypedStateWriteStatus::RuntimeWriteCommitted
        }
        Phase39gIntegrationGateDecision::CandidateOnly
        | Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => {
            Phase39hTypedStateWriteStatus::Deferred
        }
        Phase39gIntegrationGateDecision::RuntimeWriteRejected
        | Phase39gIntegrationGateDecision::InvalidRequest => {
            Phase39hTypedStateWriteStatus::Rejected
        }
    }
}

pub const fn phase39h_next_lane_from_gate_decision(
    decision: Phase39gIntegrationGateDecision,
) -> Phase39hTypedStateNextLane {
    match decision {
        Phase39gIntegrationGateDecision::DispatchToRuntimeOps
        | Phase39gIntegrationGateDecision::RuntimeWriteCommitted => {
            Phase39hTypedStateNextLane::WireReaderCallsitesDirectly
        }
        Phase39gIntegrationGateDecision::DisabledByGate
        | Phase39gIntegrationGateDecision::CandidateOnly
        | Phase39gIntegrationGateDecision::RuntimeBackendUnavailable => {
            Phase39hTypedStateNextLane::KeepBehindRuntimeGate
        }
        Phase39gIntegrationGateDecision::RuntimeWriteRejected
        | Phase39gIntegrationGateDecision::InvalidRequest => {
            Phase39hTypedStateNextLane::RepairRuntimeWriter
        }
    }
}

pub fn phase39h_marker() -> &'static str {
    PHASE_39H_TYPED_STATE_RUNTIME_CALLSITE_WIRING_MARKER
}
