use crate::vaachak_x4::physical::hardware_native_behavior_consolidation::VaachakHardwareNativeBehaviorConsolidation;
use crate::vaachak_x4::physical::input_backend_native_event_pipeline::VaachakInputBackendNativeEventPipeline;
use crate::vaachak_x4::physical::input_backend_native_event_pipeline_cleanup::VaachakInputBackendNativeEventPipelineCleanup;
use crate::vaachak_x4::physical::input_physical_sampling_native_driver::VaachakInputPhysicalSamplingNativeDriver;

/// Smoke contract for the Vaachak-native input physical sampling driver.
pub struct VaachakInputPhysicalSamplingNativeDriverSmoke;

impl VaachakInputPhysicalSamplingNativeDriverSmoke {
    pub const INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_SMOKE_MARKER: &'static str =
        "input_physical_sampling_native_driver=ok";

    pub fn smoke_ok() -> bool {
        let report = VaachakInputPhysicalSamplingNativeDriver::report();
        report.ok()
            && VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok()
            && VaachakInputBackendNativeEventPipeline::native_physical_sampling_driver_ready()
            && VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()
            && VaachakHardwareNativeBehaviorConsolidation::report().ok()
            && report.raw_adc_ladder_sample_interpretation_moved_to_vaachak
            && report.oversample_reduction_moved_to_vaachak
            && report.power_gpio_level_interpretation_moved_to_vaachak
            && !report.adc_peripheral_read_executor_moved_to_vaachak
            && !report.gpio_poll_executor_moved_to_vaachak
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputPhysicalSamplingNativeDriverSmoke;

    #[test]
    fn input_physical_sampling_native_driver_smoke_passes() {
        assert!(VaachakInputPhysicalSamplingNativeDriverSmoke::smoke_ok());
    }
}
