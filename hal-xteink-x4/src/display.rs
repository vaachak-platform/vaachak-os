use vaachak_core::hal::{DisplayDepth, DisplayHal, RefreshMode};

#[derive(Default)]
pub struct X4Display;

impl DisplayHal for X4Display {
    fn width(&self) -> u16 { 480 }
    fn height(&self) -> u16 { 800 }
    fn depth(&self) -> DisplayDepth { DisplayDepth::Mono1Bit }

    fn begin_frame(&mut self) {}
    fn draw_strip(&mut self, _y_start: u16, _buf: &[u8]) {}
    fn end_frame(&mut self, _mode: RefreshMode) {}
    fn sleep(&mut self) {}

    fn set_sunlight_fix(&mut self, _enabled: bool) {}
}
