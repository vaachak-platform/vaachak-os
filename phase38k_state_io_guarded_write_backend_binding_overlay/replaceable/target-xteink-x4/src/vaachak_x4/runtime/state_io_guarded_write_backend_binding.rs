//! Phase 38K guarded write backend binding.
//!
//! This module is a policy-only seam for the future state mutation lane. It
//! describes which typed records may be planned for mutation and which gates
//! must be satisfied before a later phase can attach a live implementation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38kStateWriteRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl Phase38kStateWriteRecordKind {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Progress => "PRG",
            Self::Theme => "THM",
            Self::Metadata => "MTA",
            Self::Bookmark => "BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    pub const fn requires_book_id(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38kStateWriteOperation {
    Create,
    Replace,
    Update,
    Delete,
    Append,
    Compact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38kWriteBindingMode {
    Disabled,
    DryRun,
    GuardedFutureBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38kWriteGate {
    RecordKindAllowed,
    OperationAllowed,
    BookIdRequiredWhenPerBook,
    PayloadRequiredWhenMutating,
    SideEffectsDeniedInThisPhase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38kWriteBindingDecision {
    AcceptedAsPlanOnly,
    RejectedRecordKind,
    RejectedOperation,
    RejectedMissingBookId,
    RejectedMissingPayload,
    RejectedLiveMutation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38kWriteBindingRequest<'a> {
    pub record_kind: Phase38kStateWriteRecordKind,
    pub operation: Phase38kStateWriteOperation,
    pub book_id_hex8: Option<&'a str>,
    pub payload_len: usize,
    pub mode: Phase38kWriteBindingMode,
}

impl<'a> Phase38kWriteBindingRequest<'a> {
    pub const fn new(
        record_kind: Phase38kStateWriteRecordKind,
        operation: Phase38kStateWriteOperation,
        book_id_hex8: Option<&'a str>,
        payload_len: usize,
        mode: Phase38kWriteBindingMode,
    ) -> Self {
        Self {
            record_kind,
            operation,
            book_id_hex8,
            payload_len,
            mode,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38kWriteBindingReport {
    pub phase_marker: &'static str,
    pub writes_enabled: bool,
    pub live_backend_bound: bool,
    pub decision: Phase38kWriteBindingDecision,
}

pub const PHASE_38K_GUARDED_WRITE_BACKEND_BINDING_MARKER: &str =
    "phase38k=x4-state-io-guarded-write-backend-binding-ok";

pub const PHASE_38K_ALLOWED_RECORD_KINDS: &[Phase38kStateWriteRecordKind] = &[
    Phase38kStateWriteRecordKind::Progress,
    Phase38kStateWriteRecordKind::Theme,
    Phase38kStateWriteRecordKind::Metadata,
    Phase38kStateWriteRecordKind::Bookmark,
    Phase38kStateWriteRecordKind::BookmarkIndex,
];

pub const PHASE_38K_ALLOWED_OPERATIONS: &[Phase38kStateWriteOperation] = &[
    Phase38kStateWriteOperation::Create,
    Phase38kStateWriteOperation::Replace,
    Phase38kStateWriteOperation::Update,
    Phase38kStateWriteOperation::Delete,
    Phase38kStateWriteOperation::Append,
    Phase38kStateWriteOperation::Compact,
];

pub const PHASE_38K_GATES: &[Phase38kWriteGate] = &[
    Phase38kWriteGate::RecordKindAllowed,
    Phase38kWriteGate::OperationAllowed,
    Phase38kWriteGate::BookIdRequiredWhenPerBook,
    Phase38kWriteGate::PayloadRequiredWhenMutating,
    Phase38kWriteGate::SideEffectsDeniedInThisPhase,
];

pub const fn phase38k_writes_enabled() -> bool {
    false
}

pub const fn phase38k_live_backend_bound() -> bool {
    false
}

pub fn phase38k_record_kind_allowed(kind: Phase38kStateWriteRecordKind) -> bool {
    PHASE_38K_ALLOWED_RECORD_KINDS.contains(&kind)
}

pub fn phase38k_operation_allowed(operation: Phase38kStateWriteOperation) -> bool {
    PHASE_38K_ALLOWED_OPERATIONS.contains(&operation)
}

pub const fn phase38k_operation_needs_payload(operation: Phase38kStateWriteOperation) -> bool {
    !matches!(operation, Phase38kStateWriteOperation::Delete)
}

pub fn phase38k_evaluate_write_binding(
    request: Phase38kWriteBindingRequest<'_>,
) -> Phase38kWriteBindingReport {
    let decision = if !phase38k_record_kind_allowed(request.record_kind) {
        Phase38kWriteBindingDecision::RejectedRecordKind
    } else if !phase38k_operation_allowed(request.operation) {
        Phase38kWriteBindingDecision::RejectedOperation
    } else if request.record_kind.requires_book_id() && request.book_id_hex8.is_none() {
        Phase38kWriteBindingDecision::RejectedMissingBookId
    } else if phase38k_operation_needs_payload(request.operation) && request.payload_len == 0 {
        Phase38kWriteBindingDecision::RejectedMissingPayload
    } else if !matches!(
        request.mode,
        Phase38kWriteBindingMode::Disabled | Phase38kWriteBindingMode::DryRun
    ) {
        Phase38kWriteBindingDecision::RejectedLiveMutation
    } else {
        Phase38kWriteBindingDecision::AcceptedAsPlanOnly
    };

    Phase38kWriteBindingReport {
        phase_marker: PHASE_38K_GUARDED_WRITE_BACKEND_BINDING_MARKER,
        writes_enabled: phase38k_writes_enabled(),
        live_backend_bound: phase38k_live_backend_bound(),
        decision,
    }
}

pub fn phase38k_acceptance_report() -> Phase38kWriteBindingReport {
    phase38k_evaluate_write_binding(Phase38kWriteBindingRequest::new(
        Phase38kStateWriteRecordKind::Progress,
        Phase38kStateWriteOperation::Update,
        Some("8A79A61F"),
        8,
        Phase38kWriteBindingMode::DryRun,
    ))
}
