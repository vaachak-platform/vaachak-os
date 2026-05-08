#![allow(dead_code)]

use super::input_pulp_backend::VaachakInputPulpBackend;

/// Vaachak-owned input runtime ownership entrypoint for Xteink X4.
///
/// This module moves input runtime ownership authority into the Vaachak target
/// layer while keeping button ADC execution, debounce/repeat handling, and
/// navigation dispatch in the existing Pulp compatibility backend. It records
/// input identity, GPIO/ladder metadata, dependency on the current shell/input
/// boundary, and safety metadata. It does not sample ADC pins, run a button
/// polling loop, debounce input, dispatch navigation events, or change
/// display/storage/reader/file-browser behavior.
pub struct VaachakInputRuntimeOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputRuntimeBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputRuntimeOperation {
    InputIdentityMetadata,
    ButtonGpioMetadata,
    AdcLadderMetadata,
    TimingPolicyMetadata,
    SemanticMappingMetadata,
    ShellInputBoundaryMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakRuntimeInputButton {
    Right,
    Left,
    Confirm,
    Back,
    VolDown,
    VolUp,
    Power,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPinMap {
    pub row1_adc_gpio: u8,
    pub row2_adc_gpio: u8,
    pub power_button_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputTimingMap {
    pub oversample_count: u32,
    pub debounce_window_ms: u64,
    pub long_press_window_ms: u64,
    pub repeat_interval_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputLadderBand {
    pub row_gpio: u8,
    pub center_mv: u16,
    pub tolerance_mv: u16,
    pub button: VaachakRuntimeInputButton,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputOperationOwnership {
    pub operation: VaachakInputRuntimeOperation,
    pub backend: VaachakInputRuntimeBackend,
    pub ownership_authority: &'static str,
    pub active_executor_owner: &'static str,
    pub current_shell_input_boundary: &'static str,
    pub behavior_execution_moved_to_vaachak: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputRuntimeOwnershipReport {
    pub input_runtime_ownership_authority_moved_to_vaachak: bool,
    pub active_backend_is_pulp_compatibility: bool,
    pub current_input_boundary_documented: bool,
    pub pin_map_ok: bool,
    pub ladder_map_ok: bool,
    pub timing_map_ok: bool,
    pub backend_bridge_ok: bool,
    pub adc_sampling_executor_moved_to_vaachak: bool,
    pub button_scan_executor_moved_to_vaachak: bool,
    pub debounce_repeat_executor_moved_to_vaachak: bool,
    pub navigation_event_routing_moved_to_vaachak: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakInputRuntimeOwnershipReport {
    pub const fn ownership_ok(self) -> bool {
        self.input_runtime_ownership_authority_moved_to_vaachak
            && self.active_backend_is_pulp_compatibility
            && self.current_input_boundary_documented
            && self.pin_map_ok
            && self.ladder_map_ok
            && self.timing_map_ok
            && self.backend_bridge_ok
            && !self.adc_sampling_executor_moved_to_vaachak
            && !self.button_scan_executor_moved_to_vaachak
            && !self.debounce_repeat_executor_moved_to_vaachak
            && !self.navigation_event_routing_moved_to_vaachak
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakInputRuntimeOwner {
    pub const INPUT_RUNTIME_OWNERSHIP_MARKER: &'static str = "x4-input-runtime-owner-ok";

    pub const INPUT_RUNTIME_IDENTITY: &'static str = "xteink-x4-button-adc-input-runtime";
    pub const INPUT_RUNTIME_OWNERSHIP_AUTHORITY: &'static str = "target-xteink-x4 Vaachak layer";
    pub const INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;

    pub const PULP_COMPATIBILITY_BACKEND: VaachakInputRuntimeBackend =
        VaachakInputRuntimeBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND: VaachakInputRuntimeBackend = Self::PULP_COMPATIBILITY_BACKEND;
    pub const ACTIVE_BACKEND_NAME: &'static str = VaachakInputPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const CURRENT_INPUT_BOUNDARY_DEPENDENCY: &'static str = "VaachakInputBoundary";
    pub const CURRENT_SEMANTIC_MAPPER_DEPENDENCY: &'static str = "VaachakActiveInputSemanticMapper";
    pub const CURRENT_ADC_CLASSIFICATION_DEPENDENCY: &'static str = "VaachakInputAdcRuntimeBridge";
    pub const CURRENT_SHELL_INPUT_BOUNDARY: &'static str =
        "Pulp AppManager input dispatch remains active";

    pub const ROW1_ADC_GPIO: u8 = 1;
    pub const ROW2_ADC_GPIO: u8 = 2;
    pub const POWER_BUTTON_GPIO: u8 = 3;

    pub const DEFAULT_TOLERANCE_MV: u16 = 150;
    pub const LOW_RAIL_TOLERANCE_MV: u16 = 50;

    pub const ROW1_RIGHT_CENTER_MV: u16 = 3;
    pub const ROW1_LEFT_CENTER_MV: u16 = 1113;
    pub const ROW1_CONFIRM_CENTER_MV: u16 = 1984;
    pub const ROW1_BACK_CENTER_MV: u16 = 2556;
    pub const ROW2_VOLDOWN_CENTER_MV: u16 = 3;
    pub const ROW2_VOLUP_CENTER_MV: u16 = 1659;

    pub const OVERSAMPLE_COUNT: u32 = 4;
    pub const DEBOUNCE_WINDOW_MS: u64 = 15;
    pub const LONG_PRESS_WINDOW_MS: u64 = 1000;
    pub const REPEAT_INTERVAL_MS: u64 = 150;

    pub const ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn pin_map() -> VaachakInputPinMap {
        VaachakInputPinMap {
            row1_adc_gpio: Self::ROW1_ADC_GPIO,
            row2_adc_gpio: Self::ROW2_ADC_GPIO,
            power_button_gpio: Self::POWER_BUTTON_GPIO,
        }
    }

    pub const fn timing_map() -> VaachakInputTimingMap {
        VaachakInputTimingMap {
            oversample_count: Self::OVERSAMPLE_COUNT,
            debounce_window_ms: Self::DEBOUNCE_WINDOW_MS,
            long_press_window_ms: Self::LONG_PRESS_WINDOW_MS,
            repeat_interval_ms: Self::REPEAT_INTERVAL_MS,
        }
    }

    pub const fn ladder_band(index: usize) -> Option<VaachakInputLadderBand> {
        match index {
            0 => Some(VaachakInputLadderBand {
                row_gpio: Self::ROW1_ADC_GPIO,
                center_mv: Self::ROW1_RIGHT_CENTER_MV,
                tolerance_mv: Self::LOW_RAIL_TOLERANCE_MV,
                button: VaachakRuntimeInputButton::Right,
            }),
            1 => Some(VaachakInputLadderBand {
                row_gpio: Self::ROW1_ADC_GPIO,
                center_mv: Self::ROW1_LEFT_CENTER_MV,
                tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
                button: VaachakRuntimeInputButton::Left,
            }),
            2 => Some(VaachakInputLadderBand {
                row_gpio: Self::ROW1_ADC_GPIO,
                center_mv: Self::ROW1_CONFIRM_CENTER_MV,
                tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
                button: VaachakRuntimeInputButton::Confirm,
            }),
            3 => Some(VaachakInputLadderBand {
                row_gpio: Self::ROW1_ADC_GPIO,
                center_mv: Self::ROW1_BACK_CENTER_MV,
                tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
                button: VaachakRuntimeInputButton::Back,
            }),
            4 => Some(VaachakInputLadderBand {
                row_gpio: Self::ROW2_ADC_GPIO,
                center_mv: Self::ROW2_VOLDOWN_CENTER_MV,
                tolerance_mv: Self::LOW_RAIL_TOLERANCE_MV,
                button: VaachakRuntimeInputButton::VolDown,
            }),
            5 => Some(VaachakInputLadderBand {
                row_gpio: Self::ROW2_ADC_GPIO,
                center_mv: Self::ROW2_VOLUP_CENTER_MV,
                tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
                button: VaachakRuntimeInputButton::VolUp,
            }),
            _ => None,
        }
    }

    pub const fn operation_ownership(
        operation: VaachakInputRuntimeOperation,
    ) -> VaachakInputOperationOwnership {
        VaachakInputOperationOwnership {
            operation,
            backend: Self::ACTIVE_BACKEND,
            ownership_authority: Self::INPUT_RUNTIME_OWNERSHIP_AUTHORITY,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            current_shell_input_boundary: Self::CURRENT_SHELL_INPUT_BOUNDARY,
            behavior_execution_moved_to_vaachak: false,
        }
    }

    pub const fn operation_metadata_is_safe(metadata: VaachakInputOperationOwnership) -> bool {
        metadata.ownership_authority.len() == Self::INPUT_RUNTIME_OWNERSHIP_AUTHORITY.len()
            && metadata.active_executor_owner.len() == Self::ACTIVE_EXECUTOR_OWNER.len()
            && metadata.current_shell_input_boundary.len()
                == Self::CURRENT_SHELL_INPUT_BOUNDARY.len()
            && matches!(
                metadata.backend,
                VaachakInputRuntimeBackend::PulpCompatibility
            )
            && !metadata.behavior_execution_moved_to_vaachak
    }

    pub const fn current_input_boundary_documented() -> bool {
        Self::CURRENT_INPUT_BOUNDARY_DEPENDENCY.len() > 0
            && Self::CURRENT_SEMANTIC_MAPPER_DEPENDENCY.len() > 0
            && Self::CURRENT_ADC_CLASSIFICATION_DEPENDENCY.len() > 0
            && Self::CURRENT_SHELL_INPUT_BOUNDARY.len() > 0
    }

    pub const fn pin_map_ok() -> bool {
        let pins = Self::pin_map();
        pins.row1_adc_gpio == 1 && pins.row2_adc_gpio == 2 && pins.power_button_gpio == 3
    }

    pub const fn ladder_map_ok() -> bool {
        let row1_right_ok = match Self::ladder_band(0) {
            Some(band) => band.row_gpio == 1 && band.center_mv == 3,
            None => false,
        };
        let row1_left_ok = match Self::ladder_band(1) {
            Some(band) => band.row_gpio == 1 && band.center_mv == 1113,
            None => false,
        };
        let row1_confirm_ok = match Self::ladder_band(2) {
            Some(band) => band.row_gpio == 1 && band.center_mv == 1984,
            None => false,
        };
        let row1_back_ok = match Self::ladder_band(3) {
            Some(band) => band.row_gpio == 1 && band.center_mv == 2556,
            None => false,
        };
        let row2_voldown_ok = match Self::ladder_band(4) {
            Some(band) => band.row_gpio == 2 && band.center_mv == 3,
            None => false,
        };
        let row2_volup_ok = match Self::ladder_band(5) {
            Some(band) => band.row_gpio == 2 && band.center_mv == 1659,
            None => false,
        };

        let no_extra_band = match Self::ladder_band(6) {
            Some(_) => false,
            None => true,
        };

        row1_right_ok
            && row1_left_ok
            && row1_confirm_ok
            && row1_back_ok
            && row2_voldown_ok
            && row2_volup_ok
            && no_extra_band
    }

    pub const fn timing_map_ok() -> bool {
        let timing = Self::timing_map();
        timing.oversample_count == 4
            && timing.debounce_window_ms == 15
            && timing.long_press_window_ms == 1000
            && timing.repeat_interval_ms == 150
    }

    pub const fn report() -> VaachakInputRuntimeOwnershipReport {
        VaachakInputRuntimeOwnershipReport {
            input_runtime_ownership_authority_moved_to_vaachak:
                Self::INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            active_backend_is_pulp_compatibility: matches!(
                Self::ACTIVE_BACKEND,
                VaachakInputRuntimeBackend::PulpCompatibility
            ),
            current_input_boundary_documented: Self::current_input_boundary_documented(),
            pin_map_ok: Self::pin_map_ok(),
            ladder_map_ok: Self::ladder_map_ok(),
            timing_map_ok: Self::timing_map_ok(),
            backend_bridge_ok: VaachakInputPulpBackend::bridge_ok(),
            adc_sampling_executor_moved_to_vaachak: Self::ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK,
            button_scan_executor_moved_to_vaachak: Self::BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK,
            debounce_repeat_executor_moved_to_vaachak:
                Self::DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK,
            navigation_event_routing_moved_to_vaachak:
                Self::NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn ownership_ok() -> bool {
        Self::report().ownership_ok()
    }
}
