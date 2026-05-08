use crate::vaachak_x4::physical::hardware_native_behavior_consolidation::VaachakHardwareNativeBehaviorConsolidation;

pub struct HardwareNativeBehaviorConsolidationSmoke;

impl HardwareNativeBehaviorConsolidationSmoke {
    pub fn validates_native_behavior_stack() -> bool {
        let report = VaachakHardwareNativeBehaviorConsolidation::report();
        report.ok()
            && report.marker == "hardware_native_behavior_consolidation=ok"
            && report.behavior_owner == "target-xteink-x4 Vaachak layer"
            && report.low_level_fallback_name == "PulpCompatibility"
            && report.input_event_pipeline_behavior_moved_to_vaachak
            && report.display_refresh_command_behavior_moved_to_vaachak
            && report.storage_sd_mmc_fat_command_behavior_moved_to_vaachak
            && !report.physical_adc_gpio_sampling_moved_to_vaachak
            && !report.ssd1677_draw_algorithm_moved_to_vaachak
            && !report.low_level_sd_mmc_block_driver_moved_to_vaachak
            && !report.low_level_fat_algorithm_moved_to_vaachak
    }
}

#[cfg(test)]
mod tests {
    use super::HardwareNativeBehaviorConsolidationSmoke;

    #[test]
    fn hardware_native_behavior_consolidation_smoke_passes() {
        assert!(HardwareNativeBehaviorConsolidationSmoke::validates_native_behavior_stack());
    }
}
