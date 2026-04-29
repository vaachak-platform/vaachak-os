//! Minimal Xteink X4 SSD1677 display smoke/home/input-navigation driver.
//!
//! Phase 7 keeps the proven Phase 5.4 display transport shape:
//! - DMA-backed `SpiDevice` chip-select ownership.
//! - SD chip-select kept high when the panel owns the shared SPI bus.
//! - Full-frame strip rendering, no full framebuffer.
//! - RED RAM then BW RAM before a full update.

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiDevice;

pub const X4_EPD_NATIVE_WIDTH: u16 = 800;
pub const X4_EPD_NATIVE_HEIGHT: u16 = 480;
pub const X4_EPD_LOGICAL_WIDTH: u16 = 480;
pub const X4_EPD_LOGICAL_HEIGHT: u16 = 800;
pub const X4_EPD_STRIP_ROWS: u16 = 40;
pub const X4_EPD_BYTES_PER_ROW: usize = X4_EPD_NATIVE_WIDTH as usize / 8;
pub const X4_EPD_STRIP_BYTES: usize = X4_EPD_BYTES_PER_ROW * X4_EPD_STRIP_ROWS as usize;
pub const X4_EPD_STRIP_COUNT: u16 = X4_EPD_NATIVE_HEIGHT / X4_EPD_STRIP_ROWS;

#[allow(dead_code)]
mod cmd {
    pub const DRIVER_OUTPUT_CONTROL: u8 = 0x01;
    pub const BOOSTER_SOFT_START: u8 = 0x0C;
    pub const DATA_ENTRY_MODE: u8 = 0x11;
    pub const SW_RESET: u8 = 0x12;
    pub const TEMPERATURE_SENSOR: u8 = 0x18;
    pub const MASTER_ACTIVATION: u8 = 0x20;
    pub const DISPLAY_UPDATE_CONTROL_1: u8 = 0x21;
    pub const DISPLAY_UPDATE_CONTROL_2: u8 = 0x22;
    pub const WRITE_RAM_BW: u8 = 0x24;
    pub const WRITE_RAM_RED: u8 = 0x26;
    pub const BORDER_WAVEFORM: u8 = 0x3C;
    pub const SET_RAM_X_RANGE: u8 = 0x44;
    pub const SET_RAM_Y_RANGE: u8 = 0x45;
    pub const SET_RAM_X_COUNTER: u8 = 0x4E;
    pub const SET_RAM_Y_COUNTER: u8 = 0x4F;
}

pub struct X4Ssd1677Smoke<SPI, DC, RST, BUSY> {
    spi: SPI,
    dc: DC,
    rst: RST,
    busy: BUSY,
    initialised: bool,
}

impl<SPI, DC, RST, BUSY, E> X4Ssd1677Smoke<SPI, DC, RST, BUSY>
where
    SPI: SpiDevice<Error = E>,
    DC: OutputPin,
    RST: OutputPin,
    BUSY: InputPin,
{
    pub fn new(spi: SPI, dc: DC, rst: RST, busy: BUSY) -> Self {
        Self {
            spi,
            dc,
            rst,
            busy,
            initialised: false,
        }
    }

    pub fn init<D: DelayNs>(&mut self, delay: &mut D) {
        self.reset(delay);
        self.init_display(delay);
    }

    pub fn draw_phase5_smoke<D: DelayNs>(&mut self, delay: &mut D) {
        self.draw_full_frame(delay, |strip_idx, strip| {
            render_smoke_strip(strip_idx, strip)
        });
    }

    pub fn draw_phase7_home<D: DelayNs>(&mut self, delay: &mut D, sd_ok: bool, battery_pct: u8) {
        self.draw_phase8_home(delay, sd_ok, battery_pct, 0);
    }

    pub fn draw_phase8_home<D: DelayNs>(
        &mut self,
        delay: &mut D,
        sd_ok: bool,
        battery_pct: u8,
        selected: u8,
    ) {
        self.draw_full_frame(delay, |strip_idx, strip| {
            render_home_nav_strip(strip_idx, strip, sd_ok, battery_pct, selected)
        });
    }

    pub fn is_busy(&mut self) -> bool {
        self.busy.is_high().unwrap_or(false)
    }

    fn draw_full_frame<D, F>(&mut self, delay: &mut D, mut render: F)
    where
        D: DelayNs,
        F: FnMut(u16, &mut [u8; X4_EPD_STRIP_BYTES]),
    {
        if !self.initialised {
            self.init_display(delay);
        }

        delay.delay_ms(1);
        let mut strip = [0xFFu8; X4_EPD_STRIP_BYTES];

        for ram_cmd in [cmd::WRITE_RAM_RED, cmd::WRITE_RAM_BW] {
            self.set_ram_area(0, 0, X4_EPD_NATIVE_WIDTH, X4_EPD_NATIVE_HEIGHT);
            self.send_command(ram_cmd);
            delay.delay_ms(1);

            for strip_idx in 0..X4_EPD_STRIP_COUNT {
                render(strip_idx, &mut strip);
                self.send_data(&strip);
            }
        }

        self.send_command(cmd::DISPLAY_UPDATE_CONTROL_1);
        self.send_data(&[0x40, 0x00]);

        self.send_command(cmd::DISPLAY_UPDATE_CONTROL_2);
        self.send_data(&[0xF7]);

        self.send_command(cmd::MASTER_ACTIVATION);
        self.wait_busy(delay, 20_000);
    }

    fn reset<D: DelayNs>(&mut self, delay: &mut D) {
        let _ = self.rst.set_high();
        delay.delay_ms(20);
        let _ = self.rst.set_low();
        delay.delay_ms(2);
        let _ = self.rst.set_high();
        delay.delay_ms(20);
    }

    fn init_display<D: DelayNs>(&mut self, delay: &mut D) {
        self.send_command(cmd::SW_RESET);
        delay.delay_ms(10);

        self.send_command(cmd::TEMPERATURE_SENSOR);
        self.send_data(&[0x80]);

        self.send_command(cmd::BOOSTER_SOFT_START);
        self.send_data(&[0xAE, 0xC7, 0xC3, 0xC0, 0x80]);

        self.send_command(cmd::DRIVER_OUTPUT_CONTROL);
        self.send_data(&[
            ((X4_EPD_NATIVE_HEIGHT - 1) & 0xFF) as u8,
            ((X4_EPD_NATIVE_HEIGHT - 1) >> 8) as u8,
            0x02,
        ]);

        self.send_command(cmd::BORDER_WAVEFORM);
        self.send_data(&[0x01]);

        self.set_ram_area(0, 0, X4_EPD_NATIVE_WIDTH, X4_EPD_NATIVE_HEIGHT);
        self.initialised = true;
    }

    // SSD1677 gates wired in reverse on the X4: X increments, Y decrements.
    fn set_ram_area(&mut self, x: u16, y: u16, w: u16, h: u16) {
        let y_flipped = X4_EPD_NATIVE_HEIGHT - y - h;

        self.send_command(cmd::DATA_ENTRY_MODE);
        self.send_data(&[0x01]);

        self.send_command(cmd::SET_RAM_X_RANGE);
        self.send_data(&[
            (x & 0xFF) as u8,
            (x >> 8) as u8,
            ((x + w - 1) & 0xFF) as u8,
            ((x + w - 1) >> 8) as u8,
        ]);

        self.send_command(cmd::SET_RAM_Y_RANGE);
        self.send_data(&[
            ((y_flipped + h - 1) & 0xFF) as u8,
            ((y_flipped + h - 1) >> 8) as u8,
            (y_flipped & 0xFF) as u8,
            (y_flipped >> 8) as u8,
        ]);

        self.send_command(cmd::SET_RAM_X_COUNTER);
        self.send_data(&[(x & 0xFF) as u8, (x >> 8) as u8]);

        self.send_command(cmd::SET_RAM_Y_COUNTER);
        self.send_data(&[
            ((y_flipped + h - 1) & 0xFF) as u8,
            ((y_flipped + h - 1) >> 8) as u8,
        ]);
    }

    fn send_command(&mut self, value: u8) {
        let _ = self.dc.set_low();
        let _ = self.spi.write(&[value]);
        let _ = self.dc.set_high();
    }

    fn send_data(&mut self, bytes: &[u8]) {
        let _ = self.dc.set_high();
        let _ = self.spi.write(bytes);
    }

    fn wait_busy<D: DelayNs>(&mut self, delay: &mut D, timeout_ms: u32) {
        let mut elapsed = 0;
        while elapsed < timeout_ms {
            if self.busy.is_low().unwrap_or(true) {
                return;
            }
            delay.delay_ms(1);
            elapsed += 1;
        }
    }
}

fn render_smoke_strip(strip_idx: u16, out: &mut [u8; X4_EPD_STRIP_BYTES]) {
    render_strip_with(strip_idx, out, smoke_pixel);
}

#[cfg(test)]
fn render_home_strip(
    strip_idx: u16,
    out: &mut [u8; X4_EPD_STRIP_BYTES],
    sd_ok: bool,
    battery_pct: u8,
) {
    render_home_nav_strip(strip_idx, out, sd_ok, battery_pct, 0);
}

fn render_home_nav_strip(
    strip_idx: u16,
    out: &mut [u8; X4_EPD_STRIP_BYTES],
    sd_ok: bool,
    battery_pct: u8,
    selected: u8,
) {
    render_strip_with(strip_idx, out, |x, y| {
        home_nav_pixel(x, y, sd_ok, battery_pct, selected)
    });
}

fn render_strip_with<F>(strip_idx: u16, out: &mut [u8; X4_EPD_STRIP_BYTES], mut pixel: F)
where
    F: FnMut(u16, u16) -> bool,
{
    out.fill(0xFF);

    let native_y0 = strip_idx * X4_EPD_STRIP_ROWS;
    for row in 0..X4_EPD_STRIP_ROWS {
        let py = native_y0 + row;
        for px in 0..X4_EPD_NATIVE_WIDTH {
            // Inverse of the proven Deg270 mapping from x4-reader-os-rs:
            // physical = (logical_y, native_height - 1 - logical_x)
            let lx = X4_EPD_NATIVE_HEIGHT - 1 - py;
            let ly = px;
            if pixel(lx, ly) {
                set_black(out, row, px);
            }
        }
    }
}

fn set_black(out: &mut [u8; X4_EPD_STRIP_BYTES], row: u16, x: u16) {
    let idx = row as usize * X4_EPD_BYTES_PER_ROW + (x / 8) as usize;
    let bit = 7 - (x % 8);
    out[idx] &= !(1 << bit);
}

fn smoke_pixel(x: u16, y: u16) -> bool {
    if !(4..X4_EPD_LOGICAL_WIDTH - 4).contains(&x) || !(4..X4_EPD_LOGICAL_HEIGHT - 4).contains(&y) {
        return true;
    }
    if (72..78).contains(&y) || (376..382).contains(&y) || (698..704).contains(&y) {
        return true;
    }

    if (24..72).contains(&x) && (24..58).contains(&y) {
        return true;
    }
    if (408..456).contains(&x) && (742..776).contains(&y) {
        return true;
    }

    text_pixel(b"VAACHAKOS", x, y, 78, 106, 6)
        || text_pixel(b"X4 DISPLAY SMOKE", x, y, 96, 204, 3)
        || text_pixel(b"PHASE 5", x, y, 156, 284, 4)
        || text_pixel(b"480X800 PORTRAIT", x, y, 108, 430, 2)
        || text_pixel(b"BOOT OK", x, y, 168, 640, 3)
}

#[cfg(test)]
fn home_pixel(x: u16, y: u16, sd_ok: bool, battery_pct: u8) -> bool {
    home_nav_pixel(x, y, sd_ok, battery_pct, 0)
}

fn home_nav_pixel(x: u16, y: u16, sd_ok: bool, battery_pct: u8, selected: u8) -> bool {
    if !(4..X4_EPD_LOGICAL_WIDTH - 4).contains(&x) || !(4..X4_EPD_LOGICAL_HEIGHT - 4).contains(&y) {
        return true;
    }
    if (132..138).contains(&y) || (672..678).contains(&y) {
        return true;
    }

    if selected_marker_pixel(x, y, selected) {
        return true;
    }

    text_pixel(b"VAACHAKOS", x, y, 66, 58, 5)
        || text_pixel(b"INPUT NAV SMOKE", x, y, 92, 116, 2)
        || text_pixel(b"CONTINUE", x, y, 82, 196, 3)
        || text_pixel(b"LIBRARY", x, y, 82, 270, 3)
        || text_pixel(b"SETTINGS", x, y, 82, 344, 3)
        || text_pixel(b"SYSTEM", x, y, 82, 418, 3)
        || text_pixel(b"UP DOWN MOVE", x, y, 110, 540, 2)
        || text_pixel(b"SELECT LOGS ITEM", x, y, 88, 584, 2)
        || text_pixel(if sd_ok { b"SD OK" } else { b"SD NO" }, x, y, 28, 724, 2)
        || battery_status_pixel(x, y, 328, 724, 2, battery_pct)
}

fn selected_marker_pixel(x: u16, y: u16, selected: u8) -> bool {
    let selected = selected.min(3) as u16;
    let y0 = 206 + selected * 74;
    (34..56).contains(&x) && (y0..y0 + 22).contains(&y)
}

fn battery_status_pixel(x: u16, y: u16, x0: u16, y0: u16, scale: u16, battery_pct: u8) -> bool {
    if text_pixel(b"BAT", x, y, x0, y0, scale) {
        return true;
    }

    let pct = battery_pct.min(100);
    let advance = 6 * scale;
    let digit_x0 = x0 + 4 * advance;

    if pct >= 100 {
        return char_pixel(b'1', x, y, digit_x0, y0, scale)
            || char_pixel(b'0', x, y, digit_x0 + advance, y0, scale)
            || char_pixel(b'0', x, y, digit_x0 + 2 * advance, y0, scale);
    }

    let tens = pct / 10;
    let ones = pct % 10;
    char_pixel(b'0' + tens, x, y, digit_x0, y0, scale)
        || char_pixel(b'0' + ones, x, y, digit_x0 + advance, y0, scale)
}

fn text_pixel(text: &[u8], x: u16, y: u16, x0: u16, y0: u16, scale: u16) -> bool {
    let glyph_w = 5 * scale;
    let glyph_h = 7 * scale;
    let advance = 6 * scale;

    if y < y0 || y >= y0 + glyph_h || x < x0 {
        return false;
    }

    let rel_x = x - x0;
    let idx = (rel_x / advance) as usize;
    if idx >= text.len() {
        return false;
    }

    let in_glyph_x = rel_x % advance;
    if in_glyph_x >= glyph_w {
        return false;
    }

    let col = (in_glyph_x / scale) as usize;
    let row = ((y - y0) / scale) as usize;
    let glyph = glyph_for(text[idx]);
    (glyph[row] & (1 << (4 - col))) != 0
}

fn char_pixel(ch: u8, x: u16, y: u16, x0: u16, y0: u16, scale: u16) -> bool {
    let glyph_w = 5 * scale;
    let glyph_h = 7 * scale;

    if y < y0 || y >= y0 + glyph_h || x < x0 || x >= x0 + glyph_w {
        return false;
    }

    let col = ((x - x0) / scale) as usize;
    let row = ((y - y0) / scale) as usize;
    let glyph = glyph_for(ch);
    (glyph[row] & (1 << (4 - col))) != 0
}

fn glyph_for(ch: u8) -> [u8; 7] {
    match ch {
        b'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        b'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        b'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        b'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        b'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        b'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        b'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        b'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        b'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        b'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        b'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        b'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        b'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        b'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        b'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        b'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        b'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        b'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        b'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        b'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        b'0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        b'1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        b'2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        b'3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        b'4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        b'5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        b'6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        b'7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        b'8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        b'9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b11100,
        ],
        b' ' => [0, 0, 0, 0, 0, 0, 0],
        _ => [0, 0, 0, 0, 0, 0, 0],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_strip_has_expected_size() {
        let mut strip = [0xFFu8; X4_EPD_STRIP_BYTES];
        render_smoke_strip(0, &mut strip);
        assert_eq!(strip.len(), 4_000);
        assert!(strip.iter().any(|b| *b != 0xFF));
    }

    #[test]
    fn logical_smoke_marks_orientation_corners() {
        assert!(smoke_pixel(2, 2));
        assert!(smoke_pixel(
            X4_EPD_LOGICAL_WIDTH - 3,
            X4_EPD_LOGICAL_HEIGHT - 3
        ));
        assert!(smoke_pixel(30, 30));
        assert!(smoke_pixel(430, 760));
        assert!(!smoke_pixel(240, 520));
    }

    #[test]
    fn phase7_home_strip_has_expected_size() {
        let mut strip = [0xFFu8; X4_EPD_STRIP_BYTES];
        render_home_strip(0, &mut strip, true, 92);
        assert_eq!(strip.len(), 4_000);
        assert!(strip.iter().any(|b| *b != 0xFF));
    }

    #[test]
    fn phase8_home_nav_strip_has_expected_size() {
        let mut strip = [0xFFu8; X4_EPD_STRIP_BYTES];
        render_home_nav_strip(0, &mut strip, true, 92, 2);
        assert_eq!(strip.len(), 4_000);
        assert!(strip.iter().any(|b| *b != 0xFF));
    }

    #[test]
    fn logical_home_marks_title_and_status() {
        assert!(home_pixel(2, 2, true, 92));
        assert!(home_pixel(38, 212, true, 92));
        assert!(home_pixel(66, 58, true, 92));
        assert!(home_pixel(30, 724, true, 92));
        assert!(!home_pixel(240, 510, true, 92));
    }

    #[test]
    fn logical_home_moves_selection_marker() {
        assert!(home_nav_pixel(38, 212, true, 92, 0));
        assert!(!home_nav_pixel(38, 212, true, 92, 1));
        assert!(home_nav_pixel(38, 286, true, 92, 1));
        assert!(home_nav_pixel(38, 434, true, 92, 3));
    }
}
