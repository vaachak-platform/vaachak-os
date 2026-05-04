//! Phase 38L — State I/O Guarded Write Backend Implementation Seam.
//!
//! This module defines the first implementation seam for the future typed-state
//! mutation backend. It is intentionally policy-gated: the default gate denies
//! live mutation and only returns a planned/seam-level decision.
//!
//! No live storage, bus, display, input, or power calls are performed here.

#![allow(dead_code)]

pub const PHASE_38L_GUARDED_WRITE_BACKEND_IMPLEMENTATION_SEAM_MARKER: &str =
    "phase38l=x4-state-io-guarded-write-backend-implementation-seam-ok";

pub const PHASE_38L_DEFAULT_LIVE_MUTATION_ENABLED: bool = false;
pub const PHASE_38L_MAX_BOOK_ID_LEN: usize = 8;
pub const PHASE_38L_MAX_STATE_PATH_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38lStateRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl Phase38lStateRecordKind {
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

    pub const fn canonical_leaf_name(self) -> &'static str {
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
pub enum Phase38lMutationIntent {
    UpsertRecord,
    ReplaceRecord,
    AppendIndex,
    CompactIndex,
    RemoveRecord,
}

impl Phase38lMutationIntent {
    pub const fn is_index_intent(self) -> bool {
        matches!(self, Self::AppendIndex | Self::CompactIndex)
    }

    pub const fn is_record_intent(self) -> bool {
        matches!(self, Self::UpsertRecord | Self::ReplaceRecord | Self::RemoveRecord)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38lMutationPolicy {
    DenyAll,
    DryRunOnly,
    AllowLiveMutation,
}

impl Phase38lMutationPolicy {
    pub const fn live_mutation_enabled(self) -> bool {
        matches!(self, Self::AllowLiveMutation)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38lGuardDecision {
    DeniedByDefault,
    AcceptedAsDryRun,
    AcceptedForBackendDispatch,
    RejectedMissingBookId,
    RejectedUnexpectedBookId,
    RejectedInvalidIntentForKind,
    RejectedEmptyPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38lBackendDispatch {
    NotDispatched,
    DryRunRecorded,
    ReadyForFutureBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38lBookId {
    bytes: [u8; PHASE_38L_MAX_BOOK_ID_LEN],
}

impl Phase38lBookId {
    pub const fn new(bytes: [u8; PHASE_38L_MAX_BOOK_ID_LEN]) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(self) -> [u8; PHASE_38L_MAX_BOOK_ID_LEN] {
        self.bytes
    }

    pub fn is_hex8(self) -> bool {
        self.bytes.iter().all(|b| b.is_ascii_hexdigit())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38lMutationRequest {
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub book_id: Option<Phase38lBookId>,
    pub payload_len: usize,
    pub policy: Phase38lMutationPolicy,
}

impl Phase38lMutationRequest {
    pub const fn new(
        kind: Phase38lStateRecordKind,
        intent: Phase38lMutationIntent,
        book_id: Option<Phase38lBookId>,
        payload_len: usize,
        policy: Phase38lMutationPolicy,
    ) -> Self {
        Self {
            kind,
            intent,
            book_id,
            payload_len,
            policy,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38lMutationPlan {
    pub decision: Phase38lGuardDecision,
    pub dispatch: Phase38lBackendDispatch,
    pub kind: Phase38lStateRecordKind,
    pub intent: Phase38lMutationIntent,
    pub payload_len: usize,
}

impl Phase38lMutationPlan {
    pub const fn mutation_permitted(self) -> bool {
        matches!(
            self.decision,
            Phase38lGuardDecision::AcceptedForBackendDispatch
        )
    }

    pub const fn dry_run_only(self) -> bool {
        matches!(self.decision, Phase38lGuardDecision::AcceptedAsDryRun)
    }
}

pub trait Phase38lGuardedBackendSeam {
    fn plan_mutation(&self, request: Phase38lMutationRequest) -> Phase38lMutationPlan;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase38lDefaultGuardedBackendSeam;

impl Phase38lGuardedBackendSeam for Phase38lDefaultGuardedBackendSeam {
    fn plan_mutation(&self, request: Phase38lMutationRequest) -> Phase38lMutationPlan {
        phase38l_plan_guarded_mutation(request)
    }
}

pub fn phase38l_plan_guarded_mutation(
    request: Phase38lMutationRequest,
) -> Phase38lMutationPlan {
    let decision = phase38l_evaluate_request(request);
    let dispatch = match decision {
        Phase38lGuardDecision::AcceptedAsDryRun => Phase38lBackendDispatch::DryRunRecorded,
        Phase38lGuardDecision::AcceptedForBackendDispatch => {
            Phase38lBackendDispatch::ReadyForFutureBackend
        }
        _ => Phase38lBackendDispatch::NotDispatched,
    };

    Phase38lMutationPlan {
        decision,
        dispatch,
        kind: request.kind,
        intent: request.intent,
        payload_len: request.payload_len,
    }
}

pub fn phase38l_evaluate_request(request: Phase38lMutationRequest) -> Phase38lGuardDecision {
    if request.kind.requires_book_id() && request.book_id.is_none() {
        return Phase38lGuardDecision::RejectedMissingBookId;
    }

    if !request.kind.requires_book_id() && request.book_id.is_some() {
        return Phase38lGuardDecision::RejectedUnexpectedBookId;
    }

    if request.payload_len == 0 && !matches!(request.intent, Phase38lMutationIntent::RemoveRecord)
    {
        return Phase38lGuardDecision::RejectedEmptyPayload;
    }

    if !phase38l_intent_matches_kind(request.kind, request.intent) {
        return Phase38lGuardDecision::RejectedInvalidIntentForKind;
    }

    match request.policy {
        Phase38lMutationPolicy::DenyAll => Phase38lGuardDecision::DeniedByDefault,
        Phase38lMutationPolicy::DryRunOnly => Phase38lGuardDecision::AcceptedAsDryRun,
        Phase38lMutationPolicy::AllowLiveMutation => {
            Phase38lGuardDecision::AcceptedForBackendDispatch
        }
    }
}

pub const fn phase38l_intent_matches_kind(
    kind: Phase38lStateRecordKind,
    intent: Phase38lMutationIntent,
) -> bool {
    match kind {
        Phase38lStateRecordKind::BookmarkIndex => {
            matches!(
                intent,
                Phase38lMutationIntent::AppendIndex | Phase38lMutationIntent::CompactIndex
            )
        }
        Phase38lStateRecordKind::Progress
        | Phase38lStateRecordKind::Theme
        | Phase38lStateRecordKind::Metadata
        | Phase38lStateRecordKind::Bookmark => matches!(
            intent,
            Phase38lMutationIntent::UpsertRecord
                | Phase38lMutationIntent::ReplaceRecord
                | Phase38lMutationIntent::RemoveRecord
        ),
    }
}

pub fn phase38l_render_state_path(
    kind: Phase38lStateRecordKind,
    book_id: Option<Phase38lBookId>,
    out: &mut [u8; PHASE_38L_MAX_STATE_PATH_LEN],
) -> Option<usize> {
    out.fill(0);

    if kind == Phase38lStateRecordKind::BookmarkIndex {
        return copy_bytes(b"STATE/BMIDX.TXT", out);
    }

    let id = book_id?;
    if !id.is_hex8() {
        return None;
    }

    let mut pos = copy_prefix(b"STATE/", out)?;
    let id_bytes = id.as_bytes();
    for byte in id_bytes {
        if pos >= out.len() {
            return None;
        }
        out[pos] = byte.to_ascii_uppercase();
        pos += 1;
    }

    if pos >= out.len() {
        return None;
    }
    out[pos] = b'.';
    pos += 1;

    copy_suffix(kind.extension().as_bytes(), out, pos)
}

fn copy_bytes(src: &[u8], out: &mut [u8]) -> Option<usize> {
    if src.len() > out.len() {
        return None;
    }
    out[..src.len()].copy_from_slice(src);
    Some(src.len())
}

fn copy_prefix(src: &[u8], out: &mut [u8]) -> Option<usize> {
    copy_bytes(src, out)
}

fn copy_suffix(src: &[u8], out: &mut [u8], start: usize) -> Option<usize> {
    let end = start.checked_add(src.len())?;
    if end > out.len() {
        return None;
    }
    out[start..end].copy_from_slice(src);
    Some(end)
}

pub fn phase38l_marker() -> &'static str {
    PHASE_38L_GUARDED_WRITE_BACKEND_IMPLEMENTATION_SEAM_MARKER
}
