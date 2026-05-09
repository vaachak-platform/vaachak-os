#![allow(dead_code)]

use crate::vaachak_x4::physical::pulp_hardware_dead_path_removal::VaachakPulpHardwareDeadPathRemoval;

pub struct VaachakPulpHardwareDeadPathRemovalSmoke;

impl VaachakPulpHardwareDeadPathRemovalSmoke {
    pub fn smoke_ok() -> bool {
        let report = VaachakPulpHardwareDeadPathRemoval::report();
        report.ok() && VaachakPulpHardwareDeadPathRemoval::removal_ok()
    }
}
