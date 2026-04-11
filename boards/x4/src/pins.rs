pub const EPD_SCLK: u8 = 8;
pub const EPD_MOSI: u8 = 10;
pub const EPD_CS: u8 = 21;
pub const EPD_DC: u8 = 4;
pub const EPD_RST: u8 = 5;
pub const EPD_BUSY: u8 = 6;
pub const EPD_BUSY_ACTIVE_HIGH: bool = true;
pub const EPD_RESET_LOW_MS: u32 = 20;
pub const EPD_RESET_RECOVERY_MS: u32 = 200;
pub const EPD_BUSY_POLL_MS: u32 = 10;
pub const EPD_BUSY_TIMEOUT_MS: u32 = 15_000;

pub const SD_CS: u8 = 12;
pub const BATTERY_ADC: u8 = 0;
pub const BUTTONS_ADC_1: u8 = 1;
pub const BUTTONS_ADC_2: u8 = 2;
pub const POWER_BUTTON: u8 = 3;
