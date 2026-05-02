//! Phase 36Z — State I/O read-only backend probe acceptance.
//!
//! This module records acceptance metadata for the Phase 36Y read-only
//! backend probe lane. It is intentionally side-effect free: no device,
//! filesystem, SPI, display, input, power, or boot-flow operation is
//! performed here.

#![allow(clippy::struct_excessive_bools)]

/// Phase marker emitted by the overlay installer/check scripts.
pub const PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE_MARKER: &str =
    "phase36z=x4-state-io-read-only-backend-probe-acceptance-ok";

/// Next intended implementation lane after this acceptance overlay.
pub const PHASE_36Z_NEXT_LANE: &str = "typed-state-real-backend-read-only-binding";

/// Record families covered by the accepted read-only probe lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyProbeAcceptedRecord {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl StateIoReadOnlyProbeAcceptedRecord {
    /// Returns the 8.3-safe suffix or fixed index filename for reporting.
    pub const fn storage_name(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns true for the shared bookmark index record.
    pub const fn is_shared_index(self) -> bool {
        matches!(self, Self::BookmarkIndex)
    }
}

/// Acceptance facts for the read-only backend probe lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoReadOnlyProbeAcceptanceItem {
    ProbeContractAccepted,
    PrimaryCandidateAccepted,
    BackupCandidateAccepted,
    MissingCandidateFallbackAccepted,
    TemporaryCandidateIgnored,
    WriteIntentRejected,
    SideEffectFree,
    BackendCallsDisabled,
    HardwareBehaviorUnmoved,
}

impl StateIoReadOnlyProbeAcceptanceItem {
    /// Stable text form for boot/runtime reporting.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProbeContractAccepted => "probe-contract-accepted",
            Self::PrimaryCandidateAccepted => "primary-candidate-accepted",
            Self::BackupCandidateAccepted => "backup-candidate-accepted",
            Self::MissingCandidateFallbackAccepted => "missing-candidate-fallback-accepted",
            Self::TemporaryCandidateIgnored => "temporary-candidate-ignored",
            Self::WriteIntentRejected => "write-intent-rejected",
            Self::SideEffectFree => "side-effect-free",
            Self::BackendCallsDisabled => "backend-calls-disabled",
            Self::HardwareBehaviorUnmoved => "hardware-behavior-unmoved",
        }
    }
}

/// Static acceptance report for the Phase 36Y read-only probe lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoReadOnlyProbeAcceptanceReport {
    pub phase_marker: &'static str,
    pub accepted: bool,
    pub side_effect_free: bool,
    pub real_backend_calls_enabled: bool,
    pub write_intent_allowed: bool,
    pub hardware_behavior_moved: bool,
    pub next_lane: &'static str,
    pub records: &'static [StateIoReadOnlyProbeAcceptedRecord],
    pub acceptance_items: &'static [StateIoReadOnlyProbeAcceptanceItem],
}

const PHASE_36Z_ACCEPTED_RECORDS: &[StateIoReadOnlyProbeAcceptedRecord] = &[
    StateIoReadOnlyProbeAcceptedRecord::Progress,
    StateIoReadOnlyProbeAcceptedRecord::Theme,
    StateIoReadOnlyProbeAcceptedRecord::Metadata,
    StateIoReadOnlyProbeAcceptedRecord::Bookmark,
    StateIoReadOnlyProbeAcceptedRecord::BookmarkIndex,
];

const PHASE_36Z_ACCEPTANCE_ITEMS: &[StateIoReadOnlyProbeAcceptanceItem] = &[
    StateIoReadOnlyProbeAcceptanceItem::ProbeContractAccepted,
    StateIoReadOnlyProbeAcceptanceItem::PrimaryCandidateAccepted,
    StateIoReadOnlyProbeAcceptanceItem::BackupCandidateAccepted,
    StateIoReadOnlyProbeAcceptanceItem::MissingCandidateFallbackAccepted,
    StateIoReadOnlyProbeAcceptanceItem::TemporaryCandidateIgnored,
    StateIoReadOnlyProbeAcceptanceItem::WriteIntentRejected,
    StateIoReadOnlyProbeAcceptanceItem::SideEffectFree,
    StateIoReadOnlyProbeAcceptanceItem::BackendCallsDisabled,
    StateIoReadOnlyProbeAcceptanceItem::HardwareBehaviorUnmoved,
];

/// Phase 36Z acceptance report.
pub const PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE: StateIoReadOnlyProbeAcceptanceReport =
    StateIoReadOnlyProbeAcceptanceReport {
        phase_marker: PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE_MARKER,
        accepted: true,
        side_effect_free: true,
        real_backend_calls_enabled: false,
        write_intent_allowed: false,
        hardware_behavior_moved: false,
        next_lane: PHASE_36Z_NEXT_LANE,
        records: PHASE_36Z_ACCEPTED_RECORDS,
        acceptance_items: PHASE_36Z_ACCEPTANCE_ITEMS,
    };

/// Returns the Phase 36Z acceptance report.
pub const fn phase36z_state_io_read_only_probe_acceptance()
-> &'static StateIoReadOnlyProbeAcceptanceReport {
    &PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE
}

/// Reports whether the Phase 36Y read-only probe lane has been accepted safely.
pub const fn phase36z_is_accepted() -> bool {
    PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE.accepted
        && PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE.side_effect_free
        && !PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE.real_backend_calls_enabled
        && !PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE.write_intent_allowed
        && !PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE.hardware_behavior_moved
}

/// Returns the next intended implementation lane.
pub const fn phase36z_next_lane() -> &'static str {
    PHASE_36Z_STATE_IO_READ_ONLY_PROBE_ACCEPTANCE.next_lane
}

/// Returns whether the accepted read-only probe lane covers a record family.
pub fn phase36z_has_record(record: StateIoReadOnlyProbeAcceptedRecord) -> bool {
    PHASE_36Z_ACCEPTED_RECORDS.contains(&record)
}

/// Returns whether the report includes a specific acceptance item.
pub fn phase36z_has_acceptance_item(item: StateIoReadOnlyProbeAcceptanceItem) -> bool {
    PHASE_36Z_ACCEPTANCE_ITEMS.contains(&item)
}
