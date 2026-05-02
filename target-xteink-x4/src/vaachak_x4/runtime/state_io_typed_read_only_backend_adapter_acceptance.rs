//! Phase 37D typed read-only backend adapter acceptance metadata.
//!
//! This module is intentionally side-effect free. It records the acceptance
//! contract for the Phase 37C typed read-only adapter lane without binding any
//! storage, SPI, display, input, or power behavior.

pub const PHASE_37D_STATE_IO_TYPED_READ_ONLY_BACKEND_ADAPTER_ACCEPTANCE_MARKER: &str =
    "phase37d=x4-state-io-typed-read-only-backend-adapter-acceptance-ok";

pub const PHASE_37D_SCOPE: &str = "typed-state-read-only-backend-adapter-acceptance";
pub const PHASE_37D_MODE: &str = "metadata-only";
pub const PHASE_37D_NEXT_LANE: &str = "typed-state-read-only-path-resolution";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37dAcceptedRecord {
    Progress,
    Theme,
    Metadata,
    Bookmarks,
    BookmarkIndex,
}

impl Phase37dAcceptedRecord {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmarks => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    pub const fn typed_adapter_lane(self) -> &'static str {
        match self {
            Self::Progress => "progress-state-read-only-adapter",
            Self::Theme => "theme-state-read-only-adapter",
            Self::Metadata => "metadata-state-read-only-adapter",
            Self::Bookmarks => "bookmark-state-read-only-adapter",
            Self::BookmarkIndex => "bookmark-index-read-only-adapter",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37dAcceptanceItem {
    SideEffectFree,
    ReadOnlyOnly,
    TypedRecordsEnumerated,
    NoWritePathEnabled,
    NoRuntimeBootFlowChange,
    ReadyForPathResolution,
}

impl Phase37dAcceptanceItem {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SideEffectFree => "side-effect-free",
            Self::ReadOnlyOnly => "read-only-only",
            Self::TypedRecordsEnumerated => "typed-records-enumerated",
            Self::NoWritePathEnabled => "no-write-path-enabled",
            Self::NoRuntimeBootFlowChange => "no-runtime-boot-flow-change",
            Self::ReadyForPathResolution => "ready-for-path-resolution",
        }
    }
}

pub const PHASE_37D_ACCEPTED_RECORDS: [Phase37dAcceptedRecord; 5] = [
    Phase37dAcceptedRecord::Progress,
    Phase37dAcceptedRecord::Theme,
    Phase37dAcceptedRecord::Metadata,
    Phase37dAcceptedRecord::Bookmarks,
    Phase37dAcceptedRecord::BookmarkIndex,
];

pub const PHASE_37D_ACCEPTANCE_ITEMS: [Phase37dAcceptanceItem; 6] = [
    Phase37dAcceptanceItem::SideEffectFree,
    Phase37dAcceptanceItem::ReadOnlyOnly,
    Phase37dAcceptanceItem::TypedRecordsEnumerated,
    Phase37dAcceptanceItem::NoWritePathEnabled,
    Phase37dAcceptanceItem::NoRuntimeBootFlowChange,
    Phase37dAcceptanceItem::ReadyForPathResolution,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37dTypedAdapterAcceptanceReport {
    pub marker: &'static str,
    pub scope: &'static str,
    pub mode: &'static str,
    pub next_lane: &'static str,
    pub accepted_record_count: usize,
    pub acceptance_item_count: usize,
    pub read_only: bool,
    pub writes_enabled: bool,
    pub behavior_moved: bool,
}

pub const PHASE_37D_TYPED_ADAPTER_ACCEPTANCE_REPORT: Phase37dTypedAdapterAcceptanceReport =
    Phase37dTypedAdapterAcceptanceReport {
        marker: PHASE_37D_STATE_IO_TYPED_READ_ONLY_BACKEND_ADAPTER_ACCEPTANCE_MARKER,
        scope: PHASE_37D_SCOPE,
        mode: PHASE_37D_MODE,
        next_lane: PHASE_37D_NEXT_LANE,
        accepted_record_count: PHASE_37D_ACCEPTED_RECORDS.len(),
        acceptance_item_count: PHASE_37D_ACCEPTANCE_ITEMS.len(),
        read_only: true,
        writes_enabled: false,
        behavior_moved: false,
    };

pub const fn phase37d_marker() -> &'static str {
    PHASE_37D_STATE_IO_TYPED_READ_ONLY_BACKEND_ADAPTER_ACCEPTANCE_MARKER
}

pub const fn phase37d_acceptance_report() -> Phase37dTypedAdapterAcceptanceReport {
    PHASE_37D_TYPED_ADAPTER_ACCEPTANCE_REPORT
}

pub fn phase37d_has_record(record: Phase37dAcceptedRecord) -> bool {
    PHASE_37D_ACCEPTED_RECORDS.contains(&record)
}

pub fn phase37d_has_acceptance_item(item: Phase37dAcceptanceItem) -> bool {
    PHASE_37D_ACCEPTANCE_ITEMS.contains(&item)
}

pub const fn phase37d_is_ready_for_path_resolution() -> bool {
    PHASE_37D_TYPED_ADAPTER_ACCEPTANCE_REPORT.read_only
        && !PHASE_37D_TYPED_ADAPTER_ACCEPTANCE_REPORT.writes_enabled
        && !PHASE_37D_TYPED_ADAPTER_ACCEPTANCE_REPORT.behavior_moved
}
