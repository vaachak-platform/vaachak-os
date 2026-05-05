#![allow(dead_code)]

pub struct VaachakInputSemanticsRuntimeBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPhysicalButton {
    Right,
    Left,
    Confirm,
    Back,
    VolUp,
    VolDown,
    Power,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakRuntimeInputAction {
    Next,
    Previous,
    NextJump,
    PreviousJump,
    Select,
    Back,
    Menu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakRuntimeInputEventKind {
    Press,
    Release,
    LongPress,
    Repeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakRuntimeInputEvent {
    pub kind: VaachakRuntimeInputEventKind,
    pub action: VaachakRuntimeInputAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakRuntimeButtonMapper {
    swap_buttons: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputSemanticsRuntimeReport {
    pub default_layout_ok: bool,
    pub swapped_layout_ok: bool,
    pub event_mapping_ok: bool,
    pub physical_input_sampling_owned: bool,
    pub debounce_repeat_owned: bool,
}

impl VaachakInputSemanticsRuntimeReport {
    pub const fn preflight_ok(self) -> bool {
        self.default_layout_ok
            && self.swapped_layout_ok
            && self.event_mapping_ok
            && !self.physical_input_sampling_owned
            && !self.debounce_repeat_owned
    }
}

impl VaachakRuntimeButtonMapper {
    pub const fn new() -> Self {
        Self {
            swap_buttons: false,
        }
    }

    pub const fn swapped() -> Self {
        Self { swap_buttons: true }
    }

    pub const fn is_swapped(self) -> bool {
        self.swap_buttons
    }

    pub const fn map_button(self, button: VaachakPhysicalButton) -> VaachakRuntimeInputAction {
        if self.swap_buttons {
            match button {
                VaachakPhysicalButton::VolDown => VaachakRuntimeInputAction::Next,
                VaachakPhysicalButton::VolUp => VaachakRuntimeInputAction::Previous,
                VaachakPhysicalButton::Right => VaachakRuntimeInputAction::Select,
                VaachakPhysicalButton::Left => VaachakRuntimeInputAction::Back,
                VaachakPhysicalButton::Confirm => VaachakRuntimeInputAction::NextJump,
                VaachakPhysicalButton::Back => VaachakRuntimeInputAction::PreviousJump,
                VaachakPhysicalButton::Power => VaachakRuntimeInputAction::Menu,
            }
        } else {
            match button {
                VaachakPhysicalButton::VolDown => VaachakRuntimeInputAction::Next,
                VaachakPhysicalButton::VolUp => VaachakRuntimeInputAction::Previous,
                VaachakPhysicalButton::Right => VaachakRuntimeInputAction::NextJump,
                VaachakPhysicalButton::Left => VaachakRuntimeInputAction::PreviousJump,
                VaachakPhysicalButton::Confirm => VaachakRuntimeInputAction::Select,
                VaachakPhysicalButton::Back => VaachakRuntimeInputAction::Back,
                VaachakPhysicalButton::Power => VaachakRuntimeInputAction::Menu,
            }
        }
    }

    pub const fn map_event(
        self,
        kind: VaachakRuntimeInputEventKind,
        button: VaachakPhysicalButton,
    ) -> VaachakRuntimeInputEvent {
        VaachakRuntimeInputEvent {
            kind,
            action: self.map_button(button),
        }
    }
}

impl VaachakInputSemanticsRuntimeBridge {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned input semantic runtime facade";
    pub const PHYSICAL_INPUT_SAMPLING_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const DEBOUNCE_REPEAT_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_INPUT_SAMPLING_OWNED_BY_BRIDGE: bool = false;
    pub const DEBOUNCE_REPEAT_OWNED_BY_BRIDGE: bool = false;

    pub fn active_runtime_preflight() -> bool {
        Self::preflight_report().preflight_ok()
    }

    pub fn preflight_report() -> VaachakInputSemanticsRuntimeReport {
        VaachakInputSemanticsRuntimeReport {
            default_layout_ok: Self::default_layout_ok(),
            swapped_layout_ok: Self::swapped_layout_ok(),
            event_mapping_ok: Self::event_mapping_ok(),
            physical_input_sampling_owned: Self::PHYSICAL_INPUT_SAMPLING_OWNED_BY_BRIDGE,
            debounce_repeat_owned: Self::DEBOUNCE_REPEAT_OWNED_BY_BRIDGE,
        }
    }

    fn default_layout_ok() -> bool {
        let mapper = VaachakRuntimeButtonMapper::new();
        !mapper.is_swapped()
            && mapper.map_button(VaachakPhysicalButton::VolDown) == VaachakRuntimeInputAction::Next
            && mapper.map_button(VaachakPhysicalButton::VolUp)
                == VaachakRuntimeInputAction::Previous
            && mapper.map_button(VaachakPhysicalButton::Right)
                == VaachakRuntimeInputAction::NextJump
            && mapper.map_button(VaachakPhysicalButton::Left)
                == VaachakRuntimeInputAction::PreviousJump
            && mapper.map_button(VaachakPhysicalButton::Confirm)
                == VaachakRuntimeInputAction::Select
            && mapper.map_button(VaachakPhysicalButton::Back) == VaachakRuntimeInputAction::Back
            && mapper.map_button(VaachakPhysicalButton::Power) == VaachakRuntimeInputAction::Menu
    }

    fn swapped_layout_ok() -> bool {
        let mapper = VaachakRuntimeButtonMapper::swapped();
        mapper.is_swapped()
            && mapper.map_button(VaachakPhysicalButton::VolDown) == VaachakRuntimeInputAction::Next
            && mapper.map_button(VaachakPhysicalButton::VolUp)
                == VaachakRuntimeInputAction::Previous
            && mapper.map_button(VaachakPhysicalButton::Right) == VaachakRuntimeInputAction::Select
            && mapper.map_button(VaachakPhysicalButton::Left) == VaachakRuntimeInputAction::Back
            && mapper.map_button(VaachakPhysicalButton::Confirm)
                == VaachakRuntimeInputAction::NextJump
            && mapper.map_button(VaachakPhysicalButton::Back)
                == VaachakRuntimeInputAction::PreviousJump
            && mapper.map_button(VaachakPhysicalButton::Power) == VaachakRuntimeInputAction::Menu
    }

    fn event_mapping_ok() -> bool {
        let mapper = VaachakRuntimeButtonMapper::new();
        mapper.map_event(
            VaachakRuntimeInputEventKind::Press,
            VaachakPhysicalButton::Confirm,
        ) == VaachakRuntimeInputEvent {
            kind: VaachakRuntimeInputEventKind::Press,
            action: VaachakRuntimeInputAction::Select,
        } && mapper.map_event(
            VaachakRuntimeInputEventKind::Repeat,
            VaachakPhysicalButton::VolDown,
        ) == VaachakRuntimeInputEvent {
            kind: VaachakRuntimeInputEventKind::Repeat,
            action: VaachakRuntimeInputAction::Next,
        } && mapper.map_event(
            VaachakRuntimeInputEventKind::LongPress,
            VaachakPhysicalButton::Back,
        ) == VaachakRuntimeInputEvent {
            kind: VaachakRuntimeInputEventKind::LongPress,
            action: VaachakRuntimeInputAction::Back,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        VaachakInputSemanticsRuntimeBridge, VaachakPhysicalButton, VaachakRuntimeButtonMapper,
        VaachakRuntimeInputAction,
    };

    #[test]
    fn runtime_input_semantics_probe_is_pure_and_valid() {
        assert!(VaachakInputSemanticsRuntimeBridge::active_runtime_preflight());
    }

    #[test]
    fn maps_default_and_swapped_layouts_like_active_runtime() {
        let default_mapper = VaachakRuntimeButtonMapper::new();
        assert_eq!(
            default_mapper.map_button(VaachakPhysicalButton::Confirm),
            VaachakRuntimeInputAction::Select
        );
        assert_eq!(
            default_mapper.map_button(VaachakPhysicalButton::Right),
            VaachakRuntimeInputAction::NextJump
        );

        let swapped = VaachakRuntimeButtonMapper::swapped();
        assert_eq!(
            swapped.map_button(VaachakPhysicalButton::Right),
            VaachakRuntimeInputAction::Select
        );
        assert_eq!(
            swapped.map_button(VaachakPhysicalButton::Confirm),
            VaachakRuntimeInputAction::NextJump
        );
    }
}
