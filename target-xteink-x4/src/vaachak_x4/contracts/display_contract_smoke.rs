#![allow(dead_code)]

/// Vaachak-owned display contract smoke.
///
/// This module validates the display contract metadata only. It intentionally
/// does not initialize SSD1677, perform SPI transactions, allocate a framebuffer,
/// render strips, or trigger e-paper refresh. Physical display behavior remains
/// owned by the imported Pulp runtime in Phase 27.
pub struct VaachakDisplayContractSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayGeometryContract {
    pub native_width: u16,
    pub native_height: u16,
    pub logical_width: u16,
    pub logical_height: u16,
    pub strip_rows: u16,
    pub rotation_degrees: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayPinContract {
    pub epd_cs_gpio: u8,
    pub epd_dc_gpio: u8,
    pub epd_rst_gpio: u8,
    pub epd_busy_gpio: u8,
    pub spi_sclk_gpio: u8,
    pub spi_mosi_gpio: u8,
    pub spi_miso_gpio: u8,
    pub sd_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ssd1677CommandContract {
    pub current_ram: u8,
    pub previous_ram: u8,
    pub display_update_control_2: u8,
    pub master_activate: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayContractCheck {
    Geometry,
    Pins,
    Ssd1677Commands,
    SharedSpi,
    StripRendering,
}

impl VaachakDisplayContractSmoke {
    pub const DISPLAY_CONTRACT_SMOKE_MARKER: &'static str = "x4-display-contract-smoke-ok";

    /// Physical behavior remains imported in Phase 27.
    pub const PHYSICAL_DISPLAY_BEHAVIOR_MOVED_TO_BOUNDARY: bool = false;
    pub const SSD1677_INIT_MOVED_TO_BOUNDARY: bool = false;
    pub const SPI_TRANSACTIONS_MOVED_TO_BOUNDARY: bool = false;
    pub const REFRESH_MOVED_TO_BOUNDARY: bool = false;
    pub const STRIP_RENDERING_MOVED_TO_BOUNDARY: bool = false;
    pub const FRAMEBUFFER_MOVED_TO_BOUNDARY: bool = false;

    /// X4 display panel geometry.
    pub const NATIVE_WIDTH: u16 = 800;
    pub const NATIVE_HEIGHT: u16 = 480;
    pub const LOGICAL_WIDTH: u16 = 480;
    pub const LOGICAL_HEIGHT: u16 = 800;
    pub const STRIP_ROWS: u16 = 40;
    pub const ROTATION_DEGREES: u16 = 270;

    /// X4 SSD1677 display pins and shared SPI pins.
    pub const EPD_CS_GPIO: u8 = 21;
    pub const EPD_DC_GPIO: u8 = 4;
    pub const EPD_RST_GPIO: u8 = 5;
    pub const EPD_BUSY_GPIO: u8 = 6;
    pub const SPI_SCLK_GPIO: u8 = 8;
    pub const SPI_MOSI_GPIO: u8 = 10;
    pub const SPI_MISO_GPIO: u8 = 7;
    pub const SD_CS_GPIO: u8 = 12;

    /// SSD1677 command contract used by the imported Pulp/X4 runtime.
    pub const SSD1677_CURRENT_RAM_CMD: u8 = 0x24;
    pub const SSD1677_PREVIOUS_RAM_CMD: u8 = 0x26;
    pub const SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD: u8 = 0x22;
    pub const SSD1677_MASTER_ACTIVATE_CMD: u8 = 0x20;

    pub const SHARES_STORAGE_SPI_BUS: bool = true;
    pub const DISPLAY_AND_SD_MUST_NOT_SELECT_TOGETHER: bool = true;

    pub const fn geometry() -> DisplayGeometryContract {
        DisplayGeometryContract {
            native_width: Self::NATIVE_WIDTH,
            native_height: Self::NATIVE_HEIGHT,
            logical_width: Self::LOGICAL_WIDTH,
            logical_height: Self::LOGICAL_HEIGHT,
            strip_rows: Self::STRIP_ROWS,
            rotation_degrees: Self::ROTATION_DEGREES,
        }
    }

    pub const fn pins() -> DisplayPinContract {
        DisplayPinContract {
            epd_cs_gpio: Self::EPD_CS_GPIO,
            epd_dc_gpio: Self::EPD_DC_GPIO,
            epd_rst_gpio: Self::EPD_RST_GPIO,
            epd_busy_gpio: Self::EPD_BUSY_GPIO,
            spi_sclk_gpio: Self::SPI_SCLK_GPIO,
            spi_mosi_gpio: Self::SPI_MOSI_GPIO,
            spi_miso_gpio: Self::SPI_MISO_GPIO,
            sd_cs_gpio: Self::SD_CS_GPIO,
        }
    }

    pub const fn ssd1677_commands() -> Ssd1677CommandContract {
        Ssd1677CommandContract {
            current_ram: Self::SSD1677_CURRENT_RAM_CMD,
            previous_ram: Self::SSD1677_PREVIOUS_RAM_CMD,
            display_update_control_2: Self::SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD,
            master_activate: Self::SSD1677_MASTER_ACTIVATE_CMD,
        }
    }

    pub const fn is_supported_ssd1677_command(cmd: u8) -> bool {
        cmd == Self::SSD1677_CURRENT_RAM_CMD
            || cmd == Self::SSD1677_PREVIOUS_RAM_CMD
            || cmd == Self::SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD
            || cmd == Self::SSD1677_MASTER_ACTIVATE_CMD
    }

    pub const fn is_valid_geometry() -> bool {
        Self::NATIVE_WIDTH == 800
            && Self::NATIVE_HEIGHT == 480
            && Self::LOGICAL_WIDTH == 480
            && Self::LOGICAL_HEIGHT == 800
            && Self::STRIP_ROWS == 40
            && Self::ROTATION_DEGREES == 270
    }

    pub const fn is_valid_pin_contract() -> bool {
        Self::EPD_CS_GPIO == 21
            && Self::EPD_DC_GPIO == 4
            && Self::EPD_RST_GPIO == 5
            && Self::EPD_BUSY_GPIO == 6
            && Self::SPI_SCLK_GPIO == 8
            && Self::SPI_MOSI_GPIO == 10
            && Self::SPI_MISO_GPIO == 7
            && Self::SD_CS_GPIO == 12
    }

    pub const fn is_valid_shared_spi_contract() -> bool {
        Self::SHARES_STORAGE_SPI_BUS && Self::DISPLAY_AND_SD_MUST_NOT_SELECT_TOGETHER
    }

    pub const fn is_valid_strip_contract(strip_rows: u16) -> bool {
        strip_rows == Self::STRIP_ROWS && Self::NATIVE_WIDTH == 800 && Self::NATIVE_HEIGHT == 480
    }

    pub const fn check(check: DisplayContractCheck) -> bool {
        match check {
            DisplayContractCheck::Geometry => Self::is_valid_geometry(),
            DisplayContractCheck::Pins => Self::is_valid_pin_contract(),
            DisplayContractCheck::Ssd1677Commands => {
                Self::is_supported_ssd1677_command(Self::SSD1677_CURRENT_RAM_CMD)
                    && Self::is_supported_ssd1677_command(Self::SSD1677_PREVIOUS_RAM_CMD)
                    && Self::is_supported_ssd1677_command(
                        Self::SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD,
                    )
                    && Self::is_supported_ssd1677_command(Self::SSD1677_MASTER_ACTIVATE_CMD)
            }
            DisplayContractCheck::SharedSpi => Self::is_valid_shared_spi_contract(),
            DisplayContractCheck::StripRendering => Self::is_valid_strip_contract(Self::STRIP_ROWS),
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::check(DisplayContractCheck::Geometry)
            && Self::check(DisplayContractCheck::Pins)
            && Self::check(DisplayContractCheck::Ssd1677Commands)
            && Self::check(DisplayContractCheck::SharedSpi)
            && Self::check(DisplayContractCheck::StripRendering)
            && !Self::PHYSICAL_DISPLAY_BEHAVIOR_MOVED_TO_BOUNDARY
            && !Self::SSD1677_INIT_MOVED_TO_BOUNDARY
            && !Self::SPI_TRANSACTIONS_MOVED_TO_BOUNDARY
            && !Self::REFRESH_MOVED_TO_BOUNDARY
            && !Self::STRIP_RENDERING_MOVED_TO_BOUNDARY
            && !Self::FRAMEBUFFER_MOVED_TO_BOUNDARY
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!("{}", Self::DISPLAY_CONTRACT_SMOKE_MARKER);
        } else {
            esp_println::println!("display-contract-smoke-failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_contract_smoke_is_valid() {
        assert!(VaachakDisplayContractSmoke::smoke_ok());
    }

    #[test]
    fn validates_ssd1677_commands() {
        assert!(VaachakDisplayContractSmoke::is_supported_ssd1677_command(
            0x24
        ));
        assert!(VaachakDisplayContractSmoke::is_supported_ssd1677_command(
            0x26
        ));
        assert!(VaachakDisplayContractSmoke::is_supported_ssd1677_command(
            0x22
        ));
        assert!(VaachakDisplayContractSmoke::is_supported_ssd1677_command(
            0x20
        ));
        assert!(!VaachakDisplayContractSmoke::is_supported_ssd1677_command(
            0xff
        ));
    }
}
