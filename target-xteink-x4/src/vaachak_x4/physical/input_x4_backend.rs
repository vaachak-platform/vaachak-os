#![allow(dead_code)]

/// X4 compatibility backend metadata for the Vaachak input runtime owner.
///
/// This backend names the still-active imported input executor. It does not
/// sample ADC pins, run debounce/repeat loops, dispatch navigation events, or
/// change reader/file-browser behavior.
pub struct VaachakInputX4Backend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputX4BackendReport {
    pub active_hardware_executor: bool,
    pub active_adc_executor_owner: &'static str,
    pub active_button_scan_executor_owner: &'static str,
    pub active_debounce_executor_owner: &'static str,
    pub active_navigation_executor_owner: &'static str,
    pub adc_sampling_executor_moved_to_vaachak: bool,
    pub button_scan_executor_moved_to_vaachak: bool,
    pub debounce_repeat_executor_moved_to_vaachak: bool,
    pub navigation_event_routing_moved_to_vaachak: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakInputX4BackendReport {
    pub const fn bridge_ok(self) -> bool {
        self.active_hardware_executor
            && self.active_adc_executor_owner.len()
                == VaachakInputX4Backend::ACTIVE_ADC_EXECUTOR_OWNER.len()
            && self.active_button_scan_executor_owner.len()
                == VaachakInputX4Backend::ACTIVE_BUTTON_SCAN_EXECUTOR_OWNER.len()
            && self.active_debounce_executor_owner.len()
                == VaachakInputX4Backend::ACTIVE_DEBOUNCE_EXECUTOR_OWNER.len()
            && self.active_navigation_executor_owner.len()
                == VaachakInputX4Backend::ACTIVE_NAVIGATION_EXECUTOR_OWNER.len()
            && !self.adc_sampling_executor_moved_to_vaachak
            && !self.button_scan_executor_moved_to_vaachak
            && !self.debounce_repeat_executor_moved_to_vaachak
            && !self.navigation_event_routing_moved_to_vaachak
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakInputX4Backend {
    pub const BACKEND_NAME: &'static str = "X4Compatibility";
    pub const ACTIVE_HARDWARE_EXECUTOR: bool = true;

    pub const ACTIVE_ADC_EXECUTOR_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const ACTIVE_BUTTON_SCAN_EXECUTOR_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const ACTIVE_DEBOUNCE_EXECUTOR_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const ACTIVE_REPEAT_EXECUTOR_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const ACTIVE_NAVIGATION_EXECUTOR_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const ACTIVE_SHELL_INPUT_EXECUTOR_OWNER: &'static str = "Vaachak-owned X4 runtime";

    pub const ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn report() -> VaachakInputX4BackendReport {
        VaachakInputX4BackendReport {
            active_hardware_executor: Self::ACTIVE_HARDWARE_EXECUTOR,
            active_adc_executor_owner: Self::ACTIVE_ADC_EXECUTOR_OWNER,
            active_button_scan_executor_owner: Self::ACTIVE_BUTTON_SCAN_EXECUTOR_OWNER,
            active_debounce_executor_owner: Self::ACTIVE_DEBOUNCE_EXECUTOR_OWNER,
            active_navigation_executor_owner: Self::ACTIVE_NAVIGATION_EXECUTOR_OWNER,
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

    pub const fn bridge_ok() -> bool {
        Self::report().bridge_ok()
    }
}
