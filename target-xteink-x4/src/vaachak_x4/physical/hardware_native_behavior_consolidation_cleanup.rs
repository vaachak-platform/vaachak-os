use super::hardware_native_behavior_consolidation::VaachakHardwareNativeBehaviorConsolidation;

/// Cleanup checkpoint for the accepted Vaachak-native hardware behavior moves.
///
/// This module does not move additional hardware behavior. It records that the
/// accepted input, display, and storage native behavior migrations are now
/// consolidated and ready for commit. Lower-level physical drivers remain
/// intentionally Pulp-compatible until a later physical-driver migration.
pub struct VaachakHardwareNativeBehaviorConsolidationCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareNativeBehaviorConsolidationCleanupReport {
    pub cleanup_marker: &'static str,
    pub behavior_consolidation_ready: bool,
    pub behavior_owner: &'static str,
    pub active_native_behavior_stack: &'static str,
    pub low_level_fallback_name: &'static str,
    pub input_event_pipeline_behavior_consolidated: bool,
    pub display_refresh_command_behavior_consolidated: bool,
    pub storage_sd_mmc_fat_command_behavior_consolidated: bool,
    pub physical_adc_gpio_sampling_moved_to_vaachak: bool,
    pub ssd1677_draw_algorithm_moved_to_vaachak: bool,
    pub waveform_or_busy_wait_moved_to_vaachak: bool,
    pub low_level_sd_mmc_block_driver_moved_to_vaachak: bool,
    pub low_level_fat_algorithm_moved_to_vaachak: bool,
    pub physical_spi_transfer_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub cleanup_artifacts_script_declared: bool,
    pub canonical_cleanup_doc_declared: bool,
}

impl VaachakHardwareNativeBehaviorConsolidationCleanupReport {
    pub const fn ok(self) -> bool {
        self.behavior_consolidation_ready
            && self.input_event_pipeline_behavior_consolidated
            && self.display_refresh_command_behavior_consolidated
            && self.storage_sd_mmc_fat_command_behavior_consolidated
            && !self.physical_adc_gpio_sampling_moved_to_vaachak
            && !self.ssd1677_draw_algorithm_moved_to_vaachak
            && !self.waveform_or_busy_wait_moved_to_vaachak
            && !self.low_level_sd_mmc_block_driver_moved_to_vaachak
            && !self.low_level_fat_algorithm_moved_to_vaachak
            && !self.physical_spi_transfer_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && self.cleanup_artifacts_script_declared
            && self.canonical_cleanup_doc_declared
    }
}

impl VaachakHardwareNativeBehaviorConsolidationCleanup {
    pub const HARDWARE_NATIVE_BEHAVIOR_CONSOLIDATION_CLEANUP_MARKER: &'static str =
        "hardware_native_behavior_consolidation_cleanup=ok";
    pub const CLEANUP_ARTIFACTS_SCRIPT_DECLARED: bool = true;
    pub const CANONICAL_CLEANUP_DOC_DECLARED: bool = true;

    pub fn report() -> VaachakHardwareNativeBehaviorConsolidationCleanupReport {
        let consolidated = VaachakHardwareNativeBehaviorConsolidation::report();

        VaachakHardwareNativeBehaviorConsolidationCleanupReport {
            cleanup_marker: Self::HARDWARE_NATIVE_BEHAVIOR_CONSOLIDATION_CLEANUP_MARKER,
            behavior_consolidation_ready:
                VaachakHardwareNativeBehaviorConsolidation::native_behavior_consolidation_ok(),
            behavior_owner: consolidated.behavior_owner,
            active_native_behavior_stack: consolidated.active_stack_name,
            low_level_fallback_name: consolidated.low_level_fallback_name,
            input_event_pipeline_behavior_consolidated: consolidated
                .input_event_pipeline_behavior_moved_to_vaachak,
            display_refresh_command_behavior_consolidated: consolidated
                .display_refresh_command_behavior_moved_to_vaachak,
            storage_sd_mmc_fat_command_behavior_consolidated: consolidated
                .storage_sd_mmc_fat_command_behavior_moved_to_vaachak,
            physical_adc_gpio_sampling_moved_to_vaachak: consolidated
                .physical_adc_gpio_sampling_moved_to_vaachak,
            ssd1677_draw_algorithm_moved_to_vaachak: consolidated
                .ssd1677_draw_algorithm_moved_to_vaachak,
            waveform_or_busy_wait_moved_to_vaachak: consolidated
                .waveform_or_busy_wait_moved_to_vaachak,
            low_level_sd_mmc_block_driver_moved_to_vaachak: consolidated
                .low_level_sd_mmc_block_driver_moved_to_vaachak,
            low_level_fat_algorithm_moved_to_vaachak: consolidated
                .low_level_fat_algorithm_moved_to_vaachak,
            physical_spi_transfer_changed: consolidated.physical_spi_transfer_changed,
            reader_file_browser_ux_changed: consolidated.reader_file_browser_ux_changed,
            app_navigation_behavior_changed: consolidated.app_navigation_behavior_changed,
            cleanup_artifacts_script_declared: Self::CLEANUP_ARTIFACTS_SCRIPT_DECLARED,
            canonical_cleanup_doc_declared: Self::CANONICAL_CLEANUP_DOC_DECLARED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_hardware_native_behavior_consolidation_cleanup_marker() {
        if Self::cleanup_ok() {
            Self::emit_line(Self::HARDWARE_NATIVE_BEHAVIOR_CONSOLIDATION_CLEANUP_MARKER);
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
    use super::VaachakHardwareNativeBehaviorConsolidationCleanup;

    #[test]
    fn hardware_native_behavior_consolidation_cleanup_is_ready() {
        assert!(VaachakHardwareNativeBehaviorConsolidationCleanup::cleanup_ok());
    }
}
