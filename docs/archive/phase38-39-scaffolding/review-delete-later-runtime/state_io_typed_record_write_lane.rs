//! Phase 39D — Typed Record Write Lane Bundle.
//!
//! This expands the Phase 39 write lane beyond `.PRG`.
//!
//! Scope:
//! - `.PRG` progress records
//! - `.THM` theme records
//! - `.MTA` metadata records
//! - `.BKM` bookmark records
//! - `BMIDX.TXT` bookmark index
//!
//! The module provides one typed-record write facade with:
//! - dry-run
//! - callback backend
//! - recording backend
//! - path rendering
//! - preflight requirement
//! - explicit commit mode
//!
//! It does not hard-code the concrete SD/FAT writer. The real writer is still
//! supplied by the caller through the callback/backend trait.

#![allow(dead_code)]

pub const PHASE_39D_TYPED_RECORD_WRITE_LANE_BUNDLE_MARKER: &str =
    "phase39d=x4-typed-record-write-lane-bundle-ok";

pub const PHASE_39D_MAX_BOOK_ID_LEN: usize = 8;
pub const PHASE_39D_MAX_RECORD_PATH_LEN: usize = 32;
pub const PHASE_39D_MAX_RECORD_PAYLOAD_LEN: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl Phase39dTypedRecordKind {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Progress => "PRG",
            Self::Theme => "THM",
            Self::Metadata => "MTA",
            Self::Bookmark => "BKM",
            Self::BookmarkIndex => "TXT",
        }
    }

    pub const fn requires_book_id(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }

    pub const fn leaf_template(self) -> &'static str {
        match self {
            Self::Progress => "<BOOKID>.PRG",
            Self::Theme => "<BOOKID>.THM",
            Self::Metadata => "<BOOKID>.MTA",
            Self::Bookmark => "<BOOKID>.BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedWriteIntent {
    Upsert,
    Replace,
    Remove,
    AppendIndex,
    CompactIndex,
}

impl Phase39dTypedWriteIntent {
    pub const fn valid_for(self, kind: Phase39dTypedRecordKind) -> bool {
        match kind {
            Phase39dTypedRecordKind::BookmarkIndex => {
                matches!(self, Self::AppendIndex | Self::CompactIndex | Self::Replace)
            }
            Phase39dTypedRecordKind::Progress
            | Phase39dTypedRecordKind::Theme
            | Phase39dTypedRecordKind::Metadata
            | Phase39dTypedRecordKind::Bookmark => {
                matches!(self, Self::Upsert | Self::Replace | Self::Remove)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedWriteMode {
    DryRun,
    CommitToCallback,
    CommitToRecordingBackend,
}

impl Phase39dTypedWriteMode {
    pub const fn commit_requested(self) -> bool {
        matches!(
            self,
            Self::CommitToCallback | Self::CommitToRecordingBackend
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedWritePreflight {
    NotProvided,
    Accepted,
    ExistingRecordMatched,
    MissingRecordAccepted,
    IndexAppendAccepted,
}

impl Phase39dTypedWritePreflight {
    pub const fn accepted(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::ExistingRecordMatched
                | Self::MissingRecordAccepted
                | Self::IndexAppendAccepted
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedWriteDecision {
    DryRunAccepted,
    DispatchToBackend,
    RejectedMissingPreflight,
    RejectedInvalidBookId,
    RejectedUnexpectedBookId,
    RejectedMissingBookId,
    RejectedInvalidIntentForKind,
    RejectedEmptyPayload,
    RejectedPayloadTooLarge,
    RejectedBackendNotBound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dBackendWriteStatus {
    NotCalled,
    Committed,
    BackendRejected,
    BackendUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedWriteOutcome {
    DryRunAccepted,
    WriteCommitted,
    WriteRejected,
    BackendUnavailable,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39dTypedBackendKind {
    None,
    Callback,
    Recording,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dBookId {
    bytes: [u8; PHASE_39D_MAX_BOOK_ID_LEN],
}

impl Phase39dBookId {
    pub const fn new(bytes: [u8; PHASE_39D_MAX_BOOK_ID_LEN]) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(self) -> [u8; PHASE_39D_MAX_BOOK_ID_LEN] {
        self.bytes
    }

    pub fn is_hex8(self) -> bool {
        self.bytes.iter().all(|b| b.is_ascii_hexdigit())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dTypedRecordPath {
    bytes: [u8; PHASE_39D_MAX_RECORD_PATH_LEN],
    len: usize,
}

impl Phase39dTypedRecordPath {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; PHASE_39D_MAX_RECORD_PATH_LEN],
            len: 0,
        }
    }

    pub fn render(kind: Phase39dTypedRecordKind, book_id: Option<Phase39dBookId>) -> Option<Self> {
        let mut bytes = [0u8; PHASE_39D_MAX_RECORD_PATH_LEN];

        if kind == Phase39dTypedRecordKind::BookmarkIndex {
            let len = copy_into(b"STATE/BMIDX.TXT", &mut bytes, 0)?;
            return Some(Self { bytes, len });
        }

        let id = book_id?;
        if !id.is_hex8() {
            return None;
        }

        let mut pos = copy_into(b"STATE/", &mut bytes, 0)?;
        for b in id.as_bytes() {
            if pos >= bytes.len() {
                return None;
            }
            bytes[pos] = b.to_ascii_uppercase();
            pos += 1;
        }

        if pos >= bytes.len() {
            return None;
        }
        bytes[pos] = b'.';
        pos += 1;

        let len = copy_into(kind.extension().as_bytes(), &mut bytes, pos)?;
        Some(Self { bytes, len })
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dTypedWriteRequest<'a> {
    pub kind: Phase39dTypedRecordKind,
    pub intent: Phase39dTypedWriteIntent,
    pub book_id: Option<Phase39dBookId>,
    pub payload: &'a [u8],
    pub mode: Phase39dTypedWriteMode,
    pub preflight: Phase39dTypedWritePreflight,
}

impl<'a> Phase39dTypedWriteRequest<'a> {
    pub const fn new(
        kind: Phase39dTypedRecordKind,
        intent: Phase39dTypedWriteIntent,
        book_id: Option<Phase39dBookId>,
        payload: &'a [u8],
        mode: Phase39dTypedWriteMode,
        preflight: Phase39dTypedWritePreflight,
    ) -> Self {
        Self {
            kind,
            intent,
            book_id,
            payload,
            mode,
            preflight,
        }
    }

    pub fn path(self) -> Option<Phase39dTypedRecordPath> {
        Phase39dTypedRecordPath::render(self.kind, self.book_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dBackendWriteResult {
    pub status: Phase39dBackendWriteStatus,
    pub bytes_written: usize,
}

impl Phase39dBackendWriteResult {
    pub const fn not_called() -> Self {
        Self {
            status: Phase39dBackendWriteStatus::NotCalled,
            bytes_written: 0,
        }
    }

    pub const fn committed(bytes_written: usize) -> Self {
        Self {
            status: Phase39dBackendWriteStatus::Committed,
            bytes_written,
        }
    }

    pub const fn rejected() -> Self {
        Self {
            status: Phase39dBackendWriteStatus::BackendRejected,
            bytes_written: 0,
        }
    }

    pub const fn unavailable() -> Self {
        Self {
            status: Phase39dBackendWriteStatus::BackendUnavailable,
            bytes_written: 0,
        }
    }
}

pub trait Phase39dTypedWriteBackend {
    fn write_typed_record(
        &mut self,
        path: Phase39dTypedRecordPath,
        request: &Phase39dTypedWriteRequest<'_>,
    ) -> Phase39dBackendWriteResult;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dTypedWriteCall<'a> {
    pub path: Phase39dTypedRecordPath,
    pub request: &'a Phase39dTypedWriteRequest<'a>,
}

pub type Phase39dTypedWriteCallback =
    for<'a> fn(Phase39dTypedWriteCall<'a>) -> Phase39dBackendWriteResult;

#[derive(Clone, Copy)]
pub struct Phase39dCallbackTypedWriteBackend {
    callback: Option<Phase39dTypedWriteCallback>,
}

impl Phase39dCallbackTypedWriteBackend {
    pub const fn unbound() -> Self {
        Self { callback: None }
    }

    pub const fn new(callback: Phase39dTypedWriteCallback) -> Self {
        Self {
            callback: Some(callback),
        }
    }

    pub const fn callback_bound(self) -> bool {
        self.callback.is_some()
    }
}

impl Phase39dTypedWriteBackend for Phase39dCallbackTypedWriteBackend {
    fn write_typed_record(
        &mut self,
        path: Phase39dTypedRecordPath,
        request: &Phase39dTypedWriteRequest<'_>,
    ) -> Phase39dBackendWriteResult {
        if let Some(callback) = self.callback {
            callback(Phase39dTypedWriteCall { path, request })
        } else {
            Phase39dBackendWriteResult::unavailable()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dRecordingTypedWriteBackend {
    pub calls: usize,
    pub last_kind: Phase39dTypedRecordKind,
    pub last_path: Phase39dTypedRecordPath,
    pub last_payload_len: usize,
    pub last_status: Phase39dBackendWriteStatus,
    pub accept_writes: bool,
}

impl Phase39dRecordingTypedWriteBackend {
    pub const fn new_accepting() -> Self {
        Self {
            calls: 0,
            last_kind: Phase39dTypedRecordKind::Progress,
            last_path: Phase39dTypedRecordPath::empty(),
            last_payload_len: 0,
            last_status: Phase39dBackendWriteStatus::NotCalled,
            accept_writes: true,
        }
    }

    pub const fn new_rejecting() -> Self {
        Self {
            calls: 0,
            last_kind: Phase39dTypedRecordKind::Progress,
            last_path: Phase39dTypedRecordPath::empty(),
            last_payload_len: 0,
            last_status: Phase39dBackendWriteStatus::NotCalled,
            accept_writes: false,
        }
    }

    pub const fn wrote_once(self) -> bool {
        self.calls == 1 && matches!(self.last_status, Phase39dBackendWriteStatus::Committed)
    }
}

impl Phase39dTypedWriteBackend for Phase39dRecordingTypedWriteBackend {
    fn write_typed_record(
        &mut self,
        path: Phase39dTypedRecordPath,
        request: &Phase39dTypedWriteRequest<'_>,
    ) -> Phase39dBackendWriteResult {
        self.calls = self.calls.saturating_add(1);
        self.last_kind = request.kind;
        self.last_path = path;
        self.last_payload_len = request.payload.len();

        if self.accept_writes {
            self.last_status = Phase39dBackendWriteStatus::Committed;
            Phase39dBackendWriteResult::committed(request.payload.len())
        } else {
            self.last_status = Phase39dBackendWriteStatus::BackendRejected;
            Phase39dBackendWriteResult::rejected()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39dTypedWriteReport {
    pub kind: Phase39dTypedRecordKind,
    pub intent: Phase39dTypedWriteIntent,
    pub decision: Phase39dTypedWriteDecision,
    pub outcome: Phase39dTypedWriteOutcome,
    pub backend_kind: Phase39dTypedBackendKind,
    pub backend_status: Phase39dBackendWriteStatus,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub path: Phase39dTypedRecordPath,
}

impl Phase39dTypedWriteReport {
    pub const fn committed(self) -> bool {
        matches!(self.outcome, Phase39dTypedWriteOutcome::WriteCommitted)
    }

    pub const fn dry_run(self) -> bool {
        matches!(self.outcome, Phase39dTypedWriteOutcome::DryRunAccepted)
    }
}

pub fn phase39d_execute_typed_record_write<B: Phase39dTypedWriteBackend>(
    backend: Option<&mut B>,
    request: Phase39dTypedWriteRequest<'_>,
    backend_kind: Phase39dTypedBackendKind,
) -> Phase39dTypedWriteReport {
    let decision = phase39d_validate_typed_write_request(request);
    let path = request
        .path()
        .unwrap_or_else(Phase39dTypedRecordPath::empty);

    match decision {
        Phase39dTypedWriteDecision::DryRunAccepted => Phase39dTypedWriteReport {
            kind: request.kind,
            intent: request.intent,
            decision,
            outcome: Phase39dTypedWriteOutcome::DryRunAccepted,
            backend_kind: Phase39dTypedBackendKind::None,
            backend_status: Phase39dBackendWriteStatus::NotCalled,
            payload_len: request.payload.len(),
            bytes_written: 0,
            path,
        },
        Phase39dTypedWriteDecision::DispatchToBackend => {
            if let Some(bound_backend) = backend {
                let result = bound_backend.write_typed_record(path, &request);
                Phase39dTypedWriteReport {
                    kind: request.kind,
                    intent: request.intent,
                    decision,
                    outcome: phase39d_outcome_from_backend_status(result.status),
                    backend_kind,
                    backend_status: result.status,
                    payload_len: request.payload.len(),
                    bytes_written: result.bytes_written,
                    path,
                }
            } else {
                Phase39dTypedWriteReport {
                    kind: request.kind,
                    intent: request.intent,
                    decision: Phase39dTypedWriteDecision::RejectedBackendNotBound,
                    outcome: Phase39dTypedWriteOutcome::BackendUnavailable,
                    backend_kind,
                    backend_status: Phase39dBackendWriteStatus::BackendUnavailable,
                    payload_len: request.payload.len(),
                    bytes_written: 0,
                    path,
                }
            }
        }
        Phase39dTypedWriteDecision::RejectedMissingPreflight
        | Phase39dTypedWriteDecision::RejectedInvalidBookId
        | Phase39dTypedWriteDecision::RejectedUnexpectedBookId
        | Phase39dTypedWriteDecision::RejectedMissingBookId
        | Phase39dTypedWriteDecision::RejectedInvalidIntentForKind
        | Phase39dTypedWriteDecision::RejectedEmptyPayload
        | Phase39dTypedWriteDecision::RejectedPayloadTooLarge
        | Phase39dTypedWriteDecision::RejectedBackendNotBound => Phase39dTypedWriteReport {
            kind: request.kind,
            intent: request.intent,
            decision,
            outcome: Phase39dTypedWriteOutcome::InvalidRequest,
            backend_kind,
            backend_status: Phase39dBackendWriteStatus::NotCalled,
            payload_len: request.payload.len(),
            bytes_written: 0,
            path,
        },
    }
}

pub fn phase39d_execute_typed_record_write_with_callback(
    callback: Option<Phase39dTypedWriteCallback>,
    request: Phase39dTypedWriteRequest<'_>,
) -> Phase39dTypedWriteReport {
    let mut backend = Phase39dCallbackTypedWriteBackend { callback };
    if backend.callback_bound() {
        phase39d_execute_typed_record_write(
            Some(&mut backend),
            request,
            Phase39dTypedBackendKind::Callback,
        )
    } else {
        phase39d_execute_typed_record_write::<Phase39dCallbackTypedWriteBackend>(
            None,
            request,
            Phase39dTypedBackendKind::Callback,
        )
    }
}

pub fn phase39d_execute_typed_record_write_with_recording_backend(
    request: Phase39dTypedWriteRequest<'_>,
    backend: &mut Phase39dRecordingTypedWriteBackend,
) -> Phase39dTypedWriteReport {
    phase39d_execute_typed_record_write(Some(backend), request, Phase39dTypedBackendKind::Recording)
}

pub fn phase39d_execute_typed_record_write_dry_run(
    request: Phase39dTypedWriteRequest<'_>,
) -> Phase39dTypedWriteReport {
    phase39d_execute_typed_record_write::<Phase39dRecordingTypedWriteBackend>(
        None,
        Phase39dTypedWriteRequest::new(
            request.kind,
            request.intent,
            request.book_id,
            request.payload,
            Phase39dTypedWriteMode::DryRun,
            request.preflight,
        ),
        Phase39dTypedBackendKind::None,
    )
}

pub fn phase39d_validate_typed_write_request(
    request: Phase39dTypedWriteRequest<'_>,
) -> Phase39dTypedWriteDecision {
    if request.kind.requires_book_id() && request.book_id.is_none() {
        return Phase39dTypedWriteDecision::RejectedMissingBookId;
    }

    if !request.kind.requires_book_id() && request.book_id.is_some() {
        return Phase39dTypedWriteDecision::RejectedUnexpectedBookId;
    }

    if let Some(book_id) = request.book_id
        && !book_id.is_hex8()
    {
        return Phase39dTypedWriteDecision::RejectedInvalidBookId;
    }

    if !request.intent.valid_for(request.kind) {
        return Phase39dTypedWriteDecision::RejectedInvalidIntentForKind;
    }

    if request.payload.is_empty() && !matches!(request.intent, Phase39dTypedWriteIntent::Remove) {
        return Phase39dTypedWriteDecision::RejectedEmptyPayload;
    }

    if request.payload.len() > PHASE_39D_MAX_RECORD_PAYLOAD_LEN {
        return Phase39dTypedWriteDecision::RejectedPayloadTooLarge;
    }

    if !request.preflight.accepted() {
        return Phase39dTypedWriteDecision::RejectedMissingPreflight;
    }

    if request.mode.commit_requested() {
        Phase39dTypedWriteDecision::DispatchToBackend
    } else {
        Phase39dTypedWriteDecision::DryRunAccepted
    }
}

pub const fn phase39d_outcome_from_backend_status(
    status: Phase39dBackendWriteStatus,
) -> Phase39dTypedWriteOutcome {
    match status {
        Phase39dBackendWriteStatus::NotCalled => Phase39dTypedWriteOutcome::DryRunAccepted,
        Phase39dBackendWriteStatus::Committed => Phase39dTypedWriteOutcome::WriteCommitted,
        Phase39dBackendWriteStatus::BackendRejected => Phase39dTypedWriteOutcome::WriteRejected,
        Phase39dBackendWriteStatus::BackendUnavailable => {
            Phase39dTypedWriteOutcome::BackendUnavailable
        }
    }
}

pub const PHASE_39D_RECORD_KINDS: &[Phase39dTypedRecordKind] = &[
    Phase39dTypedRecordKind::Progress,
    Phase39dTypedRecordKind::Theme,
    Phase39dTypedRecordKind::Metadata,
    Phase39dTypedRecordKind::Bookmark,
    Phase39dTypedRecordKind::BookmarkIndex,
];

pub fn phase39d_has_record_kind(kind: Phase39dTypedRecordKind) -> bool {
    PHASE_39D_RECORD_KINDS.contains(&kind)
}

fn copy_into(src: &[u8], dst: &mut [u8], start: usize) -> Option<usize> {
    let end = start.checked_add(src.len())?;
    if end > dst.len() {
        return None;
    }
    dst[start..end].copy_from_slice(src);
    Some(end)
}

pub fn phase39d_marker() -> &'static str {
    PHASE_39D_TYPED_RECORD_WRITE_LANE_BUNDLE_MARKER
}
