//! Daily Mantra app status model.
//!
//! This module is UI-agnostic. It exposes the current entry and the expected
//! sleep-image path so Home, Apps, Settings, or the sleep manager can show the
//! same status without duplicating logic.

use crate::vaachak_x4::sleep::daily_mantra_asset_status::daily_mantra_asset_path_for_weekday;
use crate::vaachak_x4::sleep::daily_mantra_provider::{
    DAILY_MANTRA_DEFAULT_IMAGE, DailyMantraSleepImage, DailyMantraSleepProvider,
};
use crate::vaachak_x4::time::date_provider::{DateProvider, DateTrust};
use crate::vaachak_x4::time::weekday::Weekday;

use super::daily_text::{DailyTextEntry, entry_for_weekday};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyMantraStatusKind {
    WeekdayImageReady,
    FallbackImageRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DailyMantraAppStatus<'a> {
    pub kind: DailyMantraStatusKind,
    pub weekday: Option<Weekday>,
    pub date_trust: DateTrust,
    pub entry: Option<&'a DailyTextEntry>,
    pub image_path: &'a str,
    pub title: &'static str,
    pub detail: &'static str,
}

impl<'a> DailyMantraAppStatus<'a> {
    pub const fn uses_fallback(self) -> bool {
        matches!(self.kind, DailyMantraStatusKind::FallbackImageRequired)
    }
}

pub fn daily_mantra_status_for_provider<P: DateProvider>(
    date_provider: &P,
) -> DailyMantraAppStatus<'static> {
    let provider = DailyMantraSleepProvider::prepared_weekday_bitmaps();
    match provider.resolve_for_provider(date_provider) {
        DailyMantraSleepImage::WeekdayImage {
            weekday,
            path,
            date_trust,
        } => DailyMantraAppStatus {
            kind: DailyMantraStatusKind::WeekdayImageReady,
            weekday: Some(weekday),
            date_trust,
            entry: Some(entry_for_time_weekday(weekday)),
            image_path: path,
            title: "Daily Mantra",
            detail: daily_mantra_asset_path_for_weekday(weekday),
        },
        DailyMantraSleepImage::FallbackImage {
            path, date_trust, ..
        } => DailyMantraAppStatus {
            kind: DailyMantraStatusKind::FallbackImageRequired,
            weekday: None,
            date_trust,
            entry: None,
            image_path: if path.is_empty() {
                DAILY_MANTRA_DEFAULT_IMAGE
            } else {
                path
            },
            title: "Daily Mantra",
            detail: "Date unavailable; using default sleep image",
        },
    }
}

pub fn daily_mantra_preview_lines<'a>(status: DailyMantraAppStatus<'a>) -> [&'a str; 4] {
    match status.entry {
        Some(entry) => [
            entry.sanskrit,
            entry.hindi,
            entry.english,
            status.image_path,
        ],
        None => [
            "Daily Mantra",
            "Date unavailable",
            "Using default image",
            status.image_path,
        ],
    }
}

fn entry_for_time_weekday(weekday: Weekday) -> &'static DailyTextEntry {
    match weekday {
        Weekday::Monday => entry_for_weekday(super::daily_text::Weekday::Monday),
        Weekday::Tuesday => entry_for_weekday(super::daily_text::Weekday::Tuesday),
        Weekday::Wednesday => entry_for_weekday(super::daily_text::Weekday::Wednesday),
        Weekday::Thursday => entry_for_weekday(super::daily_text::Weekday::Thursday),
        Weekday::Friday => entry_for_weekday(super::daily_text::Weekday::Friday),
        Weekday::Saturday => entry_for_weekday(super::daily_text::Weekday::Saturday),
        Weekday::Sunday => entry_for_weekday(super::daily_text::Weekday::Sunday),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DailyMantraStatusKind, daily_mantra_preview_lines, daily_mantra_status_for_provider,
    };
    use crate::vaachak_x4::time::date_provider::{CalendarDate, DateTrust, FixedDateProvider};

    #[test]
    fn trusted_date_selects_weekday_mantra_image() {
        let provider = FixedDateProvider::new(CalendarDate::new(2026, 5, 4), DateTrust::UserSet);
        let status = daily_mantra_status_for_provider(&provider);
        assert_eq!(status.kind, DailyMantraStatusKind::WeekdayImageReady);
        assert_eq!(status.image_path, "/sleep/daily/mon.bmp");
        let lines = daily_mantra_preview_lines(status);
        assert_eq!(lines[0], "ॐ नमः शिवाय");
    }

    #[test]
    fn unknown_date_uses_default_image() {
        let provider = FixedDateProvider::unknown();
        let status = daily_mantra_status_for_provider(&provider);
        assert_eq!(status.kind, DailyMantraStatusKind::FallbackImageRequired);
        assert_eq!(status.image_path, "/sleep/daily/default.bmp");
        assert!(status.uses_fallback());
    }
}
