//! Compact bitmap font asset contracts.
//!
//! These types describe the Vaachak font asset layout without parsing files or
//! drawing glyph bitmaps. Runtime loading and rendering are intentionally left
//! to later integration work.

use super::font_catalog::FontDescriptor;
use super::script::ScriptClass;

pub const VFNT_MAGIC: [u8; 4] = *b"VFNT";
pub const VFNT_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum FontBitmapFormat {
    OneBpp,
    TwoBpp,
    FourBpp,
}

impl FontBitmapFormat {
    pub const fn bits_per_pixel(self) -> u8 {
        match self {
            Self::OneBpp => 1,
            Self::TwoBpp => 2,
            Self::FourBpp => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfntHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub header_len: u16,
    pub flags: u32,
    pub pixel_size: u16,
    pub line_height: u16,
    pub ascent: i16,
    pub descent: i16,
    pub glyph_count: u32,
    pub metrics_offset: u32,
    pub bitmap_index_offset: u32,
    pub bitmap_data_offset: u32,
    pub bitmap_data_len: u32,
    pub script: ScriptClass,
    pub bitmap_format: FontBitmapFormat,
}

impl VfntHeader {
    pub const fn uses_expected_magic(self) -> bool {
        self.magic[0] == VFNT_MAGIC[0]
            && self.magic[1] == VFNT_MAGIC[1]
            && self.magic[2] == VFNT_MAGIC[2]
            && self.magic[3] == VFNT_MAGIC[3]
    }

    pub const fn uses_supported_version(self) -> bool {
        self.version == VFNT_VERSION
    }

    pub const fn is_supported(self) -> bool {
        self.uses_expected_magic()
            && self.uses_supported_version()
            && self.header_len > 0
            && self.pixel_size > 0
            && self.line_height > 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfntGlyphMetrics {
    pub glyph_id: u32,
    pub advance_x: i16,
    pub advance_y: i16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfntGlyphBitmap {
    pub glyph_id: u32,
    pub offset: u32,
    pub len: u32,
    pub row_stride: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfntAssetInfo {
    pub descriptor: FontDescriptor,
    pub header: VfntHeader,
}

impl VfntAssetInfo {
    pub fn is_supported(self) -> bool {
        self.header.is_supported() && self.descriptor.script == self.header.script
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FontBitmapFormat, VFNT_MAGIC, VFNT_VERSION, VfntGlyphBitmap, VfntGlyphMetrics, VfntHeader,
    };
    use crate::vaachak_x4::text::ScriptClass;

    fn supported_header() -> VfntHeader {
        VfntHeader {
            magic: VFNT_MAGIC,
            version: VFNT_VERSION,
            header_len: 64,
            flags: 0,
            pixel_size: 22,
            line_height: 28,
            ascent: 22,
            descent: -6,
            glyph_count: 128,
            metrics_offset: 64,
            bitmap_index_offset: 2048,
            bitmap_data_offset: 4096,
            bitmap_data_len: 8192,
            script: ScriptClass::Devanagari,
            bitmap_format: FontBitmapFormat::OneBpp,
        }
    }

    #[test]
    fn vfnt_header_rejects_wrong_magic() {
        let mut header = supported_header();
        header.magic = *b"FONT";
        assert!(!header.uses_expected_magic());
        assert!(!header.is_supported());
    }

    #[test]
    fn vfnt_header_accepts_supported_contract() {
        let header = supported_header();
        assert!(header.uses_expected_magic());
        assert!(header.uses_supported_version());
        assert!(header.is_supported());
        assert_eq!(header.bitmap_format.bits_per_pixel(), 1);
    }

    #[test]
    fn glyph_contracts_preserve_bitmap_offsets_and_metrics() {
        let metrics = VfntGlyphMetrics {
            glyph_id: 42,
            advance_x: 12,
            advance_y: 0,
            bearing_x: 1,
            bearing_y: 18,
            width: 10,
            height: 20,
        };
        let bitmap = VfntGlyphBitmap {
            glyph_id: metrics.glyph_id,
            offset: 256,
            len: 40,
            row_stride: 2,
        };

        assert_eq!(bitmap.glyph_id, metrics.glyph_id);
        assert_eq!(bitmap.row_stride, 2);
    }
}
