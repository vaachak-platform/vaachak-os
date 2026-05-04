//! Phase 39A — Guarded Progress State Write Backend Binding.
//!
//! This is the first Phase 39 write-lane module.
//!
//! Unlike the Phase 38 design/handoff modules, this module defines a real
//! backend-binding seam that can dispatch a progress-state write to a supplied
//! backend implementation. It still does not hard-code or move the platform
//! storage implementation; the caller must provide the backend.
//!
//! Scope is intentionally narrow:
//! - only `.PRG` progress-state writes
//! - no `.THM`, `.MTA`, `.BKM`, or `BMIDX.TXT` writes yet
//! - requires explicit commit mode
//! - requires read-before-write/preflight evidence
//! - backend must be supplied by the caller
//! - no display/input/power behavior is touched

#![allow(dead_code)]

pub const PHASE_39A_GUARDED_PROGRESS_WRITE_BACKEND_BINDING_MARKER: &str =
    "phase39a=x4-guarded-progress-state-write-backend-binding-ok";

pub const PHASE_39A_WRITE_LANE_STARTED: bool = true;
pub const PHASE_39A_PROGRESS_WRITE_ONLY: bool = true;
pub const PHASE_39A_MAX_BOOK_ID_LEN: usize = 8;
pub const PHASE_39A_MAX_PROGRESS_PAYLOAD_LEN: usize = 256;
pub const PHASE_39A_MAX_PROGRESS_PATH_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39aProgressWriteMode {
    DryRunOnly,
    CommitToBoundBackend,
}

impl Phase39aProgressWriteMode {
    pub const fn commit_requested(self) -> bool {
        matches!(self, Self::CommitToBoundBackend)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39aProgressWritePreflight {
    NotProvided,
    ReadBeforeWriteAccepted,
    ExistingRecordMatched,
    MissingRecordAccepted,
}

impl Phase39aProgressWritePreflight {
    pub const fn accepted(self) -> bool {
        matches!(
            self,
            Self::ReadBeforeWriteAccepted | Self::ExistingRecordMatched | Self::MissingRecordAccepted
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39aProgressWriteDecision {
    DryRunAccepted,
    DispatchToBackend,
    RejectedMissingPreflight,
    RejectedInvalidBookId,
    RejectedEmptyPayload,
    RejectedPayloadTooLarge,
    RejectedBackendNotBound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39aBackendWriteStatus {
    NotCalled,
    Committed,
    BackendRejected,
    BackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39aProgressWriteOutcome {
    DryRunAccepted,
    WriteCommitted,
    WriteRejected,
    BackendUnavailable,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39aProgressBookId {
    bytes: [u8; PHASE_39A_MAX_BOOK_ID_LEN],
}

impl Phase39aProgressBookId {
    pub const fn new(bytes: [u8; PHASE_39A_MAX_BOOK_ID_LEN]) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(self) -> [u8; PHASE_39A_MAX_BOOK_ID_LEN] {
        self.bytes
    }

    pub fn is_hex8(self) -> bool {
        self.bytes.iter().all(|b| b.is_ascii_hexdigit())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39aProgressStatePath {
    bytes: [u8; PHASE_39A_MAX_PROGRESS_PATH_LEN],
    len: usize,
}

impl Phase39aProgressStatePath {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; PHASE_39A_MAX_PROGRESS_PATH_LEN],
            len: 0,
        }
    }

    pub fn render(book_id: Phase39aProgressBookId) -> Option<Self> {
        if !book_id.is_hex8() {
            return None;
        }

        let mut bytes = [0u8; PHASE_39A_MAX_PROGRESS_PATH_LEN];
        let mut pos = 0usize;

        pos = copy_into(b"STATE/", &mut bytes, pos)?;
        for b in book_id.as_bytes() {
            if pos >= bytes.len() {
                return None;
            }
            bytes[pos] = b.to_ascii_uppercase();
            pos += 1;
        }
        pos = copy_into(b".PRG", &mut bytes, pos)?;

        Some(Self { bytes, len: pos })
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39aProgressWriteRequest<'a> {
    pub book_id: Phase39aProgressBookId,
    pub payload: &'a [u8],
    pub mode: Phase39aProgressWriteMode,
    pub preflight: Phase39aProgressWritePreflight,
}

impl<'a> Phase39aProgressWriteRequest<'a> {
    pub const fn new(
        book_id: Phase39aProgressBookId,
        payload: &'a [u8],
        mode: Phase39aProgressWriteMode,
        preflight: Phase39aProgressWritePreflight,
    ) -> Self {
        Self {
            book_id,
            payload,
            mode,
            preflight,
        }
    }

    pub fn path(self) -> Option<Phase39aProgressStatePath> {
        Phase39aProgressStatePath::render(self.book_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39aBackendWriteResult {
    pub status: Phase39aBackendWriteStatus,
    pub bytes_written: usize,
}

impl Phase39aBackendWriteResult {
    pub const fn not_called() -> Self {
        Self {
            status: Phase39aBackendWriteStatus::NotCalled,
            bytes_written: 0,
        }
    }

    pub const fn committed(bytes_written: usize) -> Self {
        Self {
            status: Phase39aBackendWriteStatus::Committed,
            bytes_written,
        }
    }

    pub const fn rejected() -> Self {
        Self {
            status: Phase39aBackendWriteStatus::BackendRejected,
            bytes_written: 0,
        }
    }

    pub const fn unavailable() -> Self {
        Self {
            status: Phase39aBackendWriteStatus::BackendUnavailable,
            bytes_written: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39aProgressWriteReport {
    pub decision: Phase39aProgressWriteDecision,
    pub outcome: Phase39aProgressWriteOutcome,
    pub backend_status: Phase39aBackendWriteStatus,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub path: Phase39aProgressStatePath,
}

impl Phase39aProgressWriteReport {
    pub const fn write_committed(self) -> bool {
        matches!(self.outcome, Phase39aProgressWriteOutcome::WriteCommitted)
    }

    pub const fn dry_run(self) -> bool {
        matches!(self.outcome, Phase39aProgressWriteOutcome::DryRunAccepted)
    }
}

pub trait Phase39aProgressWriteBackend {
    fn write_progress_record(
        &mut self,
        path: Phase39aProgressStatePath,
        request: &Phase39aProgressWriteRequest<'_>,
    ) -> Phase39aBackendWriteResult;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase39aUnboundProgressWriteBackend;

impl Phase39aProgressWriteBackend for Phase39aUnboundProgressWriteBackend {
    fn write_progress_record(
        &mut self,
        _path: Phase39aProgressStatePath,
        _request: &Phase39aProgressWriteRequest<'_>,
    ) -> Phase39aBackendWriteResult {
        Phase39aBackendWriteResult::unavailable()
    }
}

pub fn phase39a_execute_progress_write<B: Phase39aProgressWriteBackend>(
    backend: Option<&mut B>,
    request: Phase39aProgressWriteRequest<'_>,
) -> Phase39aProgressWriteReport {
    let decision = phase39a_validate_progress_write_request(request);
    let path = request.path().unwrap_or_else(Phase39aProgressStatePath::empty);

    match decision {
        Phase39aProgressWriteDecision::DryRunAccepted => Phase39aProgressWriteReport {
            decision,
            outcome: Phase39aProgressWriteOutcome::DryRunAccepted,
            backend_status: Phase39aBackendWriteStatus::NotCalled,
            payload_len: request.payload.len(),
            bytes_written: 0,
            path,
        },
        Phase39aProgressWriteDecision::DispatchToBackend => {
            if let Some(bound_backend) = backend {
                let result = bound_backend.write_progress_record(path, &request);
                let outcome = phase39a_outcome_from_backend_status(result.status);
                Phase39aProgressWriteReport {
                    decision,
                    outcome,
                    backend_status: result.status,
                    payload_len: request.payload.len(),
                    bytes_written: result.bytes_written,
                    path,
                }
            } else {
                Phase39aProgressWriteReport {
                    decision: Phase39aProgressWriteDecision::RejectedBackendNotBound,
                    outcome: Phase39aProgressWriteOutcome::BackendUnavailable,
                    backend_status: Phase39aBackendWriteStatus::BackendUnavailable,
                    payload_len: request.payload.len(),
                    bytes_written: 0,
                    path,
                }
            }
        }
        Phase39aProgressWriteDecision::RejectedMissingPreflight
        | Phase39aProgressWriteDecision::RejectedInvalidBookId
        | Phase39aProgressWriteDecision::RejectedEmptyPayload
        | Phase39aProgressWriteDecision::RejectedPayloadTooLarge
        | Phase39aProgressWriteDecision::RejectedBackendNotBound => Phase39aProgressWriteReport {
            decision,
            outcome: Phase39aProgressWriteOutcome::InvalidRequest,
            backend_status: Phase39aBackendWriteStatus::NotCalled,
            payload_len: request.payload.len(),
            bytes_written: 0,
            path,
        },
    }
}

pub fn phase39a_validate_progress_write_request(
    request: Phase39aProgressWriteRequest<'_>,
) -> Phase39aProgressWriteDecision {
    if !request.book_id.is_hex8() {
        return Phase39aProgressWriteDecision::RejectedInvalidBookId;
    }

    if request.payload.is_empty() {
        return Phase39aProgressWriteDecision::RejectedEmptyPayload;
    }

    if request.payload.len() > PHASE_39A_MAX_PROGRESS_PAYLOAD_LEN {
        return Phase39aProgressWriteDecision::RejectedPayloadTooLarge;
    }

    if !request.preflight.accepted() {
        return Phase39aProgressWriteDecision::RejectedMissingPreflight;
    }

    if request.mode.commit_requested() {
        Phase39aProgressWriteDecision::DispatchToBackend
    } else {
        Phase39aProgressWriteDecision::DryRunAccepted
    }
}

pub const fn phase39a_outcome_from_backend_status(
    status: Phase39aBackendWriteStatus,
) -> Phase39aProgressWriteOutcome {
    match status {
        Phase39aBackendWriteStatus::NotCalled => Phase39aProgressWriteOutcome::DryRunAccepted,
        Phase39aBackendWriteStatus::Committed => Phase39aProgressWriteOutcome::WriteCommitted,
        Phase39aBackendWriteStatus::BackendRejected => Phase39aProgressWriteOutcome::WriteRejected,
        Phase39aBackendWriteStatus::BackendUnavailable => {
            Phase39aProgressWriteOutcome::BackendUnavailable
        }
    }
}

fn copy_into(src: &[u8], dst: &mut [u8], start: usize) -> Option<usize> {
    let end = start.checked_add(src.len())?;
    if end > dst.len() {
        return None;
    }
    dst[start..end].copy_from_slice(src);
    Some(end)
}

pub fn phase39a_marker() -> &'static str {
    PHASE_39A_GUARDED_PROGRESS_WRITE_BACKEND_BINDING_MARKER
}
