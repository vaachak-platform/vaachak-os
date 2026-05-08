#![allow(dead_code)]

use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;
use super::input_runtime_owner::{VaachakInputRuntimeOwner, VaachakRuntimeInputButton};

/// Vaachak-native input backend executor shell.
///
/// Vaachak now owns input event normalization and intent mapping. Physical ADC
/// ladder sampling, debounce/repeat execution, and shell navigation dispatch
/// remain available through the existing Pulp-compatible implementation.
pub struct VaachakInputBackendNativeExecutor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputNativeBackend {
    VaachakInputNativeWithPulpSampling,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputNativeSamplingFallback {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputNativeOperation {
    ButtonEventNormalization,
    IntentMapping,
    ScanHandoff,
    NavigationHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakNativeInputEvent {
    NoInput,
    RightPressed,
    LeftPressed,
    ConfirmPressed,
    BackPressed,
    VolDownPressed,
    VolUpPressed,
    PowerPressed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakNativeInputIntent {
    Noop,
    MoveNext,
    MovePrevious,
    Select,
    Back,
    VolumeDown,
    VolumeUp,
    PowerHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputNativeRoute {
    pub source_button: VaachakRuntimeInputButton,
    pub normalized_event: VaachakNativeInputEvent,
    pub mapped_intent: VaachakNativeInputIntent,
    pub native_backend: VaachakInputNativeBackend,
    pub physical_sampling_fallback: VaachakInputNativeSamplingFallback,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub event_normalization_owned_by_vaachak: bool,
    pub intent_mapping_owned_by_vaachak: bool,
    pub physical_adc_sampling_moved_to_vaachak: bool,
    pub debounce_navigation_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputNativeExecutorReport {
    pub operation: VaachakInputNativeOperation,
    pub active_native_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub physical_sampling_fallback_name: &'static str,
    pub physical_sampling_executor_owner: &'static str,
    pub event_normalization_owned_by_vaachak: bool,
    pub intent_mapping_owned_by_vaachak: bool,
    pub pulp_sampling_fallback_available: bool,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub physical_adc_sampling_moved_to_vaachak: bool,
    pub debounce_navigation_behavior_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub spi_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
}

impl VaachakInputNativeRoute {
    pub const fn ok(self) -> bool {
        self.event_normalization_owned_by_vaachak
            && self.intent_mapping_owned_by_vaachak
            && matches!(
                self.native_backend,
                VaachakInputNativeBackend::VaachakInputNativeWithPulpSampling
            )
            && matches!(
                self.physical_sampling_fallback,
                VaachakInputNativeSamplingFallback::PulpCompatibility
            )
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && !self.physical_adc_sampling_moved_to_vaachak
            && !self.debounce_navigation_behavior_changed
    }
}

impl VaachakInputNativeExecutorReport {
    pub const fn ok(self) -> bool {
        self.event_normalization_owned_by_vaachak
            && self.intent_mapping_owned_by_vaachak
            && self.pulp_sampling_fallback_available
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && !self.physical_adc_sampling_moved_to_vaachak
            && !self.debounce_navigation_behavior_changed
            && !self.app_navigation_behavior_changed
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.spi_behavior_changed
            && !self.reader_file_browser_ux_changed
    }
}

impl VaachakInputBackendNativeExecutor {
    pub const INPUT_BACKEND_NATIVE_EXECUTOR_MARKER: &'static str =
        "input_backend_native_executor=ok";
    pub const ACTIVE_NATIVE_BACKEND_NAME: &'static str = "VaachakInputNativeWithPulpSampling";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const PHYSICAL_SAMPLING_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const PHYSICAL_SAMPLING_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const EVENT_NORMALIZATION_OWNED_BY_VAACHAK: bool = true;
    pub const INTENT_MAPPING_OWNED_BY_VAACHAK: bool = true;
    pub const PULP_SAMPLING_FALLBACK_AVAILABLE: bool = true;

    pub const PHYSICAL_ADC_SAMPLING_MOVED_TO_VAACHAK: bool = false;
    pub const DEBOUNCE_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;

    pub const fn normalize_button(button: VaachakRuntimeInputButton) -> VaachakNativeInputEvent {
        match button {
            VaachakRuntimeInputButton::Right => VaachakNativeInputEvent::RightPressed,
            VaachakRuntimeInputButton::Left => VaachakNativeInputEvent::LeftPressed,
            VaachakRuntimeInputButton::Confirm => VaachakNativeInputEvent::ConfirmPressed,
            VaachakRuntimeInputButton::Back => VaachakNativeInputEvent::BackPressed,
            VaachakRuntimeInputButton::VolDown => VaachakNativeInputEvent::VolDownPressed,
            VaachakRuntimeInputButton::VolUp => VaachakNativeInputEvent::VolUpPressed,
            VaachakRuntimeInputButton::Power => VaachakNativeInputEvent::PowerPressed,
        }
    }

    pub const fn map_event_to_intent(event: VaachakNativeInputEvent) -> VaachakNativeInputIntent {
        match event {
            VaachakNativeInputEvent::NoInput => VaachakNativeInputIntent::Noop,
            VaachakNativeInputEvent::RightPressed => VaachakNativeInputIntent::MoveNext,
            VaachakNativeInputEvent::LeftPressed => VaachakNativeInputIntent::MovePrevious,
            VaachakNativeInputEvent::ConfirmPressed => VaachakNativeInputIntent::Select,
            VaachakNativeInputEvent::BackPressed => VaachakNativeInputIntent::Back,
            VaachakNativeInputEvent::VolDownPressed => VaachakNativeInputIntent::VolumeDown,
            VaachakNativeInputEvent::VolUpPressed => VaachakNativeInputIntent::VolumeUp,
            VaachakNativeInputEvent::PowerPressed => VaachakNativeInputIntent::PowerHandoff,
        }
    }

    pub const fn route_button(button: VaachakRuntimeInputButton) -> VaachakInputNativeRoute {
        let normalized_event = Self::normalize_button(button);
        VaachakInputNativeRoute {
            source_button: button,
            normalized_event,
            mapped_intent: Self::map_event_to_intent(normalized_event),
            native_backend: VaachakInputNativeBackend::VaachakInputNativeWithPulpSampling,
            physical_sampling_fallback: VaachakInputNativeSamplingFallback::PulpCompatibility,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            event_normalization_owned_by_vaachak: Self::EVENT_NORMALIZATION_OWNED_BY_VAACHAK,
            intent_mapping_owned_by_vaachak: Self::INTENT_MAPPING_OWNED_BY_VAACHAK,
            physical_adc_sampling_moved_to_vaachak: Self::PHYSICAL_ADC_SAMPLING_MOVED_TO_VAACHAK,
            debounce_navigation_behavior_changed: Self::DEBOUNCE_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub const fn report(
        operation: VaachakInputNativeOperation,
    ) -> VaachakInputNativeExecutorReport {
        VaachakInputNativeExecutorReport {
            operation,
            active_native_backend_name: Self::ACTIVE_NATIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            physical_sampling_fallback_name: Self::PHYSICAL_SAMPLING_FALLBACK_NAME,
            physical_sampling_executor_owner: Self::PHYSICAL_SAMPLING_EXECUTOR_OWNER,
            event_normalization_owned_by_vaachak: Self::EVENT_NORMALIZATION_OWNED_BY_VAACHAK,
            intent_mapping_owned_by_vaachak: Self::INTENT_MAPPING_OWNED_BY_VAACHAK,
            pulp_sampling_fallback_available: Self::PULP_SAMPLING_FALLBACK_AVAILABLE,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            physical_adc_sampling_moved_to_vaachak: Self::PHYSICAL_ADC_SAMPLING_MOVED_TO_VAACHAK,
            debounce_navigation_behavior_changed: Self::DEBOUNCE_NAVIGATION_BEHAVIOR_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            spi_behavior_changed: Self::SPI_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
        }
    }

    pub fn execute_scan_handoff() -> VaachakInputNativeExecutorReport {
        Self::report(VaachakInputNativeOperation::ScanHandoff)
    }

    pub fn execute_navigation_handoff() -> VaachakInputNativeExecutorReport {
        Self::report(VaachakInputNativeOperation::NavigationHandoff)
    }

    pub const fn native_mappings_ok() -> bool {
        matches!(
            Self::route_button(VaachakRuntimeInputButton::Right).mapped_intent,
            VaachakNativeInputIntent::MoveNext
        ) && matches!(
            Self::route_button(VaachakRuntimeInputButton::Left).mapped_intent,
            VaachakNativeInputIntent::MovePrevious
        ) && matches!(
            Self::route_button(VaachakRuntimeInputButton::Confirm).mapped_intent,
            VaachakNativeInputIntent::Select
        ) && matches!(
            Self::route_button(VaachakRuntimeInputButton::Back).mapped_intent,
            VaachakNativeInputIntent::Back
        ) && matches!(
            Self::route_button(VaachakRuntimeInputButton::VolDown).mapped_intent,
            VaachakNativeInputIntent::VolumeDown
        ) && matches!(
            Self::route_button(VaachakRuntimeInputButton::VolUp).mapped_intent,
            VaachakNativeInputIntent::VolumeUp
        ) && matches!(
            Self::route_button(VaachakRuntimeInputButton::Power).mapped_intent,
            VaachakNativeInputIntent::PowerHandoff
        )
    }

    pub const fn route_safety_ok() -> bool {
        Self::route_button(VaachakRuntimeInputButton::Right).ok()
            && Self::route_button(VaachakRuntimeInputButton::Left).ok()
            && Self::route_button(VaachakRuntimeInputButton::Confirm).ok()
            && Self::route_button(VaachakRuntimeInputButton::Back).ok()
            && Self::route_button(VaachakRuntimeInputButton::VolDown).ok()
            && Self::route_button(VaachakRuntimeInputButton::VolUp).ok()
            && Self::route_button(VaachakRuntimeInputButton::Power).ok()
    }

    pub fn native_executor_ok() -> bool {
        VaachakInputRuntimeOwner::ownership_ok()
            && VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok()
            && Self::native_mappings_ok()
            && Self::route_safety_ok()
            && Self::execute_scan_handoff().ok()
            && Self::execute_navigation_handoff().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakInputBackendNativeExecutor, VaachakNativeInputIntent};
    use crate::vaachak_x4::physical::input_runtime_owner::VaachakRuntimeInputButton;

    #[test]
    fn input_backend_native_executor_is_ready() {
        assert!(VaachakInputBackendNativeExecutor::native_executor_ok());
    }

    #[test]
    fn button_intent_mapping_is_vaachak_owned() {
        assert_eq!(
            VaachakInputBackendNativeExecutor::route_button(VaachakRuntimeInputButton::Right)
                .mapped_intent,
            VaachakNativeInputIntent::MoveNext
        );
        assert_eq!(
            VaachakInputBackendNativeExecutor::route_button(VaachakRuntimeInputButton::Back)
                .mapped_intent,
            VaachakNativeInputIntent::Back
        );
    }
}
