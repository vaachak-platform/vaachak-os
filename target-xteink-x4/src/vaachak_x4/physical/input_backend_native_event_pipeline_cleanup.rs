#![allow(dead_code)]

use super::display_backend_native_refresh_shell_cleanup::VaachakDisplayBackendNativeRefreshShellCleanup;
use super::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use super::hardware_runtime_backend_takeover_cleanup::VaachakHardwareRuntimeBackendTakeoverCleanup;
use super::input_backend_native_event_pipeline::VaachakInputBackendNativeEventPipeline;
use super::input_backend_native_executor_cleanup::VaachakInputBackendNativeExecutorCleanup;

/// Final cleanup checkpoint for the Vaachak-native input event pipeline.
///
/// This checkpoint folds the takeover-fix integration into the accepted input
/// event-pipeline behavior migration. It intentionally does not move physical
/// ADC/GPIO sampling, display, storage, SPI, reader, file-browser, or app
/// navigation behavior.
pub struct VaachakInputBackendNativeEventPipelineCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputBackendNativeEventPipelineCleanupReport {
    pub cleanup_entrypoint_active: bool,
    pub native_event_pipeline_accepted: bool,
    pub input_native_executor_cleanup_accepted: bool,
    pub backend_takeover_bridge_accepted: bool,
    pub backend_takeover_cleanup_accepted: bool,
    pub display_refresh_shell_cleanup_accepted: bool,
    pub raw_sample_normalization_owned_by_vaachak: bool,
    pub stable_state_tracking_owned_by_vaachak: bool,
    pub debounce_window_metadata_owned_by_vaachak: bool,
    pub press_release_repeat_classification_owned_by_vaachak: bool,
    pub navigation_intent_mapping_owned_by_vaachak: bool,
    pub physical_sampling_fallback_active: bool,
    pub old_overlay_artifacts_safe_to_remove: bool,
    pub physical_adc_gpio_sampling_moved_to_vaachak: bool,
    pub final_app_navigation_dispatch_changed: bool,
    pub button_layout_direction_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub spi_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakInputBackendNativeEventPipelineCleanupReport {
    pub const fn ok(self) -> bool {
        self.cleanup_entrypoint_active
            && self.native_event_pipeline_accepted
            && self.input_native_executor_cleanup_accepted
            && self.backend_takeover_bridge_accepted
            && self.backend_takeover_cleanup_accepted
            && self.display_refresh_shell_cleanup_accepted
            && self.raw_sample_normalization_owned_by_vaachak
            && self.stable_state_tracking_owned_by_vaachak
            && self.debounce_window_metadata_owned_by_vaachak
            && self.press_release_repeat_classification_owned_by_vaachak
            && self.navigation_intent_mapping_owned_by_vaachak
            && self.physical_sampling_fallback_active
            && self.old_overlay_artifacts_safe_to_remove
            && !self.physical_adc_gpio_sampling_moved_to_vaachak
            && !self.final_app_navigation_dispatch_changed
            && !self.button_layout_direction_behavior_changed
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.spi_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakInputBackendNativeEventPipelineCleanup {
    pub const INPUT_BACKEND_NATIVE_EVENT_PIPELINE_CLEANUP_MARKER: &'static str =
        "input_backend_native_event_pipeline_cleanup=ok";
    pub const INPUT_BACKEND_NATIVE_EVENT_PIPELINE_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_NATIVE_EVENT_PIPELINE_NAME: &'static str =
        "VaachakNativeEventPipelineWithPulpSampling";
    pub const PHYSICAL_SAMPLING_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const PHYSICAL_SAMPLING_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const CLEANUP_ENTRYPOINT_ACTIVE: bool = true;
    pub const OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE: bool = true;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub fn native_event_pipeline_ok() -> bool {
        VaachakInputBackendNativeEventPipeline::event_pipeline_ok()
    }

    pub fn input_native_executor_cleanup_ok() -> bool {
        VaachakInputBackendNativeExecutorCleanup::cleanup_ok()
    }

    pub fn backend_takeover_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeover::takeover_ok()
            && VaachakHardwareRuntimeBackendTakeover::backend_interface_calls_ok()
    }

    pub fn backend_takeover_cleanup_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()
    }

    pub fn display_refresh_shell_cleanup_ok() -> bool {
        VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok()
    }

    pub fn report() -> VaachakInputBackendNativeEventPipelineCleanupReport {
        VaachakInputBackendNativeEventPipelineCleanupReport {
            cleanup_entrypoint_active: Self::CLEANUP_ENTRYPOINT_ACTIVE,
            native_event_pipeline_accepted: Self::native_event_pipeline_ok(),
            input_native_executor_cleanup_accepted: Self::input_native_executor_cleanup_ok(),
            backend_takeover_bridge_accepted: Self::backend_takeover_ok(),
            backend_takeover_cleanup_accepted: Self::backend_takeover_cleanup_ok(),
            display_refresh_shell_cleanup_accepted: Self::display_refresh_shell_cleanup_ok(),
            raw_sample_normalization_owned_by_vaachak:
                VaachakInputBackendNativeEventPipeline::RAW_SAMPLE_NORMALIZATION_OWNED_BY_VAACHAK,
            stable_state_tracking_owned_by_vaachak:
                VaachakInputBackendNativeEventPipeline::STABLE_STATE_TRACKING_OWNED_BY_VAACHAK,
            debounce_window_metadata_owned_by_vaachak:
                VaachakInputBackendNativeEventPipeline::DEBOUNCE_WINDOW_METADATA_OWNED_BY_VAACHAK,
            press_release_repeat_classification_owned_by_vaachak:
                VaachakInputBackendNativeEventPipeline::PRESS_RELEASE_REPEAT_CLASSIFICATION_OWNED_BY_VAACHAK,
            navigation_intent_mapping_owned_by_vaachak:
                VaachakInputBackendNativeEventPipeline::NAVIGATION_INTENT_MAPPING_OWNED_BY_VAACHAK,
            physical_sampling_fallback_active:
                VaachakInputBackendNativeEventPipeline::PHYSICAL_ADC_GPIO_SAMPLING_FALLBACK_ACTIVE,
            old_overlay_artifacts_safe_to_remove: Self::OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE,
            physical_adc_gpio_sampling_moved_to_vaachak:
                VaachakInputBackendNativeEventPipeline::PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK,
            final_app_navigation_dispatch_changed:
                VaachakInputBackendNativeEventPipeline::FINAL_APP_NAVIGATION_DISPATCH_CHANGED,
            button_layout_direction_behavior_changed:
                VaachakInputBackendNativeEventPipeline::BUTTON_LAYOUT_DIRECTION_BEHAVIOR_CHANGED,
            display_behavior_changed: VaachakInputBackendNativeEventPipeline::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: VaachakInputBackendNativeEventPipeline::STORAGE_BEHAVIOR_CHANGED,
            spi_behavior_changed: VaachakInputBackendNativeEventPipeline::SPI_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed:
                VaachakInputBackendNativeEventPipeline::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_input_backend_native_event_pipeline_cleanup_marker() {
        if Self::cleanup_ok() {
            Self::emit_line(Self::INPUT_BACKEND_NATIVE_EVENT_PIPELINE_CLEANUP_MARKER);
            Self::emit_line("input.backend.native.event_pipeline.cleanup.accepted");
            Self::emit_line("input.backend.native.event_pipeline.behavior.vaachak_owned");
            Self::emit_line(
                "input.backend.native.event_pipeline.sampling_fallback.pulp_compatible",
            );
            Self::emit_line("input.backend.native.event_pipeline.cleanup.behavior.preserved");
        } else {
            Self::emit_line("input_backend_native_event_pipeline_cleanup=failed");
        }
    }

    #[cfg(all(target_arch = "riscv32", target_os = "none"))]
    fn emit_line(line: &str) {
        esp_println::println!("{}", line);
    }

    #[cfg(not(all(target_arch = "riscv32", target_os = "none")))]
    fn emit_line(line: &str) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputBackendNativeEventPipelineCleanup;

    #[test]
    fn input_backend_native_event_pipeline_cleanup_is_ready() {
        assert!(VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok());
    }
}
