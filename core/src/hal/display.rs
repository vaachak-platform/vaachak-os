#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefreshMode {
    Full,
    Partial,
    Fast,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayDepth {
    Mono1Bit,
    Gray2Bit,
}

pub trait DisplayHal {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn depth(&self) -> DisplayDepth;

    fn begin_frame(&mut self);
    fn draw_strip(&mut self, y_start: u16, buf: &[u8]);
    fn end_frame(&mut self, mode: RefreshMode);
    fn sleep(&mut self);

    fn set_sunlight_fix(&mut self, _enabled: bool) {}
}
