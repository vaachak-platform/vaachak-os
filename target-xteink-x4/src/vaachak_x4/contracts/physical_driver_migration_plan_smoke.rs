use crate::vaachak_x4::physical::physical_driver_migration_plan::{
    VaachakPhysicalDriverMigrationPlan, VaachakPhysicalDriverMigrationRisk,
    VaachakPhysicalDriverMigrationTarget,
};

pub fn physical_driver_migration_plan_smoke_ok() -> bool {
    let report = VaachakPhysicalDriverMigrationPlan::report();
    let steps = VaachakPhysicalDriverMigrationPlan::migration_steps();

    report.ok()
        && report.marker == "physical_driver_migration_plan=ok"
        && report.active_low_level_fallback == "PulpCompatibility"
        && report.first_driver_target == VaachakPhysicalDriverMigrationTarget::InputPhysicalSampling
        && steps[0].target == VaachakPhysicalDriverMigrationTarget::InputPhysicalSampling
        && steps[0].risk == VaachakPhysicalDriverMigrationRisk::Low
        && steps[1].target == VaachakPhysicalDriverMigrationTarget::SpiPhysicalTransaction
        && steps[2].target == VaachakPhysicalDriverMigrationTarget::DisplaySsd1677PhysicalRefresh
        && steps[3].target == VaachakPhysicalDriverMigrationTarget::StorageSdMmcBlockDriver
        && steps[4].target == VaachakPhysicalDriverMigrationTarget::StorageFatAlgorithm
        && report.destructive_storage_operations_deferred
        && !report.reader_file_browser_ux_changed
        && !report.app_navigation_behavior_changed
}

#[cfg(test)]
mod tests {
    use super::physical_driver_migration_plan_smoke_ok;

    #[test]
    fn physical_driver_migration_plan_contract_is_ready() {
        assert!(physical_driver_migration_plan_smoke_ok());
    }
}
