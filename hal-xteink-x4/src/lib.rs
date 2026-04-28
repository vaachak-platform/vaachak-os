use vaachak_core::hal::{
    DirEntry, DisplayDepth, DisplayHal, Hal, InputEvent, InputHal, OpenMode, PowerHal,
    RefreshMode, StorageError, StorageHal,
};

pub mod display;
pub mod input;
pub mod power;
pub mod storage;

pub use display::X4Display;
pub use input::X4Input;
pub use power::X4Power;
pub use storage::{X4Dir, X4File, X4Storage};

pub struct X4Hal {
    display: X4Display,
    input: X4Input,
    power: X4Power,
    storage: X4Storage,
}

impl X4Hal {
    pub fn new_placeholder() -> Self {
        Self {
            display: X4Display::default(),
            input: X4Input::default(),
            power: X4Power::default(),
            storage: X4Storage::default(),
        }
    }
}

impl Hal for X4Hal {
    type Display = X4Display;
    type Input = X4Input;
    type Power = X4Power;
    type Storage = X4Storage;

    fn display(&mut self) -> &mut Self::Display { &mut self.display }
    fn input(&mut self) -> &mut Self::Input { &mut self.input }
    fn power(&mut self) -> &mut Self::Power { &mut self.power }
    fn storage(&mut self) -> &mut Self::Storage { &mut self.storage }

    const CAP_PSRAM: bool = false;
    const CAP_4GRAY: bool = false;
    const CAP_RTC: bool = false;
    const CAP_IMU: bool = false;
    const CAP_ENV_SENSOR: bool = false;
    const CAP_TOUCH: bool = false;
    const CAP_XTC_FORMAT: bool = true;
    const CAP_SUNLIGHT_FIX: bool = true;
}
