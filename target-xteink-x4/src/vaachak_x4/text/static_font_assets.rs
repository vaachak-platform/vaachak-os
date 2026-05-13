//! Generated build-time static font asset bridge for Vaachak X4.
//!
//! Do not edit by hand. Regenerate with:
//!   python3 tools/build_static_font_assets_from_vfn.py

use core::sync::atomic::{AtomicU8, Ordering};

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::PrimitiveStyle};

use crate::vaachak_x4::text::sd_vfn_runtime;
use crate::vaachak_x4::x4_apps::fonts::bitmap::BitmapFont;
use crate::vaachak_x4::x4_apps::ui::{Alignment, Region};
use crate::vaachak_x4::x4_kernel::drivers::strip::StripBuffer;

pub const STATIC_READER_FONT_MARKER: &str = "reader-fonts=x4-reader-crossink-static-fonts-ok";
pub const STATIC_UI_FONT_MARKER: &str = "ui-fonts=x4-ui-crossink-static-fonts-ok";

static UI_FONT_SOURCE: AtomicU8 = AtomicU8::new(0);

const CHARIS18_VFN: &[u8] =
    include_bytes!("../../../../examples/sd-card/VAACHAK/FONTS/CHARIS18.VFN");
const BITTER18_VFN: &[u8] =
    include_bytes!("../../../../examples/sd-card/VAACHAK/FONTS/BITTER18.VFN");
const LEXEND18_VFN: &[u8] =
    include_bytes!("../../../../examples/sd-card/VAACHAK/FONTS/LEXEND18.VFN");
const INTER14_VFN: &[u8] = include_bytes!("../../../../examples/sd-card/VAACHAK/FONTS/INTER14.VFN");
const LEXUI14_VFN: &[u8] = include_bytes!("../../../../examples/sd-card/VAACHAK/FONTS/LEXUI14.VFN");

pub fn set_ui_font_source(source: u8) {
    UI_FONT_SOURCE.store(source.min(2), Ordering::Relaxed);
}

pub fn ui_font_source() -> u8 {
    UI_FONT_SOURCE.load(Ordering::Relaxed)
}

pub fn reader_font_for_source(source: u8) -> Option<&'static [u8]> {
    match source {
        1 => Some(CHARIS18_VFN), // Charis
        2 => Some(BITTER18_VFN), // Bitter
        3 => Some(LEXEND18_VFN), // Lexend Deca
        _ => None,
    }
}

pub fn ui_font_for_source(source: u8) -> Option<&'static [u8]> {
    match source {
        1 => Some(INTER14_VFN), // Inter
        2 => Some(LEXUI14_VFN), // Lexend Deca UI
        _ => None,
    }
}

pub fn draw_ui_text(
    strip: &mut StripBuffer,
    region: Region,
    text: &str,
    _fallback: &'static BitmapFont,
    alignment: Alignment,
    inverted: bool,
) -> bool {
    if inverted || !text.is_ascii() || !region.intersects(strip.logical_window()) {
        return false;
    }

    let Some(data) = ui_font_for_source(ui_font_source()) else {
        return false;
    };
    let Some(metrics) = sd_vfn_runtime::metrics(data) else {
        return false;
    };

    region
        .to_rect()
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(strip)
        .ok();

    let width = sd_vfn_runtime::measure_str(data, text) as u32;
    let height = u32::from(metrics.line_height.max(1));
    let pos = alignment.position(
        region,
        embedded_graphics::geometry::Size::new(width, height),
    );
    let baseline = pos.y + i32::from(metrics.ascent.max(1));
    sd_vfn_runtime::draw_str(strip, data, text, pos.x, baseline);
    true
}
