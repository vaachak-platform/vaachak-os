use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::input_backend_native_event_pipeline::{
    VaachakInputBackendNativeEventPipeline, VaachakInputPipelineEvent,
    VaachakInputStableButtonState, VaachakRawSampledButtonState,
};
use super::input_runtime_owner::VaachakRuntimeInputButton;

/// Vaachak-native physical input sampling driver for Xteink X4.
///
/// This is the first lower-level physical driver behavior moved from the
/// imported Pulp runtime into the Vaachak target layer. Vaachak now owns raw
/// ADC ladder sample interpretation, oversample reduction, power-button GPIO
/// level interpretation, and conversion into the already-accepted native input
/// event pipeline. The actual ADC peripheral read and GPIO poll executor remain
/// Pulp-compatible fallback for this slice.
pub struct VaachakInputPhysicalSamplingNativeDriver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputPhysicalSamplingBackend {
    VaachakPhysicalSamplingWithPulpAdcGpioReadFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputPhysicalSampleSource {
    Row1Gpio1AdcLadder,
    Row2Gpio2AdcLadder,
    PowerGpio3DigitalLevel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputPhysicalSampleKind {
    AdcMillivolts,
    DigitalLevelLowActive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputAdcLadderBandNative {
    pub source: VaachakInputPhysicalSampleSource,
    pub center_mv: u16,
    pub tolerance_mv: u16,
    pub button: VaachakRuntimeInputButton,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPhysicalRawSample {
    pub source: VaachakInputPhysicalSampleSource,
    pub kind: VaachakInputPhysicalSampleKind,
    pub millivolts: u16,
    pub digital_low_active: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPhysicalOversampleWindow {
    pub source: VaachakInputPhysicalSampleSource,
    pub samples_mv: [u16; 4],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPhysicalSamplingResult {
    pub source: VaachakInputPhysicalSampleSource,
    pub averaged_mv: u16,
    pub raw_button_state: VaachakRawSampledButtonState,
    pub adc_ladder_interpretation_owned_by_vaachak: bool,
    pub oversample_reduction_owned_by_vaachak: bool,
    pub power_gpio_level_interpretation_owned_by_vaachak: bool,
    pub adc_peripheral_read_executor_moved_to_vaachak: bool,
    pub gpio_poll_executor_moved_to_vaachak: bool,
    pub low_level_backend: VaachakHardwareExecutorBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPhysicalSamplingReport {
    pub marker: &'static str,
    pub active_backend_name: &'static str,
    pub behavior_owner: &'static str,
    pub physical_read_fallback_name: &'static str,
    pub raw_adc_ladder_sample_interpretation_moved_to_vaachak: bool,
    pub oversample_reduction_moved_to_vaachak: bool,
    pub power_gpio_level_interpretation_moved_to_vaachak: bool,
    pub adc_peripheral_read_executor_moved_to_vaachak: bool,
    pub gpio_poll_executor_moved_to_vaachak: bool,
    pub native_event_pipeline_ready: bool,
    pub row1_classification_ok: bool,
    pub row2_classification_ok: bool,
    pub power_button_classification_ok: bool,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub spi_behavior_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakInputPhysicalSamplingResult {
    pub const fn ok(self) -> bool {
        self.adc_ladder_interpretation_owned_by_vaachak
            && self.oversample_reduction_owned_by_vaachak
            && self.power_gpio_level_interpretation_owned_by_vaachak
            && !self.adc_peripheral_read_executor_moved_to_vaachak
            && !self.gpio_poll_executor_moved_to_vaachak
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
    }
}

impl VaachakInputPhysicalSamplingReport {
    pub const fn ok(self) -> bool {
        self.raw_adc_ladder_sample_interpretation_moved_to_vaachak
            && self.oversample_reduction_moved_to_vaachak
            && self.power_gpio_level_interpretation_moved_to_vaachak
            && !self.adc_peripheral_read_executor_moved_to_vaachak
            && !self.gpio_poll_executor_moved_to_vaachak
            && self.native_event_pipeline_ready
            && self.row1_classification_ok
            && self.row2_classification_ok
            && self.power_button_classification_ok
            && matches!(
                self.low_level_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.spi_behavior_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakInputPhysicalSamplingNativeDriver {
    pub const INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_MARKER: &'static str =
        "input_physical_sampling_native_driver=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str =
        "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback";
    pub const BEHAVIOR_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const PHYSICAL_READ_FALLBACK_NAME: &'static str = "PulpCompatibility";
    pub const PHYSICAL_ADC_GPIO_READ_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const LOW_LEVEL_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const ROW1_ADC_GPIO: u8 = 1;
    pub const ROW2_ADC_GPIO: u8 = 2;
    pub const POWER_BUTTON_GPIO: u8 = 3;

    pub const OVERSAMPLE_COUNT: usize = 4;
    pub const LOW_RAIL_TOLERANCE_MV: u16 = 50;
    pub const DEFAULT_TOLERANCE_MV: u16 = 150;

    pub const ROW1_RIGHT_CENTER_MV: u16 = 3;
    pub const ROW1_LEFT_CENTER_MV: u16 = 1113;
    pub const ROW1_CONFIRM_CENTER_MV: u16 = 1984;
    pub const ROW1_BACK_CENTER_MV: u16 = 2556;
    pub const ROW2_VOLDOWN_CENTER_MV: u16 = 3;
    pub const ROW2_VOLUP_CENTER_MV: u16 = 1659;

    pub const RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK: bool = true;
    pub const OVERSAMPLE_REDUCTION_MOVED_TO_VAACHAK: bool = true;
    pub const POWER_GPIO_LEVEL_INTERPRETATION_MOVED_TO_VAACHAK: bool = true;
    pub const ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const GPIO_POLL_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub const LADDER_BANDS: [VaachakInputAdcLadderBandNative; 6] = [
        VaachakInputAdcLadderBandNative {
            source: VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
            center_mv: Self::ROW1_RIGHT_CENTER_MV,
            tolerance_mv: Self::LOW_RAIL_TOLERANCE_MV,
            button: VaachakRuntimeInputButton::Right,
        },
        VaachakInputAdcLadderBandNative {
            source: VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
            center_mv: Self::ROW1_LEFT_CENTER_MV,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakRuntimeInputButton::Left,
        },
        VaachakInputAdcLadderBandNative {
            source: VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
            center_mv: Self::ROW1_CONFIRM_CENTER_MV,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakRuntimeInputButton::Confirm,
        },
        VaachakInputAdcLadderBandNative {
            source: VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
            center_mv: Self::ROW1_BACK_CENTER_MV,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakRuntimeInputButton::Back,
        },
        VaachakInputAdcLadderBandNative {
            source: VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder,
            center_mv: Self::ROW2_VOLDOWN_CENTER_MV,
            tolerance_mv: Self::LOW_RAIL_TOLERANCE_MV,
            button: VaachakRuntimeInputButton::VolDown,
        },
        VaachakInputAdcLadderBandNative {
            source: VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder,
            center_mv: Self::ROW2_VOLUP_CENTER_MV,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakRuntimeInputButton::VolUp,
        },
    ];

    pub const fn gpio_for_source(source: VaachakInputPhysicalSampleSource) -> u8 {
        match source {
            VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder => Self::ROW1_ADC_GPIO,
            VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder => Self::ROW2_ADC_GPIO,
            VaachakInputPhysicalSampleSource::PowerGpio3DigitalLevel => Self::POWER_BUTTON_GPIO,
        }
    }

    pub const fn reduce_oversample_window(window: VaachakInputPhysicalOversampleWindow) -> u16 {
        let sum = window.samples_mv[0] as u32
            + window.samples_mv[1] as u32
            + window.samples_mv[2] as u32
            + window.samples_mv[3] as u32;
        (sum / Self::OVERSAMPLE_COUNT as u32) as u16
    }

    pub const fn classify_adc_ladder_mv(
        source: VaachakInputPhysicalSampleSource,
        millivolts: u16,
    ) -> VaachakRawSampledButtonState {
        let mut idx = 0;
        while idx < Self::LADDER_BANDS.len() {
            let band = Self::LADDER_BANDS[idx];
            if matches!(
                band.source,
                VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder
            ) == matches!(source, VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder)
                && matches!(
                    band.source,
                    VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder
                ) == matches!(source, VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder)
            {
                let low = band.center_mv.saturating_sub(band.tolerance_mv);
                let high = band.center_mv.saturating_add(band.tolerance_mv);
                if millivolts >= low && millivolts <= high {
                    return VaachakRawSampledButtonState::Button(band.button);
                }
            }
            idx += 1;
        }
        VaachakRawSampledButtonState::NoButton
    }

    pub const fn classify_power_gpio_level(
        digital_low_active: bool,
    ) -> VaachakRawSampledButtonState {
        if digital_low_active {
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Power)
        } else {
            VaachakRawSampledButtonState::NoButton
        }
    }

    pub const fn interpret_raw_sample(
        sample: VaachakInputPhysicalRawSample,
    ) -> VaachakInputPhysicalSamplingResult {
        let averaged_mv = sample.millivolts;
        let raw_button_state = match sample.source {
            VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder
            | VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder => {
                Self::classify_adc_ladder_mv(sample.source, averaged_mv)
            }
            VaachakInputPhysicalSampleSource::PowerGpio3DigitalLevel => {
                Self::classify_power_gpio_level(sample.digital_low_active)
            }
        };

        VaachakInputPhysicalSamplingResult {
            source: sample.source,
            averaged_mv,
            raw_button_state,
            adc_ladder_interpretation_owned_by_vaachak:
                Self::RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK,
            oversample_reduction_owned_by_vaachak: Self::OVERSAMPLE_REDUCTION_MOVED_TO_VAACHAK,
            power_gpio_level_interpretation_owned_by_vaachak:
                Self::POWER_GPIO_LEVEL_INTERPRETATION_MOVED_TO_VAACHAK,
            adc_peripheral_read_executor_moved_to_vaachak:
                Self::ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK,
            gpio_poll_executor_moved_to_vaachak: Self::GPIO_POLL_EXECUTOR_MOVED_TO_VAACHAK,
            low_level_backend: Self::LOW_LEVEL_BACKEND,
        }
    }

    pub const fn interpret_oversampled_adc_window(
        window: VaachakInputPhysicalOversampleWindow,
    ) -> VaachakInputPhysicalSamplingResult {
        Self::interpret_raw_sample(VaachakInputPhysicalRawSample {
            source: window.source,
            kind: VaachakInputPhysicalSampleKind::AdcMillivolts,
            millivolts: Self::reduce_oversample_window(window),
            digital_low_active: false,
        })
    }

    pub fn handoff_to_native_event_pipeline(
        previous_raw: VaachakRawSampledButtonState,
        sample: VaachakInputPhysicalSamplingResult,
        stable_elapsed_ms: u16,
        held_elapsed_ms: u16,
    ) -> VaachakInputPipelineEvent {
        let state: VaachakInputStableButtonState =
            VaachakInputBackendNativeEventPipeline::stable_state_from_raw(
                previous_raw,
                sample.raw_button_state,
                stable_elapsed_ms,
                held_elapsed_ms,
            );
        VaachakInputBackendNativeEventPipeline::generate_event(state)
    }

    pub fn report() -> VaachakInputPhysicalSamplingReport {
        VaachakInputPhysicalSamplingReport {
            marker: Self::INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_MARKER,
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            behavior_owner: Self::BEHAVIOR_OWNER,
            physical_read_fallback_name: Self::PHYSICAL_READ_FALLBACK_NAME,
            raw_adc_ladder_sample_interpretation_moved_to_vaachak:
                Self::RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK,
            oversample_reduction_moved_to_vaachak: Self::OVERSAMPLE_REDUCTION_MOVED_TO_VAACHAK,
            power_gpio_level_interpretation_moved_to_vaachak:
                Self::POWER_GPIO_LEVEL_INTERPRETATION_MOVED_TO_VAACHAK,
            adc_peripheral_read_executor_moved_to_vaachak:
                Self::ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK,
            gpio_poll_executor_moved_to_vaachak: Self::GPIO_POLL_EXECUTOR_MOVED_TO_VAACHAK,
            native_event_pipeline_ready: VaachakInputBackendNativeEventPipeline::event_pipeline_ok(
            ),
            row1_classification_ok: Self::row1_classification_ok(),
            row2_classification_ok: Self::row2_classification_ok(),
            power_button_classification_ok: Self::power_button_classification_ok(),
            low_level_backend: Self::LOW_LEVEL_BACKEND,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            spi_behavior_changed: Self::SPI_BEHAVIOR_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn native_physical_sampling_ok() -> bool {
        let row1 = Self::interpret_oversampled_adc_window(VaachakInputPhysicalOversampleWindow {
            source: VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
            samples_mv: [1980, 1984, 1986, 1986],
        });
        let event = Self::handoff_to_native_event_pipeline(
            VaachakRawSampledButtonState::NoButton,
            row1,
            40,
            40,
        );
        Self::report().ok()
            && row1.ok()
            && matches!(
                row1.raw_button_state,
                VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Confirm)
            )
            && event.ok()
    }

    fn row1_classification_ok() -> bool {
        matches!(
            Self::classify_adc_ladder_mv(
                VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
                Self::ROW1_RIGHT_CENTER_MV,
            ),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Right)
        ) && matches!(
            Self::classify_adc_ladder_mv(
                VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
                Self::ROW1_LEFT_CENTER_MV,
            ),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Left)
        ) && matches!(
            Self::classify_adc_ladder_mv(
                VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
                Self::ROW1_CONFIRM_CENTER_MV,
            ),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Confirm)
        ) && matches!(
            Self::classify_adc_ladder_mv(
                VaachakInputPhysicalSampleSource::Row1Gpio1AdcLadder,
                Self::ROW1_BACK_CENTER_MV,
            ),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Back)
        )
    }

    fn row2_classification_ok() -> bool {
        matches!(
            Self::classify_adc_ladder_mv(
                VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder,
                Self::ROW2_VOLDOWN_CENTER_MV,
            ),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::VolDown)
        ) && matches!(
            Self::classify_adc_ladder_mv(
                VaachakInputPhysicalSampleSource::Row2Gpio2AdcLadder,
                Self::ROW2_VOLUP_CENTER_MV,
            ),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::VolUp)
        )
    }

    fn power_button_classification_ok() -> bool {
        matches!(
            Self::classify_power_gpio_level(true),
            VaachakRawSampledButtonState::Button(VaachakRuntimeInputButton::Power)
        ) && matches!(
            Self::classify_power_gpio_level(false),
            VaachakRawSampledButtonState::NoButton
        )
    }
}

impl VaachakInputBackendNativeEventPipeline {
    pub fn execute_native_physical_sampling_handoff(
        previous_raw: VaachakRawSampledButtonState,
        sample: VaachakInputPhysicalSamplingResult,
        stable_elapsed_ms: u16,
        held_elapsed_ms: u16,
    ) -> VaachakInputPipelineEvent {
        VaachakInputPhysicalSamplingNativeDriver::handoff_to_native_event_pipeline(
            previous_raw,
            sample,
            stable_elapsed_ms,
            held_elapsed_ms,
        )
    }

    pub fn native_physical_sampling_driver_ready() -> bool {
        VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok()
    }
}
