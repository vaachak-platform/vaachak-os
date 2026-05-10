// Settings app implementation.
//
// The active runtime already owns AppId::Settings.  This screen keeps the
// existing loaded SystemSettings available to the app manager, while the reader-preference work
// persists the safe Settings rows through the existing _X4/SETTINGS.TXT path.

use core::fmt::Write as _;

use crate::vaachak_x4::x4_apps::apps::{
    App, AppContext, AppId, PendingSetting, Transition, reader_state,
};
use crate::vaachak_x4::x4_apps::fonts;
use crate::vaachak_x4::x4_apps::ui::{
    Alignment, BUTTON_BAR_H, BitmapLabel, CONTENT_TOP, FULL_CONTENT_W, HEADER_W, LARGE_MARGIN,
    Region, StackFmt, TITLE_Y, wrap_next, wrap_prev,
};
use crate::vaachak_x4::x4_kernel::board::action::{Action, ActionEvent};
use crate::vaachak_x4::x4_kernel::board::{SCREEN_H, SCREEN_W};
use crate::vaachak_x4::x4_kernel::drivers::strip::StripBuffer;
use crate::vaachak_x4::x4_kernel::kernel::KernelHandle;
use crate::vaachak_x4::x4_kernel::kernel::config::{
    self, ReaderPreferences, SystemSettings, WifiConfig, parse_settings_txt, write_settings_txt,
};

const ROW_H: u16 = 34;
const ROW_GAP: u16 = 4;
const ROW_STRIDE: u16 = ROW_H + ROW_GAP;
const HEADER_LIST_GAP: u16 = 8;

const LABEL_X: u16 = LARGE_MARGIN;
const VALUE_W: u16 = 156;
const VALUE_X: u16 = SCREEN_W - LARGE_MARGIN - VALUE_W;
const LABEL_W: u16 = FULL_CONTENT_W - VALUE_W - 8;

const NUM_ROWS: usize = 25;

const SLEEP_IMAGE_MODE_FILE: &str = "SLPMODE.TXT";
const SLEEP_IMAGE_MODE_COUNT: u8 = 6;
const SLEEP_IMAGE_MODE_VALUES: [&str; SLEEP_IMAGE_MODE_COUNT as usize] = [
    "daily",
    "fast-daily",
    "static",
    "cached",
    "text",
    "no-redraw",
];
const SLEEP_IMAGE_MODE_LABELS: [&str; SLEEP_IMAGE_MODE_COUNT as usize] = [
    "Daily",
    "Fast Daily",
    "Static",
    "Cached",
    "Text",
    "No Redraw",
];

const TITLE_CACHE_ACTION_NONE: u8 = 0;
const TITLE_CACHE_ACTION_RELOAD: u8 = 1;
const TITLE_CACHE_ACTION_REBUILD: u8 = 2;

const TITLE_CACHE_MODE_COUNT: u8 = 2;
const TITLE_CACHE_MODE_LABELS: [&str; TITLE_CACHE_MODE_COUNT as usize] = ["Runtime", "Reset stale"];

const TITLE_CACHE_STATUS_READY: u8 = 0;
const TITLE_CACHE_STATUS_QUEUED: u8 = 1;
const TITLE_CACHE_STATUS_RUNNING: u8 = 2;
const TITLE_CACHE_STATUS_DONE: u8 = 3;
const TITLE_CACHE_STATUS_FAILED: u8 = 4;
const TITLE_CACHE_STATUS_LABELS: [&str; 5] = ["Run now", "Queued", "Running", "Run again", "Retry"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingsRowKind {
    Section(&'static str),
    ReaderFont,
    ReaderTheme,
    ReaderPreparedProfile,
    ReaderFallbackPolicy,
    ReaderProgress,
    DisplayRefresh,
    DisplayInvert,
    DisplayContrast,
    StorageSdStatus,
    StorageBooksCount,
    StorageTitleCache,
    StorageRebuildCache,
    DeviceBattery,
    DeviceSleepTimeout,
    DeviceSleepImageMode,
    DeviceButtonTest,
    AboutOs,
    AboutDevice,
    AboutBuild,
    AboutStorage,
}

#[derive(Clone, Copy)]
struct SettingsRow {
    label: &'static str,
    kind: SettingsRowKind,
}

const ROWS: [SettingsRow; NUM_ROWS] = [
    SettingsRow {
        label: "Reader",
        kind: SettingsRowKind::Section("Reader"),
    },
    SettingsRow {
        label: "Font size",
        kind: SettingsRowKind::ReaderFont,
    },
    SettingsRow {
        label: "Reading theme",
        kind: SettingsRowKind::ReaderTheme,
    },
    SettingsRow {
        label: "Prepared profile",
        kind: SettingsRowKind::ReaderPreparedProfile,
    },
    SettingsRow {
        label: "Fallback",
        kind: SettingsRowKind::ReaderFallbackPolicy,
    },
    SettingsRow {
        label: "Show progress",
        kind: SettingsRowKind::ReaderProgress,
    },
    SettingsRow {
        label: "Display",
        kind: SettingsRowKind::Section("Display"),
    },
    SettingsRow {
        label: "Refresh mode",
        kind: SettingsRowKind::DisplayRefresh,
    },
    SettingsRow {
        label: "Invert colors",
        kind: SettingsRowKind::DisplayInvert,
    },
    SettingsRow {
        label: "Contrast",
        kind: SettingsRowKind::DisplayContrast,
    },
    SettingsRow {
        label: "Storage",
        kind: SettingsRowKind::Section("Storage"),
    },
    SettingsRow {
        label: "SD status",
        kind: SettingsRowKind::StorageSdStatus,
    },
    SettingsRow {
        label: "Books count",
        kind: SettingsRowKind::StorageBooksCount,
    },
    SettingsRow {
        label: "Title cache",
        kind: SettingsRowKind::StorageTitleCache,
    },
    SettingsRow {
        label: "Rebuild title cache",
        kind: SettingsRowKind::StorageRebuildCache,
    },
    SettingsRow {
        label: "Device",
        kind: SettingsRowKind::Section("Device"),
    },
    SettingsRow {
        label: "Battery",
        kind: SettingsRowKind::DeviceBattery,
    },
    SettingsRow {
        label: "Sleep timeout",
        kind: SettingsRowKind::DeviceSleepTimeout,
    },
    SettingsRow {
        label: "Sleep image",
        kind: SettingsRowKind::DeviceSleepImageMode,
    },
    SettingsRow {
        label: "Button test",
        kind: SettingsRowKind::DeviceButtonTest,
    },
    SettingsRow {
        label: "About",
        kind: SettingsRowKind::Section("About"),
    },
    SettingsRow {
        label: "VaachakOS",
        kind: SettingsRowKind::AboutOs,
    },
    SettingsRow {
        label: "Xteink X4",
        kind: SettingsRowKind::AboutDevice,
    },
    SettingsRow {
        label: "Build",
        kind: SettingsRowKind::AboutBuild,
    },
    SettingsRow {
        label: "Storage layout",
        kind: SettingsRowKind::AboutStorage,
    },
];

impl Default for SettingsApp {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SettingsApp {
    settings: SystemSettings,
    wifi: WifiConfig,
    selected: usize,
    scroll: usize,
    loaded: bool,
    save_needed: bool,
    ui_fonts: fonts::UiFonts,
    rows_top: u16,

    reader_font: u8,
    reader_theme: u8,
    reader_prepared_profile: u8,
    reader_fallback_policy: u8,
    reader_show_progress: bool,
    display_refresh: u8,
    display_invert: bool,
    display_contrast: bool,
    device_sleep_timeout: u8,
    device_battery_mv: u16,
    sleep_image_mode: u8,
    title_cache_mode: u8,
    title_cache_status: u8,
    title_cache_action: u8,
}

impl SettingsApp {
    pub fn new() -> Self {
        let uf = fonts::UiFonts::for_size(0);
        Self {
            settings: SystemSettings::defaults(),
            wifi: WifiConfig::empty(),
            selected: 0,
            scroll: 0,
            loaded: false,
            save_needed: false,
            ui_fonts: uf,
            rows_top: TITLE_Y + uf.heading.line_height + HEADER_LIST_GAP,
            reader_font: config::DEFAULT_FONT_SIZE_IDX,
            reader_theme: config::DEFAULT_READING_THEME,
            reader_prepared_profile: config::DEFAULT_PREPARED_FONT_PROFILE,
            reader_fallback_policy: config::DEFAULT_PREPARED_FALLBACK_POLICY,
            reader_show_progress: true,
            display_refresh: 1,
            display_invert: false,
            display_contrast: false,
            device_sleep_timeout: 1,
            device_battery_mv: 0,
            sleep_image_mode: 0,
            title_cache_mode: 0,
            title_cache_status: TITLE_CACHE_STATUS_READY,
            title_cache_action: TITLE_CACHE_ACTION_NONE,
        }
    }

    pub fn set_ui_font_size(&mut self, idx: u8) {
        self.ui_fonts = fonts::UiFonts::for_size(idx);
        self.rows_top = TITLE_Y + self.ui_fonts.heading.line_height + HEADER_LIST_GAP;
    }

    pub fn system_settings(&self) -> &SystemSettings {
        &self.settings
    }

    pub fn system_settings_mut(&mut self) -> &mut SystemSettings {
        &mut self.settings
    }

    pub fn wifi_config(&self) -> &WifiConfig {
        &self.wifi
    }

    pub fn mark_save_needed(&mut self) {
        self.save_needed = true;
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn load_eager(&mut self, k: &mut KernelHandle<'_>) {
        self.load(k);
        self.set_ui_font_size(self.settings.ui_font_size_idx);
    }

    fn load(&mut self, k: &mut KernelHandle<'_>) {
        let mut buf = [0u8; 1024];
        self.device_battery_mv = k.battery_mv();

        self.settings = SystemSettings::defaults();
        self.wifi = WifiConfig::empty();

        match k.read_app_data_start(config::SETTINGS_FILE, &mut buf) {
            Ok((_size, n)) if n > 0 => {
                parse_settings_txt(&buf[..n], &mut self.settings, &mut self.wifi);
                self.settings.sanitize();
                log::info!("settings: loaded from {}", config::SETTINGS_FILE);
            }
            _ => {
                log::info!("settings: no file found, using defaults");
            }
        }

        self.sync_local_from_system_settings();
        self.load_sleep_image_mode(k);
        self.loaded = true;
    }

    fn read_wifi_credentials_from_settings_file(k: &mut KernelHandle<'_>) -> WifiConfig {
        let mut buf = [0u8; 1024];
        let mut settings = SystemSettings::defaults();
        let mut wifi = WifiConfig::empty();

        if let Ok((_size, n)) = k.read_app_data_start(config::SETTINGS_FILE, &mut buf) {
            if n > 0 {
                parse_settings_txt(&buf[..n], &mut settings, &mut wifi);
            }
        }

        wifi
    }

    fn preserve_wifi_credentials_before_save(&mut self, k: &mut KernelHandle<'_>) {
        if self.wifi.has_credentials() && !self.wifi.password().is_empty() {
            return;
        }

        let existing = Self::read_wifi_credentials_from_settings_file(k);
        if existing.has_credentials() && !existing.password().is_empty() {
            self.wifi = existing;
        }
    }

    fn load_sleep_image_mode(&mut self, k: &mut KernelHandle<'_>) {
        let mut buf = [0u8; 32];
        self.sleep_image_mode = match k.read_file_start(SLEEP_IMAGE_MODE_FILE, &mut buf) {
            Ok((_size, n)) if n > 0 => parse_sleep_image_mode(&buf[..n]),
            _ => 0,
        };
    }

    fn save_sleep_image_mode(&self, k: &mut KernelHandle<'_>) -> bool {
        let mut buf = [0u8; 16];
        let len = write_sleep_image_mode(self.sleep_image_mode, &mut buf);
        match k.write_file(SLEEP_IMAGE_MODE_FILE, &buf[..len]) {
            Ok(_) => true,
            Err(e) => {
                log::error!("settings: sleep image mode save failed: {}", e);
                false
            }
        }
    }

    fn queue_title_cache_action(&mut self, action: u8) {
        self.title_cache_action = action;
        self.title_cache_status = TITLE_CACHE_STATUS_QUEUED;
    }

    fn run_title_cache_action(&mut self, k: &mut KernelHandle<'_>) {
        let action = self.title_cache_action;
        if action == TITLE_CACHE_ACTION_NONE {
            return;
        }

        self.title_cache_action = TITLE_CACHE_ACTION_NONE;
        self.title_cache_status = TITLE_CACHE_STATUS_RUNNING;

        let result = match action {
            TITLE_CACHE_ACTION_RELOAD => {
                k.invalidate_dir_cache();
                k.ensure_dir_cache_loaded().map(|_| ())
            }
            TITLE_CACHE_ACTION_REBUILD => {
                let _ = k.delete_cache(crate::vaachak_x4::x4_kernel::drivers::storage::TITLES_FILE);
                k.invalidate_dir_cache();
                k.ensure_dir_cache_loaded().map(|_| ())
            }
            _ => Ok(()),
        };

        match result {
            Ok(()) => {
                self.title_cache_status = TITLE_CACHE_STATUS_DONE;
            }
            Err(e) => {
                log::warn!("settings: title cache action failed: {}", e);
                self.title_cache_status = TITLE_CACHE_STATUS_FAILED;
            }
        }
    }

    fn sync_local_from_system_settings(&mut self) {
        self.reader_font = self.settings.book_font_size_idx.min(fonts::max_size_idx());
        self.reader_theme = self
            .settings
            .reading_theme
            .min(reader_theme_count().saturating_sub(1));
        self.reader_prepared_profile = self
            .settings
            .prepared_font_profile
            .min(config::PREPARED_FONT_PROFILE_COUNT - 1);
        self.reader_fallback_policy = self
            .settings
            .prepared_fallback_policy
            .min(config::PREPARED_FALLBACK_POLICY_COUNT - 1);
        self.reader_show_progress = self.settings.reader_show_progress;
        self.display_refresh = self.settings.display_refresh_mode.min(2);
        self.display_invert = self.settings.display_invert_colors;
        self.display_contrast = self.settings.display_contrast_high;
        self.device_sleep_timeout = match self.settings.sleep_timeout {
            0 => 3,
            1..=5 => 0,
            6..=10 => 1,
            _ => 2,
        };
    }

    fn sync_system_from_local(&mut self) {
        self.settings.set_reader_preferences(ReaderPreferences {
            book_font: self.reader_font.min(fonts::max_size_idx()),
            reading_theme: self
                .reader_theme
                .min(reader_theme_count().saturating_sub(1)),
            show_progress: self.reader_show_progress,
            prepared_font_profile: self
                .reader_prepared_profile
                .min(config::PREPARED_FONT_PROFILE_COUNT - 1),
            prepared_fallback_policy: self
                .reader_fallback_policy
                .min(config::PREPARED_FALLBACK_POLICY_COUNT - 1),
        });
        self.settings.display_refresh_mode = self.display_refresh.min(2);
        self.settings.display_invert_colors = self.display_invert;
        self.settings.display_contrast_high = self.display_contrast;
        self.settings.sleep_timeout = match self.device_sleep_timeout {
            0 => 5,
            1 => 10,
            2 => 30,
            _ => 0,
        };
        self.settings.sanitize();
    }

    fn save(&mut self, k: &mut KernelHandle<'_>) -> bool {
        let mut buf = [0u8; 1024];
        self.preserve_wifi_credentials_before_save(k);
        let len = write_settings_txt(&self.settings, &self.wifi, &mut buf);
        let settings_saved = match k.write_app_data(config::SETTINGS_FILE, &buf[..len]) {
            Ok(_) => {
                log::info!("settings: saved to {}", config::SETTINGS_FILE);
                true
            }
            Err(e) => {
                log::error!("settings: save failed: {}", e);
                false
            }
        };

        let sleep_mode_saved = self.save_sleep_image_mode(k);
        settings_saved && sleep_mode_saved
    }

    fn visible_rows(&self) -> usize {
        let avail = SCREEN_H.saturating_sub(self.rows_top + BUTTON_BAR_H);
        let count = (avail / ROW_STRIDE) as usize;
        count.clamp(1, NUM_ROWS)
    }

    fn scroll_into_view(&mut self) {
        let vis = self.visible_rows();
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + vis {
            self.scroll = self.selected + 1 - vis;
        }
    }

    fn row_region(&self, visible_idx: usize) -> Region {
        Region::new(
            LABEL_X,
            self.rows_top + visible_idx as u16 * ROW_STRIDE,
            FULL_CONTENT_W,
            ROW_H,
        )
    }

    fn label_region(&self, visible_idx: usize) -> Region {
        Region::new(
            LABEL_X,
            self.rows_top + visible_idx as u16 * ROW_STRIDE,
            LABEL_W,
            ROW_H,
        )
    }

    fn value_region(&self, visible_idx: usize) -> Region {
        Region::new(
            VALUE_X,
            self.rows_top + visible_idx as u16 * ROW_STRIDE,
            VALUE_W,
            ROW_H,
        )
    }

    fn list_region(&self) -> Region {
        Region::new(
            LABEL_X,
            self.rows_top,
            FULL_CONTENT_W,
            self.visible_rows() as u16 * ROW_STRIDE,
        )
    }

    fn move_next(&mut self, ctx: &mut AppContext) {
        self.move_to(wrap_next(self.selected, NUM_ROWS), ctx);
    }

    fn move_prev(&mut self, ctx: &mut AppContext) {
        self.move_to(wrap_prev(self.selected, NUM_ROWS), ctx);
    }

    fn move_to(&mut self, selected: usize, ctx: &mut AppContext) {
        let old_selected = self.selected;
        let old_scroll = self.scroll;
        self.selected = selected.min(NUM_ROWS - 1);
        self.scroll_into_view();

        if self.scroll != old_scroll {
            ctx.mark_dirty(self.list_region());
        } else if self.selected != old_selected {
            ctx.mark_dirty(self.row_region(old_selected - old_scroll));
            ctx.mark_dirty(self.row_region(self.selected - self.scroll));
        }
    }

    fn cycle_selected(&mut self, delta: isize, ctx: &mut AppContext) {
        let row = ROWS[self.selected];
        let changed = match row.kind {
            SettingsRowKind::ReaderFont => {
                self.reader_font =
                    cycle_index(self.reader_font, fonts::FONT_SIZE_COUNT as u8, delta);
                true
            }
            SettingsRowKind::ReaderTheme => {
                self.reader_theme = cycle_index(self.reader_theme, reader_theme_count(), delta);
                true
            }
            SettingsRowKind::ReaderPreparedProfile => {
                self.reader_prepared_profile = cycle_index(
                    self.reader_prepared_profile,
                    config::PREPARED_FONT_PROFILE_COUNT,
                    delta,
                );
                true
            }
            SettingsRowKind::ReaderFallbackPolicy => {
                self.reader_fallback_policy = cycle_index(
                    self.reader_fallback_policy,
                    config::PREPARED_FALLBACK_POLICY_COUNT,
                    delta,
                );
                true
            }
            SettingsRowKind::ReaderProgress => {
                self.reader_show_progress = !self.reader_show_progress;
                true
            }
            SettingsRowKind::DisplayRefresh => {
                self.display_refresh = cycle_index(self.display_refresh, 3, delta);
                true
            }
            SettingsRowKind::DisplayInvert => {
                self.display_invert = !self.display_invert;
                true
            }
            SettingsRowKind::DisplayContrast => {
                self.display_contrast = !self.display_contrast;
                true
            }
            SettingsRowKind::DeviceSleepTimeout => {
                self.device_sleep_timeout = cycle_index(self.device_sleep_timeout, 4, delta);
                true
            }
            SettingsRowKind::StorageTitleCache => {
                self.title_cache_mode =
                    cycle_index(self.title_cache_mode, TITLE_CACHE_MODE_COUNT, delta);
                self.title_cache_status = TITLE_CACHE_STATUS_READY;
                true
            }
            SettingsRowKind::StorageRebuildCache => {
                let action = if self.title_cache_mode == 0 {
                    TITLE_CACHE_ACTION_RELOAD
                } else {
                    TITLE_CACHE_ACTION_REBUILD
                };
                self.queue_title_cache_action(action);
                true
            }
            SettingsRowKind::DeviceSleepImageMode => {
                self.sleep_image_mode =
                    cycle_index(self.sleep_image_mode, SLEEP_IMAGE_MODE_COUNT, delta);
                true
            }
            _ => false,
        };

        if changed {
            self.sync_system_from_local();
            self.save_needed = true;
        }
        ctx.mark_dirty(self.row_region(self.selected - self.scroll));
    }

    fn format_value(&self, kind: SettingsRowKind, buf: &mut StackFmt<40>) {
        buf.clear();
        match kind {
            SettingsRowKind::Section(name) => {
                let _ = write!(buf, "{}", name);
            }
            SettingsRowKind::ReaderFont => {
                let _ = write!(buf, "{}", fonts::font_size_name(self.reader_font));
            }
            SettingsRowKind::ReaderTheme => {
                let _ = write!(buf, "{}", reader_theme_name(self.reader_theme));
            }
            SettingsRowKind::ReaderPreparedProfile => {
                let idx =
                    self.reader_prepared_profile
                        .min(config::PREPARED_FONT_PROFILE_COUNT - 1) as usize;
                let _ = write!(buf, "{}", config::PREPARED_FONT_PROFILE_LABELS[idx]);
            }
            SettingsRowKind::ReaderFallbackPolicy => {
                let idx = self
                    .reader_fallback_policy
                    .min(config::PREPARED_FALLBACK_POLICY_COUNT - 1)
                    as usize;
                let _ = write!(buf, "{}", config::PREPARED_FALLBACK_POLICY_LABELS[idx]);
            }
            SettingsRowKind::ReaderProgress => {
                let _ = write!(buf, "{}", on_off(self.reader_show_progress));
            }
            SettingsRowKind::DisplayRefresh => {
                let _ = write!(
                    buf,
                    "{}",
                    ["Full", "Balanced", "Fast"][self.display_refresh as usize]
                );
            }
            SettingsRowKind::DisplayInvert => {
                let _ = write!(buf, "{}", on_off(self.display_invert));
            }
            SettingsRowKind::DisplayContrast => {
                let _ = write!(
                    buf,
                    "{}",
                    if self.display_contrast {
                        "High"
                    } else {
                        "Normal"
                    }
                );
            }
            SettingsRowKind::StorageSdStatus => {
                let _ = write!(buf, "Ready");
            }
            SettingsRowKind::StorageBooksCount => {
                let _ = write!(buf, "Library");
            }
            SettingsRowKind::StorageTitleCache => {
                let _ = write!(buf, "{}", title_cache_mode_name(self.title_cache_mode));
            }
            SettingsRowKind::StorageRebuildCache => {
                let _ = write!(buf, "{}", title_cache_status_name(self.title_cache_status));
            }
            SettingsRowKind::DeviceBattery => {
                if let Some(pct) = battery_percent_value(self.device_battery_mv) {
                    let _ = write!(buf, "{}% {}mV", pct, self.device_battery_mv);
                } else {
                    let _ = write!(buf, "--");
                }
            }
            SettingsRowKind::DeviceSleepTimeout => {
                let _ = write!(
                    buf,
                    "{}",
                    ["5 min", "10 min", "30 min", "Never"][self.device_sleep_timeout as usize]
                );
            }
            SettingsRowKind::DeviceSleepImageMode => {
                let _ = write!(buf, "{}", sleep_image_mode_name(self.sleep_image_mode));
            }
            SettingsRowKind::DeviceButtonTest => {
                let _ = write!(buf, "Coming soon");
            }
            SettingsRowKind::AboutOs => {
                let _ = write!(buf, "VaachakOS");
            }
            SettingsRowKind::AboutDevice => {
                let _ = write!(buf, "Xteink X4");
            }
            SettingsRowKind::AboutBuild => {
                let _ = write!(buf, "riscv32 release");
            }
            SettingsRowKind::AboutStorage => {
                let _ = write!(buf, "_X4 + TITLES.BIN");
            }
        }
    }
}

impl App<AppId> for SettingsApp {
    fn on_enter(&mut self, ctx: &mut AppContext, k: &mut KernelHandle<'_>) {
        self.device_battery_mv = k.battery_mv();
        if self.loaded {
            self.sync_local_from_system_settings();
        }
        self.selected = 0;
        self.scroll = 0;
        ctx.mark_dirty(Region::new(
            0,
            CONTENT_TOP,
            SCREEN_W,
            SCREEN_H - CONTENT_TOP,
        ));
    }

    fn on_event(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                Transition::Pop
            }
            ActionEvent::Press(Action::Next) | ActionEvent::Repeat(Action::Next) => {
                self.move_next(ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Prev) | ActionEvent::Repeat(Action::Prev) => {
                self.move_prev(ctx);
                Transition::None
            }
            ActionEvent::Press(Action::NextJump)
            | ActionEvent::Repeat(Action::NextJump)
            | ActionEvent::Press(Action::Select) => {
                self.cycle_selected(1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::PrevJump) | ActionEvent::Repeat(Action::PrevJump) => {
                self.cycle_selected(-1, ctx);
                Transition::None
            }
            _ => Transition::None,
        }
    }

    async fn background(&mut self, ctx: &mut AppContext, k: &mut KernelHandle<'_>) {
        let battery_mv = k.battery_mv();
        if battery_mv != self.device_battery_mv {
            self.device_battery_mv = battery_mv;
            if self.loaded {
                ctx.request_full_redraw();
            }
        }

        if !self.loaded {
            self.load(k);
            ctx.request_full_redraw();
            return;
        }

        if self.title_cache_action != TITLE_CACHE_ACTION_NONE {
            self.run_title_cache_action(k);
            ctx.mark_dirty(self.row_region(self.selected - self.scroll));
        }

        if self.save_needed && self.save(k) {
            self.save_needed = false;
        }
    }

    fn pending_setting(&self) -> Option<PendingSetting> {
        Some(PendingSetting::ReaderPreferences(
            self.settings.reader_preferences(),
        ))
    }

    fn has_background_when_suspended(&self) -> bool {
        self.save_needed || self.title_cache_action != TITLE_CACHE_ACTION_NONE
    }

    fn background_suspended(&mut self, k: &mut KernelHandle<'_>) {
        if self.title_cache_action != TITLE_CACHE_ACTION_NONE {
            self.run_title_cache_action(k);
        }

        if self.save_needed && self.save(k) {
            self.save_needed = false;
        }
    }

    fn draw(&self, strip: &mut StripBuffer) {
        let header_region = Region::new(
            LARGE_MARGIN,
            TITLE_Y,
            HEADER_W,
            self.ui_fonts.heading.line_height,
        );
        BitmapLabel::new(header_region, "Settings", self.ui_fonts.heading)
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();

        let context_region = Region::new(
            SCREEN_W.saturating_sub(LARGE_MARGIN).saturating_sub(96),
            TITLE_Y,
            96,
            self.ui_fonts.heading.line_height,
        );
        BitmapLabel::new(context_region, "x4", fonts::chrome_font())
            .alignment(Alignment::CenterRight)
            .draw(strip)
            .unwrap();

        if !self.loaded {
            BitmapLabel::new(self.row_region(0), "Loading...", self.ui_fonts.body)
                .alignment(Alignment::CenterLeft)
                .draw(strip)
                .unwrap();
            return;
        }

        let visible = self
            .visible_rows()
            .min(NUM_ROWS.saturating_sub(self.scroll));
        let mut val_buf = StackFmt::<40>::new();

        for vi in 0..visible {
            let idx = self.scroll + vi;
            let row = ROWS[idx];
            let selected = idx == self.selected;
            let is_section = matches!(row.kind, SettingsRowKind::Section(_));

            if is_section {
                let section_font = fonts::FontSet::for_size(self.settings.ui_font_size_idx)
                    .font(fonts::Style::Bold);
                BitmapLabel::new(self.row_region(vi), row.label, section_font)
                    .alignment(Alignment::CenterLeft)
                    .inverted(selected)
                    .draw(strip)
                    .unwrap();
            } else {
                BitmapLabel::new(self.label_region(vi), row.label, self.ui_fonts.body)
                    .alignment(Alignment::CenterLeft)
                    .inverted(selected)
                    .draw(strip)
                    .unwrap();

                self.format_value(row.kind, &mut val_buf);
                BitmapLabel::new(self.value_region(vi), val_buf.as_str(), self.ui_fonts.body)
                    .alignment(Alignment::CenterRight)
                    .inverted(selected)
                    .draw(strip)
                    .unwrap();
            }
        }
    }
}

fn title_cache_mode_name(idx: u8) -> &'static str {
    TITLE_CACHE_MODE_LABELS
        .get(idx as usize)
        .copied()
        .unwrap_or("Runtime")
}

fn title_cache_status_name(idx: u8) -> &'static str {
    TITLE_CACHE_STATUS_LABELS
        .get(idx as usize)
        .copied()
        .unwrap_or("Run now")
}

fn sleep_image_mode_name(idx: u8) -> &'static str {
    SLEEP_IMAGE_MODE_LABELS
        .get(idx as usize)
        .copied()
        .unwrap_or("Daily")
}

fn sleep_image_mode_value(idx: u8) -> &'static str {
    SLEEP_IMAGE_MODE_VALUES
        .get(idx as usize)
        .copied()
        .unwrap_or("daily")
}

fn parse_sleep_image_mode(data: &[u8]) -> u8 {
    let Ok(text) = core::str::from_utf8(data) else {
        return 0;
    };
    let trimmed = text.trim();
    for (idx, value) in SLEEP_IMAGE_MODE_VALUES.iter().enumerate() {
        if trimmed.eq_ignore_ascii_case(value) {
            return idx as u8;
        }
    }
    if trimmed.eq_ignore_ascii_case("off") {
        return 5;
    }
    0
}

fn write_sleep_image_mode(idx: u8, out: &mut [u8]) -> usize {
    let value = sleep_image_mode_value(idx).as_bytes();
    let mut pos = 0usize;
    while pos < value.len() && pos < out.len() {
        out[pos] = value[pos];
        pos += 1;
    }
    if pos < out.len() {
        out[pos] = b'\n';
        pos += 1;
    }
    pos
}

fn cycle_index(value: u8, count: u8, delta: isize) -> u8 {
    if count == 0 {
        return 0;
    }
    (value as isize + delta).rem_euclid(count as isize) as u8
}

fn reader_theme_count() -> u8 {
    reader_state::THEME_NAMES.len() as u8
}

fn reader_theme_name(idx: u8) -> &'static str {
    reader_state::THEME_NAMES
        .get(idx as usize)
        .copied()
        .unwrap_or("Default")
}

fn battery_percent_value(mv: u16) -> Option<u8> {
    if mv == 0 {
        None
    } else {
        Some(crate::vaachak_x4::x4_kernel::drivers::battery::battery_percentage(mv))
    }
}

const fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}
