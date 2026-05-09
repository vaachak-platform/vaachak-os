#![allow(dead_code)]

use crate::vaachak_x4::physical::vendor_pulp_os_scope_reduction::VaachakVendorPulpOsScopeReduction;

pub struct VaachakVendorPulpOsScopeReductionSmoke;

impl VaachakVendorPulpOsScopeReductionSmoke {
    pub fn smoke_ok() -> bool {
        let report = VaachakVendorPulpOsScopeReduction::report();
        report.ok() && VaachakVendorPulpOsScopeReduction::scope_reduction_ok()
    }
}
