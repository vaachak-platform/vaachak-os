//! Phase 36Y — State I/O read-only backend probe.
//!
//! This module is intentionally side-effect free. It models the first
//! read-only probe lane for future Vaachak-owned state I/O backend binding,
//! but it does not call SD/FAT/SPI/display/input/power code.

/// Phase 36Y acceptance marker.
pub const PHASE_36Y_STATE_IO_READ_ONLY_BACKEND_PROBE_MARKER: &str =
    "phase36y=x4-state-io-read-only-backend-probe-ok";

/// State records that are eligible for the read-only backend probe lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyProbeRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl StateIoReadOnlyProbeRecordKind {
    /// Returns the 8.3-safe record suffix or fixed index name.
    pub const fn storage_name(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns true when the record is shared rather than directly keyed by one book id.
    pub const fn is_shared_index(self) -> bool {
        matches!(self, Self::BookmarkIndex)
    }
}

/// Candidate file role used by the future real backend read probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyProbeCandidateRole {
    Primary,
    Backup,
    Temporary,
    Missing,
}

impl StateIoReadOnlyProbeCandidateRole {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Backup => "backup",
            Self::Temporary => "temporary",
            Self::Missing => "missing",
        }
    }
}

/// Read-only decision made after a future backend probes a candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyProbeDecision {
    UsePrimary,
    UseBackup,
    UseDefaultState,
    IgnoreTemporary,
    RejectWriteIntent,
}

impl StateIoReadOnlyProbeDecision {
    pub const fn is_read_only_safe(self) -> bool {
        matches!(
            self,
            Self::UsePrimary | Self::UseBackup | Self::UseDefaultState | Self::IgnoreTemporary
        )
    }
}

/// Compile-only request envelope for the read-only probe lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyProbeRequest {
    pub record_kind: StateIoReadOnlyProbeRecordKind,
    pub candidate_role: StateIoReadOnlyProbeCandidateRole,
    pub write_intent_requested: bool,
}

impl StateIoReadOnlyProbeRequest {
    pub const fn new(
        record_kind: StateIoReadOnlyProbeRecordKind,
        candidate_role: StateIoReadOnlyProbeCandidateRole,
    ) -> Self {
        Self {
            record_kind,
            candidate_role,
            write_intent_requested: false,
        }
    }

    pub const fn rejected_write_intent(record_kind: StateIoReadOnlyProbeRecordKind) -> Self {
        Self {
            record_kind,
            candidate_role: StateIoReadOnlyProbeCandidateRole::Primary,
            write_intent_requested: true,
        }
    }
}

/// Compile-only outcome envelope for a read-only probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyProbeOutcome {
    pub record_kind: StateIoReadOnlyProbeRecordKind,
    pub candidate_role: StateIoReadOnlyProbeCandidateRole,
    pub decision: StateIoReadOnlyProbeDecision,
}

impl StateIoReadOnlyProbeOutcome {
    pub const fn is_accepted(self) -> bool {
        self.decision.is_read_only_safe()
    }
}

/// Trait shape for a later backend. Phase 36Y provides the contract only.
pub trait StateIoReadOnlyProbeBackend {
    fn probe_read_only(&self, request: StateIoReadOnlyProbeRequest) -> StateIoReadOnlyProbeOutcome;
}

/// Side-effect-free planner for the read-only probe lane.
pub const fn phase36y_plan_read_only_probe(
    request: StateIoReadOnlyProbeRequest,
) -> StateIoReadOnlyProbeOutcome {
    let decision = if request.write_intent_requested {
        StateIoReadOnlyProbeDecision::RejectWriteIntent
    } else {
        match request.candidate_role {
            StateIoReadOnlyProbeCandidateRole::Primary => StateIoReadOnlyProbeDecision::UsePrimary,
            StateIoReadOnlyProbeCandidateRole::Backup => StateIoReadOnlyProbeDecision::UseBackup,
            StateIoReadOnlyProbeCandidateRole::Temporary => {
                StateIoReadOnlyProbeDecision::IgnoreTemporary
            }
            StateIoReadOnlyProbeCandidateRole::Missing => {
                StateIoReadOnlyProbeDecision::UseDefaultState
            }
        }
    };

    StateIoReadOnlyProbeOutcome {
        record_kind: request.record_kind,
        candidate_role: request.candidate_role,
        decision,
    }
}

/// Records covered by the read-only probe lane.
pub const PHASE_36Y_READ_ONLY_PROBE_RECORDS: [StateIoReadOnlyProbeRecordKind; 5] = [
    StateIoReadOnlyProbeRecordKind::Progress,
    StateIoReadOnlyProbeRecordKind::Theme,
    StateIoReadOnlyProbeRecordKind::Metadata,
    StateIoReadOnlyProbeRecordKind::Bookmark,
    StateIoReadOnlyProbeRecordKind::BookmarkIndex,
];

/// Returns true when a state record participates in Phase 36Y.
pub fn phase36y_has_record_kind(kind: StateIoReadOnlyProbeRecordKind) -> bool {
    PHASE_36Y_READ_ONLY_PROBE_RECORDS.contains(&kind)
}

/// Acceptance summary for Phase 36Y.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyProbeAcceptance {
    pub marker: &'static str,
    pub record_count: usize,
    pub allows_write_intent: bool,
    pub performs_backend_io: bool,
    pub next_lane: &'static str,
}

/// Returns the compile-time Phase 36Y acceptance summary.
pub const fn phase36y_acceptance() -> StateIoReadOnlyProbeAcceptance {
    StateIoReadOnlyProbeAcceptance {
        marker: PHASE_36Y_STATE_IO_READ_ONLY_BACKEND_PROBE_MARKER,
        record_count: PHASE_36Y_READ_ONLY_PROBE_RECORDS.len(),
        allows_write_intent: false,
        performs_backend_io: false,
        next_lane: "read-only-real-backend-probe-acceptance",
    }
}
