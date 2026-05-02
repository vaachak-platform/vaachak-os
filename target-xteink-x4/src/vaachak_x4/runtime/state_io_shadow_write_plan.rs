//! Phase 36N — State I/O Shadow Write Plan Overlay.
//!
//! This module defines a Vaachak-owned, side-effect-free plan for how typed
//! state records should eventually be shadow-written to the X4 SD/FAT backend.
//! It intentionally performs no filesystem, SPI, display, input, or power work.
//!
//! The purpose is to make the future write sequence explicit before binding it
//! to a real backend.

#![allow(dead_code)]

/// Phase 36N boot/build marker.
pub const PHASE_36N_STATE_IO_SHADOW_WRITE_PLAN_MARKER: &str =
    "phase36n=x4-state-io-shadow-write-plan-ok";

/// Typed state records covered by the shadow-write plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowWriteRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl ShadowWriteRecordKind {
    /// Human-readable record label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Theme => "theme",
            Self::Metadata => "metadata",
            Self::Bookmark => "bookmark",
            Self::BookmarkIndex => "bookmark-index",
        }
    }

    /// Final on-card suffix/name for the state record type.
    pub const fn final_name(self) -> &'static str {
        match self {
            Self::Progress => "<BOOKID>.PRG",
            Self::Theme => "<BOOKID>.THM",
            Self::Metadata => "<BOOKID>.MTA",
            Self::Bookmark => "<BOOKID>.BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Shadow/temp name to use before the final record is replaced.
    pub const fn shadow_name(self) -> &'static str {
        match self {
            Self::Progress => "<BOOKID>.P~G",
            Self::Theme => "<BOOKID>.T~M",
            Self::Metadata => "<BOOKID>.M~A",
            Self::Bookmark => "<BOOKID>.B~M",
            Self::BookmarkIndex => "BMIDX.TMP",
        }
    }

    /// Optional rollback/backup name for later real backend implementation.
    pub const fn backup_name(self) -> &'static str {
        match self {
            Self::Progress => "<BOOKID>.PBA",
            Self::Theme => "<BOOKID>.TBA",
            Self::Metadata => "<BOOKID>.MBA",
            Self::Bookmark => "<BOOKID>.BBA",
            Self::BookmarkIndex => "BMIDX.BAK",
        }
    }
}

/// Side-effect-free description of a single shadow-write operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShadowWriteTarget {
    pub kind: ShadowWriteRecordKind,
    pub state_dir: &'static str,
    pub final_name: &'static str,
    pub shadow_name: &'static str,
    pub backup_name: &'static str,
}

impl ShadowWriteTarget {
    pub const fn for_kind(kind: ShadowWriteRecordKind) -> Self {
        Self {
            kind,
            state_dir: "state/",
            final_name: kind.final_name(),
            shadow_name: kind.shadow_name(),
            backup_name: kind.backup_name(),
        }
    }
}

/// Logical step names for the future write sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowWriteStep {
    BuildRecordBytes,
    WriteShadowRecord,
    VerifyShadowRecord,
    PreservePreviousRecord,
    PromoteShadowRecord,
    CleanupShadowRecord,
    ReportCommitOutcome,
}

impl ShadowWriteStep {
    pub const fn label(self) -> &'static str {
        match self {
            Self::BuildRecordBytes => "build-record-bytes",
            Self::WriteShadowRecord => "write-shadow-record",
            Self::VerifyShadowRecord => "verify-shadow-record",
            Self::PreservePreviousRecord => "preserve-previous-record",
            Self::PromoteShadowRecord => "promote-shadow-record",
            Self::CleanupShadowRecord => "cleanup-shadow-record",
            Self::ReportCommitOutcome => "report-commit-outcome",
        }
    }
}

/// Ordered, compile-time-only plan for future backend writes.
pub const SHADOW_WRITE_STEPS: [ShadowWriteStep; 7] = [
    ShadowWriteStep::BuildRecordBytes,
    ShadowWriteStep::WriteShadowRecord,
    ShadowWriteStep::VerifyShadowRecord,
    ShadowWriteStep::PreservePreviousRecord,
    ShadowWriteStep::PromoteShadowRecord,
    ShadowWriteStep::CleanupShadowRecord,
    ShadowWriteStep::ReportCommitOutcome,
];

/// Records covered by the plan.
pub const SHADOW_WRITE_RECORD_KINDS: [ShadowWriteRecordKind; 5] = [
    ShadowWriteRecordKind::Progress,
    ShadowWriteRecordKind::Theme,
    ShadowWriteRecordKind::Metadata,
    ShadowWriteRecordKind::Bookmark,
    ShadowWriteRecordKind::BookmarkIndex,
];

/// Side-effect-free summary of the shadow-write lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShadowWritePlanSummary {
    pub marker: &'static str,
    pub state_dir: &'static str,
    pub record_count: usize,
    pub step_count: usize,
    pub side_effect_free: bool,
    pub backend_bound: bool,
    pub moves_storage_behavior: bool,
    pub moves_display_behavior: bool,
    pub moves_input_behavior: bool,
    pub moves_power_behavior: bool,
}

impl ShadowWritePlanSummary {
    pub const fn accepted() -> Self {
        Self {
            marker: PHASE_36N_STATE_IO_SHADOW_WRITE_PLAN_MARKER,
            state_dir: "state/",
            record_count: SHADOW_WRITE_RECORD_KINDS.len(),
            step_count: SHADOW_WRITE_STEPS.len(),
            side_effect_free: true,
            backend_bound: false,
            moves_storage_behavior: false,
            moves_display_behavior: false,
            moves_input_behavior: false,
            moves_power_behavior: false,
        }
    }

    pub const fn is_safe_to_compile(self) -> bool {
        self.side_effect_free
            && !self.backend_bound
            && !self.moves_storage_behavior
            && !self.moves_display_behavior
            && !self.moves_input_behavior
            && !self.moves_power_behavior
    }
}

pub const STATE_IO_SHADOW_WRITE_PLAN_SUMMARY: ShadowWritePlanSummary =
    ShadowWritePlanSummary::accepted();

/// Build a side-effect-free target descriptor for a typed state record.
pub const fn shadow_write_target(kind: ShadowWriteRecordKind) -> ShadowWriteTarget {
    ShadowWriteTarget::for_kind(kind)
}

/// Return the accepted marker for boot/runtime status reporting.
pub const fn phase36n_marker() -> &'static str {
    PHASE_36N_STATE_IO_SHADOW_WRITE_PLAN_MARKER
}

/// Return whether this module remains a compile-only plan.
pub const fn phase36n_is_side_effect_free() -> bool {
    STATE_IO_SHADOW_WRITE_PLAN_SUMMARY.is_safe_to_compile()
}
