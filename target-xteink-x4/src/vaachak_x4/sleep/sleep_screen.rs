//! Sleep-screen rendering interface.
//!
//! The display should be drawn before low-power sleep. The e-paper panel then
//! keeps the last image without continuous refresh.

use crate::vaachak_x4::apps::daily_text::DailyTextEntry;
use crate::vaachak_x4::text::{
    FontDescriptor, ScriptClass, TextLayoutStyle, font_for_script, shape_placeholder_run,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SleepScreenLayoutStyle {
    DailyText,
    CenteredQuote,
    CompactStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SleepScreenRequest<'a> {
    pub entry: &'a DailyTextEntry,
    pub style: SleepScreenLayoutStyle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreparedSleepScreen<'a> {
    pub request: SleepScreenRequest<'a>,
    pub title_font: FontDescriptor,
    pub sanskrit_font: FontDescriptor,
    pub hindi_font: FontDescriptor,
    pub english_font: FontDescriptor,
    pub layout: TextLayoutStyle,
    pub placeholder_glyphs: usize,
    pub requires_prepared_shaping: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SleepScreenError {
    MissingEntry,
    MissingFont,
    RendererRejected,
}

pub trait SleepScreenRenderer {
    fn prepare<'a>(
        &mut self,
        request: SleepScreenRequest<'a>,
    ) -> Result<PreparedSleepScreen<'a>, SleepScreenError>;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlaceholderSleepScreenRenderer;

impl SleepScreenRenderer for PlaceholderSleepScreenRenderer {
    fn prepare<'a>(
        &mut self,
        request: SleepScreenRequest<'a>,
    ) -> Result<PreparedSleepScreen<'a>, SleepScreenError> {
        let title_script = crate::vaachak_x4::text::dominant_script(request.entry.title);
        let sanskrit_script = crate::vaachak_x4::text::dominant_script(request.entry.sanskrit);
        let hindi_script = crate::vaachak_x4::text::dominant_script(request.entry.hindi);
        let english_script = crate::vaachak_x4::text::dominant_script(request.entry.english);

        let title_font = font_for_script(title_script);
        let sanskrit_font = font_for_script(sanskrit_script);
        let hindi_font = font_for_script(hindi_script);
        let english_font = font_for_script(english_script);

        let title_run = shape_placeholder_run(request.entry.title, title_font);
        let sanskrit_run = shape_placeholder_run(request.entry.sanskrit, sanskrit_font);
        let hindi_run = shape_placeholder_run(request.entry.hindi, hindi_font);
        let english_run = shape_placeholder_run(request.entry.english, english_font);

        let requires_prepared_shaping = sanskrit_script.needs_complex_shaping()
            || hindi_script.needs_complex_shaping()
            || english_script == ScriptClass::Gujarati;

        Ok(PreparedSleepScreen {
            request,
            title_font,
            sanskrit_font,
            hindi_font,
            english_font,
            layout: TextLayoutStyle::sleep_screen(),
            placeholder_glyphs: title_run.glyph_count
                + sanskrit_run.glyph_count
                + hindi_run.glyph_count
                + english_run.glyph_count,
            requires_prepared_shaping,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PlaceholderSleepScreenRenderer, SleepScreenLayoutStyle, SleepScreenRenderer,
        SleepScreenRequest,
    };
    use crate::vaachak_x4::apps::daily_text::{Weekday, entry_for_weekday};
    use crate::vaachak_x4::text::FontFamily;

    #[test]
    fn sleep_screen_selects_devanagari_and_latin_system_fonts() {
        let entry = entry_for_weekday(Weekday::Monday);
        let mut renderer = PlaceholderSleepScreenRenderer;
        let prepared = renderer
            .prepare(SleepScreenRequest {
                entry,
                style: SleepScreenLayoutStyle::DailyText,
            })
            .unwrap();

        assert_eq!(prepared.sanskrit_font.family, FontFamily::SystemDevanagari);
        assert_eq!(prepared.hindi_font.family, FontFamily::SystemDevanagari);
        assert_eq!(prepared.english_font.family, FontFamily::SystemLatin);
        assert!(prepared.requires_prepared_shaping);
    }
}
