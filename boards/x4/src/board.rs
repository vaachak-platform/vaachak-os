use core::convert::Infallible;

use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    prelude::Point,
};
use esp_hal::{
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    peripherals::Peripherals,
};
use esp_println::println;
use vaachak_core::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use vaachak_drivers::ssd1677::{NATIVE_WIDTH, PanelIo, Ssd1677};

use crate::pins::{
    EPD_BUSY_ACTIVE_HIGH, EPD_BUSY_POLL_MS, EPD_BUSY_TIMEOUT_MS, EPD_RESET_LOW_MS,
    EPD_RESET_RECOVERY_MS,
};

pub const STRIP_HEIGHT: u16 = 40;
pub const STRIP_BYTES: usize = (NATIVE_WIDTH as usize / 8) * STRIP_HEIGHT as usize;

pub type X4Display<'d> = Ssd1677<X4DisplayIo<'d>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X4DisplayError {
    BusyTimeout,
}

pub struct X4Board<'d> {
    io: X4DisplayIo<'d>,
}

pub struct X4DisplayIo<'d> {
    sclk: Output<'d>,
    mosi: Output<'d>,
    cs: Output<'d>,
    dc: Output<'d>,
    rst: Output<'d>,
    busy: Input<'d>,
}

pub struct X4StripTarget<'a> {
    native_y_start: u16,
    buffer: &'a mut [u8; STRIP_BYTES],
}

impl X4Board<'static> {
    pub fn new(peripherals: Peripherals) -> Self {
        let output = OutputConfig::default();

        let io = X4DisplayIo {
            sclk: Output::new(peripherals.GPIO8, Level::Low, output),
            mosi: Output::new(peripherals.GPIO10, Level::Low, output),
            cs: Output::new(peripherals.GPIO21, Level::High, output),
            dc: Output::new(peripherals.GPIO4, Level::Low, output),
            rst: Output::new(peripherals.GPIO5, Level::High, output),
            busy: Input::new(peripherals.GPIO6, InputConfig::default()),
        };

        Self { io }
    }

    pub fn into_display(self) -> X4Display<'static> {
        Ssd1677::new(self.io)
    }
}

impl<'a> X4StripTarget<'a> {
    pub fn new(native_y_start: u16, buffer: &'a mut [u8; STRIP_BYTES]) -> Self {
        buffer.fill(0xFF);
        Self {
            native_y_start,
            buffer,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buffer[..]
    }
}

impl OriginDimensions for X4StripTarget<'_> {
    fn size(&self) -> Size {
        Size::new(LOGICAL_WIDTH, LOGICAL_HEIGHT)
    }
}

impl DrawTarget for X4StripTarget<'_> {
    type Color = BinaryColor;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels {
            if x < 0 || y < 0 {
                continue;
            }

            let x = x as u16;
            let y = y as u16;

            if x >= LOGICAL_WIDTH as u16 || y >= LOGICAL_HEIGHT as u16 {
                continue;
            }

            let x_native = y;
            let y_native = (LOGICAL_WIDTH as u16 - 1) - x;

            if y_native < self.native_y_start || y_native >= self.native_y_start + STRIP_HEIGHT {
                continue;
            }

            let local_y = (y_native - self.native_y_start) as usize;
            let byte_index = local_y * (NATIVE_WIDTH as usize / 8) + (x_native as usize / 8);
            let mask = 0x80 >> (x_native % 8);

            match color {
                BinaryColor::On => self.buffer[byte_index] &= !mask,
                BinaryColor::Off => self.buffer[byte_index] |= mask,
            }
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buffer.fill(match color {
            BinaryColor::On => 0x00,
            BinaryColor::Off => 0xFF,
        });
        Ok(())
    }
}

impl X4DisplayIo<'_> {
    fn busy_level(&mut self) -> bool {
        self.busy.is_high()
    }

    fn busy_is_active(&mut self) -> bool {
        let level = self.busy_level();
        if EPD_BUSY_ACTIVE_HIGH { level } else { !level }
    }

    fn short_delay() {
        for _ in 0..16 {
            core::hint::spin_loop();
        }
    }

    fn write_byte(&mut self, mut byte: u8) {
        for _ in 0..8 {
            if byte & 0x80 == 0 {
                let _ = self.mosi.set_low();
            } else {
                let _ = self.mosi.set_high();
            }

            let _ = self.sclk.set_high();
            Self::short_delay();
            let _ = self.sclk.set_low();
            Self::short_delay();

            byte <<= 1;
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }
}

impl PanelIo for X4DisplayIo<'_> {
    type Error = X4DisplayError;

    fn hard_reset<D>(&mut self, delay: &mut D) -> Result<(), Self::Error>
    where
        D: embedded_hal::delay::DelayNs,
    {
        println!("busy before reset={}", self.busy_level() as u8);
        println!("display reset asserted");
        let _ = self.rst.set_low();
        delay.delay_ms(EPD_RESET_LOW_MS);
        println!("display reset released");
        let _ = self.rst.set_high();
        delay.delay_ms(EPD_RESET_RECOVERY_MS);
        println!("busy after reset={}", self.busy_level() as u8);
        Ok(())
    }

    fn wait_while_busy<D>(&mut self, delay: &mut D) -> Result<(), Self::Error>
    where
        D: embedded_hal::delay::DelayNs,
    {
        let start_level = self.busy_level();
        println!("wait busy start={}", start_level as u8);

        let poll_count = EPD_BUSY_TIMEOUT_MS / EPD_BUSY_POLL_MS;
        for poll in 0..poll_count {
            if !self.busy_is_active() {
                println!("wait busy done polls={}", poll);
                return Ok(());
            }
            delay.delay_ms(EPD_BUSY_POLL_MS);
        }

        println!("wait busy timeout level={}", self.busy_level() as u8);
        Err(X4DisplayError::BusyTimeout)
    }

    fn command(&mut self, command: u8) -> Result<(), Self::Error> {
        let _ = self.cs.set_low();
        let _ = self.dc.set_low();
        self.write_byte(command);
        let _ = self.cs.set_high();
        Ok(())
    }

    fn data(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        let _ = self.cs.set_low();
        let _ = self.dc.set_high();
        self.write_bytes(bytes);
        let _ = self.cs.set_high();
        Ok(())
    }
}
