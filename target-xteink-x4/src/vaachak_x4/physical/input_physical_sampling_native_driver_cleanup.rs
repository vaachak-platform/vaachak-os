use super::input_backend_native_event_pipeline_cleanup::VaachakInputBackendNativeEventPipelineCleanup;
use super::input_physical_sampling_native_driver::VaachakInputPhysicalSamplingNativeDriver;
use super::physical_driver_migration_plan::VaachakPhysicalDriverMigrationPlan;

/// Final cleanup checkpoint for the Vaachak-native input physical sampling driver.
///
/// This checkpoint folds the accepted input physical sampling driver into the
/// canonical physical-driver migration stack. It intentionally does not move the
/// ADC peripheral read executor, GPIO polling executor, final app navigation
/// dispatch, display, storage, or SPI behavior.
pub struct VaachakInputPhysicalSamplingNativeDriverCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPhysicalSamplingNativeDriverCleanupReport {
    pub cleanup_marker: &'static str,
    pub physical_sampling_driver_accepted: bool,
    pub input_event_pipeline_cleanup_accepted: bool,
    pub physical_driver_migration_plan_accepted: bool,
    pub active_driver_name: &'static str,
    pub behavior_owner: &'static str,
    pub physical_read_fallback_name: &'static str,
    pub adc_ladder_interpretation_consolidated: bool,
    pub oversample_reduction_consolidated: bool,
    pub power_gpio_level_interpretation_consolidated: bool,
    pub native_event_pipeline_handoff_consolidated: bool,
    pub adc_peripheral_read_executor_moved_to_vaachak: bool,
    pub gpio_poll_executor_moved_to_vaachak: bool,
    pub final_app_navigation_dispatch_changed: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub spi_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub cleanup_artifacts_script_declared: bool,
    pub canonical_cleanup_doc_declared: bool,
}

impl VaachakInputPhysicalSamplingNativeDriverCleanupReport {
    pub const fn ok(self) -> bool {
        self.physical_sampling_driver_accepted
            && self.input_event_pipeline_cleanup_accepted
            && self.physical_driver_migration_plan_accepted
            && self.adc_ladder_interpretation_consolidated
            && self.oversample_reduction_consolidated
            && self.power_gpio_level_interpretation_consolidated
            && self.native_event_pipeline_handoff_consolidated
            && !self.adc_peripheral_read_executor_moved_to_vaachak
            && !self.gpio_poll_executor_moved_to_vaachak
            && !self.final_app_navigation_dispatch_changed
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.spi_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && self.cleanup_artifacts_script_declared
            && self.canonical_cleanup_doc_declared
    }
}

impl VaachakInputPhysicalSamplingNativeDriverCleanup {
    pub const INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_CLEANUP_MARKER: &'static str =
        "input_physical_sampling_native_driver_cleanup=ok";
    pub const ACTIVE_DRIVER_NAME: &'static str =
        "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback";
    pub const BEHAVIOR_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const PHYSICAL_READ_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const CLEANUP_ARTIFACTS_SCRIPT_DECLARED: bool = true;
    pub const CANONICAL_CLEANUP_DOC_DECLARED: bool = true;
    pub const FINAL_APP_NAVIGATION_DISPATCH_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub fn physical_sampling_driver_ok() -> bool {
        VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok()
    }

    pub fn input_event_pipeline_cleanup_ok() -> bool {
        VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()
    }

    pub fn physical_driver_migration_plan_ok() -> bool {
        VaachakPhysicalDriverMigrationPlan::migration_plan_ok()
    }

    pub fn report() -> VaachakInputPhysicalSamplingNativeDriverCleanupReport {
        let physical_report = VaachakInputPhysicalSamplingNativeDriver::report();

        VaachakInputPhysicalSamplingNativeDriverCleanupReport {
            cleanup_marker: Self::INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_CLEANUP_MARKER,
            physical_sampling_driver_accepted: Self::physical_sampling_driver_ok(),
            input_event_pipeline_cleanup_accepted: Self::input_event_pipeline_cleanup_ok(),
            physical_driver_migration_plan_accepted: Self::physical_driver_migration_plan_ok(),
            active_driver_name: Self::ACTIVE_DRIVER_NAME,
            behavior_owner: Self::BEHAVIOR_OWNER,
            physical_read_fallback_name: Self::PHYSICAL_READ_FALLBACK_NAME,
            adc_ladder_interpretation_consolidated: physical_report
                .raw_adc_ladder_sample_interpretation_moved_to_vaachak,
            oversample_reduction_consolidated: physical_report
                .oversample_reduction_moved_to_vaachak,
            power_gpio_level_interpretation_consolidated: physical_report
                .power_gpio_level_interpretation_moved_to_vaachak,
            native_event_pipeline_handoff_consolidated: physical_report.native_event_pipeline_ready,
            adc_peripheral_read_executor_moved_to_vaachak: physical_report
                .adc_peripheral_read_executor_moved_to_vaachak,
            gpio_poll_executor_moved_to_vaachak: physical_report
                .gpio_poll_executor_moved_to_vaachak,
            final_app_navigation_dispatch_changed: Self::FINAL_APP_NAVIGATION_DISPATCH_CHANGED,
            display_behavior_changed: physical_report.display_behavior_changed,
            storage_behavior_changed: physical_report.storage_behavior_changed,
            spi_behavior_changed: physical_report.spi_behavior_changed,
            reader_file_browser_ux_changed: physical_report.reader_file_browser_ux_changed,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            cleanup_artifacts_script_declared: Self::CLEANUP_ARTIFACTS_SCRIPT_DECLARED,
            canonical_cleanup_doc_declared: Self::CANONICAL_CLEANUP_DOC_DECLARED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_input_physical_sampling_native_driver_cleanup_marker() {
        if Self::cleanup_ok() {
            Self::emit_line(Self::INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_CLEANUP_MARKER);
            Self::emit_line("input.physical_sampling.native_driver.cleanup.accepted");
            Self::emit_line("input.physical_sampling.adc_ladder_interpretation.vaachak_owned");
            Self::emit_line("input.physical_sampling.oversample_reduction.vaachak_owned");
            Self::emit_line("input.physical_sampling.physical_reads.pulp_compatible");
            Self::emit_line("input.physical_sampling.cleanup.behavior.preserved");
        } else {
            Self::emit_line("input_physical_sampling_native_driver_cleanup=failed");
        }
    }

    #[cfg(any(target_arch = "riscv32", target_os = "none"))]
    fn emit_line(line: &str) {
        esp_println::println!("{}", line);
    }

    #[cfg(not(any(target_arch = "riscv32", target_os = "none")))]
    fn emit_line(line: &str) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputPhysicalSamplingNativeDriverCleanup;

    #[test]
    fn input_physical_sampling_native_driver_cleanup_is_ready() {
        assert!(VaachakInputPhysicalSamplingNativeDriverCleanup::cleanup_ok());
    }
}
