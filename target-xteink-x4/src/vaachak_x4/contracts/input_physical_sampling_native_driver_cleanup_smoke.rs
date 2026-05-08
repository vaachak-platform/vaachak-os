use crate::vaachak_x4::physical::input_physical_sampling_native_driver::VaachakInputPhysicalSamplingNativeDriver;
use crate::vaachak_x4::physical::input_physical_sampling_native_driver_cleanup::VaachakInputPhysicalSamplingNativeDriverCleanup;

/// Smoke contract for the input physical sampling native-driver cleanup checkpoint.
pub struct VaachakInputPhysicalSamplingNativeDriverCleanupSmoke;

impl VaachakInputPhysicalSamplingNativeDriverCleanupSmoke {
    pub const INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_CLEANUP_SMOKE_MARKER: &'static str =
        "input_physical_sampling_native_driver_cleanup=ok";

    pub fn smoke_ok() -> bool {
        let report = VaachakInputPhysicalSamplingNativeDriverCleanup::report();
        report.ok()
            && VaachakInputPhysicalSamplingNativeDriverCleanup::cleanup_ok()
            && VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok()
            && report.adc_ladder_interpretation_consolidated
            && report.oversample_reduction_consolidated
            && report.power_gpio_level_interpretation_consolidated
            && !report.adc_peripheral_read_executor_moved_to_vaachak
            && !report.gpio_poll_executor_moved_to_vaachak
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputPhysicalSamplingNativeDriverCleanupSmoke;

    #[test]
    fn input_physical_sampling_native_driver_cleanup_smoke_passes() {
        assert!(VaachakInputPhysicalSamplingNativeDriverCleanupSmoke::smoke_ok());
    }
}
