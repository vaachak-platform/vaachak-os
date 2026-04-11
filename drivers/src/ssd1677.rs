use core::fmt::Debug;
use embedded_hal::delay::DelayNs;

pub const NATIVE_WIDTH: u16 = 800;
pub const NATIVE_HEIGHT: u16 = 480;

const SOFT_RESET: u8 = 0x12;
const BOOSTER_SOFT_START: u8 = 0x0C;
const DRIVER_OUTPUT_CONTROL: u8 = 0x01;
const BORDER_WAVEFORM: u8 = 0x3C;
const TEMP_SENSOR_CONTROL: u8 = 0x18;
const DATA_ENTRY_MODE: u8 = 0x11;
const SET_RAM_X_RANGE: u8 = 0x44;
const SET_RAM_Y_RANGE: u8 = 0x45;
const SET_RAM_X_COUNTER: u8 = 0x4E;
const SET_RAM_Y_COUNTER: u8 = 0x4F;
const WRITE_RAM_BW: u8 = 0x24;
const WRITE_RAM_RED: u8 = 0x26;
const DISPLAY_UPDATE_CTRL1: u8 = 0x21;
const DISPLAY_UPDATE_CTRL2: u8 = 0x22;
const MASTER_ACTIVATION: u8 = 0x20;
const DEEP_SLEEP: u8 = 0x10;
const WRITE_VCOM: u8 = 0x2C;

const CTRL1_BYPASS_RED: u8 = 0x40;
const DISPLAY_UPDATE_CTRL2_FULL: u8 = 0xF7;
const DISPLAY_POWER_ON_BITS: u8 = 0xC0;
const DISPLAY_POWER_OFF_BITS: u8 = 0x03;
const FULL_FRAME_BYTES: usize = (NATIVE_WIDTH as usize * NATIVE_HEIGHT as usize) / 8;

pub trait PanelIo {
    type Error: Debug;

    fn hard_reset<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Self::Error>;
    fn wait_while_busy<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Self::Error>;
    fn command(&mut self, command: u8) -> Result<(), Self::Error>;
    fn data(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
}

pub struct Ssd1677<IO> {
    io: IO,
    display_on: bool,
}

impl<IO> Ssd1677<IO>
where
    IO: PanelIo,
{
    fn trace<F>(trace: &mut F, message: &'static str)
    where
        F: FnMut(&'static str),
    {
        trace(message);
    }

    pub fn new(io: IO) -> Self {
        Self {
            io,
            display_on: false,
        }
    }

    pub fn init<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), IO::Error> {
        self.init_with_trace(delay, &mut |_| {})
    }

    pub fn init_with_trace<D, F>(&mut self, delay: &mut D, trace: &mut F) -> Result<(), IO::Error>
    where
        D: DelayNs,
        F: FnMut(&'static str),
    {
        Self::trace(trace, "display init start");
        self.io.hard_reset(delay)?;

        Self::trace(trace, "cmd soft reset");
        self.io.command(SOFT_RESET)?;
        Self::trace(trace, "waiting for panel ready after soft reset");
        self.io.wait_while_busy(delay)?;
        Self::trace(trace, "panel ready after soft reset");

        Self::trace(trace, "cmd temp sensor");
        self.io.command(TEMP_SENSOR_CONTROL)?;
        self.io.data(&[0x80])?;

        Self::trace(trace, "cmd booster");
        self.io.command(BOOSTER_SOFT_START)?;
        self.io.data(&[0xAE, 0xC7, 0xC3, 0xC0, 0x40])?;

        Self::trace(trace, "cmd driver output");
        self.io.command(DRIVER_OUTPUT_CONTROL)?;
        self.io.data(&[
            ((NATIVE_HEIGHT - 1) & 0xFF) as u8,
            ((NATIVE_HEIGHT - 1) >> 8) as u8,
            0x02,
        ])?;

        Self::trace(trace, "cmd border waveform");
        self.io.command(BORDER_WAVEFORM)?;
        self.io.data(&[0x01])?;

        Self::trace(trace, "cmd vcom");
        self.io.command(WRITE_VCOM)?;
        self.io.data(&[0x3C])?;

        Ok(())
    }

    pub fn begin_full_frame(&mut self) -> Result<(), IO::Error> {
        self.begin_full_frame_with_trace(&mut |_| {})
    }

    pub fn begin_full_frame_with_trace<F>(&mut self, trace: &mut F) -> Result<(), IO::Error>
    where
        F: FnMut(&'static str),
    {
        Self::trace(trace, "cmd data entry");
        self.io.command(DATA_ENTRY_MODE)?;
        self.io.data(&[0x01])?;

        Self::trace(trace, "cmd ram x range");
        self.io.command(SET_RAM_X_RANGE)?;
        self.io.data(&[0x00, 0x00, 0x1F, 0x03])?;

        Self::trace(trace, "cmd ram y range");
        self.io.command(SET_RAM_Y_RANGE)?;
        self.io.data(&[0xDF, 0x01, 0x00, 0x00])?;

        Self::trace(trace, "cmd ram x counter");
        self.io.command(SET_RAM_X_COUNTER)?;
        self.io.data(&[0x00, 0x00])?;

        Self::trace(trace, "cmd ram y counter");
        self.io.command(SET_RAM_Y_COUNTER)?;
        self.io.data(&[0xDF, 0x01])?;

        Self::trace(trace, "cmd write bw ram");
        self.io.command(WRITE_RAM_BW)?;
        Ok(())
    }

    pub fn write_native_strip(&mut self, bytes: &[u8]) -> Result<(), IO::Error> {
        self.io.data(bytes)
    }

    pub fn write_full_red_plane_with_trace<F>(&mut self, trace: &mut F) -> Result<(), IO::Error>
    where
        F: FnMut(&'static str),
    {
        Self::trace(trace, "cmd write red ram");
        self.io.command(WRITE_RAM_RED)?;

        let blank = [0x00; 256];
        let mut remaining = FULL_FRAME_BYTES;
        while remaining > 0 {
            let chunk = remaining.min(blank.len());
            self.io.data(&blank[..chunk])?;
            remaining -= chunk;
        }

        Ok(())
    }

    pub fn refresh_full<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), IO::Error> {
        self.refresh_full_with_trace(delay, &mut |_| {})
    }

    pub fn refresh_full_with_trace<D, F>(
        &mut self,
        delay: &mut D,
        trace: &mut F,
    ) -> Result<(), IO::Error>
    where
        D: DelayNs,
        F: FnMut(&'static str),
    {
        Self::trace(trace, "cmd display update ctrl1");
        self.io.command(DISPLAY_UPDATE_CTRL1)?;
        self.io.data(&[CTRL1_BYPASS_RED])?;

        let mut mode = DISPLAY_UPDATE_CTRL2_FULL;
        if !self.display_on {
            mode |= DISPLAY_POWER_ON_BITS;
        }
        self.display_on = true;

        Self::trace(trace, "cmd display update ctrl2");
        self.io.command(DISPLAY_UPDATE_CTRL2)?;
        self.io.data(&[mode])?;

        Self::trace(trace, "waiting for panel ready after full refresh");
        self.io.command(MASTER_ACTIVATION)?;
        self.io.wait_while_busy(delay)?;
        Self::trace(trace, "panel ready after full refresh");

        Ok(())
    }

    pub fn sleep<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), IO::Error> {
        if self.display_on {
            self.io.command(DISPLAY_UPDATE_CTRL1)?;
            self.io.data(&[CTRL1_BYPASS_RED])?;
            self.io.command(DISPLAY_UPDATE_CTRL2)?;
            self.io.data(&[DISPLAY_POWER_OFF_BITS])?;
            self.io.command(MASTER_ACTIVATION)?;
            self.io.wait_while_busy(delay)?;
            self.display_on = false;
        }

        self.io.command(DEEP_SLEEP)?;
        self.io.data(&[0x01])?;
        Ok(())
    }
}
