#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonId {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Power,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonEventType {
    Press,
    Hold,
    Repeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputEvent {
    pub button: ButtonId,
    pub kind: ButtonEventType,
}

pub trait InputHal {
    fn poll(&mut self) -> Option<InputEvent>;
}
