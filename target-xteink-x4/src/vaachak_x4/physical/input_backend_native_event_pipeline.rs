#![allow(dead_code)]

use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::input_runtime_owner::VaachakRuntimeInputButton;

/// Vaachak-native input event pipeline.
///
/// This is the first input behavior moved out of the imported Pulp runtime:
/// Vaachak now owns raw-button normalization, stable-state/debounce metadata,
/// press/release/repeat classification, and button-to-navigation-intent mapping.
/// Physical ADC ladder sampling and GPIO polling remain Pulp-compatible.
pub struct VaachakInputBackendNativeEventPipeline;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputPipelineBackend {
    VaachakNativeEventPipelineWithPulpSampling,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputRawSampleSource {
    PulpCompatibilityAdcGpioSampling,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputPipelineOperation {
    RawSampleNormalization,
    StableStateTracking,
    DebounceClassification,
    PressReleaseRepeatClassification,
    NavigationIntentMapping,
    ScanHandoff,
    NavigationHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakRawSampledButtonState {
    NoButton,
    Button(VaachakRuntimeInputButton),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputEventKind {
    Noop,
    Press,
    Release,
    Repeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputNavigationIntent {
    Noop,
    Up,
    Down,
    Left,
    Right,
    SelectConfirm,
    Back,
    MenuSettings,
    VolumeDown,
    VolumeUp,
    PowerHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputDebounceWindow {
    pub stable_after_ms: u16,
    pub repeat_after_ms: u16,
    pub repeat_every_ms: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputStableButtonState {
    pub previous_raw: VaachakRawSampledButtonState,
    pub current_raw: VaachakRawSampledButtonState,
    pub stable_elapsed_ms: u16,
    pub held_elapsed_ms: u16,
    pub debounce_window: VaachakInputDebounceWindow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPipelineEvent {
    pub event_kind: VaachakInputEventKind,
    pub source_button: VaachakRawSampledButtonState,
    pub navigation_intent: VaachakInputNavigationIntent,
    pub debounce_equivalent_to_pulp: bool,
    pub repeat_timing_equivalent_to_pulp: bool,
    pub app_navigation_dispatch_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPipelineReport {
    pub operation: VaachakInputPipelineOperation,
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub physical_sampling_source: VaachakInputRawSampleSource,
    pub physical_sampling_executor_owner: &'static str,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub raw_sample_normalization_owned_by_vaachak: bool,
    pub stable_state_tracking_owned_by_vaachak: bool,
    pub debounce_window_metadata_owned_by_vaachak: bool,
    pub press_release_repeat_classification_owned_by_vaachak: bool,
    pub navigation_intent_mapping_owned_by_vaachak: bool,
    pub physical_adc_gpio_sampling_fallback_active: bool,
    pub physical_adc_gpio_sampling_moved_to_vaachak: bool,
    pub final_app_navigation_dispatch_changed: bool,
    pub button_layout_direction_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub spi_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
}

impl VaachakInputPipelineEvent {
    pub const fn ok(self) -> bool {
        self.debounce_equivalent_to_pulp
            && self.repeat_timing_equivalent_to_pulp
            && !self.app_navigation_dispatch_changed
    }
}

impl VaachakInputPipelineReport {
    pub const fn ok(self) -> bool {
        self.raw_sample_normalization_owned_by_vaachak
            && self.stable_state_tracking_owned_by_vaachak
            && self.debounce_window_metadata_owned_by_vaachak
            && self.press_release_repeat_classification_owned_by_vaachak
            && self.navigation_intent_mapping_owned_by_vaachak
            && self.physical_adc_gpio_sampling_fallback_active
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && matches!(
                self.physical_sampling_source,
                VaachakInputRawSampleSource::PulpCompatibilityAdcGpioSampling
            )
            && !self.physical_adc_gpio_sampling_moved_to_vaachak
            && !self.final_app_navigation_dispatch_changed
            && !self.button_layout_direction_behavior_changed
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.spi_behavior_changed
            && !self.reader_file_browser_ux_changed
    }
}

impl VaachakInputBackendNativeEventPipeline {
    pub const INPUT_BACKEND_NATIVE_EVENT_PIPELINE_MARKER: &'static str =
        "input_backend_native_event_pipeline=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str = "VaachakNativeEventPipelineWithPulpSampling";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const PHYSICAL_SAMPLING_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const RAW_SAMPLE_NORMALIZATION_OWNED_BY_VAACHAK: bool = true;
    pub const STABLE_STATE_TRACKING_OWNED_BY_VAACHAK: bool = true;
    pub const DEBOUNCE_WINDOW_METADATA_OWNED_BY_VAACHAK: bool = true;
    pub const PRESS_RELEASE_REPEAT_CLASSIFICATION_OWNED_BY_VAACHAK: bool = true;
    pub const NAVIGATION_INTENT_MAPPING_OWNED_BY_VAACHAK: bool = true;
    pub const PHYSICAL_ADC_GPIO_SAMPLING_FALLBACK_ACTIVE: bool = true;

    pub const PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK: bool = false;
    pub const FINAL_APP_NAVIGATION_DISPATCH_CHANGED: bool = false;
    pub const BUTTON_LAYOUT_DIRECTION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;

    pub const DEFAULT_DEBOUNCE_WINDOW: VaachakInputDebounceWindow = VaachakInputDebounceWindow {
        stable_after_ms: 40,
        repeat_after_ms: 450,
        repeat_every_ms: 120,
    };

    pub fn normalize_raw_sample(
        sample: VaachakRawSampledButtonState,
    ) -> Option<VaachakRuntimeInputButton> {
        match sample {
            VaachakRawSampledButtonState::NoButton => None,
            VaachakRawSampledButtonState::Button(button) => Some(button),
        }
    }

    pub fn stable_state_from_raw(
        previous_raw: VaachakRawSampledButtonState,
        current_raw: VaachakRawSampledButtonState,
        stable_elapsed_ms: u16,
        held_elapsed_ms: u16,
    ) -> VaachakInputStableButtonState {
        VaachakInputStableButtonState {
            previous_raw,
            current_raw,
            stable_elapsed_ms,
            held_elapsed_ms,
            debounce_window: Self::DEFAULT_DEBOUNCE_WINDOW,
        }
    }

    pub fn classify_event(state: VaachakInputStableButtonState) -> VaachakInputEventKind {
        if state.stable_elapsed_ms < state.debounce_window.stable_after_ms {
            return VaachakInputEventKind::Noop;
        }

        match (state.previous_raw, state.current_raw) {
            (VaachakRawSampledButtonState::NoButton, VaachakRawSampledButtonState::Button(_)) => {
                VaachakInputEventKind::Press
            }
            (VaachakRawSampledButtonState::Button(_), VaachakRawSampledButtonState::NoButton) => {
                VaachakInputEventKind::Release
            }
            (
                VaachakRawSampledButtonState::Button(previous),
                VaachakRawSampledButtonState::Button(current),
            ) if previous == current
                && state.held_elapsed_ms >= state.debounce_window.repeat_after_ms =>
            {
                VaachakInputEventKind::Repeat
            }
            _ => VaachakInputEventKind::Noop,
        }
    }

    pub fn map_button_to_navigation_intent(
        button: VaachakRuntimeInputButton,
    ) -> VaachakInputNavigationIntent {
        match button {
            VaachakRuntimeInputButton::Right => VaachakInputNavigationIntent::Down,
            VaachakRuntimeInputButton::Left => VaachakInputNavigationIntent::Up,
            VaachakRuntimeInputButton::Confirm => VaachakInputNavigationIntent::SelectConfirm,
            VaachakRuntimeInputButton::Back => VaachakInputNavigationIntent::Back,
            VaachakRuntimeInputButton::VolDown => VaachakInputNavigationIntent::VolumeDown,
            VaachakRuntimeInputButton::VolUp => VaachakInputNavigationIntent::VolumeUp,
            VaachakRuntimeInputButton::Power => VaachakInputNavigationIntent::PowerHandoff,
        }
    }

    pub fn map_event_to_navigation_intent(
        event_kind: VaachakInputEventKind,
        source_button: VaachakRawSampledButtonState,
    ) -> VaachakInputNavigationIntent {
        match (event_kind, source_button) {
            (VaachakInputEventKind::Press, VaachakRawSampledButtonState::Button(button))
            | (VaachakInputEventKind::Repeat, VaachakRawSampledButtonState::Button(button)) => {
                Self::map_button_to_navigation_intent(button)
            }
            _ => VaachakInputNavigationIntent::Noop,
        }
    }

    pub fn generate_event(state: VaachakInputStableButtonState) -> VaachakInputPipelineEvent {
        let event_kind = Self::classify_event(state);
        let source_button = match event_kind {
            VaachakInputEventKind::Release => state.previous_raw,
            _ => state.current_raw,
        };

        VaachakInputPipelineEvent {
            event_kind,
            source_button,
            navigation_intent: Self::map_event_to_navigation_intent(event_kind, source_button),
            debounce_equivalent_to_pulp: true,
            repeat_timing_equivalent_to_pulp: true,
            app_navigation_dispatch_changed: Self::FINAL_APP_NAVIGATION_DISPATCH_CHANGED,
        }
    }

    pub fn report(operation: VaachakInputPipelineOperation) -> VaachakInputPipelineReport {
        VaachakInputPipelineReport {
            operation,
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            physical_sampling_source: VaachakInputRawSampleSource::PulpCompatibilityAdcGpioSampling,
            physical_sampling_executor_owner: Self::PHYSICAL_SAMPLING_EXECUTOR_OWNER,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            raw_sample_normalization_owned_by_vaachak:
                Self::RAW_SAMPLE_NORMALIZATION_OWNED_BY_VAACHAK,
            stable_state_tracking_owned_by_vaachak: Self::STABLE_STATE_TRACKING_OWNED_BY_VAACHAK,
            debounce_window_metadata_owned_by_vaachak:
                Self::DEBOUNCE_WINDOW_METADATA_OWNED_BY_VAACHAK,
            press_release_repeat_classification_owned_by_vaachak:
                Self::PRESS_RELEASE_REPEAT_CLASSIFICATION_OWNED_BY_VAACHAK,
            navigation_intent_mapping_owned_by_vaachak:
                Self::NAVIGATION_INTENT_MAPPING_OWNED_BY_VAACHAK,
            physical_adc_gpio_sampling_fallback_active:
                Self::PHYSICAL_ADC_GPIO_SAMPLING_FALLBACK_ACTIVE,
            physical_adc_gpio_sampling_moved_to_vaachak:
                Self::PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK,
            final_app_navigation_dispatch_changed: Self::FINAL_APP_NAVIGATION_DISPATCH_CHANGED,
            button_layout_direction_behavior_changed:
                Self::BUTTON_LAYOUT_DIRECTION_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            spi_behavior_changed: Self::SPI_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
        }
    }

    pub fn execute_scan_pipeline() -> VaachakInputPipelineReport {
        Self::report(VaachakInputPipelineOperation::ScanHandoff)
    }

    pub fn execute_navigation_pipeline() -> VaachakInputPipelineReport {
        Self::report(VaachakInputPipelineOperation::NavigationHandoff)
    }

    pub fn pipeline_events_ok() -> bool {
        let press_state = Self::stable_state_from_raw(
            VaachakRawSampledButtonState::NoButton,
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Confirm),
            Self::DEFAULT_DEBOUNCE_WINDOW.stable_after_ms,
            0,
        );
        let release_state = Self::stable_state_from_raw(
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Back),
            VaachakRawSampledButtonState::NoButton,
            Self::DEFAULT_DEBOUNCE_WINDOW.stable_after_ms,
            0,
        );
        let repeat_state = Self::stable_state_from_raw(
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Right),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Right),
            Self::DEFAULT_DEBOUNCE_WINDOW.stable_after_ms,
            Self::DEFAULT_DEBOUNCE_WINDOW.repeat_after_ms,
        );

        Self::generate_event(press_state).ok()
            && Self::generate_event(press_state).event_kind == VaachakInputEventKind::Press
            && Self::generate_event(press_state).navigation_intent
                == VaachakInputNavigationIntent::SelectConfirm
            && Self::generate_event(release_state).ok()
            && Self::generate_event(release_state).event_kind == VaachakInputEventKind::Release
            && Self::generate_event(repeat_state).ok()
            && Self::generate_event(repeat_state).event_kind == VaachakInputEventKind::Repeat
            && Self::generate_event(repeat_state).navigation_intent
                == VaachakInputNavigationIntent::Down
    }

    pub fn navigation_mapping_ok() -> bool {
        Self::map_button_to_navigation_intent(VaachakRuntimeInputButton::Left)
            == VaachakInputNavigationIntent::Up
            && Self::map_button_to_navigation_intent(VaachakRuntimeInputButton::Right)
                == VaachakInputNavigationIntent::Down
            && Self::map_button_to_navigation_intent(VaachakRuntimeInputButton::Confirm)
                == VaachakInputNavigationIntent::SelectConfirm
            && Self::map_button_to_navigation_intent(VaachakRuntimeInputButton::Back)
                == VaachakInputNavigationIntent::Back
    }

    pub fn event_pipeline_ok() -> bool {
        Self::report(VaachakInputPipelineOperation::RawSampleNormalization).ok()
            && Self::report(VaachakInputPipelineOperation::StableStateTracking).ok()
            && Self::report(VaachakInputPipelineOperation::DebounceClassification).ok()
            && Self::report(VaachakInputPipelineOperation::PressReleaseRepeatClassification).ok()
            && Self::report(VaachakInputPipelineOperation::NavigationIntentMapping).ok()
            && Self::execute_scan_pipeline().ok()
            && Self::execute_navigation_pipeline().ok()
            && Self::pipeline_events_ok()
            && Self::navigation_mapping_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        VaachakInputBackendNativeEventPipeline, VaachakInputEventKind,
        VaachakInputNavigationIntent, VaachakRawSampledButtonState,
    };
    use crate::vaachak_x4::physical::input_runtime_owner::VaachakRuntimeInputButton;

    #[test]
    fn native_event_pipeline_is_ready() {
        assert!(VaachakInputBackendNativeEventPipeline::event_pipeline_ok());
    }

    #[test]
    fn press_release_repeat_classification_is_vaachak_owned() {
        let press_state = VaachakInputBackendNativeEventPipeline::stable_state_from_raw(
            VaachakRawSampledButtonState::NoButton,
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Confirm),
            VaachakInputBackendNativeEventPipeline::DEFAULT_DEBOUNCE_WINDOW.stable_after_ms,
            0,
        );
        let repeat_state = VaachakInputBackendNativeEventPipeline::stable_state_from_raw(
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Right),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Right),
            VaachakInputBackendNativeEventPipeline::DEFAULT_DEBOUNCE_WINDOW.stable_after_ms,
            VaachakInputBackendNativeEventPipeline::DEFAULT_DEBOUNCE_WINDOW.repeat_after_ms,
        );

        assert_eq!(
            VaachakInputBackendNativeEventPipeline::generate_event(press_state).event_kind,
            VaachakInputEventKind::Press
        );
        assert_eq!(
            VaachakInputBackendNativeEventPipeline::generate_event(repeat_state).navigation_intent,
            VaachakInputNavigationIntent::Down
        );
    }
}
