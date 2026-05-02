//! Phase 36P — State I/O Backend Handoff Checklist Overlay.
//!
//! This module defines a side-effect-free checklist for handing the typed state
//! I/O lane from compile-only planning into a later real backend design phase.
//! It intentionally performs no filesystem, SPI, display, input, or power work.
//!
//! The purpose is to make the backend handoff auditable before binding the
//! Vaachak-owned typed state records to the X4 SD/FAT implementation.

#![allow(dead_code)]

use super::state_io_shadow_write_acceptance::{
    ShadowWriteAcceptanceReport, phase36o_acceptance_report, phase36o_marker,
};

/// Phase 36P boot/build marker.
pub const PHASE_36P_STATE_IO_BACKEND_HANDOFF_CHECKLIST_MARKER: &str =
    "phase36p=x4-state-io-backend-handoff-checklist-ok";

/// Prior side-effect-free planning markers covered by this handoff checklist.
pub const STATE_IO_HANDOFF_COVERED_MARKERS: [&str; 4] = [
    "phase36j=x4-state-io-backend-dry-run-ok",
    "phase36l=x4-state-io-commit-plan-ok",
    "phase36n=x4-state-io-shadow-write-plan-ok",
    "phase36o=x4-state-io-shadow-write-acceptance-ok",
];

/// Typed state records that must remain covered before backend work starts.
pub const STATE_IO_HANDOFF_RECORDS: [&str; 5] = [
    "state/<BOOKID>.PRG",
    "state/<BOOKID>.THM",
    "state/<BOOKID>.MTA",
    "state/<BOOKID>.BKM",
    "state/BMIDX.TXT",
];

/// Compile-time checklist items for the future backend lane.
pub const STATE_IO_BACKEND_HANDOFF_ITEMS: [&str; 8] = [
    "typed-record-coverage-complete",
    "dry-run-lane-recorded",
    "commit-plan-lane-recorded",
    "shadow-write-plan-recorded",
    "shadow-write-acceptance-required",
    "backend-binding-disabled",
    "hardware-behavior-unmoved",
    "next-lane-real-backend-design-only",
];

/// Minimum number of covered markers required for handoff readiness.
pub const REQUIRED_HANDOFF_MARKER_COUNT: usize = 4;

/// Minimum number of typed records required for handoff readiness.
pub const REQUIRED_HANDOFF_RECORD_COUNT: usize = 5;

/// Minimum number of checklist items required for handoff readiness.
pub const REQUIRED_HANDOFF_ITEM_COUNT: usize = 8;

/// Compile-time handoff decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendHandoffDecision {
    Ready,
    Blocked,
}

impl BackendHandoffDecision {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }

    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Side-effect-free handoff checklist for a future typed state backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendHandoffChecklist {
    pub marker: &'static str,
    pub accepted_shadow_write_marker: &'static str,
    pub decision: BackendHandoffDecision,
    pub covered_marker_count: usize,
    pub record_count: usize,
    pub checklist_item_count: usize,
    pub required_marker_count: usize,
    pub required_record_count: usize,
    pub required_item_count: usize,
    pub shadow_write_accepted: bool,
    pub backend_bound: bool,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
    pub next_lane: &'static str,
}

impl BackendHandoffChecklist {
    pub const fn from_shadow_write_acceptance(report: ShadowWriteAcceptanceReport) -> Self {
        let covered_marker_count = STATE_IO_HANDOFF_COVERED_MARKERS.len();
        let record_count = STATE_IO_HANDOFF_RECORDS.len();
        let checklist_item_count = STATE_IO_BACKEND_HANDOFF_ITEMS.len();
        let has_required_markers = covered_marker_count >= REQUIRED_HANDOFF_MARKER_COUNT;
        let has_required_records = record_count >= REQUIRED_HANDOFF_RECORD_COUNT;
        let has_required_items = checklist_item_count >= REQUIRED_HANDOFF_ITEM_COUNT;
        let shadow_write_accepted = report.is_accepted();
        let backend_bound = report.backend_bound;
        let storage_behavior_moved = report.storage_behavior_moved;
        let display_behavior_moved = report.display_behavior_moved;
        let input_behavior_moved = report.input_behavior_moved;
        let power_behavior_moved = report.power_behavior_moved;
        let ready = has_required_markers
            && has_required_records
            && has_required_items
            && shadow_write_accepted
            && !backend_bound
            && !storage_behavior_moved
            && !display_behavior_moved
            && !input_behavior_moved
            && !power_behavior_moved;

        Self {
            marker: PHASE_36P_STATE_IO_BACKEND_HANDOFF_CHECKLIST_MARKER,
            accepted_shadow_write_marker: phase36o_marker(),
            decision: if ready {
                BackendHandoffDecision::Ready
            } else {
                BackendHandoffDecision::Blocked
            },
            covered_marker_count,
            record_count,
            checklist_item_count,
            required_marker_count: REQUIRED_HANDOFF_MARKER_COUNT,
            required_record_count: REQUIRED_HANDOFF_RECORD_COUNT,
            required_item_count: REQUIRED_HANDOFF_ITEM_COUNT,
            shadow_write_accepted,
            backend_bound,
            storage_behavior_moved,
            display_behavior_moved,
            input_behavior_moved,
            power_behavior_moved,
            next_lane: "state-io-real-backend-design",
        }
    }

    pub const fn current() -> Self {
        Self::from_shadow_write_acceptance(phase36o_acceptance_report())
    }

    pub const fn is_ready(self) -> bool {
        self.decision.is_ready()
    }

    pub const fn decision_label(self) -> &'static str {
        self.decision.label()
    }
}

/// Compile-time handoff checklist for the current state I/O backend lane.
pub const STATE_IO_BACKEND_HANDOFF_CHECKLIST: BackendHandoffChecklist =
    BackendHandoffChecklist::current();

/// Return the accepted Phase 36P marker for boot/runtime status reporting.
pub const fn phase36p_marker() -> &'static str {
    PHASE_36P_STATE_IO_BACKEND_HANDOFF_CHECKLIST_MARKER
}

/// Return the side-effect-free backend handoff checklist.
pub const fn phase36p_handoff_checklist() -> BackendHandoffChecklist {
    STATE_IO_BACKEND_HANDOFF_CHECKLIST
}

/// Return whether the future backend lane is ready for design-only work.
pub const fn phase36p_is_ready_for_backend_design() -> bool {
    STATE_IO_BACKEND_HANDOFF_CHECKLIST.is_ready()
}
