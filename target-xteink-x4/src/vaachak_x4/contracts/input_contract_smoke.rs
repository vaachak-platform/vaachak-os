#![allow(dead_code)]

use crate::vaachak_x4::contracts::input::{VaachakButtonRole, VaachakInputBoundary};

/// Vaachak-owned input contract smoke.
///
/// Phase 26 validates the pure input contract owned by Vaachak without moving
/// physical ADC reads, debounce/repeat handling, ladder thresholds, or runtime
/// input dispatch away from the imported Pulp/X4 runtime.
pub struct VaachakInputContractSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputContext {
    Library,
    Reader,
    System,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakReaderInputAction {
    NavigateLibrary,
    SelectOrOpen,
    BackToLibrary,
    NextPage,
    PreviousPage,
    BookmarkOrMenu,
    PowerOrWake,
    Stay,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPinContract {
    pub row1_adc_gpio: u8,
    pub row2_adc_gpio: u8,
    pub power_button_gpio: u8,
}

impl VaachakInputContractSmoke {
    pub const PHASE26_MARKER: &'static str = "phase26=x4-input-contract-smoke-ok";

    /// Runtime behavior remains imported. Phase 26 owns only pure metadata and
    /// contract validation helpers.
    pub const IMPLEMENTATION_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_ADC_READS_MOVED_IN_PHASE26: bool = false;
    pub const BUTTON_LADDER_CALIBRATION_MOVED_IN_PHASE26: bool = false;
    pub const DEBOUNCE_REPEAT_HANDLING_MOVED_IN_PHASE26: bool = false;
    pub const BUTTON_EVENT_ROUTING_MOVED_IN_PHASE26: bool = false;

    pub const REQUIRED_BUTTON_ROLES: [VaachakButtonRole; 7] = [
        VaachakButtonRole::Back,
        VaachakButtonRole::Select,
        VaachakButtonRole::Up,
        VaachakButtonRole::Down,
        VaachakButtonRole::Left,
        VaachakButtonRole::Right,
        VaachakButtonRole::Power,
    ];

    pub const fn physical_input_pin_contract() -> VaachakInputPinContract {
        VaachakInputPinContract {
            row1_adc_gpio: VaachakInputBoundary::ROW1_ADC_GPIO,
            row2_adc_gpio: VaachakInputBoundary::ROW2_ADC_GPIO,
            power_button_gpio: VaachakInputBoundary::POWER_BUTTON_GPIO,
        }
    }

    pub fn is_expected_physical_input_pin(gpio: u8) -> bool {
        let pins = Self::physical_input_pin_contract();
        gpio == pins.row1_adc_gpio || gpio == pins.row2_adc_gpio || gpio == pins.power_button_gpio
    }

    pub fn is_button_role_supported(role: VaachakButtonRole) -> bool {
        matches!(
            role,
            VaachakButtonRole::Back
                | VaachakButtonRole::Select
                | VaachakButtonRole::Up
                | VaachakButtonRole::Down
                | VaachakButtonRole::Left
                | VaachakButtonRole::Right
                | VaachakButtonRole::Power
        )
    }

    pub fn action_for_role(
        context: VaachakInputContext,
        role: VaachakButtonRole,
    ) -> VaachakReaderInputAction {
        match (context, role) {
            (VaachakInputContext::Library, VaachakButtonRole::Select) => {
                VaachakReaderInputAction::SelectOrOpen
            }
            (
                VaachakInputContext::Library,
                VaachakButtonRole::Up
                | VaachakButtonRole::Down
                | VaachakButtonRole::Left
                | VaachakButtonRole::Right,
            ) => VaachakReaderInputAction::NavigateLibrary,
            (VaachakInputContext::Library, VaachakButtonRole::Back) => {
                VaachakReaderInputAction::Stay
            }
            (VaachakInputContext::Reader, VaachakButtonRole::Back | VaachakButtonRole::Left) => {
                VaachakReaderInputAction::BackToLibrary
            }
            (VaachakInputContext::Reader, VaachakButtonRole::Right | VaachakButtonRole::Down) => {
                VaachakReaderInputAction::NextPage
            }
            (VaachakInputContext::Reader, VaachakButtonRole::Up) => {
                VaachakReaderInputAction::PreviousPage
            }
            (VaachakInputContext::Reader, VaachakButtonRole::Select) => {
                VaachakReaderInputAction::BookmarkOrMenu
            }
            (VaachakInputContext::System, VaachakButtonRole::Power)
            | (VaachakInputContext::Library, VaachakButtonRole::Power)
            | (VaachakInputContext::Reader, VaachakButtonRole::Power) => {
                VaachakReaderInputAction::PowerOrWake
            }
            (_, VaachakButtonRole::Unknown) => VaachakReaderInputAction::Unsupported,
            _ => VaachakReaderInputAction::Unsupported,
        }
    }

    pub fn smoke_validate_contract() -> bool {
        let pins = Self::physical_input_pin_contract();

        pins.row1_adc_gpio == VaachakInputBoundary::ROW1_ADC_GPIO
            && pins.row2_adc_gpio == VaachakInputBoundary::ROW2_ADC_GPIO
            && pins.power_button_gpio == VaachakInputBoundary::POWER_BUTTON_GPIO
            && Self::is_expected_physical_input_pin(VaachakInputBoundary::ROW1_ADC_GPIO)
            && Self::is_expected_physical_input_pin(VaachakInputBoundary::ROW2_ADC_GPIO)
            && Self::is_expected_physical_input_pin(VaachakInputBoundary::POWER_BUTTON_GPIO)
            && !Self::is_expected_physical_input_pin(99)
            && Self::REQUIRED_BUTTON_ROLES
                .iter()
                .copied()
                .all(Self::is_button_role_supported)
            && Self::action_for_role(VaachakInputContext::Library, VaachakButtonRole::Select)
                == VaachakReaderInputAction::SelectOrOpen
            && Self::action_for_role(VaachakInputContext::Reader, VaachakButtonRole::Back)
                == VaachakReaderInputAction::BackToLibrary
            && Self::action_for_role(VaachakInputContext::Reader, VaachakButtonRole::Right)
                == VaachakReaderInputAction::NextPage
            && Self::action_for_role(VaachakInputContext::Reader, VaachakButtonRole::Down)
                == VaachakReaderInputAction::NextPage
            && Self::action_for_role(VaachakInputContext::Reader, VaachakButtonRole::Up)
                == VaachakReaderInputAction::PreviousPage
            && Self::action_for_role(VaachakInputContext::Reader, VaachakButtonRole::Select)
                == VaachakReaderInputAction::BookmarkOrMenu
            && !Self::PHYSICAL_ADC_READS_MOVED_IN_PHASE26
            && !Self::BUTTON_LADDER_CALIBRATION_MOVED_IN_PHASE26
            && !Self::DEBOUNCE_REPEAT_HANDLING_MOVED_IN_PHASE26
            && !Self::BUTTON_EVENT_ROUTING_MOVED_IN_PHASE26
    }

    pub fn emit_contract_marker() {
        if Self::smoke_validate_contract() {
            esp_println::println!("{}", Self::PHASE26_MARKER);
        } else {
            esp_println::println!("input-contract-smoke-failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakInputContext, VaachakInputContractSmoke, VaachakReaderInputAction};
    use crate::vaachak_x4::contracts::input::{VaachakButtonRole, VaachakInputBoundary};

    #[test]
    fn validates_physical_input_pin_contract() {
        assert!(VaachakInputContractSmoke::is_expected_physical_input_pin(
            VaachakInputBoundary::ROW1_ADC_GPIO
        ));
        assert!(VaachakInputContractSmoke::is_expected_physical_input_pin(
            VaachakInputBoundary::ROW2_ADC_GPIO
        ));
        assert!(VaachakInputContractSmoke::is_expected_physical_input_pin(
            VaachakInputBoundary::POWER_BUTTON_GPIO
        ));
        assert!(!VaachakInputContractSmoke::is_expected_physical_input_pin(
            99
        ));
    }

    #[test]
    fn maps_reader_actions() {
        assert_eq!(
            VaachakInputContractSmoke::action_for_role(
                VaachakInputContext::Reader,
                VaachakButtonRole::Back
            ),
            VaachakReaderInputAction::BackToLibrary
        );
        assert_eq!(
            VaachakInputContractSmoke::action_for_role(
                VaachakInputContext::Reader,
                VaachakButtonRole::Right
            ),
            VaachakReaderInputAction::NextPage
        );
        assert_eq!(
            VaachakInputContractSmoke::action_for_role(
                VaachakInputContext::Reader,
                VaachakButtonRole::Up
            ),
            VaachakReaderInputAction::PreviousPage
        );
        assert_eq!(
            VaachakInputContractSmoke::action_for_role(
                VaachakInputContext::Reader,
                VaachakButtonRole::Select
            ),
            VaachakReaderInputAction::BookmarkOrMenu
        );
    }
}
