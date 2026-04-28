use vaachak_core::hal::{InputEvent, InputHal};

#[derive(Default)]
pub struct X4Input;

impl InputHal for X4Input {
    fn poll(&mut self) -> Option<InputEvent> {
        None
    }
}
