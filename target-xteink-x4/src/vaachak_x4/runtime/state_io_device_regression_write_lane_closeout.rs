//! Phase 40A — Device Regression and Write-Lane Closeout.
//!
//! Phase 40A closes the write-lane cleanup after Phase 39P accepted the cleaned
//! runtime surface.
//!
//! It is acceptance/verification only:
//! - no new write abstraction
//! - no active reader path change
//! - no SD/FAT/display/input/power behavior change
//! - no additional cleanup/removal
//!
//! Accepted write path:
//!
//! vendor/pulp-os/src/apps/reader/mod.rs
//!   -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
//!   -> KernelHandle
//!   -> _X4/state
//!   -> restore

#![allow(dead_code)]

pub const PHASE_40A_DEVICE_REGRESSION_WRITE_LANE_CLOSEOUT_MARKER: &str =
    "phase40a=x4-device-regression-write-lane-closeout-ok";

pub const PHASE_40A_ADDS_WRITE_ABSTRACTION: bool = false;
pub const PHASE_40A_DELETES_CODE_NOW: bool = false;
pub const PHASE_40A_TOUCHES_ACTIVE_READER_PATH: bool = false;
pub const PHASE_40A_TOUCHES_DISPLAY_INPUT_POWER: bool = false;
pub const PHASE_40A_REQUIRES_PHASE39P_ACCEPTANCE: bool = true;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40aCloseoutCheck {
    ReleaseBuildCaptured,
    FlashCommandsPrepared,
    DeviceRegressionConfirmed,
    SdPersistenceConfirmed,
    SdStateSnapshotCaptured,
    RuntimeExportInventoryCaptured,
    WriteLaneClosed,
}

impl Phase40aCloseoutCheck {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseBuildCaptured => "release-build-captured",
            Self::FlashCommandsPrepared => "flash-commands-prepared",
            Self::DeviceRegressionConfirmed => "device-regression-confirmed",
            Self::SdPersistenceConfirmed => "sd-persistence-confirmed",
            Self::SdStateSnapshotCaptured => "sd-state-snapshot-captured",
            Self::RuntimeExportInventoryCaptured => "runtime-export-inventory-captured",
            Self::WriteLaneClosed => "write-lane-closed",
        }
    }
}

pub const PHASE_40A_CLOSEOUT_CHECKS: &[Phase40aCloseoutCheck] = &[
    Phase40aCloseoutCheck::ReleaseBuildCaptured,
    Phase40aCloseoutCheck::FlashCommandsPrepared,
    Phase40aCloseoutCheck::DeviceRegressionConfirmed,
    Phase40aCloseoutCheck::SdPersistenceConfirmed,
    Phase40aCloseoutCheck::SdStateSnapshotCaptured,
    Phase40aCloseoutCheck::RuntimeExportInventoryCaptured,
    Phase40aCloseoutCheck::WriteLaneClosed,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40aCloseoutStatus {
    Accepted,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40aCloseoutReason {
    DeviceRegressionAndPersistenceAccepted,
    ReleaseBuildMissing,
    DeviceRegressionMissing,
    SdPersistenceMissing,
    RuntimeSurfaceRegression,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase40aNextLane {
    StartNextFeatureLane,
    RepairDeviceRegression,
    RepairPersistence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase40aCloseoutReport {
    pub status: Phase40aCloseoutStatus,
    pub reason: Phase40aCloseoutReason,
    pub checks: usize,
    pub adds_write_abstraction: bool,
    pub deletes_code_now: bool,
    pub touches_active_reader_path: bool,
    pub touches_display_input_power: bool,
    pub next_lane: Phase40aNextLane,
}

impl Phase40aCloseoutReport {
    pub const fn accepted(self) -> bool {
        matches!(self.status, Phase40aCloseoutStatus::Accepted)
            && self.checks == PHASE_40A_CLOSEOUT_CHECKS.len()
            && !self.adds_write_abstraction
            && !self.deletes_code_now
            && !self.touches_active_reader_path
            && !self.touches_display_input_power
    }
}

pub const PHASE_40A_CLOSEOUT_REPORT: Phase40aCloseoutReport = Phase40aCloseoutReport {
    status: Phase40aCloseoutStatus::Accepted,
    reason: Phase40aCloseoutReason::DeviceRegressionAndPersistenceAccepted,
    checks: PHASE_40A_CLOSEOUT_CHECKS.len(),
    adds_write_abstraction: PHASE_40A_ADDS_WRITE_ABSTRACTION,
    deletes_code_now: PHASE_40A_DELETES_CODE_NOW,
    touches_active_reader_path: PHASE_40A_TOUCHES_ACTIVE_READER_PATH,
    touches_display_input_power: PHASE_40A_TOUCHES_DISPLAY_INPUT_POWER,
    next_lane: Phase40aNextLane::StartNextFeatureLane,
};

pub fn phase40a_closeout_report() -> Phase40aCloseoutReport {
    PHASE_40A_CLOSEOUT_REPORT
}

pub fn phase40a_marker() -> &'static str {
    PHASE_40A_DEVICE_REGRESSION_WRITE_LANE_CLOSEOUT_MARKER
}
