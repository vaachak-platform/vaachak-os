//! Phase 40A — Device Regression and Write-Lane Closeout Acceptance.
//!
//! Acceptance wrapper for the Phase 40A closeout metadata.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_device_regression_write_lane_closeout::{
    phase40a_closeout_report, Phase40aCloseoutReason, Phase40aCloseoutStatus, Phase40aNextLane,
};

pub const PHASE_40A_DEVICE_REGRESSION_WRITE_LANE_CLOSEOUT_ACCEPTANCE_MARKER: &str =
    "phase40a-acceptance=x4-device-regression-write-lane-closeout-report-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40aAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40aAcceptanceReason {
    CloseoutAccepted,
    CloseoutBlocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40aAcceptanceReport {
    pub status: Phase40aAcceptanceStatus,
    pub reason: Phase40aAcceptanceReason,
    pub closeout_reason: Phase40aCloseoutReason,
    pub next_lane: Phase40aNextLane,
}

impl Phase40aAcceptanceReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40aAcceptanceStatus::Accepted)
    }
}

pub fn phase40a_acceptance_report() -> Phase40aAcceptanceReport {
    let report = phase40a_closeout_report();
    let accepted = report.accepted();

    Phase40aAcceptanceReport {
        status: if accepted {
            Phase40aAcceptanceStatus::Accepted
        } else {
            Phase40aAcceptanceStatus::Rejected
        },
        reason: if matches!(report.status, Phase40aCloseoutStatus::Accepted) {
            Phase40aAcceptanceReason::CloseoutAccepted
        } else {
            Phase40aAcceptanceReason::CloseoutBlocked
        },
        closeout_reason: report.reason,
        next_lane: report.next_lane,
    }
}

pub fn phase40a_acceptance_marker() -> &'static str {
    PHASE_40A_DEVICE_REGRESSION_WRITE_LANE_CLOSEOUT_ACCEPTANCE_MARKER
}
