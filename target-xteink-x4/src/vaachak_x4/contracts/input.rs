#![allow(dead_code)]

/// Vaachak-owned input boundary metadata for the Xteink X4 target.
///
/// Phase 22 intentionally does not move physical ADC reads, debounce/repeat
/// handling, or button ladder calibration. The working implementation remains
/// in the imported Pulp/X4 runtime while Vaachak records the contract it will
/// own in later extraction phases.
pub struct VaachakInputBoundary;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakButtonRole {
    Back,
    Select,
    Up,
    Down,
    Left,
    Right,
    Power,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputOwner {
    ImportedPulpRuntime,
    VaachakBoundaryMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakButtonRoleInfo {
    pub role: VaachakButtonRole,
    pub label: &'static str,
    pub imported_owner: VaachakInputOwner,
}

impl VaachakInputBoundary {
    pub const PHASE_MARKER: &'static str = "phase22=x4-input-boundary-ok";

    /// Current source of truth for physical input behavior.
    pub const IMPLEMENTATION_OWNER: &'static str = "vendor/pulp-os imported runtime";

    /// X4 ADC ladder / button GPIO metadata.
    pub const ROW1_ADC_GPIO: u8 = 1;
    pub const ROW2_ADC_GPIO: u8 = 2;
    pub const POWER_BUTTON_GPIO: u8 = 3;

    /// Phase 22 records the boundary only. It does not move runtime behavior.
    pub const PHYSICAL_ADC_READS_MOVED_IN_PHASE22: bool = false;
    pub const BUTTON_LADDER_CALIBRATION_MOVED_IN_PHASE22: bool = false;
    pub const DEBOUNCE_REPEAT_HANDLING_MOVED_IN_PHASE22: bool = false;
    pub const BUTTON_EVENT_ROUTING_MOVED_IN_PHASE22: bool = false;

    /// Reader footer/action labels expected by Vaachak. The imported runtime
    /// continues to render and route actions in Phase 22.
    pub const READER_FOOTER_ACTION_LABELS: [&'static str; 4] = ["Back", "Select", "Open", "Stay"];

    /// Role order used for documentation/checking. This is not the physical
    /// ADC threshold order; threshold ownership remains imported.
    pub const BUTTON_ROLES: [VaachakButtonRole; 7] = [
        VaachakButtonRole::Back,
        VaachakButtonRole::Select,
        VaachakButtonRole::Up,
        VaachakButtonRole::Down,
        VaachakButtonRole::Left,
        VaachakButtonRole::Right,
        VaachakButtonRole::Power,
    ];

    pub fn emit_boot_marker() {
        esp_println::println!("{}", Self::PHASE_MARKER);
    }

    pub const fn owns_physical_input_behavior() -> bool {
        false
    }

    pub const fn owner_for_runtime_behavior() -> VaachakInputOwner {
        VaachakInputOwner::ImportedPulpRuntime
    }

    pub const fn is_adc_ladder_gpio(gpio: u8) -> bool {
        gpio == Self::ROW1_ADC_GPIO || gpio == Self::ROW2_ADC_GPIO
    }

    pub const fn is_power_button_gpio(gpio: u8) -> bool {
        gpio == Self::POWER_BUTTON_GPIO
    }

    pub const fn role_name(role: VaachakButtonRole) -> &'static str {
        match role {
            VaachakButtonRole::Back => "Back",
            VaachakButtonRole::Select => "Select",
            VaachakButtonRole::Up => "Up",
            VaachakButtonRole::Down => "Down",
            VaachakButtonRole::Left => "Left",
            VaachakButtonRole::Right => "Right",
            VaachakButtonRole::Power => "Power",
            VaachakButtonRole::Unknown => "Unknown",
        }
    }

    pub const fn role_info(role: VaachakButtonRole) -> VaachakButtonRoleInfo {
        VaachakButtonRoleInfo {
            role,
            label: Self::role_name(role),
            imported_owner: VaachakInputOwner::ImportedPulpRuntime,
        }
    }

    pub const fn role_is_navigation(role: VaachakButtonRole) -> bool {
        matches!(
            role,
            VaachakButtonRole::Up
                | VaachakButtonRole::Down
                | VaachakButtonRole::Left
                | VaachakButtonRole::Right
        )
    }

    pub const fn role_is_reader_action(role: VaachakButtonRole) -> bool {
        matches!(
            role,
            VaachakButtonRole::Back
                | VaachakButtonRole::Select
                | VaachakButtonRole::Left
                | VaachakButtonRole::Right
        )
    }

    pub const fn role_is_system_action(role: VaachakButtonRole) -> bool {
        matches!(role, VaachakButtonRole::Power)
    }

    pub fn role_from_label(label: &str) -> VaachakButtonRole {
        if label.eq_ignore_ascii_case("back") {
            VaachakButtonRole::Back
        } else if label.eq_ignore_ascii_case("select") {
            VaachakButtonRole::Select
        } else if label.eq_ignore_ascii_case("up") {
            VaachakButtonRole::Up
        } else if label.eq_ignore_ascii_case("down") {
            VaachakButtonRole::Down
        } else if label.eq_ignore_ascii_case("left") {
            VaachakButtonRole::Left
        } else if label.eq_ignore_ascii_case("right") {
            VaachakButtonRole::Right
        } else if label.eq_ignore_ascii_case("power") {
            VaachakButtonRole::Power
        } else {
            VaachakButtonRole::Unknown
        }
    }
}
