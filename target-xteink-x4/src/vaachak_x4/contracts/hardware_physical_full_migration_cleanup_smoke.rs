#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_physical_full_migration_cleanup::VaachakHardwarePhysicalFullMigrationCleanup;

pub struct VaachakHardwarePhysicalFullMigrationCleanupSmoke;

impl VaachakHardwarePhysicalFullMigrationCleanupSmoke {
    pub const MARKER: &'static str = "hardware_physical_full_migration_cleanup=ok";

    pub fn smoke_ok() -> bool {
        VaachakHardwarePhysicalFullMigrationCleanup::cleanup_ok()
    }
}
