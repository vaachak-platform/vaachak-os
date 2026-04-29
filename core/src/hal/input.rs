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
pub enum ButtonEventKind {
    Press,
    Release,
    LongPress,
    Repeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputEvent {
    pub button: ButtonId,
    pub kind: ButtonEventKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonThreshold {
    pub center_mv: u16,
    pub tolerance_mv: u16,
    pub logical: ButtonId,
}

pub trait InputHal {
    /// Non-blocking poll.
    fn poll(&mut self) -> Option<InputEvent>;

    /// Mirrors the current X4 `InputDriver::reset_hold_state()` seam so the
    /// runtime can consume a navigation event without immediately triggering a
    /// second hold/repeat action.
    fn reset_hold_state(&mut self) {}

    /// X4-specific ladder thresholds exported as data so the future HAL can
    /// keep the proven decode values close to the hardware implementation.
    fn row1_thresholds(&self) -> &[ButtonThreshold] {
        &[]
    }

    fn row2_thresholds(&self) -> &[ButtonThreshold] {
        &[]
    }
}
