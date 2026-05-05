#![allow(dead_code)]

use crate::vaachak_x4::contracts::display_geometry::VaachakDisplayGeometry;

pub struct VaachakDisplayGeometryRuntimeBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRotation {
    Deg270,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl VaachakDisplayRect {
    pub const fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }

    pub const fn right(self) -> u16 {
        self.x + self.w
    }

    pub const fn bottom(self) -> u16 {
        self.y + self.h
    }

    pub const fn fits_within(self, bounds: Self) -> bool {
        self.x >= bounds.x
            && self.y >= bounds.y
            && self.right() <= bounds.right()
            && self.bottom() <= bounds.bottom()
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayGeometryRuntimeReport {
    pub native_bounds_ok: bool,
    pub logical_bounds_ok: bool,
    pub rotation_mapping_ok: bool,
    pub strip_mapping_ok: bool,
    pub reader_bounds_ok: bool,
    pub physical_display_init_owned: bool,
    pub refresh_or_strip_render_owned: bool,
}

impl VaachakDisplayGeometryRuntimeReport {
    pub const fn preflight_ok(self) -> bool {
        self.native_bounds_ok
            && self.logical_bounds_ok
            && self.rotation_mapping_ok
            && self.strip_mapping_ok
            && self.reader_bounds_ok
            && !self.physical_display_init_owned
            && !self.refresh_or_strip_render_owned
    }
}

impl VaachakDisplayGeometryRuntimeBridge {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned display geometry runtime facade";
    pub const PHYSICAL_DISPLAY_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_DISPLAY_INIT_OWNED_BY_BRIDGE: bool = false;
    pub const REFRESH_OR_STRIP_RENDER_OWNED_BY_BRIDGE: bool = false;

    pub const NATIVE_BOUNDS: VaachakDisplayRect = VaachakDisplayRect::new(
        0,
        0,
        VaachakDisplayGeometry::NATIVE_WIDTH,
        VaachakDisplayGeometry::NATIVE_HEIGHT,
    );
    pub const LOGICAL_BOUNDS: VaachakDisplayRect = VaachakDisplayRect::new(
        0,
        0,
        VaachakDisplayGeometry::LOGICAL_WIDTH,
        VaachakDisplayGeometry::LOGICAL_HEIGHT,
    );

    pub const STATUS_BAR_HEIGHT: u16 = 4;
    pub const READER_MARGIN: u16 = 8;
    pub const READER_HEADER_Y: u16 = Self::STATUS_BAR_HEIGHT + 4 - 1;
    pub const READER_HEADER_H: u16 = 22;
    pub const READER_TEXT_Y: u16 = Self::READER_HEADER_Y + Self::READER_HEADER_H + 4;
    pub const READER_TEXT_W: u16 =
        VaachakDisplayGeometry::LOGICAL_WIDTH - (2 * Self::READER_MARGIN);
    pub const READER_TEXT_H: u16 = VaachakDisplayGeometry::LOGICAL_HEIGHT - Self::READER_TEXT_Y - 4;

    pub fn active_runtime_preflight() -> bool {
        Self::preflight_report().preflight_ok()
    }

    pub fn preflight_report() -> VaachakDisplayGeometryRuntimeReport {
        VaachakDisplayGeometryRuntimeReport {
            native_bounds_ok: Self::native_bounds_ok(),
            logical_bounds_ok: Self::logical_bounds_ok(),
            rotation_mapping_ok: Self::rotation_mapping_ok(),
            strip_mapping_ok: Self::strip_mapping_ok(),
            reader_bounds_ok: Self::reader_bounds_ok(),
            physical_display_init_owned: Self::PHYSICAL_DISPLAY_INIT_OWNED_BY_BRIDGE,
            refresh_or_strip_render_owned: Self::REFRESH_OR_STRIP_RENDER_OWNED_BY_BRIDGE,
        }
    }

    pub const fn logical_to_native_rect(rect: VaachakDisplayRect) -> VaachakDisplayRect {
        VaachakDisplayRect {
            x: rect.y,
            y: VaachakDisplayGeometry::NATIVE_HEIGHT - rect.x - rect.w,
            w: rect.h,
            h: rect.w,
        }
    }

    pub const fn native_strip_rect(strip_idx: u16) -> VaachakDisplayRect {
        let y = strip_idx * VaachakDisplayGeometry::STRIP_ROWS;
        let remaining = VaachakDisplayGeometry::NATIVE_HEIGHT - y;
        let h = if remaining < VaachakDisplayGeometry::STRIP_ROWS {
            remaining
        } else {
            VaachakDisplayGeometry::STRIP_ROWS
        };
        VaachakDisplayRect::new(0, y, VaachakDisplayGeometry::NATIVE_WIDTH, h)
    }

    pub const fn native_strip_count() -> u16 {
        VaachakDisplayGeometry::NATIVE_HEIGHT / VaachakDisplayGeometry::STRIP_ROWS
    }

    pub const fn reader_text_bounds() -> VaachakDisplayRect {
        VaachakDisplayRect::new(
            Self::READER_MARGIN,
            Self::READER_TEXT_Y,
            Self::READER_TEXT_W,
            Self::READER_TEXT_H,
        )
    }

    pub const fn reader_page_bounds() -> VaachakDisplayRect {
        VaachakDisplayRect::new(
            0,
            Self::READER_HEADER_Y,
            VaachakDisplayGeometry::LOGICAL_WIDTH,
            VaachakDisplayGeometry::LOGICAL_HEIGHT - Self::READER_HEADER_Y,
        )
    }

    fn native_bounds_ok() -> bool {
        Self::NATIVE_BOUNDS.w == 800
            && Self::NATIVE_BOUNDS.h == 480
            && VaachakDisplayGeometry::ROTATION_DEGREES == 270
            && matches!(
                VaachakDisplayRotation::Deg270,
                VaachakDisplayRotation::Deg270
            )
    }

    fn logical_bounds_ok() -> bool {
        Self::LOGICAL_BOUNDS.w == 480
            && Self::LOGICAL_BOUNDS.h == 800
            && (u32::from(Self::NATIVE_BOUNDS.w) * u32::from(Self::NATIVE_BOUNDS.h))
                == (u32::from(Self::LOGICAL_BOUNDS.w) * u32::from(Self::LOGICAL_BOUNDS.h))
    }

    fn rotation_mapping_ok() -> bool {
        Self::logical_to_native_rect(Self::LOGICAL_BOUNDS) == Self::NATIVE_BOUNDS
            && Self::logical_to_native_rect(VaachakDisplayRect::new(8, 31, 464, 765))
                == VaachakDisplayRect::new(31, 8, 765, 464)
    }

    fn strip_mapping_ok() -> bool {
        Self::native_strip_count() == 12
            && Self::native_strip_rect(0) == VaachakDisplayRect::new(0, 0, 800, 40)
            && Self::native_strip_rect(11) == VaachakDisplayRect::new(0, 440, 800, 40)
    }

    fn reader_bounds_ok() -> bool {
        let text = Self::reader_text_bounds();
        let page = Self::reader_page_bounds();
        text == VaachakDisplayRect::new(8, 29, 464, 767)
            && page == VaachakDisplayRect::new(0, 3, 480, 797)
            && text.fits_within(Self::LOGICAL_BOUNDS)
            && page.fits_within(Self::LOGICAL_BOUNDS)
            && text.intersects(page)
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakDisplayGeometryRuntimeBridge, VaachakDisplayRect};

    #[test]
    fn runtime_display_geometry_probe_is_pure_and_valid() {
        assert!(VaachakDisplayGeometryRuntimeBridge::active_runtime_preflight());
    }

    #[test]
    fn maps_logical_portrait_rect_to_native_landscape_rect() {
        assert_eq!(
            VaachakDisplayGeometryRuntimeBridge::logical_to_native_rect(VaachakDisplayRect::new(
                0, 0, 480, 800
            )),
            VaachakDisplayRect::new(0, 0, 800, 480)
        );
    }
}
