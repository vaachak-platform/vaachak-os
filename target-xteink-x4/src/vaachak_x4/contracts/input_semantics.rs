#![allow(dead_code)]

pub struct VaachakInputSemantics;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSemanticButtonRole {
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
pub enum VaachakReaderAction {
    BackToLibrary,
    OpenOrSelect,
    NextPage,
    PreviousPage,
    BookmarkOrMenu,
    Stay,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakNavigationAction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputPinContract {
    pub row1_adc_gpio: u8,
    pub row2_adc_gpio: u8,
    pub power_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputSemanticAdoptionReport {
    pub pins_ok: bool,
    pub roles_ok: bool,
    pub reader_actions_ok: bool,
    pub navigation_actions_ok: bool,
    pub physical_input_moved: bool,
}

impl VaachakInputSemanticAdoptionReport {
    pub const fn adoption_ok(self) -> bool {
        self.pins_ok
            && self.roles_ok
            && self.reader_actions_ok
            && self.navigation_actions_ok
            && !self.physical_input_moved
    }
}

impl VaachakInputSemantics {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned pure input semantic helpers";
    pub const PHYSICAL_INPUT_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_INPUT_MOVED_TO_BOUNDARY: bool = false;

    pub const ROW1_ADC_GPIO: u8 = 1;
    pub const ROW2_ADC_GPIO: u8 = 2;
    pub const POWER_BUTTON_GPIO: u8 = 3;

    pub const BUTTON_ROLES: [VaachakSemanticButtonRole; 7] = [
        VaachakSemanticButtonRole::Back,
        VaachakSemanticButtonRole::Select,
        VaachakSemanticButtonRole::Up,
        VaachakSemanticButtonRole::Down,
        VaachakSemanticButtonRole::Left,
        VaachakSemanticButtonRole::Right,
        VaachakSemanticButtonRole::Power,
    ];

    pub const fn pin_contract() -> VaachakInputPinContract {
        VaachakInputPinContract {
            row1_adc_gpio: Self::ROW1_ADC_GPIO,
            row2_adc_gpio: Self::ROW2_ADC_GPIO,
            power_gpio: Self::POWER_BUTTON_GPIO,
        }
    }

    pub const fn is_known_input_pin(gpio: u8) -> bool {
        gpio == Self::ROW1_ADC_GPIO
            || gpio == Self::ROW2_ADC_GPIO
            || gpio == Self::POWER_BUTTON_GPIO
    }

    pub const fn role_name(role: VaachakSemanticButtonRole) -> &'static str {
        match role {
            VaachakSemanticButtonRole::Back => "Back",
            VaachakSemanticButtonRole::Select => "Select",
            VaachakSemanticButtonRole::Up => "Up",
            VaachakSemanticButtonRole::Down => "Down",
            VaachakSemanticButtonRole::Left => "Left",
            VaachakSemanticButtonRole::Right => "Right",
            VaachakSemanticButtonRole::Power => "Power",
            VaachakSemanticButtonRole::Unknown => "Unknown",
        }
    }

    pub const fn is_navigation_role(role: VaachakSemanticButtonRole) -> bool {
        matches!(
            role,
            VaachakSemanticButtonRole::Up
                | VaachakSemanticButtonRole::Down
                | VaachakSemanticButtonRole::Left
                | VaachakSemanticButtonRole::Right
        )
    }

    pub const fn navigation_action_for_role(
        role: VaachakSemanticButtonRole,
    ) -> VaachakNavigationAction {
        match role {
            VaachakSemanticButtonRole::Up => VaachakNavigationAction::Up,
            VaachakSemanticButtonRole::Down => VaachakNavigationAction::Down,
            VaachakSemanticButtonRole::Left => VaachakNavigationAction::Left,
            VaachakSemanticButtonRole::Right => VaachakNavigationAction::Right,
            _ => VaachakNavigationAction::None,
        }
    }

    pub const fn reader_action_for_role(role: VaachakSemanticButtonRole) -> VaachakReaderAction {
        match role {
            VaachakSemanticButtonRole::Back | VaachakSemanticButtonRole::Left => {
                VaachakReaderAction::BackToLibrary
            }
            VaachakSemanticButtonRole::Select => VaachakReaderAction::OpenOrSelect,
            VaachakSemanticButtonRole::Right | VaachakSemanticButtonRole::Down => {
                VaachakReaderAction::NextPage
            }
            VaachakSemanticButtonRole::Up => VaachakReaderAction::PreviousPage,
            VaachakSemanticButtonRole::Power => VaachakReaderAction::BookmarkOrMenu,
            VaachakSemanticButtonRole::Unknown => VaachakReaderAction::Unsupported,
        }
    }

    pub fn input_semantics_adoption_report() -> VaachakInputSemanticAdoptionReport {
        let pins = Self::pin_contract();

        VaachakInputSemanticAdoptionReport {
            pins_ok: pins.row1_adc_gpio == 1
                && pins.row2_adc_gpio == 2
                && pins.power_gpio == 3
                && Self::is_known_input_pin(1)
                && Self::is_known_input_pin(2)
                && Self::is_known_input_pin(3)
                && !Self::is_known_input_pin(99),
            roles_ok: Self::BUTTON_ROLES.len() == 7
                && Self::role_name(VaachakSemanticButtonRole::Back) == "Back"
                && Self::role_name(VaachakSemanticButtonRole::Select) == "Select"
                && Self::role_name(VaachakSemanticButtonRole::Power) == "Power",
            reader_actions_ok: Self::reader_action_for_role(VaachakSemanticButtonRole::Back)
                == VaachakReaderAction::BackToLibrary
                && Self::reader_action_for_role(VaachakSemanticButtonRole::Select)
                    == VaachakReaderAction::OpenOrSelect
                && Self::reader_action_for_role(VaachakSemanticButtonRole::Right)
                    == VaachakReaderAction::NextPage
                && Self::reader_action_for_role(VaachakSemanticButtonRole::Up)
                    == VaachakReaderAction::PreviousPage
                && Self::reader_action_for_role(VaachakSemanticButtonRole::Power)
                    == VaachakReaderAction::BookmarkOrMenu,
            navigation_actions_ok: Self::navigation_action_for_role(VaachakSemanticButtonRole::Up)
                == VaachakNavigationAction::Up
                && Self::navigation_action_for_role(VaachakSemanticButtonRole::Down)
                    == VaachakNavigationAction::Down
                && Self::navigation_action_for_role(VaachakSemanticButtonRole::Left)
                    == VaachakNavigationAction::Left
                && Self::navigation_action_for_role(VaachakSemanticButtonRole::Right)
                    == VaachakNavigationAction::Right,
            physical_input_moved: Self::PHYSICAL_INPUT_MOVED_TO_BOUNDARY,
        }
    }

    pub fn active_runtime_adoption_probe() -> bool {
        Self::input_semantics_adoption_report().adoption_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakInputSemantics, VaachakReaderAction, VaachakSemanticButtonRole};

    #[test]
    fn input_semantics_adoption_probe_is_pure_and_valid() {
        assert!(VaachakInputSemantics::active_runtime_adoption_probe());
    }

    #[test]
    fn maps_reader_semantic_actions() {
        assert_eq!(
            VaachakInputSemantics::reader_action_for_role(VaachakSemanticButtonRole::Back),
            VaachakReaderAction::BackToLibrary
        );
        assert_eq!(
            VaachakInputSemantics::reader_action_for_role(VaachakSemanticButtonRole::Select),
            VaachakReaderAction::OpenOrSelect
        );
    }
}
