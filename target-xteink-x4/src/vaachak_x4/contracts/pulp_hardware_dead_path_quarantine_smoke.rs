#![allow(dead_code)]

use crate::vaachak_x4::physical::pulp_hardware_dead_path_quarantine::{
    VaachakPulpHardwareDeadPathQuarantine, VaachakPulpHardwareQuarantineDisposition,
};

pub struct VaachakPulpHardwareDeadPathQuarantineSmoke;

impl VaachakPulpHardwareDeadPathQuarantineSmoke {
    pub const MARKER: &'static str = "pulp_hardware_dead_path_quarantine=ok";

    pub fn smoke_ok() -> bool {
        let report = VaachakPulpHardwareDeadPathQuarantine::report();
        let entries = VaachakPulpHardwareDeadPathQuarantine::QUARANTINE_ENTRIES;
        report.ok()
            && VaachakPulpHardwareDeadPathQuarantine::quarantine_ok()
            && report.marker == Self::MARKER
            && entries.iter().any(|entry| {
                matches!(
                    entry.disposition,
                    VaachakPulpHardwareQuarantineDisposition::QuarantineDeadLegacyHardwarePath
                ) && entry.quarantined
                    && !entry.runtime_hardware_active
                    && !entry.deletion_performed
            })
            && entries.iter().any(|entry| {
                matches!(
                    entry.disposition,
                    VaachakPulpHardwareQuarantineDisposition::KeepRequiredRuntimeDependency
                ) && entry.vendor_tree_required
                    && !entry.deletion_performed
            })
            && entries.iter().any(|entry| {
                matches!(
                    entry.disposition,
                    VaachakPulpHardwareQuarantineDisposition::KeepCompatibilityImportBoundary
                ) && !entry.runtime_hardware_active
            })
    }
}
