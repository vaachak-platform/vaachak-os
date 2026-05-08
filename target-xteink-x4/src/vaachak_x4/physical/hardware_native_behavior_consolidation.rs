use super::display_backend_native_refresh_command_executor::VaachakDisplayBackendNativeRefreshCommandExecutor;
use super::display_backend_native_refresh_command_executor_cleanup::VaachakDisplayBackendNativeRefreshCommandExecutorCleanup;
use super::input_backend_native_event_pipeline::VaachakInputBackendNativeEventPipeline;
use super::input_backend_native_event_pipeline_cleanup::VaachakInputBackendNativeEventPipelineCleanup;
use super::storage_backend_native_sd_mmc_fat_executor::VaachakStorageBackendNativeSdMmcFatExecutor;
use super::storage_backend_native_sd_mmc_fat_executor_cleanup::VaachakStorageBackendNativeSdMmcFatExecutorCleanup;

/// Canonical native behavior consolidation map for the hardware extraction path.
///
/// This is not another behavior migration. It records the accepted Vaachak-native
/// behavior moves that are now active above the Pulp-compatible low-level driver
/// layer:
///
/// - input event pipeline behavior
/// - display refresh command selection behavior
/// - storage SD/MMC/FAT command-decision behavior
///
/// Lower-level physical drivers remain intentionally Pulp-compatible until a
/// later native-driver migration.
pub struct VaachakHardwareNativeBehaviorConsolidation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareNativeBehaviorConsolidationReport {
    pub marker: &'static str,
    pub behavior_owner: &'static str,
    pub active_stack_name: &'static str,
    pub low_level_fallback_name: &'static str,
    pub input_event_pipeline_accepted: bool,
    pub display_refresh_command_executor_accepted: bool,
    pub storage_sd_mmc_fat_executor_accepted: bool,
    pub input_cleanup_accepted: bool,
    pub display_cleanup_accepted: bool,
    pub storage_cleanup_accepted: bool,
    pub input_event_pipeline_behavior_moved_to_vaachak: bool,
    pub display_refresh_command_behavior_moved_to_vaachak: bool,
    pub storage_sd_mmc_fat_command_behavior_moved_to_vaachak: bool,
    pub physical_adc_gpio_sampling_moved_to_vaachak: bool,
    pub ssd1677_draw_algorithm_moved_to_vaachak: bool,
    pub waveform_or_busy_wait_moved_to_vaachak: bool,
    pub low_level_sd_mmc_block_driver_moved_to_vaachak: bool,
    pub low_level_fat_algorithm_moved_to_vaachak: bool,
    pub physical_spi_transfer_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakHardwareNativeBehaviorConsolidationReport {
    pub const fn ok(self) -> bool {
        self.input_event_pipeline_accepted
            && self.display_refresh_command_executor_accepted
            && self.storage_sd_mmc_fat_executor_accepted
            && self.input_cleanup_accepted
            && self.display_cleanup_accepted
            && self.storage_cleanup_accepted
            && self.input_event_pipeline_behavior_moved_to_vaachak
            && self.display_refresh_command_behavior_moved_to_vaachak
            && self.storage_sd_mmc_fat_command_behavior_moved_to_vaachak
            && !self.physical_adc_gpio_sampling_moved_to_vaachak
            && !self.ssd1677_draw_algorithm_moved_to_vaachak
            && !self.waveform_or_busy_wait_moved_to_vaachak
            && !self.low_level_sd_mmc_block_driver_moved_to_vaachak
            && !self.low_level_fat_algorithm_moved_to_vaachak
            && !self.physical_spi_transfer_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakHardwareNativeBehaviorConsolidation {
    pub const HARDWARE_NATIVE_BEHAVIOR_CONSOLIDATION_MARKER: &'static str =
        "hardware_native_behavior_consolidation=ok";
    pub const BEHAVIOR_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_NATIVE_BEHAVIOR_STACK: &'static str =
        "InputEventPipeline+DisplayRefreshCommandExecutor+StorageSdMmcFatExecutor";
    pub const LOW_LEVEL_FALLBACK_NAME: &'static str = "PulpCompatibility";

    pub const INPUT_EVENT_PIPELINE_BEHAVIOR_MOVED_TO_VAACHAK: bool = true;
    pub const DISPLAY_REFRESH_COMMAND_BEHAVIOR_MOVED_TO_VAACHAK: bool = true;
    pub const STORAGE_SD_MMC_FAT_COMMAND_BEHAVIOR_MOVED_TO_VAACHAK: bool = true;

    pub const PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK: bool = false;
    pub const SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK: bool = false;
    pub const WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK: bool = false;
    pub const LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK: bool = false;
    pub const LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK: bool = false;
    pub const PHYSICAL_SPI_TRANSFER_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub fn input_event_pipeline_accepted() -> bool {
        VaachakInputBackendNativeEventPipeline::event_pipeline_ok()
    }

    pub fn display_refresh_command_executor_accepted() -> bool {
        VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok()
    }

    pub fn storage_sd_mmc_fat_executor_accepted() -> bool {
        VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok()
    }

    pub fn input_event_pipeline_cleanup_accepted() -> bool {
        VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()
    }

    pub fn display_refresh_command_executor_cleanup_accepted() -> bool {
        VaachakDisplayBackendNativeRefreshCommandExecutorCleanup::cleanup_ok()
    }

    pub fn storage_sd_mmc_fat_executor_cleanup_accepted() -> bool {
        VaachakStorageBackendNativeSdMmcFatExecutorCleanup::cleanup_ok()
    }

    pub fn report() -> VaachakHardwareNativeBehaviorConsolidationReport {
        VaachakHardwareNativeBehaviorConsolidationReport {
            marker: Self::HARDWARE_NATIVE_BEHAVIOR_CONSOLIDATION_MARKER,
            behavior_owner: Self::BEHAVIOR_OWNER,
            active_stack_name: Self::ACTIVE_NATIVE_BEHAVIOR_STACK,
            low_level_fallback_name: Self::LOW_LEVEL_FALLBACK_NAME,
            input_event_pipeline_accepted: Self::input_event_pipeline_accepted(),
            display_refresh_command_executor_accepted:
                Self::display_refresh_command_executor_accepted(),
            storage_sd_mmc_fat_executor_accepted: Self::storage_sd_mmc_fat_executor_accepted(),
            input_cleanup_accepted: Self::input_event_pipeline_cleanup_accepted(),
            display_cleanup_accepted: Self::display_refresh_command_executor_cleanup_accepted(),
            storage_cleanup_accepted: Self::storage_sd_mmc_fat_executor_cleanup_accepted(),
            input_event_pipeline_behavior_moved_to_vaachak:
                Self::INPUT_EVENT_PIPELINE_BEHAVIOR_MOVED_TO_VAACHAK,
            display_refresh_command_behavior_moved_to_vaachak:
                Self::DISPLAY_REFRESH_COMMAND_BEHAVIOR_MOVED_TO_VAACHAK,
            storage_sd_mmc_fat_command_behavior_moved_to_vaachak:
                Self::STORAGE_SD_MMC_FAT_COMMAND_BEHAVIOR_MOVED_TO_VAACHAK,
            physical_adc_gpio_sampling_moved_to_vaachak:
                Self::PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK
                    || VaachakInputBackendNativeEventPipeline::PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK,
            ssd1677_draw_algorithm_moved_to_vaachak:
                Self::SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK
                    || VaachakDisplayBackendNativeRefreshCommandExecutor::SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK,
            waveform_or_busy_wait_moved_to_vaachak:
                Self::WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK
                    || VaachakDisplayBackendNativeRefreshCommandExecutor::WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK,
            low_level_sd_mmc_block_driver_moved_to_vaachak:
                Self::LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK
                    || VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK,
            low_level_fat_algorithm_moved_to_vaachak: Self::LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK
                || VaachakStorageBackendNativeSdMmcFatExecutor::LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK,
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED
                || VaachakStorageBackendNativeSdMmcFatExecutor::PHYSICAL_SPI_TRANSFER_CHANGED
                || VaachakDisplayBackendNativeRefreshCommandExecutor::SPI_TRANSFER_OR_CHIP_SELECT_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED
                || VaachakInputBackendNativeEventPipeline::READER_FILE_BROWSER_UX_CHANGED
                || VaachakDisplayBackendNativeRefreshCommandExecutor::READER_FILE_BROWSER_UX_CHANGED
                || VaachakStorageBackendNativeSdMmcFatExecutor::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED
                || VaachakInputBackendNativeEventPipeline::FINAL_APP_NAVIGATION_DISPATCH_CHANGED
                || VaachakDisplayBackendNativeRefreshCommandExecutor::APP_NAVIGATION_BEHAVIOR_CHANGED
                || VaachakStorageBackendNativeSdMmcFatExecutor::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn native_behavior_consolidation_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_hardware_native_behavior_consolidation_marker() {
        if Self::native_behavior_consolidation_ok() {
            Self::emit_line(Self::HARDWARE_NATIVE_BEHAVIOR_CONSOLIDATION_MARKER);
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
    use super::VaachakHardwareNativeBehaviorConsolidation;

    #[test]
    fn hardware_native_behavior_consolidation_is_ready() {
        assert!(VaachakHardwareNativeBehaviorConsolidation::native_behavior_consolidation_ok());
    }
}
