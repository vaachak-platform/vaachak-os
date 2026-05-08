#![allow(dead_code)]

use crate::vaachak_x4::physical::display_physical_ssd1677_native_driver::{
    VaachakDisplayPhysicalSsd1677NativeDriver, VaachakSsd1677RefreshMode,
};

/// Compile-time smoke contract for the Vaachak-owned SSD1677 physical driver.
pub struct VaachakDisplayPhysicalSsd1677NativeDriverSmoke;

impl VaachakDisplayPhysicalSsd1677NativeDriverSmoke {
    pub const MARKER: &'static str = "display_physical_ssd1677_full_migration=ok";

    pub const fn full_refresh_sequence_ok() -> bool {
        VaachakDisplayPhysicalSsd1677NativeDriver::sequence_for_mode(
            VaachakSsd1677RefreshMode::FullRefresh,
        )
        .ok()
    }

    pub const fn partial_refresh_sequence_ok() -> bool {
        VaachakDisplayPhysicalSsd1677NativeDriver::sequence_for_mode(
            VaachakSsd1677RefreshMode::PartialRefresh,
        )
        .ok()
    }

    pub const fn smoke_ok() -> bool {
        VaachakDisplayPhysicalSsd1677NativeDriver::full_migration_ok()
            && Self::full_refresh_sequence_ok()
            && Self::partial_refresh_sequence_ok()
    }
}

pub const DISPLAY_PHYSICAL_SSD1677_NATIVE_DRIVER_SMOKE_OK: bool =
    VaachakDisplayPhysicalSsd1677NativeDriverSmoke::smoke_ok();
