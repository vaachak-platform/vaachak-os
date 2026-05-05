use crate::vaachak_x4::time::date_provider::{DateProvider, DateTrust, ResolvedDate};
use crate::vaachak_x4::time::weekday::Weekday;

pub const DAILY_MANTRA_SLEEP_DIR: &str = "/sleep/daily";
pub const DAILY_MANTRA_DEFAULT_IMAGE: &str = "/sleep/daily/default.bmp";
pub const SYSTEM_LIGHT_SLEEP_IMAGE: &str = "/sleep/light.bmp";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyMantraSchedule {
    Weekday,
    MonthDay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyMantraImagePolicy {
    PreparedBitmap,
    DynamicText,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyMantraFallbackReason {
    DateUnknown,
    DateInvalid,
    UnsupportedSchedule,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyMantraSleepImage<'a> {
    WeekdayImage {
        weekday: Weekday,
        path: &'a str,
        date_trust: DateTrust,
    },
    FallbackImage {
        path: &'a str,
        reason: DailyMantraFallbackReason,
        date_trust: DateTrust,
    },
}

impl<'a> DailyMantraSleepImage<'a> {
    pub const fn path(self) -> &'a str {
        match self {
            Self::WeekdayImage { path, .. } | Self::FallbackImage { path, .. } => path,
        }
    }

    pub const fn is_fallback(self) -> bool {
        match self {
            Self::WeekdayImage { .. } => false,
            Self::FallbackImage { .. } => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DailyMantraSleepProvider<'a> {
    pub schedule: DailyMantraSchedule,
    pub image_policy: DailyMantraImagePolicy,
    pub daily_dir: &'a str,
    pub default_image: &'a str,
}

impl<'a> DailyMantraSleepProvider<'a> {
    pub const fn prepared_weekday_bitmaps() -> Self {
        Self {
            schedule: DailyMantraSchedule::Weekday,
            image_policy: DailyMantraImagePolicy::PreparedBitmap,
            daily_dir: DAILY_MANTRA_SLEEP_DIR,
            default_image: DAILY_MANTRA_DEFAULT_IMAGE,
        }
    }

    pub fn resolve_for_provider<P: DateProvider>(
        &self,
        date_provider: &P,
    ) -> DailyMantraSleepImage<'a> {
        self.resolve(date_provider.resolved_date())
    }

    pub fn resolve(&self, resolved_date: ResolvedDate) -> DailyMantraSleepImage<'a> {
        if self.schedule != DailyMantraSchedule::Weekday {
            return DailyMantraSleepImage::FallbackImage {
                path: self.default_image,
                reason: DailyMantraFallbackReason::UnsupportedSchedule,
                date_trust: resolved_date.trust,
            };
        }

        match resolved_date.trusted_weekday() {
            Some(weekday) => DailyMantraSleepImage::WeekdayImage {
                weekday,
                path: daily_mantra_weekday_image_path(weekday),
                date_trust: resolved_date.trust,
            },
            None => {
                let reason = match resolved_date.date {
                    Some(date) if !date.is_valid() => DailyMantraFallbackReason::DateInvalid,
                    _ => DailyMantraFallbackReason::DateUnknown,
                };

                DailyMantraSleepImage::FallbackImage {
                    path: self.default_image,
                    reason,
                    date_trust: resolved_date.trust,
                }
            }
        }
    }
}

pub const fn daily_mantra_weekday_image_path(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "/sleep/daily/mon.bmp",
        Weekday::Tuesday => "/sleep/daily/tue.bmp",
        Weekday::Wednesday => "/sleep/daily/wed.bmp",
        Weekday::Thursday => "/sleep/daily/thu.bmp",
        Weekday::Friday => "/sleep/daily/fri.bmp",
        Weekday::Saturday => "/sleep/daily/sat.bmp",
        Weekday::Sunday => "/sleep/daily/sun.bmp",
    }
}

#[cfg(test)]
mod tests {
    use super::{DailyMantraSleepImage, DailyMantraSleepProvider, daily_mantra_weekday_image_path};
    use crate::vaachak_x4::time::date_provider::{CalendarDate, DateTrust, FixedDateProvider};
    use crate::vaachak_x4::time::weekday::Weekday;

    #[test]
    fn weekday_paths_use_prepared_bitmap_names() {
        assert_eq!(
            daily_mantra_weekday_image_path(Weekday::Monday),
            "/sleep/daily/mon.bmp"
        );
        assert_eq!(
            daily_mantra_weekday_image_path(Weekday::Sunday),
            "/sleep/daily/sun.bmp"
        );
    }

    #[test]
    fn trusted_date_selects_matching_weekday_sleep_image() {
        let date = FixedDateProvider::new(CalendarDate::new(2026, 5, 5), DateTrust::SyncProvided);
        let provider = DailyMantraSleepProvider::prepared_weekday_bitmaps();
        let image = provider.resolve_for_provider(&date);

        assert_eq!(image.path(), "/sleep/daily/tue.bmp");
        assert!(!image.is_fallback());

        match image {
            DailyMantraSleepImage::WeekdayImage { weekday, .. } => {
                assert_eq!(weekday, Weekday::Tuesday);
            }
            DailyMantraSleepImage::FallbackImage { .. } => panic!("expected weekday image"),
        }
    }

    #[test]
    fn unknown_date_uses_default_mantra_image() {
        let date = FixedDateProvider::unknown();
        let provider = DailyMantraSleepProvider::prepared_weekday_bitmaps();
        let image = provider.resolve_for_provider(&date);

        assert_eq!(image.path(), "/sleep/daily/default.bmp");
        assert!(image.is_fallback());
    }
}
