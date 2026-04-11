#![no_std]

use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_6X13_BOLD, FONT_10X20},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};

pub const LOGICAL_WIDTH: u32 = 480;
pub const LOGICAL_HEIGHT: u32 = 800;

pub fn draw_bringup_screen<T>(target: &mut T)
where
    T: DrawTarget<Color = BinaryColor>,
{
    let _ = target.clear(BinaryColor::Off);

    let title = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let detail = MonoTextStyle::new(&FONT_6X13_BOLD, BinaryColor::On);

    let _ = Text::with_baseline("VAACHAKOS", Point::new(32, 92), title, Baseline::Top).draw(target);
    let _ =
        Text::with_baseline("X4 BRING-UP", Point::new(32, 126), title, Baseline::Top).draw(target);
    let _ =
        Text::with_baseline("DISPLAY OK", Point::new(32, 160), title, Baseline::Top).draw(target);

    let _ = Rectangle::new(Point::new(32, 210), Size::new(320, 2))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(target);

    let _ = Text::with_baseline("REFRESH FULL", Point::new(32, 236), detail, Baseline::Top)
        .draw(target);
    let _ =
        Text::with_baseline("EPD 800X480", Point::new(32, 260), detail, Baseline::Top).draw(target);
    let _ = Text::with_baseline("ROT 270", Point::new(32, 284), detail, Baseline::Top).draw(target);
}
