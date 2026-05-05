use super::daily_mantra_provider::{DAILY_MANTRA_DEFAULT_IMAGE, daily_mantra_weekday_image_path};
use crate::vaachak_x4::time::weekday::Weekday;

pub const DAILY_MANTRA_EXPECTED_BITMAP_WIDTH: u16 = 800;
pub const DAILY_MANTRA_EXPECTED_BITMAP_HEIGHT: u16 = 480;
pub const DAILY_MANTRA_REQUIRED_IMAGE_COUNT: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DailyMantraAssetPath {
    pub key: &'static str,
    pub path: &'static str,
}

pub const DAILY_MANTRA_ASSET_PATHS: [DailyMantraAssetPath; DAILY_MANTRA_REQUIRED_IMAGE_COUNT] = [
    DailyMantraAssetPath {
        key: "mon",
        path: "/sleep/daily/mon.bmp",
    },
    DailyMantraAssetPath {
        key: "tue",
        path: "/sleep/daily/tue.bmp",
    },
    DailyMantraAssetPath {
        key: "wed",
        path: "/sleep/daily/wed.bmp",
    },
    DailyMantraAssetPath {
        key: "thu",
        path: "/sleep/daily/thu.bmp",
    },
    DailyMantraAssetPath {
        key: "fri",
        path: "/sleep/daily/fri.bmp",
    },
    DailyMantraAssetPath {
        key: "sat",
        path: "/sleep/daily/sat.bmp",
    },
    DailyMantraAssetPath {
        key: "sun",
        path: "/sleep/daily/sun.bmp",
    },
    DailyMantraAssetPath {
        key: "default",
        path: DAILY_MANTRA_DEFAULT_IMAGE,
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyMantraAssetStatus {
    Ready,
    Missing,
    InvalidBitmap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DailyMantraAssetCheck {
    pub asset: DailyMantraAssetPath,
    pub status: DailyMantraAssetStatus,
}

pub const fn daily_mantra_asset_path_for_weekday(weekday: Weekday) -> &'static str {
    daily_mantra_weekday_image_path(weekday)
}

pub fn is_daily_mantra_asset_path(path: &str) -> bool {
    DAILY_MANTRA_ASSET_PATHS
        .iter()
        .any(|asset| asset.path == path)
}

pub const fn daily_mantra_default_asset_path() -> &'static str {
    DAILY_MANTRA_DEFAULT_IMAGE
}

pub const fn daily_mantra_asset_dimensions() -> (u16, u16) {
    (
        DAILY_MANTRA_EXPECTED_BITMAP_WIDTH,
        DAILY_MANTRA_EXPECTED_BITMAP_HEIGHT,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        DAILY_MANTRA_ASSET_PATHS, DAILY_MANTRA_REQUIRED_IMAGE_COUNT, daily_mantra_asset_dimensions,
        daily_mantra_asset_path_for_weekday, daily_mantra_default_asset_path,
        is_daily_mantra_asset_path,
    };
    use crate::vaachak_x4::time::weekday::Weekday;

    #[test]
    fn asset_manifest_covers_weekdays_and_default_image() {
        assert_eq!(
            DAILY_MANTRA_ASSET_PATHS.len(),
            DAILY_MANTRA_REQUIRED_IMAGE_COUNT
        );
        assert_eq!(
            daily_mantra_asset_path_for_weekday(Weekday::Monday),
            "/sleep/daily/mon.bmp"
        );
        assert_eq!(
            daily_mantra_asset_path_for_weekday(Weekday::Sunday),
            "/sleep/daily/sun.bmp"
        );
        assert_eq!(
            daily_mantra_default_asset_path(),
            "/sleep/daily/default.bmp"
        );
    }

    #[test]
    fn asset_path_checker_accepts_only_known_daily_images() {
        assert!(is_daily_mantra_asset_path("/sleep/daily/tue.bmp"));
        assert!(is_daily_mantra_asset_path("/sleep/daily/default.bmp"));
        assert!(!is_daily_mantra_asset_path("/sleep/random.bmp"));
    }

    #[test]
    fn expected_bitmap_geometry_matches_x4_portrait_panel() {
        assert_eq!(daily_mantra_asset_dimensions(), (800, 480));
    }
}
