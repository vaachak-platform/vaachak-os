use super::daily_mantra_provider::{DailyMantraSleepImage, DailyMantraSleepProvider};
use crate::vaachak_x4::time::date_provider::{DateProvider, ResolvedDate};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectedSleepBitmap<'a> {
    pub path: &'a str,
    pub used_fallback: bool,
}

impl<'a> SelectedSleepBitmap<'a> {
    pub const fn new(path: &'a str, used_fallback: bool) -> Self {
        Self {
            path,
            used_fallback,
        }
    }
}

pub fn select_daily_mantra_sleep_bitmap<P: DateProvider>(
    date_provider: &P,
) -> SelectedSleepBitmap<'static> {
    select_daily_mantra_sleep_bitmap_for_date(date_provider.resolved_date())
}

pub fn select_daily_mantra_sleep_bitmap_for_date(
    resolved_date: ResolvedDate,
) -> SelectedSleepBitmap<'static> {
    let provider = DailyMantraSleepProvider::prepared_weekday_bitmaps();
    match provider.resolve(resolved_date) {
        DailyMantraSleepImage::WeekdayImage { path, .. } => SelectedSleepBitmap::new(path, false),
        DailyMantraSleepImage::FallbackImage { path, .. } => SelectedSleepBitmap::new(path, true),
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectedSleepBitmap, select_daily_mantra_sleep_bitmap_for_date};
    use crate::vaachak_x4::time::date_provider::{CalendarDate, DateTrust, ResolvedDate};

    #[test]
    fn trusted_date_uses_weekday_bitmap() {
        let selected = select_daily_mantra_sleep_bitmap_for_date(ResolvedDate::new(
            CalendarDate::new(2026, 5, 5),
            DateTrust::SyncProvided,
        ));
        assert_eq!(
            selected,
            SelectedSleepBitmap::new("/sleep/daily/tue.bmp", false)
        );
    }

    #[test]
    fn untrusted_date_uses_default_bitmap() {
        let selected = select_daily_mantra_sleep_bitmap_for_date(ResolvedDate::new(
            CalendarDate::new(2026, 5, 5),
            DateTrust::Unknown,
        ));
        assert_eq!(
            selected,
            SelectedSleepBitmap::new("/sleep/daily/default.bmp", true)
        );
    }
}
