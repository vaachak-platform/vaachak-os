use crate::vaachak_x4::physical::hardware_native_behavior_consolidation::VaachakHardwareNativeBehaviorConsolidation;
use crate::vaachak_x4::physical::hardware_native_behavior_consolidation_cleanup::VaachakHardwareNativeBehaviorConsolidationCleanup;

pub fn hardware_native_behavior_consolidation_cleanup_smoke_ok() -> bool {
    VaachakHardwareNativeBehaviorConsolidation::native_behavior_consolidation_ok()
        && VaachakHardwareNativeBehaviorConsolidationCleanup::cleanup_ok()
        && VaachakHardwareNativeBehaviorConsolidationCleanup::report().ok()
        && VaachakHardwareNativeBehaviorConsolidationCleanup::report().cleanup_marker
            == "hardware_native_behavior_consolidation_cleanup=ok"
}

#[cfg(test)]
mod tests {
    use super::hardware_native_behavior_consolidation_cleanup_smoke_ok;

    #[test]
    fn hardware_native_behavior_consolidation_cleanup_contract_is_ready() {
        assert!(hardware_native_behavior_consolidation_cleanup_smoke_ok());
    }
}
