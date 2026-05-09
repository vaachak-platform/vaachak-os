#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_physical_full_migration_consolidation::VaachakHardwarePhysicalFullMigrationConsolidation;

pub struct VaachakHardwarePhysicalFullMigrationConsolidationSmoke;

impl VaachakHardwarePhysicalFullMigrationConsolidationSmoke {
    pub const MARKER: &'static str = "hardware_physical_full_migration_consolidation=ok";

    pub fn smoke_ok() -> bool {
        VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok()
    }
}
