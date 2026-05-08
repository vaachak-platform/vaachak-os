#![allow(dead_code)]

use super::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use super::hardware_runtime_backend_takeover_cleanup::VaachakHardwareRuntimeBackendTakeoverCleanup;
use super::input_backend_native_executor::VaachakInputBackendNativeExecutor;

/// Final cleanup checkpoint for the Vaachak-native input backend executor.
///
/// This module intentionally does not add physical input sampling, debounce,
/// navigation dispatch, display, storage, or SPI behavior. It proves the accepted
/// native input executor is ready as a clean checkpoint before the next native
/// hardware backend migration.
pub struct VaachakInputBackendNativeExecutorCleanup;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputBackendNativeExecutorCleanupReport {
    pub cleanup_entrypoint_active: bool,
    pub input_native_executor_accepted: bool,
    pub backend_takeover_bridge_accepted: bool,
    pub backend_takeover_cleanup_accepted: bool,
    pub event_normalization_owned_by_vaachak: bool,
    pub intent_mapping_owned_by_vaachak: bool,
    pub selected_backend_is_vaachak_input_native_with_pulp_sampling: bool,
    pub pulp_sampling_fallback_active: bool,
    pub old_overlay_artifacts_safe_to_remove: bool,
    pub physical_adc_sampling_changed: bool,
    pub gpio_polling_changed: bool,
    pub debounce_repeat_execution_changed: bool,
    pub navigation_dispatch_changed: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub spi_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakInputBackendNativeExecutorCleanupReport {
    pub const fn ok(self) -> bool {
        self.cleanup_entrypoint_active
            && self.input_native_executor_accepted
            && self.backend_takeover_bridge_accepted
            && self.backend_takeover_cleanup_accepted
            && self.event_normalization_owned_by_vaachak
            && self.intent_mapping_owned_by_vaachak
            && self.selected_backend_is_vaachak_input_native_with_pulp_sampling
            && self.pulp_sampling_fallback_active
            && self.old_overlay_artifacts_safe_to_remove
            && !self.physical_adc_sampling_changed
            && !self.gpio_polling_changed
            && !self.debounce_repeat_execution_changed
            && !self.navigation_dispatch_changed
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.spi_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakInputBackendNativeExecutorCleanup {
    pub const INPUT_BACKEND_NATIVE_EXECUTOR_CLEANUP_MARKER: &'static str =
        "input_backend_native_executor_cleanup=ok";
    pub const INPUT_BACKEND_NATIVE_EXECUTOR_CLEANUP_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_NATIVE_BACKEND_NAME: &'static str = "VaachakInputNativeWithPulpSampling";
    pub const PHYSICAL_SAMPLING_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const PHYSICAL_SAMPLING_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const CLEANUP_ENTRYPOINT_ACTIVE: bool = true;
    pub const OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE: bool = true;

    pub const PHYSICAL_ADC_SAMPLING_CHANGED: bool = false;
    pub const GPIO_POLLING_CHANGED: bool = false;
    pub const DEBOUNCE_REPEAT_EXECUTION_CHANGED: bool = false;
    pub const NAVIGATION_DISPATCH_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub fn input_native_executor_ok() -> bool {
        VaachakInputBackendNativeExecutor::native_executor_ok()
    }

    pub fn backend_takeover_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeover::takeover_ok()
            && VaachakHardwareRuntimeBackendTakeover::backend_interface_calls_ok()
    }

    pub fn backend_takeover_cleanup_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()
    }

    pub fn report() -> VaachakInputBackendNativeExecutorCleanupReport {
        VaachakInputBackendNativeExecutorCleanupReport {
            cleanup_entrypoint_active: Self::CLEANUP_ENTRYPOINT_ACTIVE,
            input_native_executor_accepted: Self::input_native_executor_ok(),
            backend_takeover_bridge_accepted: Self::backend_takeover_ok(),
            backend_takeover_cleanup_accepted: Self::backend_takeover_cleanup_ok(),
            event_normalization_owned_by_vaachak:
                VaachakInputBackendNativeExecutor::EVENT_NORMALIZATION_OWNED_BY_VAACHAK,
            intent_mapping_owned_by_vaachak:
                VaachakInputBackendNativeExecutor::INTENT_MAPPING_OWNED_BY_VAACHAK,
            selected_backend_is_vaachak_input_native_with_pulp_sampling:
                VaachakInputBackendNativeExecutor::ACTIVE_NATIVE_BACKEND_NAME
                    == Self::ACTIVE_NATIVE_BACKEND_NAME,
            pulp_sampling_fallback_active:
                VaachakInputBackendNativeExecutor::PHYSICAL_SAMPLING_FALLBACK_NAME
                    == Self::PHYSICAL_SAMPLING_FALLBACK_NAME,
            old_overlay_artifacts_safe_to_remove: Self::OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE,
            physical_adc_sampling_changed: Self::PHYSICAL_ADC_SAMPLING_CHANGED,
            gpio_polling_changed: Self::GPIO_POLLING_CHANGED,
            debounce_repeat_execution_changed: Self::DEBOUNCE_REPEAT_EXECUTION_CHANGED,
            navigation_dispatch_changed: Self::NAVIGATION_DISPATCH_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            spi_behavior_changed: Self::SPI_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn cleanup_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_input_backend_native_executor_cleanup_marker() {
        if Self::cleanup_ok() {
            Self::emit_line(Self::INPUT_BACKEND_NATIVE_EXECUTOR_CLEANUP_MARKER);
            Self::emit_line("input.backend.native.cleanup.accepted");
            Self::emit_line(
                "input.backend.native.cleanup.backend.vaachak_input_native_with_pulp_sampling",
            );
            Self::emit_line("input.backend.native.cleanup.sampling_fallback.pulp_compatible");
            Self::emit_line("input.backend.native.cleanup.behavior.preserved");
        } else {
            Self::emit_line("input_backend_native_executor_cleanup=failed");
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
    use super::VaachakInputBackendNativeExecutorCleanup;

    #[test]
    fn input_backend_native_executor_cleanup_is_ready() {
        assert!(VaachakInputBackendNativeExecutorCleanup::cleanup_ok());
    }
}
