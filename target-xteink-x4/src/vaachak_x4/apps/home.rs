// Vaachak-owned launcher screen: menu, bookmarks browser, network entry points

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write as _;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;

pub const X4_KEYBOARD_CROSSPOINT_SESSION_REPAIR_MARKER: &str =
    "x4-keyboard-crosspoint-session-repair-ok";

pub const X4_KEYBOARD_CROSSPOINT_DIRTY_FIELD_COMPILE_REPAIR_MARKER: &str =
    "x4-keyboard-crosspoint-dirty-field-compile-repair-ok";

use crate::vaachak_x4::network::{time_status, wifi_scan};
use crate::vaachak_x4::x4_apps::apps::reader_state;
use crate::vaachak_x4::x4_apps::apps::widgets::text_keyboard::{self, TextKeyboardAction};
use crate::vaachak_x4::x4_apps::apps::{App, AppContext, AppId, RECENT_FILE, Transition};
use crate::vaachak_x4::x4_apps::fonts;
use crate::vaachak_x4::x4_apps::fonts::bitmap::BitmapFont;
use crate::vaachak_x4::x4_apps::ui::{
    Alignment, BUTTON_BAR_H, BitmapDynLabel, BitmapLabel, CONTENT_TOP, FULL_CONTENT_W, HEADER_W,
    LARGE_MARGIN, Region, SECTION_GAP, TITLE_Y_OFFSET,
};
use crate::vaachak_x4::x4_kernel::app::HomeMenuItem;
use crate::vaachak_x4::x4_kernel::board::action::{Action, ActionEvent};
use crate::vaachak_x4::x4_kernel::board::{SCREEN_H, SCREEN_W};
use crate::vaachak_x4::x4_kernel::drivers::strip::StripBuffer;
use crate::vaachak_x4::x4_kernel::kernel::KernelHandle;
use crate::vaachak_x4::x4_kernel::kernel::config::{
    self, SystemSettings, WifiConfig, write_settings_txt,
};

use crate::vaachak_x4::lua::calendar_script::{
    LUA_CALENDAR_APP_FOLDER, LUA_CALENDAR_APP_MARKER, LUA_CALENDAR_ENTRY_FILE,
    LUA_CALENDAR_EVENTS_FILE, LUA_CALENDAR_MANIFEST_FILE, LUA_CALENDAR_SD_RUNTIME_MARKER,
    LuaCalendarScreen, build_calendar_sd_runtime,
};
use crate::vaachak_x4::lua::daily_mantra_script::{
    LUA_DAILY_MANTRA_APP_FOLDER, LUA_DAILY_MANTRA_APP_MARKER, LUA_DAILY_MANTRA_ENTRY_FILE,
    LUA_DAILY_MANTRA_MANIFEST_FILE, LUA_DAILY_MANTRA_MANTRAS_FILE,
    LUA_DAILY_MANTRA_SD_RUNTIME_MARKER, LuaDailyMantraScreen,
};
use crate::vaachak_x4::lua::panchang_script::{
    LUA_PANCHANG_APP_FOLDER, LUA_PANCHANG_APP_MARKER, LUA_PANCHANG_DATA_FILE,
    LUA_PANCHANG_ENTRY_FILE, LUA_PANCHANG_MANIFEST_FILE, LUA_PANCHANG_SD_RUNTIME_MARKER,
    LuaPanchangScreen, build_panchang_sd_runtime,
};

pub const HOME_DASHBOARD_MARKER: &str = "x4-biscuit-home-dashboard-active-ok";
pub const HOME_NAV_POLISH_MARKER: &str = "x4-biscuit-home-nav-polish-placeholder-routing-ok";
pub const WIFI_SETTINGS_UI_EDITOR_MARKER: &str = "wifi-settings-shared-keyboard-ok";

const HOME_CARD_COUNT: usize = 6;
const HOME_GRID_COLS: usize = 2;
const HOME_GRID_ROWS: usize = 3;
const HOME_GRID_GAP: u16 = 12;
const HOME_CARD_PAD_X: u16 = 10;
const HOME_CARD_PAD_Y: u16 = 8;
const HOME_HEADER_Y: u16 = CONTENT_TOP + 8;
const HOME_HEADER_RULE_GAP: u16 = 6;
const HOME_GRID_TOP_GAP: u16 = 10;
const HOME_FOOTER_RESERVED_H: u16 = BUTTON_BAR_H + 8;
const HOME_FOOTER_GAP: u16 = 10;
const HOME_CARD_TEXT_GAP: u16 = 5;

const TOOLS_CATEGORY_ITEM_COUNT: usize = 4;

const MAX_ITEMS: usize = HOME_CARD_COUNT;
const RECENT_BUF_LEN: usize = 160;
const NETWORK_SSID_BUF_LEN: usize = config::WIFI_SSID_CAP;

const NETWORK_PASS_BUF_LEN: usize = config::WIFI_PASS_CAP;
const WIFI_EDITOR_FIELD_COUNT: u8 = 6;
const WIFI_EDITOR_MODE_ROWS: u8 = 0;
const WIFI_EDITOR_MODE_KEYBOARD_LOWER: u8 = 1;
const WIFI_EDITOR_MODE_KEYBOARD_UPPER: u8 = 2;
const WIFI_EDITOR_MODE_KEYBOARD_SYMBOLS: u8 = 3;
const WIFI_EDITOR_SAVE_MSG: &str = "Scan SSID, enter password, then Save";
const WIFI_SCAN_MAX_RESULTS: usize = 8;
const WIFI_SCAN_SSID_BUF_LEN: usize = config::WIFI_SSID_CAP;
const NETWORK_STATUS_BUF_LEN: usize = 1024;
const NETWORK_STATUS_LINE_GAP: u16 = 12;
const TIME_STATUS_BUF_LEN: usize = 512;
const CALENDAR_WEEKDAY_LABELS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const CALENDAR_ROWS: usize = 6;
const CALENDAR_COLS: usize = 7;
const CALENDAR_CELL_GAP: u16 = 2;
const CALENDAR_GRID_TOP_GAP: u16 = 12;
const PANCHANG_LINE_GAP: u16 = 10;
const PANCHANG_VALUE_W: u16 = 440;
const PANCHANG_MANTRA_BLOCK_GAP: u16 = 14;
const PANCHANG_MANTRA_FILE: &str = "yearly_hindu_mantras.txt";
const PANCHANG_MANTRA_ALT_FILE: &str = "YEARLY_HINDU_MANTRAS.TXT";
const PANCHANG_MANTRA_SHORT_FILE: &str = "Yearly_h.txt";
const PANCHANG_MANTRA_SHORT_UPPER_FILE: &str = "YEARLY_H.TXT";
const PANCHANG_MANTRA_SHORT_83_FILE: &str = "YEARLY~1.TXT";
const PANCHANG_MANTRA_BUF_LEN: usize = 176;
const PANCHANG_MANTRA_SCAN_BUF_LEN: usize = 1024;
const PANCHANG_MANTRA_LINE_BUF_LEN: usize = 512;

const DAILY_MANTRA_DEFAULT_IMAGE: &str = "/sleep/daily/default.bmp";
const DAILY_MANTRA_WEEKDAY_IMAGES: [&str; 7] = [
    "/sleep/daily/sun.bmp",
    "/sleep/daily/mon.bmp",
    "/sleep/daily/tue.bmp",
    "/sleep/daily/wed.bmp",
    "/sleep/daily/thu.bmp",
    "/sleep/daily/fri.bmp",
    "/sleep/daily/sat.bmp",
];
const DAILY_MANTRA_TITLES: [&str; 7] = [
    "Sunday - Ravivar",
    "Monday - Somvar",
    "Tuesday - Mangalvar",
    "Wednesday - Budhvar",
    "Thursday - Guruvar",
    "Friday - Shukravar",
    "Saturday - Shanivar",
];
const DAILY_MANTRA_DEDICATIONS: [&str; 7] = [
    "Dedicated to Surya Dev",
    "Dedicated to Lord Shiva",
    "Dedicated to Lord Hanuman and Lord Ganesha",
    "Dedicated to Lord Ganesha and Lord Krishna",
    "Dedicated to Lord Vishnu and Brihaspati",
    "Dedicated to Goddess Lakshmi and Devi",
    "Dedicated to Lord Shani",
];
const DAILY_MANTRA_ENGLISH: [&str; 7] = [
    "Om Suryaya Namah",
    "Om Namah Shivaya",
    "Om Hanumate Namah",
    "Om Gam Ganapataye Namah",
    "Om Namo Bhagavate Vasudevaya",
    "Om Shreem Mahalakshmyai Namah",
    "Om Shanaishcharaya Namah",
];

// bookmark list layout (matches Files app)
// Group same-book bookmarks under one title to avoid row clipping.
const BM_ROW_H: u16 = 52;
const BM_ROW_GAP: u16 = 4;
const BM_ROW_STRIDE: u16 = BM_ROW_H + BM_ROW_GAP;
const BM_TITLE_Y: u16 = CONTENT_TOP + TITLE_Y_OFFSET;
const BM_HEADER_LIST_GAP: u16 = SECTION_GAP;
const BM_BOOK_TITLE_GAP: u16 = 6;
const BM_MAX_TITLE_CHARS: usize = 34;
const BM_MAX_MIXED_TITLE_CHARS: usize = 16;
const BM_MAX_ROW_CHARS: usize = 42;
const BM_STATUS_W: u16 = 144;
const BM_STATUS_X: u16 = SCREEN_W - LARGE_MARGIN - BM_STATUS_W;

const CONTENT_REGION: Region = Region::new(0, CONTENT_TOP, SCREEN_W, SCREEN_H - CONTENT_TOP);

fn compute_item_regions(heading_line_h: u16) -> [Region; MAX_ITEMS] {
    let rule_y = HOME_HEADER_Y + heading_line_h + HOME_HEADER_RULE_GAP;
    let grid_y = rule_y + 2 + HOME_GRID_TOP_GAP;
    let footer_y = SCREEN_H.saturating_sub(HOME_FOOTER_RESERVED_H);
    let grid_h = footer_y.saturating_sub(grid_y + HOME_FOOTER_GAP);
    let card_w = (FULL_CONTENT_W.saturating_sub(HOME_GRID_GAP)) / HOME_GRID_COLS as u16;
    let card_h = (grid_h.saturating_sub(HOME_GRID_GAP * (HOME_GRID_ROWS as u16 - 1)))
        / HOME_GRID_ROWS as u16;

    [
        Region::new(LARGE_MARGIN, grid_y, card_w, card_h),
        Region::new(
            LARGE_MARGIN + card_w + HOME_GRID_GAP,
            grid_y,
            card_w,
            card_h,
        ),
        Region::new(
            LARGE_MARGIN,
            grid_y + card_h + HOME_GRID_GAP,
            card_w,
            card_h,
        ),
        Region::new(
            LARGE_MARGIN + card_w + HOME_GRID_GAP,
            grid_y + card_h + HOME_GRID_GAP,
            card_w,
            card_h,
        ),
        Region::new(
            LARGE_MARGIN,
            grid_y + (card_h + HOME_GRID_GAP) * 2,
            card_w,
            card_h,
        ),
        Region::new(
            LARGE_MARGIN + card_w + HOME_GRID_GAP,
            grid_y + (card_h + HOME_GRID_GAP) * 2,
            card_w,
            card_h,
        ),
    ]
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum HomeState {
    Menu,
    ShowApps,
    ShowCategoryItems,
    ShowDailyMantra,
    ShowLuaDailyMantra,
    ShowLuaCalendar,
    ShowLuaPanchang,
    ShowCalendar,
    ShowPanchangLite,
    ShowBookmarks,
    ShowNetworkStatus,
    ShowWifiConnect,
    ShowDateTime,
    ShowPlaceholder,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ReturnTarget {
    Menu,
    CategoryItems,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PanchangMantraScan {
    Missing,
    FileFound,
    Found,
}

enum MenuAction {
    OpenCategory(usize),
}

pub struct HomeApp {
    state: HomeState,
    selected: usize,
    ui_fonts: fonts::UiFonts,
    item_regions: [Region; MAX_ITEMS],
    item_count: usize,
    active_category: usize,
    return_target: ReturnTarget,
    placeholder_title: &'static str,
    placeholder_detail: &'static str,

    network_ssid: [u8; NETWORK_SSID_BUF_LEN],
    network_ssid_len: usize,
    network_pass: [u8; NETWORK_PASS_BUF_LEN],
    network_pass_len: usize,
    network_scan_results: [[u8; WIFI_SCAN_SSID_BUF_LEN]; WIFI_SCAN_MAX_RESULTS],
    network_scan_lens: [u8; WIFI_SCAN_MAX_RESULTS],
    network_scan_count: usize,
    network_scan_selected: usize,
    resume_wifi_after_scan: bool,
    network_wifi: WifiConfig,
    network_profile_slot: u8,
    network_default_slot: u8,
    wifi_editor_field: u8,
    wifi_editor_char_idx: u8,
    wifi_editor_mode: u8,
    wifi_editor_save_pending: bool,
    wifi_editor_dirty: bool,
    wifi_editor_message: &'static str,
    network_status_loaded: bool,
    network_settings_found: bool,
    network_wifi_configured: bool,
    network_wifi_password_saved: bool,
    network_sd_ok: bool,
    network_battery_mv: u16,
    network_uptime_secs: u32,
    needs_load_network_status: bool,

    time_cache: time_status::TimeCache,
    time_status_loaded: bool,
    resume_date_time_after_sync: bool,
    time_uptime_secs: u32,
    home_battery_mv: u16,
    needs_load_time_status: bool,
    calendar_month_offset: i16,

    panchang_mantra: [u8; PANCHANG_MANTRA_BUF_LEN],
    panchang_mantra_len: usize,
    panchang_mantra_loaded: bool,
    panchang_mantra_file_found: bool,
    panchang_mantra_found: bool,
    needs_load_panchang_mantra: bool,

    lua_daily_mantra_screen: LuaDailyMantraScreen,
    needs_load_lua_daily_mantra: bool,
    lua_panchang_screen: LuaPanchangScreen,
    needs_load_lua_panchang: bool,
    lua_calendar_screen: LuaCalendarScreen,
    needs_load_lua_calendar: bool,

    recent_book: [u8; RECENT_BUF_LEN],
    recent_book_len: usize,
    recent_record: Option<reader_state::RecentBookRecord>,
    needs_load_recent: bool,

    bm_entries: Vec<reader_state::BookmarkIndexRecord>,
    bm_count: usize,
    bm_selected: usize,
    bm_scroll: usize,
    needs_load_bookmarks: bool,
}

impl Default for HomeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl HomeApp {
    pub fn new() -> Self {
        let uf = fonts::UiFonts::for_size(0);
        Self {
            state: HomeState::Menu,
            selected: 0,
            ui_fonts: uf,
            item_regions: compute_item_regions(fonts::heading_font(0).line_height),
            item_count: HOME_CARD_COUNT,
            active_category: 0,
            return_target: ReturnTarget::Menu,
            placeholder_title: "Coming soon",
            placeholder_detail: "Placeholder",
            network_ssid: [0u8; NETWORK_SSID_BUF_LEN],
            network_ssid_len: 0,
            network_pass: [0u8; NETWORK_PASS_BUF_LEN],
            network_pass_len: 0,
            network_scan_results: [[0u8; WIFI_SCAN_SSID_BUF_LEN]; WIFI_SCAN_MAX_RESULTS],
            network_scan_lens: [0u8; WIFI_SCAN_MAX_RESULTS],
            network_scan_count: 0,
            network_scan_selected: 0,
            resume_wifi_after_scan: false,
            network_wifi: WifiConfig::empty(),
            network_profile_slot: 0,
            network_default_slot: 0,
            wifi_editor_field: 0,
            wifi_editor_char_idx: 0,
            wifi_editor_mode: WIFI_EDITOR_MODE_ROWS,
            wifi_editor_save_pending: false,
            wifi_editor_dirty: false,
            wifi_editor_message: WIFI_EDITOR_SAVE_MSG,
            network_status_loaded: false,
            network_settings_found: false,
            network_wifi_configured: false,
            network_wifi_password_saved: false,
            network_sd_ok: false,
            network_battery_mv: 0,
            network_uptime_secs: 0,
            needs_load_network_status: false,
            time_cache: time_status::TimeCache::default(),
            time_status_loaded: false,
            resume_date_time_after_sync: false,
            time_uptime_secs: 0,
            home_battery_mv: 0,
            needs_load_time_status: false,
            calendar_month_offset: 0,

            panchang_mantra: [0u8; PANCHANG_MANTRA_BUF_LEN],
            panchang_mantra_len: 0,
            panchang_mantra_loaded: false,
            panchang_mantra_file_found: false,
            panchang_mantra_found: false,
            needs_load_panchang_mantra: false,
            lua_daily_mantra_screen: LuaDailyMantraScreen::default(),
            needs_load_lua_daily_mantra: false,
            lua_panchang_screen: LuaPanchangScreen::default(),
            needs_load_lua_panchang: false,
            lua_calendar_screen: LuaCalendarScreen::default(),
            needs_load_lua_calendar: false,
            recent_book: [0u8; RECENT_BUF_LEN],
            recent_book_len: 0,
            recent_record: None,
            needs_load_recent: false,
            bm_entries: Vec::new(),
            bm_count: 0,
            bm_selected: 0,
            bm_scroll: 0,
            needs_load_bookmarks: false,
        }
    }

    pub fn set_ui_font_size(&mut self, idx: u8) {
        self.ui_fonts = fonts::UiFonts::for_size(idx);
        self.refresh_menu_layout();
    }

    fn refresh_menu_layout(&mut self) {
        self.item_regions = compute_item_regions(self.header_font().line_height);
    }

    // Session state accessors for RTC persistence
    #[inline]
    pub fn state_id(&self) -> u8 {
        match self.state {
            HomeState::Menu => 0,
            HomeState::ShowApps => 1,
            HomeState::ShowDailyMantra => 2,
            HomeState::ShowLuaDailyMantra => 11,
            HomeState::ShowLuaCalendar => 12,
            HomeState::ShowBookmarks => 3,
            HomeState::ShowCategoryItems => 4,
            HomeState::ShowPlaceholder => 5,
            HomeState::ShowNetworkStatus => 6,
            HomeState::ShowWifiConnect => 7,
            HomeState::ShowDateTime => 8,
            HomeState::ShowCalendar => 9,
            HomeState::ShowPanchangLite => 10,
            HomeState::ShowLuaPanchang => 10,
        }
    }

    #[inline]
    pub fn selected(&self) -> usize {
        self.selected
    }

    #[inline]
    pub fn bm_selected(&self) -> usize {
        self.bm_selected
    }

    #[inline]
    pub fn bm_scroll(&self) -> usize {
        self.bm_scroll
    }

    // restore home state from RTC session data
    pub fn restore_state(
        &mut self,
        state_id: u8,
        selected: usize,
        bm_selected: usize,
        bm_scroll: usize,
    ) {
        self.state = match state_id {
            1 => HomeState::ShowApps,
            2 => HomeState::ShowDailyMantra,
            11 => HomeState::ShowLuaDailyMantra,
            12 => HomeState::ShowLuaCalendar,
            3 => HomeState::ShowBookmarks,
            4 => HomeState::ShowCategoryItems,
            5 => HomeState::ShowPlaceholder,
            6 => HomeState::ShowNetworkStatus,
            7 => HomeState::ShowWifiConnect,
            8 => HomeState::ShowDateTime,
            9 => HomeState::ShowCalendar,
            10 => HomeState::ShowPanchangLite,
            _ => HomeState::Menu,
        };
        self.selected = selected.min(HOME_CARD_COUNT - 1);
        self.active_category = self
            .active_category
            .min(Self::category_count().saturating_sub(1));
        self.bm_selected = bm_selected;
        self.bm_scroll = bm_scroll;
        if self.state == HomeState::ShowBookmarks {
            self.needs_load_bookmarks = true;
        }
        if self.state == HomeState::ShowNetworkStatus || self.state == HomeState::ShowWifiConnect {
            self.network_status_loaded = false;
            self.needs_load_network_status = true;
        }
        if matches!(
            self.state,
            HomeState::ShowCalendar
                | HomeState::ShowPanchangLite
                | HomeState::ShowDailyMantra
                | HomeState::ShowDateTime
                | HomeState::ShowNetworkStatus
        ) {
            self.time_status_loaded = false;
            self.needs_load_time_status = true;
        }
        if self.state == HomeState::ShowLuaDailyMantra {
            self.needs_load_lua_daily_mantra = true;
        }
        if self.state == HomeState::ShowLuaCalendar {
            self.needs_load_lua_calendar = true;
        }
        log::info!(
            "home: restore_state state={:?} selected={}",
            self.state,
            selected
        );
    }

    fn clear_recent(&mut self) {
        self.recent_book_len = 0;
        self.recent_record = None;
    }

    fn set_recent_record(&mut self, rec: reader_state::RecentBookRecord) {
        let path = rec.open_path().trim();
        if path.is_empty() {
            self.clear_recent();
            return;
        }

        let bytes = path.as_bytes();
        let n = bytes.len().min(self.recent_book.len());
        self.recent_book[..n].copy_from_slice(&bytes[..n]);
        self.recent_book_len = n;
        self.recent_record = Some(rec);
    }

    fn load_recent_record_from_state(&mut self, k: &mut KernelHandle<'_>) -> bool {
        let mut buf = [0u8; 192];
        let size = match k.read_app_subdir_chunk(
            reader_state::STATE_DIR,
            reader_state::RECENT_RECORD_FILE,
            0,
            &mut buf,
        ) {
            Ok(n) if n > 0 => n,
            _ => return false,
        };

        let text = match core::str::from_utf8(&buf[..size]) {
            Ok(s) => s.trim(),
            Err(_) => return false,
        };

        let Some(rec) = reader_state::RecentBookRecord::decode_line(text) else {
            log::warn!("reader-state: ignored invalid typed recent record");
            return false;
        };

        if rec.source_path.trim().is_empty() {
            return false;
        }

        log::info!(
            "reader-slice: home loaded typed recent book_id={} path={}",
            rec.book_id.as_str(),
            rec.open_path()
        );
        self.set_recent_record(rec);
        true
    }

    fn load_recent_legacy(&mut self, k: &mut KernelHandle<'_>) -> bool {
        let mut buf = [0u8; RECENT_BUF_LEN];
        match k.read_app_data_start(RECENT_FILE, &mut buf) {
            Ok((_, n)) if n > 0 => {
                let text = match core::str::from_utf8(&buf[..n.min(buf.len())]) {
                    Ok(s) => s.trim(),
                    Err(_) => {
                        self.clear_recent();
                        return false;
                    }
                };

                if text.is_empty() || text.contains('|') {
                    // Continue must come from typed state. Legacy fallback is accepted
                    // only as a raw path, never as an encoded recent record
                    // leaked into UI/routing.
                    self.clear_recent();
                    return false;
                }

                let rec = reader_state::RecentBookRecord::from_path(text);
                log::info!(
                    "reader-state: upgraded legacy recent path to typed record book_id={} path={}",
                    rec.book_id.as_str(),
                    rec.source_path
                );
                self.set_recent_record(rec);
                true
            }
            _ => {
                self.clear_recent();
                false
            }
        }
    }

    pub fn load_recent(&mut self, k: &mut KernelHandle<'_>) {
        if !self.load_recent_record_from_state(k) && !self.load_recent_legacy(k) {
            self.clear_recent();
        }
        self.rebuild_item_count();
        self.refresh_menu_layout();
        self.needs_load_recent = false;
    }

    fn clear_network_ssid(&mut self) {
        self.network_ssid_len = 0;
    }

    fn set_network_ssid(&mut self, ssid: &str) {
        let bytes = ssid.as_bytes();
        let n = bytes.len().min(self.network_ssid.len());
        self.network_ssid[..n].copy_from_slice(&bytes[..n]);
        self.network_ssid_len = n;
    }

    fn network_ssid_str(&self) -> &str {
        core::str::from_utf8(&self.network_ssid[..self.network_ssid_len]).unwrap_or("")
    }

    fn clear_network_pass(&mut self) {
        self.network_pass_len = 0;
    }

    fn set_network_pass(&mut self, pass: &str) {
        let bytes = pass.as_bytes();
        let n = bytes.len().min(self.network_pass.len());
        self.network_pass[..n].copy_from_slice(&bytes[..n]);
        self.network_pass_len = n;
    }

    fn network_pass_str(&self) -> &str {
        core::str::from_utf8(&self.network_pass[..self.network_pass_len]).unwrap_or("")
    }

    fn current_wifi_profile_slot(&self) -> usize {
        (self.network_profile_slot as usize).min(config::WIFI_PROFILE_COUNT - 1)
    }

    fn current_wifi_profile_name(&self) -> &str {
        self.network_wifi
            .profile_name(self.current_wifi_profile_slot())
    }

    fn sync_wifi_editor_from_slot(&mut self) {
        let slot = self.current_wifi_profile_slot();

        let mut ssid_buf = [0u8; NETWORK_SSID_BUF_LEN];
        let ssid_len = {
            let bytes = self.network_wifi.profile_ssid(slot).as_bytes();
            let n = bytes.len().min(ssid_buf.len());
            ssid_buf[..n].copy_from_slice(&bytes[..n]);
            n
        };

        let mut pass_buf = [0u8; NETWORK_PASS_BUF_LEN];
        let pass_len = {
            let bytes = self.network_wifi.profile_password(slot).as_bytes();
            let n = bytes.len().min(pass_buf.len());
            pass_buf[..n].copy_from_slice(&bytes[..n]);
            n
        };

        self.network_ssid = ssid_buf;
        self.network_ssid_len = ssid_len;
        self.network_pass = pass_buf;
        self.network_pass_len = pass_len;
        self.network_default_slot = self.network_wifi.default_slot();
    }

    fn sync_wifi_editor_to_slot(&mut self) {
        let slot = self.current_wifi_profile_slot();

        let mut ssid_buf = [0u8; NETWORK_SSID_BUF_LEN];
        let ssid_len = self.network_ssid_len.min(ssid_buf.len());
        ssid_buf[..ssid_len].copy_from_slice(&self.network_ssid[..ssid_len]);

        let mut pass_buf = [0u8; NETWORK_PASS_BUF_LEN];
        let pass_len = self.network_pass_len.min(pass_buf.len());
        pass_buf[..pass_len].copy_from_slice(&self.network_pass[..pass_len]);

        let ssid_text = core::str::from_utf8(&ssid_buf[..ssid_len]).unwrap_or("");
        let pass_text = core::str::from_utf8(&pass_buf[..pass_len]).unwrap_or("");

        self.network_wifi
            .set_profile_credentials_from_str(slot, ssid_text, pass_text);
        self.network_wifi
            .set_default_slot(self.network_default_slot);
    }

    fn cycle_wifi_profile_slot(&mut self, delta: isize) {
        self.sync_wifi_editor_to_slot();
        self.network_profile_slot = (self.network_profile_slot as isize + delta)
            .rem_euclid(config::WIFI_PROFILE_COUNT as isize)
            as u8;
        self.sync_wifi_editor_from_slot();
        self.wifi_editor_message = "Profile changed; edit or set default";
    }

    fn set_current_wifi_profile_as_default(&mut self) {
        self.sync_wifi_editor_to_slot();
        self.network_default_slot = self.network_profile_slot;
        self.network_wifi
            .set_default_slot(self.network_default_slot);
        self.wifi_editor_message = "Default profile selected; Save to persist";
    }

    fn wifi_keyboard_layout(&self) -> u8 {
        match self.wifi_editor_mode {
            WIFI_EDITOR_MODE_KEYBOARD_UPPER => text_keyboard::LAYOUT_UPPER,
            WIFI_EDITOR_MODE_KEYBOARD_SYMBOLS => text_keyboard::LAYOUT_SYMBOLS,
            _ => text_keyboard::LAYOUT_LOWER,
        }
    }

    fn set_wifi_keyboard_layout(&mut self, layout: u8) {
        self.wifi_editor_mode = match text_keyboard::normalize_layout(layout) {
            text_keyboard::LAYOUT_UPPER => WIFI_EDITOR_MODE_KEYBOARD_UPPER,
            text_keyboard::LAYOUT_SYMBOLS => WIFI_EDITOR_MODE_KEYBOARD_SYMBOLS,
            _ => WIFI_EDITOR_MODE_KEYBOARD_LOWER,
        };
    }

    fn is_wifi_text_editing(&self) -> bool {
        self.wifi_editor_mode != WIFI_EDITOR_MODE_ROWS
    }

    fn enter_wifi_text_editor(&mut self) {
        if self.wifi_editor_field == 3 {
            // CrossPoint-style keyboard ownership: the active field buffer
            // is authoritative until Back/Done/Save. Do not allow Home
            // background reloads to pull an older password from SETTINGS.TXT.
            self.needs_load_network_status = false;
            self.network_status_loaded = true;
            self.sync_wifi_editor_to_slot();
            self.set_wifi_keyboard_layout(text_keyboard::LAYOUT_LOWER);
            self.wifi_editor_char_idx = text_keyboard::default_index();
            self.wifi_editor_message = "Keyboard active; Back returns to list";
        }
    }

    fn exit_wifi_text_editor(&mut self) {
        self.wifi_editor_mode = WIFI_EDITOR_MODE_ROWS;
        self.wifi_editor_message = "Select Save to write SETTINGS.TXT";
        self.wifi_editor_dirty = false;
    }

    fn move_wifi_editor_field(&mut self, delta: isize) {
        self.wifi_editor_field = (self.wifi_editor_field as isize + delta)
            .rem_euclid(WIFI_EDITOR_FIELD_COUNT as isize) as u8;
        self.wifi_editor_message = WIFI_EDITOR_SAVE_MSG;
    }

    fn move_wifi_keyboard_horizontal(&mut self, delta: isize) {
        self.wifi_editor_char_idx = text_keyboard::move_horizontal(
            self.wifi_keyboard_layout(),
            self.wifi_editor_char_idx,
            delta,
        );
        self.wifi_editor_message = "OK enters key; Back returns to list";
    }

    fn move_wifi_keyboard_vertical(&mut self, delta: isize) {
        self.wifi_editor_char_idx = text_keyboard::move_vertical(
            self.wifi_keyboard_layout(),
            self.wifi_editor_char_idx,
            delta,
        );
        self.wifi_editor_message = "OK enters key; Back returns to list";
    }

    fn switch_wifi_keyboard_layout(&mut self, layout: u8) {
        let (new_layout, new_idx) = text_keyboard::switch_layout(
            self.wifi_keyboard_layout(),
            layout,
            self.wifi_editor_char_idx,
        );
        self.set_wifi_keyboard_layout(new_layout);
        self.wifi_editor_char_idx = new_idx;
        self.wifi_editor_message = "Keyboard layout changed";
    }

    fn push_wifi_text_byte(&mut self, ch: u8) {
        match self.wifi_editor_field {
            3 if self.network_pass_len < self.network_pass.len() => {
                self.network_pass[self.network_pass_len] = ch;
                self.network_pass_len += 1;
                self.wifi_editor_message = "Password edited; Back then Save";
            }
            3 => {
                self.wifi_editor_message = "Field is full";
            }
            _ => {}
        }
    }

    fn append_wifi_editor_char(&mut self) {
        if !self.is_wifi_text_editing() {
            match self.wifi_editor_field {
                0 => self.cycle_wifi_profile_slot(1),
                1 => {
                    self.resume_wifi_after_scan = true;
                    self.wifi_editor_message = "Scanning SSIDs...";
                }
                2 => self.apply_selected_wifi_scan_result(),
                3 => self.enter_wifi_text_editor(),
                4 => self.set_current_wifi_profile_as_default(),
                _ => {
                    self.sync_wifi_editor_to_slot();
                    self.wifi_editor_save_pending = true;
                    self.wifi_editor_message = "Saving SETTINGS.TXT...";
                }
            }
            return;
        }

        match text_keyboard::activate(self.wifi_keyboard_layout(), self.wifi_editor_char_idx) {
            TextKeyboardAction::Insert(ch) => self.push_wifi_text_byte(ch),
            TextKeyboardAction::Space => self.push_wifi_text_byte(b' '),
            TextKeyboardAction::Delete => self.delete_wifi_editor_char(),
            TextKeyboardAction::Clear => self.clear_wifi_text_buffer(),
            TextKeyboardAction::ToggleCase => {
                if self.wifi_keyboard_layout() == text_keyboard::LAYOUT_UPPER {
                    self.switch_wifi_keyboard_layout(text_keyboard::LAYOUT_LOWER);
                } else {
                    self.switch_wifi_keyboard_layout(text_keyboard::LAYOUT_UPPER);
                }
            }
            TextKeyboardAction::ToggleSymbols => {
                if self.wifi_keyboard_layout() == text_keyboard::LAYOUT_SYMBOLS {
                    self.switch_wifi_keyboard_layout(text_keyboard::LAYOUT_LOWER);
                } else {
                    self.switch_wifi_keyboard_layout(text_keyboard::LAYOUT_SYMBOLS);
                }
            }
            TextKeyboardAction::Done => self.exit_wifi_text_editor(),
            TextKeyboardAction::None => {}
        }
    }

    fn delete_wifi_editor_char(&mut self) {
        if !self.is_wifi_text_editing() {
            self.wifi_editor_message = "Select field first";
            return;
        }

        match self.wifi_editor_field {
            3 if self.network_pass_len > 0 => {
                self.network_pass_len -= 1;
                self.wifi_editor_message = "Deleted password character";
            }
            3 => {
                self.wifi_editor_message = "Nothing to delete";
            }
            _ => {}
        }
    }

    fn clear_wifi_text_buffer(&mut self) {
        match self.wifi_editor_field {
            3 => {
                self.network_pass_len = 0;
                self.wifi_editor_message = "Password cleared";
            }
            _ => {}
        }
    }

    fn clear_wifi_editor_field(&mut self) {
        if self.is_wifi_text_editing() {
            self.clear_wifi_text_buffer();
            return;
        }

        match self.wifi_editor_field {
            0 => self.cycle_wifi_profile_slot(1),
            1 => self.wifi_editor_message = "Select starts scan mode",
            2 => self.apply_selected_wifi_scan_result(),
            3 => self.enter_wifi_text_editor(),
            4 => self.set_current_wifi_profile_as_default(),
            _ => {
                self.wifi_editor_message = "Select Save to write SETTINGS.TXT";
            }
        }
    }

    fn clear_wifi_scan_results(&mut self) {
        self.network_scan_results = [[0u8; WIFI_SCAN_SSID_BUF_LEN]; WIFI_SCAN_MAX_RESULTS];
        self.network_scan_lens = [0u8; WIFI_SCAN_MAX_RESULTS];
        self.network_scan_count = 0;
        self.network_scan_selected = 0;
    }

    fn wifi_scan_result_str(&self, idx: usize) -> &str {
        if idx >= self.network_scan_count {
            return "";
        }
        let n = self.network_scan_lens[idx] as usize;
        core::str::from_utf8(&self.network_scan_results[idx][..n]).unwrap_or("")
    }

    fn wifi_scan_selected_str(&self) -> &str {
        if self.network_scan_count == 0 {
            ""
        } else {
            self.wifi_scan_result_str(self.network_scan_selected.min(self.network_scan_count - 1))
        }
    }

    fn wifi_scan_contains(&self, ssid: &[u8]) -> bool {
        let mut idx = 0usize;
        while idx < self.network_scan_count {
            let n = self.network_scan_lens[idx] as usize;
            if &self.network_scan_results[idx][..n] == ssid {
                return true;
            }
            idx += 1;
        }
        false
    }

    fn load_wifi_scan_results(&mut self, k: &mut KernelHandle<'_>) {
        self.clear_wifi_scan_results();
        let mut buf = [0u8; 768];
        let Ok(n) = k.read_cache_chunk(wifi_scan::WIFI_SCAN_FILE, 0, &mut buf) else {
            return;
        };
        for raw in buf[..n].split(|&b| b == b'\n') {
            let line = Self::trim_ascii(raw);
            if line.is_empty() || line[0] == b'#' || self.wifi_scan_contains(line) {
                continue;
            }
            if self.network_scan_count >= WIFI_SCAN_MAX_RESULTS {
                break;
            }
            let idx = self.network_scan_count;
            let len = line.len().min(WIFI_SCAN_SSID_BUF_LEN);
            self.network_scan_results[idx][..len].copy_from_slice(&line[..len]);
            self.network_scan_lens[idx] = len as u8;
            self.network_scan_count += 1;
        }
        if self.network_scan_count > 0 {
            self.network_scan_selected =
                self.network_scan_selected.min(self.network_scan_count - 1);
        }
    }

    fn cycle_wifi_scan_result(&mut self, delta: isize) {
        if self.network_scan_count == 0 {
            self.wifi_editor_message = "Select Scan SSIDs first";
            return;
        }
        self.network_scan_selected = (self.network_scan_selected as isize + delta)
            .rem_euclid(self.network_scan_count as isize)
            as usize;
        self.wifi_editor_message = "Select applies scanned SSID";
    }

    fn apply_selected_wifi_scan_result(&mut self) {
        if self.network_scan_count == 0 {
            self.wifi_editor_message = "No scan results; select Scan SSIDs";
            return;
        }
        let idx = self.network_scan_selected.min(self.network_scan_count - 1);
        let n = self.network_scan_lens[idx] as usize;
        self.network_ssid = [0u8; NETWORK_SSID_BUF_LEN];
        let len = n.min(self.network_ssid.len());
        self.network_ssid[..len].copy_from_slice(&self.network_scan_results[idx][..len]);
        self.network_ssid_len = len;
        self.sync_wifi_editor_to_slot();
        self.wifi_editor_message = "SSID selected; enter password then Save";
    }

    fn save_wifi_settings(&mut self, k: &mut KernelHandle<'_>) -> bool {
        let mut read_buf = [0u8; NETWORK_STATUS_BUF_LEN];
        let mut settings = SystemSettings::defaults();
        let mut existing_wifi = WifiConfig::empty();

        if let Ok((_size, n)) = k.read_app_data_start(config::SETTINGS_FILE, &mut read_buf) {
            if n > 0 {
                config::parse_settings_txt(&read_buf[..n], &mut settings, &mut existing_wifi);
            }
        }

        settings.sanitize();
        self.sync_wifi_editor_to_slot();
        let wifi = self.network_wifi;

        let mut out = [0u8; NETWORK_STATUS_BUF_LEN];
        let len = write_settings_txt(&settings, &wifi, &mut out);
        match k.write_app_data(config::SETTINGS_FILE, &out[..len]) {
            Ok(_) => {
                self.wifi_editor_message = "Saved to _x4/SETTINGS.TXT";
                true
            }
            Err(_) => {
                self.wifi_editor_message = "Save failed";
                false
            }
        }
    }

    fn load_network_status(&mut self, k: &mut KernelHandle<'_>) {
        self.network_sd_ok = k.sd_ok();
        self.network_battery_mv = k.battery_mv();
        self.network_uptime_secs = k.uptime_secs();
        if self.state == HomeState::ShowWifiConnect
            && (self.is_wifi_text_editing() || self.wifi_editor_dirty)
        {
            // Do not reload SETTINGS.TXT over an active/dirty keyboard session.
            self.network_status_loaded = true;
            self.network_wifi
                .set_default_slot(self.network_default_slot);
            self.sync_wifi_editor_to_slot();
            self.network_wifi_configured = self.network_wifi.has_credentials();
            self.network_wifi_password_saved = !self.network_wifi.password().is_empty();
            self.needs_load_network_status = false;
            return;
        }
        self.network_settings_found = false;
        self.network_wifi_configured = false;
        self.network_wifi_password_saved = false;
        let preserve_profile_slot = self
            .network_profile_slot
            .min((config::WIFI_PROFILE_COUNT - 1) as u8);
        let preserve_wifi_screen = self.state == HomeState::ShowWifiConnect;
        self.network_wifi = WifiConfig::empty();
        self.network_default_slot = 0;
        self.network_profile_slot = 0;
        self.clear_network_ssid();
        self.clear_network_pass();

        let mut buf = [0u8; NETWORK_STATUS_BUF_LEN];
        if let Ok((_size, n)) = k.read_app_data_start(config::SETTINGS_FILE, &mut buf) {
            if n > 0 {
                let mut settings = SystemSettings::defaults();
                let mut wifi = WifiConfig::empty();
                config::parse_settings_txt(&buf[..n], &mut settings, &mut wifi);
                self.network_settings_found = true;
                self.network_wifi = wifi;
                self.network_default_slot = wifi.default_slot();
                self.network_profile_slot = if preserve_wifi_screen {
                    preserve_profile_slot
                } else {
                    self.network_default_slot
                };
                self.sync_wifi_editor_from_slot();
                if wifi.has_credentials() {
                    self.network_wifi_configured = true;
                    self.network_wifi_password_saved = !wifi.password().is_empty();
                }
            }
        }

        self.load_wifi_scan_results(k);
        self.network_status_loaded = true;
        self.needs_load_network_status = false;
    }

    fn load_panchang_mantra(&mut self, k: &mut KernelHandle<'_>) {
        self.clear_panchang_mantra();
        self.panchang_mantra_loaded = true;
        self.needs_load_panchang_mantra = false;

        if !self.time_status_loaded {
            return;
        }

        let Some(panchang) = self.time_cache.display_panchang_lite(self.time_uptime_secs) else {
            return;
        };

        let Some(tithi_key) = Self::panchang_tithi_file_key(panchang.tithi) else {
            return;
        };

        let files = [
            PANCHANG_MANTRA_FILE,
            PANCHANG_MANTRA_ALT_FILE,
            PANCHANG_MANTRA_SHORT_FILE,
            PANCHANG_MANTRA_SHORT_UPPER_FILE,
            PANCHANG_MANTRA_SHORT_83_FILE,
        ];
        for name in files {
            match self.scan_panchang_mantra_file(
                k,
                name,
                panchang.month,
                panchang.paksha,
                tithi_key,
            ) {
                PanchangMantraScan::Found => {
                    self.panchang_mantra_file_found = true;
                    self.panchang_mantra_found = true;
                    return;
                }
                PanchangMantraScan::FileFound => {
                    self.panchang_mantra_file_found = true;
                }
                PanchangMantraScan::Missing => {}
            }
        }
    }

    fn scan_panchang_mantra_file(
        &mut self,
        k: &mut KernelHandle<'_>,
        name: &str,
        month: &str,
        paksha: &str,
        tithi_key: &str,
    ) -> PanchangMantraScan {
        let Ok(size) = k.file_size(name) else {
            return PanchangMantraScan::Missing;
        };

        let mut chunk = [0u8; PANCHANG_MANTRA_SCAN_BUF_LEN];
        let mut line = [0u8; PANCHANG_MANTRA_LINE_BUF_LEN];
        let mut line_len = 0usize;
        let mut offset = 0u32;

        while offset < size {
            let Ok(n) = k.read_chunk(name, offset, &mut chunk) else {
                break;
            };
            if n == 0 {
                break;
            }

            for &b in &chunk[..n] {
                if b == b'\n' {
                    if Self::capture_panchang_mantra_line(
                        &line[..line_len],
                        month,
                        paksha,
                        tithi_key,
                        &mut self.panchang_mantra,
                        &mut self.panchang_mantra_len,
                    ) {
                        return PanchangMantraScan::Found;
                    }
                    line_len = 0;
                } else if line_len < line.len() {
                    line[line_len] = b;
                    line_len += 1;
                }
            }

            offset = offset.saturating_add(n as u32);
            if n < chunk.len() {
                break;
            }
        }

        if line_len > 0
            && Self::capture_panchang_mantra_line(
                &line[..line_len],
                month,
                paksha,
                tithi_key,
                &mut self.panchang_mantra,
                &mut self.panchang_mantra_len,
            )
        {
            return PanchangMantraScan::Found;
        }

        PanchangMantraScan::FileFound
    }

    fn capture_panchang_mantra_line(
        line: &[u8],
        month: &str,
        paksha: &str,
        tithi_key: &str,
        out: &mut [u8; PANCHANG_MANTRA_BUF_LEN],
        out_len: &mut usize,
    ) -> bool {
        let Some(month_field) = Self::panchang_field(line, 0) else {
            return false;
        };
        let Some(paksha_field) = Self::panchang_field(line, 1) else {
            return false;
        };
        let Some(tithi_field) = Self::panchang_field(line, 2) else {
            return false;
        };
        let Some(english_field) = Self::panchang_field(line, 5) else {
            return false;
        };

        if !Self::panchang_month_matches(month_field, month) {
            return false;
        }
        if !Self::panchang_paksha_matches(paksha_field, paksha) {
            return false;
        }
        if !Self::panchang_tithi_matches(tithi_field, tithi_key) {
            return false;
        }

        *out_len = Self::copy_panchang_ascii(english_field, out);
        *out_len > 0
    }

    fn panchang_field(line: &[u8], index: usize) -> Option<&[u8]> {
        let mut start = 0usize;
        let mut field = 0usize;

        for (pos, &b) in line.iter().enumerate() {
            if b == b'|' {
                if field == index {
                    return Some(Self::trim_ascii(&line[start..pos]));
                }
                field += 1;
                start = pos + 1;
            }
        }

        if field == index {
            return Some(Self::trim_ascii(&line[start..]));
        }

        None
    }

    fn trim_ascii(mut input: &[u8]) -> &[u8] {
        while let Some((&first, rest)) = input.split_first() {
            if first == b' ' || first == b'\t' || first == b'\r' {
                input = rest;
            } else {
                break;
            }
        }

        while let Some((&last, rest)) = input.split_last() {
            if last == b' ' || last == b'\t' || last == b'\r' {
                input = rest;
            } else {
                break;
            }
        }

        input
    }

    fn panchang_month_matches(field: &[u8], month: &str) -> bool {
        Self::ascii_eq_ignore_case(field, month.as_bytes())
            || match month {
                "Vaishakha" => Self::ascii_eq_ignore_case(field, b"Vaisakha"),
                "Jyeshtha" => Self::ascii_eq_ignore_case(field, b"Jyaistha"),
                "Ashwin" => Self::ascii_eq_ignore_case(field, b"Ashvina"),
                _ => false,
            }
    }

    fn panchang_paksha_matches(field: &[u8], paksha: &str) -> bool {
        let trimmed = Self::trim_ascii(field);
        let paksha = paksha.as_bytes();
        trimmed.len() >= paksha.len()
            && Self::ascii_eq_ignore_case(&trimmed[..paksha.len()], paksha)
    }

    fn panchang_tithi_matches(field: &[u8], key: &str) -> bool {
        let value = if let Some(colon) = field.iter().position(|&b| b == b':') {
            Self::trim_ascii(&field[colon + 1..])
        } else {
            Self::trim_ascii(field)
        };
        Self::ascii_eq_ignore_case(value, key.as_bytes())
    }

    fn ascii_eq_ignore_case(left: &[u8], right: &[u8]) -> bool {
        if left.len() != right.len() {
            return false;
        }

        left.iter()
            .zip(right.iter())
            .all(|(&a, &b)| Self::ascii_lower(a) == Self::ascii_lower(b))
    }

    fn ascii_lower(b: u8) -> u8 {
        if b.is_ascii_uppercase() { b + 32 } else { b }
    }

    fn panchang_tithi_file_key(tithi: &str) -> Option<&'static str> {
        match tithi {
            "Pratipada" => Some("1"),
            "Dwitiya" => Some("2"),
            "Tritiya" => Some("3"),
            "Chaturthi" => Some("4"),
            "Panchami" => Some("5"),
            "Shashthi" => Some("6"),
            "Saptami" => Some("7"),
            "Ashtami" => Some("8"),
            "Navami" => Some("9"),
            "Dashami" => Some("10"),
            "Ekadashi" => Some("11"),
            "Dwadashi" => Some("12"),
            "Trayodashi" => Some("13"),
            "Chaturdashi" => Some("14"),
            "Purnima" => Some("Purnima"),
            "Amavasya" => Some("Amavasya"),
            _ => None,
        }
    }

    fn copy_panchang_ascii(src: &[u8], out: &mut [u8; PANCHANG_MANTRA_BUF_LEN]) -> usize {
        let src = Self::trim_ascii(src);
        let mut len = 0usize;

        for &b in src {
            if len >= out.len().saturating_sub(1) {
                break;
            }

            if b == b'\r' || b == b'\n' {
                break;
            }

            if b.is_ascii() && !b.is_ascii_control() {
                out[len] = b;
                len += 1;
            }
        }

        len
    }

    fn clear_panchang_mantra(&mut self) {
        self.panchang_mantra_len = 0;
        self.panchang_mantra_found = false;
        self.panchang_mantra_file_found = false;
        self.panchang_mantra.fill(0);
    }

    fn panchang_mantra_status(&self) -> &str {
        if self.panchang_mantra_found {
            return self.panchang_mantra_str().unwrap_or("--");
        }

        if !self.panchang_mantra_loaded {
            "Loading mantra from SD"
        } else if !self.panchang_mantra_file_found {
            "Mantra file not found: try Yearly_h.txt"
        } else {
            "No matching mantra in file"
        }
    }

    fn panchang_mantra_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.panchang_mantra[..self.panchang_mantra_len])
            .ok()
            .filter(|s| !s.trim().is_empty())
    }

    fn load_lua_daily_mantra(&mut self, k: &mut KernelHandle<'_>) {
        let mut manifest_buf = [0u8; 1024];
        let mut script_buf = [0u8; 4096];
        let mut mantras_buf = [0u8; 2048];

        self.lua_daily_mantra_screen = match k.read_lua_app_file_start(
            LUA_DAILY_MANTRA_APP_FOLDER,
            LUA_DAILY_MANTRA_MANIFEST_FILE,
            &mut manifest_buf,
        ) {
            Ok((_size, manifest_n)) if manifest_n > 0 => {
                let manifest = match core::str::from_utf8(&manifest_buf[..manifest_n]) {
                    Ok(text) => text,
                    Err(_) => {
                        self.needs_load_lua_daily_mantra = false;
                        self.lua_daily_mantra_screen =
                            LuaDailyMantraScreen::invalid_manifest_utf8();
                        log::info!(
                            "lua-app marker={} app=daily_mantra source={}",
                            LUA_DAILY_MANTRA_APP_MARKER,
                            self.lua_daily_mantra_screen.source.label()
                        );
                        return;
                    }
                };

                match k.read_lua_app_file_start(
                    LUA_DAILY_MANTRA_APP_FOLDER,
                    LUA_DAILY_MANTRA_ENTRY_FILE,
                    &mut script_buf,
                ) {
                    Ok((_size, script_n)) if script_n > 0 => {
                        let script = match core::str::from_utf8(&script_buf[..script_n]) {
                            Ok(text) => text,
                            Err(_) => {
                                self.needs_load_lua_daily_mantra = false;
                                self.lua_daily_mantra_screen =
                                    LuaDailyMantraScreen::invalid_script_utf8();
                                log::info!(
                                    "lua-app marker={} app=daily_mantra source={}",
                                    LUA_DAILY_MANTRA_APP_MARKER,
                                    self.lua_daily_mantra_screen.source.label()
                                );
                                return;
                            }
                        };

                        match k.read_lua_app_file_start(
                            LUA_DAILY_MANTRA_APP_FOLDER,
                            LUA_DAILY_MANTRA_MANTRAS_FILE,
                            &mut mantras_buf,
                        ) {
                            Ok((_size, mantras_n)) if mantras_n > 0 => {
                                match core::str::from_utf8(&mantras_buf[..mantras_n]) {
                                    Ok(mantras) => {
                                        #[cfg(feature = "lua-vm")]
                                        {
                                            crate::vaachak_x4::lua::daily_mantra_vm_bridge::build_daily_mantra_vm_sd_runtime(
                                                    manifest, script, mantras,
                                                )
                                        }
                                        #[cfg(not(feature = "lua-vm"))]
                                        {
                                            crate::vaachak_x4::lua::daily_mantra_script::build_daily_mantra_sd_runtime(
                                                    manifest, script, mantras,
                                                )
                                        }
                                    }
                                    Err(_) => LuaDailyMantraScreen::invalid_mantras_utf8(),
                                }
                            }
                            _ => LuaDailyMantraScreen::missing_mantras(),
                        }
                    }
                    _ => LuaDailyMantraScreen::missing_entry(),
                }
            }
            _ => LuaDailyMantraScreen::missing_manifest(),
        };
        self.needs_load_lua_daily_mantra = false;
        log::info!(
            "lua-app marker={} sd_runtime_marker={} app=daily_mantra source={}",
            LUA_DAILY_MANTRA_APP_MARKER,
            LUA_DAILY_MANTRA_SD_RUNTIME_MARKER,
            self.lua_daily_mantra_screen.source.label()
        );
    }

    fn load_lua_panchang(&mut self, k: &mut KernelHandle<'_>) {
        let mut manifest_buf = [0u8; 1024];
        let mut script_buf = [0u8; 4096];
        let mut data_buf = [0u8; 2048];

        self.lua_panchang_screen = match k.read_lua_app_file_start(
            LUA_PANCHANG_APP_FOLDER,
            LUA_PANCHANG_MANIFEST_FILE,
            &mut manifest_buf,
        ) {
            Ok((_size, manifest_n)) if manifest_n > 0 => {
                let manifest = match core::str::from_utf8(&manifest_buf[..manifest_n]) {
                    Ok(text) => text,
                    Err(_) => {
                        self.lua_panchang_screen = LuaPanchangScreen::invalid_manifest_utf8();
                        self.needs_load_lua_panchang = false;
                        return;
                    }
                };

                match k.read_lua_app_file_start(
                    LUA_PANCHANG_APP_FOLDER,
                    LUA_PANCHANG_ENTRY_FILE,
                    &mut script_buf,
                ) {
                    Ok((_size, script_n)) if script_n > 0 => {
                        let script = match core::str::from_utf8(&script_buf[..script_n]) {
                            Ok(text) => text,
                            Err(_) => {
                                self.lua_panchang_screen = LuaPanchangScreen::invalid_script_utf8();
                                self.needs_load_lua_panchang = false;
                                return;
                            }
                        };

                        match k.read_lua_panchang_y2026_start(&mut data_buf) {
                            Ok((_size, data_n)) if data_n > 0 => {
                                match core::str::from_utf8(&data_buf[..data_n]) {
                                    Ok(data) => build_panchang_sd_runtime(manifest, script, data),
                                    Err(_) => LuaPanchangScreen::invalid_data_utf8(),
                                }
                            }
                            _ => {
                                // Compatibility fallback for emergency/manual recovery: if the
                                // nested DATA folder cannot be opened, allow a flat app-root copy
                                // at `/VAACHAK/APPS/PANCHANG/Y2026.TXT`. The canonical contract
                                // remains `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT`.
                                match k.read_lua_app_file_start(
                                    LUA_PANCHANG_APP_FOLDER,
                                    LUA_PANCHANG_DATA_FILE,
                                    &mut data_buf,
                                ) {
                                    Ok((_size, data_n)) if data_n > 0 => {
                                        match core::str::from_utf8(&data_buf[..data_n]) {
                                            Ok(data) => {
                                                build_panchang_sd_runtime(manifest, script, data)
                                            }
                                            Err(_) => LuaPanchangScreen::invalid_data_utf8(),
                                        }
                                    }
                                    _ => LuaPanchangScreen::missing_data(),
                                }
                            }
                        }
                    }
                    _ => LuaPanchangScreen::missing_entry(),
                }
            }
            _ => LuaPanchangScreen::missing_manifest(),
        };
        self.needs_load_lua_panchang = false;
        log::info!(
            "lua-app marker={} sd_runtime_marker={} app=panchang source={}",
            LUA_PANCHANG_APP_MARKER,
            LUA_PANCHANG_SD_RUNTIME_MARKER,
            self.lua_panchang_screen.source.label()
        );
    }

    fn load_lua_calendar(&mut self, k: &mut KernelHandle<'_>) {
        let mut manifest_buf = [0u8; 1024];
        let mut script_buf = [0u8; 4096];
        let mut events_buf = [0u8; 2048];

        self.lua_calendar_screen = match k.read_lua_app_file_start(
            LUA_CALENDAR_APP_FOLDER,
            LUA_CALENDAR_MANIFEST_FILE,
            &mut manifest_buf,
        ) {
            Ok((_size, manifest_n)) if manifest_n > 0 => {
                let manifest = match core::str::from_utf8(&manifest_buf[..manifest_n]) {
                    Ok(text) => text,
                    Err(_) => {
                        self.needs_load_lua_calendar = false;
                        self.lua_calendar_screen = LuaCalendarScreen::invalid_manifest_utf8();
                        log::info!(
                            "lua-app marker={} app=calendar source={}",
                            LUA_CALENDAR_APP_MARKER,
                            self.lua_calendar_screen.source.label()
                        );
                        return;
                    }
                };

                match k.read_lua_app_file_start(
                    LUA_CALENDAR_APP_FOLDER,
                    LUA_CALENDAR_ENTRY_FILE,
                    &mut script_buf,
                ) {
                    Ok((_size, script_n)) if script_n > 0 => {
                        let script = match core::str::from_utf8(&script_buf[..script_n]) {
                            Ok(text) => text,
                            Err(_) => {
                                self.needs_load_lua_calendar = false;
                                self.lua_calendar_screen = LuaCalendarScreen::invalid_script_utf8();
                                log::info!(
                                    "lua-app marker={} app=calendar source={}",
                                    LUA_CALENDAR_APP_MARKER,
                                    self.lua_calendar_screen.source.label()
                                );
                                return;
                            }
                        };

                        match k.read_lua_app_file_start(
                            LUA_CALENDAR_APP_FOLDER,
                            LUA_CALENDAR_EVENTS_FILE,
                            &mut events_buf,
                        ) {
                            Ok((_size, events_n)) if events_n > 0 => {
                                match core::str::from_utf8(&events_buf[..events_n]) {
                                    Ok(events) => {
                                        build_calendar_sd_runtime(manifest, script, events)
                                    }
                                    Err(_) => LuaCalendarScreen::invalid_events_utf8(),
                                }
                            }
                            _ => LuaCalendarScreen::missing_events(),
                        }
                    }
                    _ => LuaCalendarScreen::missing_entry(),
                }
            }
            _ => LuaCalendarScreen::missing_manifest(),
        };
        self.needs_load_lua_calendar = false;
        log::info!(
            "lua-app marker={} sd_runtime_marker={} app=calendar source={}",
            LUA_CALENDAR_APP_MARKER,
            LUA_CALENDAR_SD_RUNTIME_MARKER,
            self.lua_calendar_screen.source.label()
        );
    }

    fn load_time_status(&mut self, k: &mut KernelHandle<'_>) {
        self.time_uptime_secs = k.uptime_secs();
        self.home_battery_mv = k.battery_mv();

        let mut buf = [0u8; TIME_STATUS_BUF_LEN];
        self.time_cache = match k.read_app_data_start(time_status::TIME_FILE, &mut buf) {
            Ok((_size, n)) if n > 0 => time_status::parse_time_txt(&buf[..n]),
            _ => time_status::TimeCache::default(),
        };
        self.time_status_loaded = true;
        self.needs_load_time_status = false;
    }

    fn rebuild_item_count(&mut self) {
        self.item_count = HOME_CARD_COUNT;
        if self.selected >= self.item_count {
            self.selected = 0;
        }
    }

    fn has_recent(&self) -> bool {
        self.recent_record.is_some() && self.recent_book_len > 0
    }

    pub fn recent_book_bytes(&self) -> Option<&[u8]> {
        if self.has_recent() {
            Some(&self.recent_book[..self.recent_book_len])
        } else {
            None
        }
    }

    pub fn recent_book_str(&self) -> Option<&str> {
        self.recent_book_bytes()
            .and_then(|bytes| core::str::from_utf8(bytes).ok())
            .filter(|s| !s.trim().is_empty())
    }

    pub fn recent_record(&self) -> Option<reader_state::RecentBookRecord> {
        self.recent_record.clone()
    }

    pub fn recent_book_id(&self) -> Option<String> {
        self.recent_record().map(|rec| rec.book_id.0)
    }

    pub fn recent_source_path(&self) -> Option<String> {
        self.recent_record().map(|rec| rec.source_path)
    }

    fn basename(path: &str) -> &str {
        path.rsplit('/').next().unwrap_or(path)
    }

    pub fn shell_menu_items(&self) -> Vec<HomeMenuItem> {
        let mut items = Vec::with_capacity(self.item_count);
        items.push(HomeMenuItem::ContinueReading);
        items.push(HomeMenuItem::FileBrowser);
        items.push(HomeMenuItem::Bookmarks);
        items.push(HomeMenuItem::Settings);
        items.push(HomeMenuItem::Sync);
        items.push(HomeMenuItem::Upload);
        items
    }

    fn item_action(&self, idx: usize) -> MenuAction {
        MenuAction::OpenCategory(idx.min(Self::category_count().saturating_sub(1)))
    }

    fn set_selection(&mut self, selected: usize, ctx: &mut AppContext) {
        let count = self.item_count;
        if count == 0 {
            return;
        }
        let new = selected.min(count - 1);
        if new != self.selected {
            ctx.mark_dirty(self.item_regions[self.selected]);
            self.selected = new;
            ctx.mark_dirty(self.item_regions[self.selected]);
        }
    }

    fn move_selection_row(&mut self, delta: isize, ctx: &mut AppContext) {
        let col = self.selected % HOME_GRID_COLS;
        let row = self.selected / HOME_GRID_COLS;
        let next_row = if delta > 0 {
            (row + 1).min(HOME_GRID_ROWS - 1)
        } else {
            row.saturating_sub(1)
        };
        self.set_selection(next_row * HOME_GRID_COLS + col, ctx);
    }

    fn move_selection_col(&mut self, delta: isize, ctx: &mut AppContext) {
        let col = self.selected % HOME_GRID_COLS;
        let row = self.selected / HOME_GRID_COLS;
        let next_col = if delta > 0 {
            (col + 1).min(HOME_GRID_COLS - 1)
        } else {
            col.saturating_sub(1)
        };
        self.set_selection(row * HOME_GRID_COLS + next_col, ctx);
    }

    #[inline]
    fn header_font(&self) -> &'static BitmapFont {
        fonts::heading_font(0)
    }

    #[inline]
    fn card_title_font(&self) -> &'static BitmapFont {
        fonts::body_font(1)
    }

    #[inline]
    fn card_meta_font(&self) -> &'static BitmapFont {
        fonts::chrome_font()
    }

    fn bm_group_title(&self) -> Option<&str> {
        let first = self.bm_entries.first()?;
        let first_id = first.book_id.as_str();

        if self
            .bm_entries
            .iter()
            .all(|entry| entry.book_id.as_str() == first_id)
        {
            let title = first.display_title.trim();
            if !title.is_empty() {
                Some(title)
            } else {
                Some(Self::basename(&first.source_path))
            }
        } else {
            None
        }
    }

    fn ellipsize_ascii(s: &str, max_chars: usize) -> String {
        let trimmed = s.trim();
        let mut out = String::new();
        let mut chars = trimmed.chars();

        for _ in 0..max_chars {
            match chars.next() {
                Some(ch) => out.push(ch),
                None => return out,
            }
        }

        if chars.next().is_some() && max_chars > 3 {
            for _ in 0..3 {
                out.pop();
            }
            out.push_str("...");
        }

        out
    }

    fn bm_detail_label(entry: &reader_state::BookmarkIndexRecord) -> String {
        let detail = entry.label.trim();
        if !detail.is_empty() {
            return Self::ellipsize_ascii(detail, BM_MAX_ROW_CHARS);
        }

        let mut out = String::new();
        let _ = write!(out, "Ch {}", u32::from(entry.chapter) + 1);
        if entry.byte_offset > 0 {
            let _ = write!(out, " · Off {}", entry.byte_offset);
        }
        out
    }

    fn bm_mixed_book_label(entry: &reader_state::BookmarkIndexRecord) -> String {
        let title = if !entry.display_title.trim().is_empty() {
            entry.display_title.trim()
        } else {
            Self::basename(&entry.source_path)
        };

        let mut out = Self::ellipsize_ascii(title, BM_MAX_MIXED_TITLE_CHARS);
        out.push_str(" · ");
        out.push_str(&Self::bm_detail_label(entry));
        Self::ellipsize_ascii(&out, BM_MAX_ROW_CHARS)
    }

    fn bm_subtitle_region(&self) -> Region {
        Region::new(
            LARGE_MARGIN,
            BM_TITLE_Y + self.ui_fonts.heading.line_height + BM_BOOK_TITLE_GAP,
            FULL_CONTENT_W,
            self.ui_fonts.body.line_height,
        )
    }

    fn bm_list_y(&self) -> u16 {
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + BM_HEADER_LIST_GAP;
        if self.bm_group_title().is_some() {
            y += self.ui_fonts.body.line_height + BM_BOOK_TITLE_GAP;
        }
        y
    }

    fn bm_visible_lines(&self) -> usize {
        let available = SCREEN_H.saturating_sub(self.bm_list_y());
        let rows = (available / BM_ROW_STRIDE) as usize;
        rows.max(1).min(64)
    }

    fn bm_row_region(&self, i: usize) -> Region {
        Region::new(
            LARGE_MARGIN,
            self.bm_list_y() + i as u16 * BM_ROW_STRIDE,
            FULL_CONTENT_W,
            BM_ROW_H,
        )
    }

    fn bm_list_region(&self) -> Region {
        let vis = self.bm_visible_lines();
        let subtitle_extra = if self.bm_group_title().is_some() {
            self.ui_fonts.body.line_height + BM_BOOK_TITLE_GAP
        } else {
            0
        };

        Region::new(
            LARGE_MARGIN,
            BM_TITLE_Y + self.ui_fonts.heading.line_height + BM_BOOK_TITLE_GAP,
            FULL_CONTENT_W,
            subtitle_extra + BM_ROW_STRIDE * vis as u16,
        )
    }

    fn bm_status_region(&self) -> Region {
        Region::new(
            BM_STATUS_X,
            BM_TITLE_Y,
            BM_STATUS_W,
            self.ui_fonts.heading.line_height,
        )
    }
}

impl App<AppId> for HomeApp {
    fn on_enter(&mut self, ctx: &mut AppContext, _k: &mut KernelHandle<'_>) {
        ctx.clear_message();
        self.state = HomeState::Menu;
        self.selected = 0;
        self.needs_load_recent = true;
        self.needs_load_bookmarks = true;
        self.needs_load_time_status = true;
        ctx.mark_dirty(CONTENT_REGION);
    }

    fn on_resume(&mut self, ctx: &mut AppContext, _k: &mut KernelHandle<'_>) {
        if self.resume_wifi_after_scan {
            self.resume_wifi_after_scan = false;
            self.state = HomeState::ShowWifiConnect;
            self.return_target = ReturnTarget::CategoryItems;
            self.network_status_loaded = false;
            self.needs_load_network_status = true;
            self.wifi_editor_field = 2;
            self.wifi_editor_message = "Scan complete; choose SSID then enter password";
        } else if self.resume_date_time_after_sync {
            self.resume_date_time_after_sync = false;
            self.state = HomeState::ShowDateTime;
            self.return_target = ReturnTarget::CategoryItems;
        } else {
            self.state = HomeState::Menu;
            self.selected = 0;
        }
        self.needs_load_recent = true;
        self.needs_load_bookmarks = true;
        self.needs_load_time_status = true;
        ctx.mark_dirty(CONTENT_REGION);
    }

    async fn background(&mut self, ctx: &mut AppContext, k: &mut KernelHandle<'_>) {
        if self.state == HomeState::ShowWifiConnect && self.is_wifi_text_editing() {
            // CrossPoint-style keyboard session active: no Home background SD
            // reloads and no background-requested redraws while navigating the
            // on-screen keyboard. The in-memory SSID/password buffers remain
            // authoritative until the user exits or saves.
            self.needs_load_network_status = false;
            self.network_status_loaded = true;
            self.sync_wifi_editor_to_slot();
            let _ = ctx.take_redraw();
            return;
        }

        if self.needs_load_recent {
            let old_count = self.item_count;
            if !self.load_recent_record_from_state(k) && !self.load_recent_legacy(k) {
                self.clear_recent();
            }
            self.rebuild_item_count();
            self.needs_load_recent = false;
            self.refresh_menu_layout();
            if self.item_count != old_count {
                ctx.request_full_redraw();
            } else if self.state == HomeState::Menu {
                ctx.mark_dirty(self.item_regions[0]);
            }
        }

        if self.needs_load_network_status {
            self.load_network_status(k);
            if self.state == HomeState::ShowNetworkStatus
                || self.state == HomeState::ShowWifiConnect
            {
                ctx.request_full_redraw();
            }
        }

        if self.wifi_editor_save_pending {
            self.wifi_editor_save_pending = false;
            self.wifi_editor_dirty = false;
            let _ = self.save_wifi_settings(k);
            self.load_network_status(k);
            if self.state == HomeState::ShowWifiConnect
                || self.state == HomeState::ShowNetworkStatus
            {
                ctx.request_full_redraw();
            }
        }

        if self.needs_load_time_status {
            self.load_time_status(k);
            if matches!(
                self.state,
                HomeState::Menu
                    | HomeState::ShowApps
                    | HomeState::ShowDailyMantra
                    | HomeState::ShowCalendar
                    | HomeState::ShowPanchangLite
                    | HomeState::ShowNetworkStatus
                    | HomeState::ShowDateTime
            ) {
                ctx.request_full_redraw();
            }
        }

        if self.needs_load_panchang_mantra {
            self.load_panchang_mantra(k);
            if self.state == HomeState::ShowPanchangLite {
                ctx.request_full_redraw();
            }
        }

        if self.needs_load_lua_daily_mantra {
            self.load_lua_daily_mantra(k);
            if self.state == HomeState::ShowLuaDailyMantra {
                ctx.request_full_redraw();
            }
        }

        if self.needs_load_lua_calendar {
            self.load_lua_calendar(k);
            if self.state == HomeState::ShowLuaCalendar {
                ctx.request_full_redraw();
            }
        }

        if self.needs_load_lua_panchang {
            self.load_lua_panchang(k);
            if self.state == HomeState::ShowLuaPanchang {
                ctx.request_full_redraw();
            }
        }

        if self.needs_load_bookmarks {
            let mut buf = [0u8; 4096];
            self.bm_entries.clear();

            if let Ok(n) = k.read_app_subdir_chunk(
                reader_state::STATE_DIR,
                reader_state::BOOKMARKS_INDEX_FILE,
                0,
                &mut buf,
            ) {
                if n > 0 {
                    if let Ok(payload) = core::str::from_utf8(&buf[..n]) {
                        self.bm_entries = reader_state::decode_bookmarks_index(payload);
                    }
                }
            }

            self.bm_count = self.bm_entries.len();
            log::info!(
                "bookmark-index: home loaded {} item(s) from global bookmark index",
                self.bm_count
            );
            if self.bm_count == 0 {
                self.bm_selected = 0;
                self.bm_scroll = 0;
            } else {
                if self.bm_selected >= self.bm_count {
                    self.bm_selected = self.bm_count - 1;
                }
                let vis = self.bm_visible_lines();
                if self.bm_scroll + vis <= self.bm_selected {
                    self.bm_scroll = self.bm_selected.saturating_sub(vis.saturating_sub(1));
                }
            }

            self.needs_load_bookmarks = false;
            if self.state == HomeState::ShowBookmarks {
                ctx.mark_dirty(self.bm_list_region());
                ctx.mark_dirty(self.bm_status_region());
            } else if self.state == HomeState::Menu {
                ctx.mark_dirty(self.item_regions[3]);
            }
        }
    }

    fn on_event(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match self.state {
            HomeState::Menu => self.on_event_menu(event, ctx),
            HomeState::ShowApps => self.on_event_apps(event, ctx),
            HomeState::ShowCategoryItems => self.on_event_category_items(event, ctx),
            HomeState::ShowDailyMantra => self.on_event_daily_mantra(event, ctx),
            HomeState::ShowLuaDailyMantra => self.on_event_lua_daily_mantra(event, ctx),
            HomeState::ShowLuaPanchang => self.on_event_lua_panchang(event, ctx),
            HomeState::ShowLuaCalendar => self.on_event_lua_calendar(event, ctx),
            HomeState::ShowCalendar => self.on_event_calendar(event, ctx),
            HomeState::ShowPanchangLite => self.on_event_panchang_lite(event, ctx),
            HomeState::ShowBookmarks => self.on_event_bookmarks(event, ctx),
            HomeState::ShowNetworkStatus => self.on_event_network_status(event, ctx),
            HomeState::ShowWifiConnect => self.on_event_wifi_connect(event, ctx),
            HomeState::ShowDateTime => self.on_event_date_time(event, ctx),
            HomeState::ShowPlaceholder => self.on_event_placeholder(event, ctx),
        }
    }

    fn draw(&self, strip: &mut StripBuffer) {
        match self.state {
            HomeState::Menu => self.draw_menu(strip),
            HomeState::ShowApps => self.draw_apps(strip),
            HomeState::ShowCategoryItems => self.draw_category_items(strip),
            HomeState::ShowDailyMantra => self.draw_daily_mantra(strip),
            HomeState::ShowLuaDailyMantra => self.draw_lua_daily_mantra(strip),
            HomeState::ShowLuaPanchang => self.draw_lua_panchang(strip),
            HomeState::ShowLuaCalendar => self.draw_lua_calendar(strip),
            HomeState::ShowCalendar => self.draw_calendar(strip),
            HomeState::ShowPanchangLite => self.draw_panchang_lite(strip),
            HomeState::ShowBookmarks => self.draw_bookmarks(strip),
            HomeState::ShowNetworkStatus => self.draw_network_status(strip),
            HomeState::ShowWifiConnect => self.draw_wifi_connect(strip),
            HomeState::ShowDateTime => self.draw_date_time(strip),
            HomeState::ShowPlaceholder => self.draw_placeholder(strip),
        }
    }
}

impl HomeApp {
    fn on_event_menu(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Next) | ActionEvent::Repeat(Action::Next) => {
                self.move_selection_row(1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Prev) | ActionEvent::Repeat(Action::Prev) => {
                self.move_selection_row(-1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::NextJump) | ActionEvent::Repeat(Action::NextJump) => {
                self.move_selection_col(1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::PrevJump) | ActionEvent::Repeat(Action::PrevJump) => {
                self.move_selection_col(-1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Select) => match self.item_action(self.selected) {
                MenuAction::OpenCategory(category) => {
                    self.open_category(category, ctx);
                    Transition::None
                }
            },
            _ => Transition::None,
        }
    }

    fn open_reader_entry(&mut self, ctx: &mut AppContext) -> Transition {
        if let Some(rec) = self.recent_record() {
            log::info!(
                "reader-slice: continue from typed recent book_id={} path={}",
                rec.book_id.as_str(),
                rec.open_path()
            );
            ctx.set_message(rec.open_path().as_bytes());
            Transition::Push(AppId::Reader)
        } else {
            log::info!("home-dashboard: reader app selected without recent; opening library");
            ctx.clear_message();
            Transition::Push(AppId::Files)
        }
    }

    fn open_category(&mut self, category: usize, ctx: &mut AppContext) {
        self.active_category = category.min(Self::category_count().saturating_sub(1));
        self.selected = 0;
        self.state = HomeState::ShowCategoryItems;
        ctx.request_full_redraw();
    }

    fn open_daily_mantra(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.time_status_loaded = false;
        self.needs_load_time_status = true;
        self.state = HomeState::ShowDailyMantra;
        ctx.request_full_redraw();
    }

    #[allow(dead_code)]
    fn open_lua_panchang(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.needs_load_lua_panchang = true;
        self.state = HomeState::ShowLuaPanchang;
        ctx.request_full_redraw();
    }

    fn open_lua_daily_mantra(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.needs_load_lua_daily_mantra = true;
        self.state = HomeState::ShowLuaDailyMantra;
        ctx.request_full_redraw();
    }

    fn open_lua_calendar(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.needs_load_lua_calendar = true;
        self.state = HomeState::ShowLuaCalendar;
        ctx.request_full_redraw();
    }

    fn open_panchang_lite(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.time_status_loaded = false;
        self.panchang_mantra_loaded = false;
        self.needs_load_time_status = true;
        self.needs_load_panchang_mantra = true;
        self.state = HomeState::ShowPanchangLite;
        ctx.request_full_redraw();
    }

    fn open_calendar(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.calendar_month_offset = 0;
        self.time_status_loaded = false;
        self.needs_load_time_status = true;
        self.state = HomeState::ShowCalendar;
        ctx.request_full_redraw();
    }

    fn open_bookmarks(&mut self, return_target: ReturnTarget, ctx: &mut AppContext) {
        self.return_target = return_target;
        self.bm_selected = 0;
        self.bm_scroll = 0;
        self.needs_load_bookmarks = true;
        self.state = HomeState::ShowBookmarks;
        ctx.request_full_redraw();
    }

    fn open_wifi_connect(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.network_status_loaded = false;
        self.needs_load_network_status = true;
        self.wifi_editor_field = 0;
        self.wifi_editor_char_idx = 0;
        self.wifi_editor_mode = WIFI_EDITOR_MODE_ROWS;
        self.wifi_editor_save_pending = false;
        self.wifi_editor_dirty = false;
        self.wifi_editor_message = WIFI_EDITOR_SAVE_MSG;
        self.state = HomeState::ShowWifiConnect;
        ctx.request_full_redraw();
    }

    fn open_network_status(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.network_status_loaded = false;
        self.needs_load_network_status = true;
        self.time_status_loaded = false;
        self.needs_load_time_status = true;
        self.state = HomeState::ShowNetworkStatus;
        ctx.request_full_redraw();
    }

    fn open_date_time(&mut self, ctx: &mut AppContext) {
        self.return_target = ReturnTarget::CategoryItems;
        self.time_status_loaded = false;
        self.needs_load_time_status = true;
        self.state = HomeState::ShowDateTime;
        ctx.request_full_redraw();
    }

    fn open_placeholder(
        &mut self,
        title: &'static str,
        detail: &'static str,
        return_target: ReturnTarget,
        ctx: &mut AppContext,
    ) {
        self.placeholder_title = title;
        self.placeholder_detail = detail;
        self.return_target = return_target;
        self.state = HomeState::ShowPlaceholder;
        ctx.request_full_redraw();
    }

    fn return_to_target(&mut self, ctx: &mut AppContext) -> Transition {
        match self.return_target {
            ReturnTarget::Menu => {
                self.state = HomeState::Menu;
                self.selected = 0;
            }
            ReturnTarget::CategoryItems => {
                self.state = HomeState::ShowCategoryItems;
                self.selected = 0;
            }
        }
        ctx.request_full_redraw();
        Transition::None
    }

    fn on_event_apps(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Next) | ActionEvent::Repeat(Action::Next) => {
                self.move_selection_row(1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Prev) | ActionEvent::Repeat(Action::Prev) => {
                self.move_selection_row(-1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::NextJump) | ActionEvent::Repeat(Action::NextJump) => {
                self.move_selection_col(1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::PrevJump) | ActionEvent::Repeat(Action::PrevJump) => {
                self.move_selection_col(-1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                self.selected = 0;
                self.state = HomeState::Menu;
                ctx.request_full_redraw();
                Transition::None
            }
            ActionEvent::Press(Action::Select) => {
                self.open_category(self.selected, ctx);
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn on_event_category_items(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Next) | ActionEvent::Repeat(Action::Next) => {
                let count = Self::category_item_count(self.active_category);
                if count > 0 {
                    let previous = self.selected.min(count.saturating_sub(1));
                    let next = (previous + 1).min(count.saturating_sub(1));
                    self.selected = next;
                    self.request_apps_list_selection_redraw(ctx, previous, next);
                }
                Transition::None
            }
            ActionEvent::Press(Action::Prev) | ActionEvent::Repeat(Action::Prev) => {
                let count = Self::category_item_count(self.active_category);
                if count > 0 {
                    let previous = self.selected.min(count.saturating_sub(1));
                    let next = previous.saturating_sub(1);
                    self.selected = next;
                    self.request_apps_list_selection_redraw(ctx, previous, next);
                }
                Transition::None
            }
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                self.selected = self.active_category;
                self.state = HomeState::Menu;
                ctx.request_full_redraw();
                Transition::None
            }
            ActionEvent::Press(Action::Select) | ActionEvent::Press(Action::NextJump) => {
                self.open_category_item(ctx)
            }
            _ => Transition::None,
        }
    }

    fn open_category_item(&mut self, ctx: &mut AppContext) -> Transition {
        match (self.active_category, self.selected) {
            // Network
            (0, 0) => Transition::Push(AppId::BiscuitWifi),
            (0, 1) => Transition::Push(AppId::Upload),
            (0, _) => {
                self.open_network_status(ctx);
                Transition::None
            }

            // Productivity
            (1, 0) => {
                self.open_daily_mantra(ctx);
                Transition::None
            }
            (1, 1) => {
                self.open_calendar(ctx);
                Transition::None
            }
            (1, 2) => {
                self.open_panchang_lite(ctx);
                Transition::None
            }
            (1, _) => {
                self.open_lua_calendar(ctx);
                Transition::None
            }

            // Games
            (2, _) => {
                self.open_placeholder(
                    "Coming soon",
                    "Games placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }

            // Reader
            (3, 0) => self.open_reader_entry(ctx),
            (3, 1) => Transition::Push(AppId::Files),
            (3, _) => {
                self.open_bookmarks(ReturnTarget::CategoryItems, ctx);
                Transition::None
            }

            // System
            (4, 0) => Transition::Push(AppId::Settings),
            (4, 1) => {
                self.open_date_time(ctx);
                Transition::None
            }
            (4, 2) => {
                self.open_placeholder(
                    "Sleep Image",
                    "Placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }
            (4, _) => {
                self.open_placeholder(
                    "Device Info",
                    "Placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }

            // Tools
            (5, 0) => Transition::Push(AppId::Files),
            (5, 1) => {
                self.open_lua_daily_mantra(ctx);
                Transition::None
            }
            (5, 2) => {
                self.open_lua_panchang(ctx);
                Transition::None
            }
            (5, _) => {
                self.open_placeholder(
                    "QR Generator",
                    "Placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn on_event_daily_mantra(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back)
            | ActionEvent::LongPress(Action::Back)
            | ActionEvent::Press(Action::Select) => self.return_to_target(ctx),
            _ => Transition::None,
        }
    }

    fn on_event_lua_daily_mantra(
        &mut self,
        event: ActionEvent,
        ctx: &mut AppContext,
    ) -> Transition {
        match event {
            ActionEvent::Press(Action::Back)
            | ActionEvent::LongPress(Action::Back)
            | ActionEvent::Press(Action::Select) => self.return_to_target(ctx),
            _ => Transition::None,
        }
    }

    fn on_event_lua_panchang(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back)
            | ActionEvent::LongPress(Action::Back)
            | ActionEvent::Press(Action::Select) => self.return_to_target(ctx),
            _ => Transition::None,
        }
    }

    fn on_event_panchang_lite(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back)
            | ActionEvent::LongPress(Action::Back)
            | ActionEvent::Press(Action::Select) => self.return_to_target(ctx),
            _ => Transition::None,
        }
    }

    fn on_event_lua_calendar(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back)
            | ActionEvent::LongPress(Action::Back)
            | ActionEvent::Press(Action::Select) => self.return_to_target(ctx),
            _ => Transition::None,
        }
    }

    fn on_event_calendar(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                self.return_to_target(ctx)
            }
            ActionEvent::Press(Action::Prev)
            | ActionEvent::Repeat(Action::Prev)
            | ActionEvent::Press(Action::PrevJump)
            | ActionEvent::Repeat(Action::PrevJump) => {
                self.move_calendar_month(-1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Next)
            | ActionEvent::Repeat(Action::Next)
            | ActionEvent::Press(Action::NextJump)
            | ActionEvent::Repeat(Action::NextJump) => {
                self.move_calendar_month(1, ctx);
                Transition::None
            }
            ActionEvent::Press(Action::Select) => {
                if self.calendar_month_offset != 0 {
                    self.calendar_month_offset = 0;
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn move_calendar_month(&mut self, delta: i16, ctx: &mut AppContext) {
        self.calendar_month_offset = self
            .calendar_month_offset
            .saturating_add(delta)
            .clamp(-120, 120);
        ctx.request_full_redraw();
    }

    fn on_event_placeholder(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back)
            | ActionEvent::LongPress(Action::Back)
            | ActionEvent::Press(Action::Select) => self.return_to_target(ctx),
            _ => Transition::None,
        }
    }

    fn on_event_network_status(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                self.return_to_target(ctx)
            }
            ActionEvent::Press(Action::Select) => {
                self.network_status_loaded = false;
                self.needs_load_network_status = true;
                ctx.request_full_redraw();
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn suppress_wifi_keyboard_background_and_redraw(&mut self, ctx: &mut AppContext) {
        // CrossPoint-style keyboard session: while the keyboard is active,
        // the edit buffer is owned by the keyboard/session, not by SETTINGS.TXT
        // reloads or Home background refresh.  Navigation changes only RAM
        // cursor state, so drop any stale redraw queued before this navigation
        // event can flush to e-paper.
        if self.state == HomeState::ShowWifiConnect && self.is_wifi_text_editing() {
            self.needs_load_network_status = false;
            self.network_status_loaded = true;
            self.sync_wifi_editor_to_slot();
            let _ = ctx.take_redraw();
        }
    }

    fn activate_wifi_editor_row(&mut self, ctx: &mut AppContext) -> Transition {
        if self.wifi_editor_field == 1 {
            self.resume_wifi_after_scan = true;
            return Transition::Push(AppId::BiscuitWifi);
        }

        self.append_wifi_editor_char();
        ctx.request_full_redraw();
        Transition::None
    }

    fn activate_wifi_keyboard_key(&mut self, ctx: &mut AppContext) -> Transition {
        self.append_wifi_editor_char();
        if self.is_wifi_text_editing() {
            ctx.mark_dirty(CONTENT_REGION);
        } else {
            ctx.request_full_redraw();
        }
        Transition::None
    }

    fn on_event_wifi_connect(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                if self.is_wifi_text_editing() {
                    self.exit_wifi_text_editor();
                    ctx.request_full_redraw();
                    Transition::None
                } else {
                    self.return_to_target(ctx)
                }
            }
            ActionEvent::Press(Action::Next) | ActionEvent::Repeat(Action::Next) => {
                if self.is_wifi_text_editing() {
                    self.move_wifi_keyboard_vertical(1);
                    self.suppress_wifi_keyboard_background_and_redraw(ctx);
                } else {
                    self.move_wifi_editor_field(1);
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            ActionEvent::Press(Action::Prev) | ActionEvent::Repeat(Action::Prev) => {
                if self.is_wifi_text_editing() {
                    self.move_wifi_keyboard_vertical(-1);
                    self.suppress_wifi_keyboard_background_and_redraw(ctx);
                } else {
                    self.move_wifi_editor_field(-1);
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            ActionEvent::Press(Action::Select) => {
                if self.is_wifi_text_editing() {
                    self.activate_wifi_keyboard_key(ctx)
                } else {
                    self.activate_wifi_editor_row(ctx)
                }
            }
            ActionEvent::Press(Action::NextJump) => {
                if self.is_wifi_text_editing() {
                    // In normal button layout NextJump is the physical Right key, so keep
                    // it as keyboard navigation. In swapped layout the physical Confirm/OK
                    // button may also arrive as NextJump; callers can still activate with
                    // the semantic Select path, and row-mode activation below handles OK
                    // aliases before the keyboard opens.
                    self.move_wifi_keyboard_horizontal(1);
                    self.suppress_wifi_keyboard_background_and_redraw(ctx);
                    Transition::None
                } else if self.wifi_editor_field == 2 {
                    self.cycle_wifi_scan_result(1);
                    ctx.request_full_redraw();
                    Transition::None
                } else {
                    // OK/Confirm can map to NextJump when swap_buttons is enabled. Treat a
                    // press on rows as Activate so Wi-Fi Networks, Password, Default, and
                    // Save still work. Repeat remains navigation/no-op to avoid repeated Save.
                    self.activate_wifi_editor_row(ctx)
                }
            }
            ActionEvent::Repeat(Action::NextJump) => {
                if self.is_wifi_text_editing() {
                    self.move_wifi_keyboard_horizontal(1);
                    self.suppress_wifi_keyboard_background_and_redraw(ctx);
                } else if self.wifi_editor_field == 2 {
                    self.cycle_wifi_scan_result(1);
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            ActionEvent::Press(Action::PrevJump) | ActionEvent::Repeat(Action::PrevJump) => {
                if self.is_wifi_text_editing() {
                    self.move_wifi_keyboard_horizontal(-1);
                    self.suppress_wifi_keyboard_background_and_redraw(ctx);
                } else if self.wifi_editor_field == 2 {
                    self.cycle_wifi_scan_result(-1);
                    ctx.request_full_redraw();
                } else {
                    self.move_wifi_editor_field(-1);
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            ActionEvent::LongPress(Action::Select) => {
                self.delete_wifi_editor_char();
                self.suppress_wifi_keyboard_background_and_redraw(ctx);
                if self.is_wifi_text_editing() {
                    ctx.mark_dirty(CONTENT_REGION);
                } else {
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            ActionEvent::LongPress(Action::NextJump) | ActionEvent::LongPress(Action::PrevJump) => {
                self.clear_wifi_editor_field();
                self.suppress_wifi_keyboard_background_and_redraw(ctx);
                if self.is_wifi_text_editing() {
                    ctx.mark_dirty(CONTENT_REGION);
                } else {
                    ctx.request_full_redraw();
                }
                Transition::None
            }
            _ => Transition::None,
        }
    }

    fn on_event_date_time(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                self.return_to_target(ctx)
            }
            ActionEvent::Press(Action::Select) => {
                self.resume_date_time_after_sync = true;
                Transition::Push(AppId::TimeSync)
            }
            _ => Transition::None,
        }
    }

    fn on_event_bookmarks(&mut self, event: ActionEvent, ctx: &mut AppContext) -> Transition {
        match event {
            ActionEvent::Press(Action::Back) | ActionEvent::LongPress(Action::Back) => {
                self.return_to_target(ctx)
            }

            ActionEvent::Press(Action::Next) | ActionEvent::Repeat(Action::Next) => {
                if self.bm_count > 0 {
                    let old = self.bm_selected;
                    let vis = self.bm_visible_lines();
                    if self.bm_selected + 1 < self.bm_count {
                        self.bm_selected += 1;
                        if self.bm_selected >= self.bm_scroll + vis {
                            self.bm_scroll = self.bm_selected + 1 - vis;
                            ctx.mark_dirty(self.bm_list_region());
                        } else {
                            ctx.mark_dirty(self.bm_row_region(old - self.bm_scroll));
                            ctx.mark_dirty(self.bm_row_region(self.bm_selected - self.bm_scroll));
                        }
                    } else {
                        self.bm_selected = 0;
                        self.bm_scroll = 0;
                        ctx.mark_dirty(self.bm_list_region());
                    }
                    ctx.mark_dirty(self.bm_status_region());
                }
                Transition::None
            }

            ActionEvent::Press(Action::Prev) | ActionEvent::Repeat(Action::Prev) => {
                if self.bm_count > 0 {
                    let old = self.bm_selected;
                    let vis = self.bm_visible_lines();
                    if self.bm_selected > 0 {
                        self.bm_selected -= 1;
                        if self.bm_selected < self.bm_scroll {
                            self.bm_scroll = self.bm_selected;
                            ctx.mark_dirty(self.bm_list_region());
                        } else {
                            ctx.mark_dirty(self.bm_row_region(old - self.bm_scroll));
                            ctx.mark_dirty(self.bm_row_region(self.bm_selected - self.bm_scroll));
                        }
                    } else {
                        self.bm_selected = self.bm_count - 1;
                        if self.bm_selected >= vis {
                            self.bm_scroll = self.bm_selected + 1 - vis;
                        }
                        ctx.mark_dirty(self.bm_list_region());
                    }
                    ctx.mark_dirty(self.bm_status_region());
                }
                Transition::None
            }

            ActionEvent::Press(Action::NextJump) => {
                if self.bm_count > 0 {
                    let vis = self.bm_visible_lines();
                    self.bm_selected = (self.bm_selected + vis).min(self.bm_count - 1);
                    if self.bm_selected >= self.bm_scroll + vis {
                        self.bm_scroll = self.bm_selected + 1 - vis;
                    }
                    ctx.mark_dirty(self.bm_list_region());
                    ctx.mark_dirty(self.bm_status_region());
                }
                Transition::None
            }

            ActionEvent::Press(Action::PrevJump) => {
                let vis = self.bm_visible_lines();
                self.bm_selected = self.bm_selected.saturating_sub(vis);
                if self.bm_selected < self.bm_scroll {
                    self.bm_scroll = self.bm_selected;
                }
                ctx.mark_dirty(self.bm_list_region());
                ctx.mark_dirty(self.bm_status_region());
                Transition::None
            }

            ActionEvent::Press(Action::Select) => {
                if self.bm_count > 0 && self.bm_selected < self.bm_count {
                    let slot = &self.bm_entries[self.bm_selected];
                    log::info!(
                        "bookmark-index: home bookmark selected idx={} ch={} off={} label={}",
                        self.bm_selected,
                        slot.chapter,
                        slot.byte_offset,
                        slot.display_label()
                    );
                    let jump = slot.jump_message();
                    ctx.set_message(jump.as_bytes());
                    self.state = HomeState::Menu;
                    Transition::Push(AppId::Reader)
                } else {
                    Transition::None
                }
            }

            _ => Transition::None,
        }
    }
}

impl HomeApp {
    fn draw_home_header_status(&self, strip: &mut StripBuffer, y: u16) {
        let meta_font = self.card_meta_font();
        let battery_w: u16 = 64;
        let time_w: u16 = 190;
        let battery_x = SCREEN_W
            .saturating_sub(LARGE_MARGIN)
            .saturating_sub(battery_w);
        let time_x = battery_x.saturating_sub(time_w + 8);

        let mut time = BitmapDynLabel::<80>::new(
            Region::new(time_x, y, time_w, meta_font.line_height),
            meta_font,
        )
        .alignment(Alignment::CenterRight);
        let _ = self
            .time_cache
            .write_home_time(self.time_uptime_secs, &mut time);
        time.draw(strip).unwrap();

        let pct = time_status::battery_percent_value(self.home_battery_mv);
        let icon_w: u16 = 22;
        let icon_h: u16 = 10;
        let icon_x = SCREEN_W
            .saturating_sub(LARGE_MARGIN)
            .saturating_sub(icon_w + 4);
        let icon_y = y + meta_font.line_height.saturating_sub(icon_h) / 2;
        let pct_w: u16 = battery_w.saturating_sub(icon_w + 8);
        let mut pct_label = BitmapDynLabel::<12>::new(
            Region::new(battery_x, y, pct_w, meta_font.line_height),
            meta_font,
        )
        .alignment(Alignment::CenterRight);
        if let Some(pct) = pct {
            let _ = write!(pct_label, "{}%", pct);
        } else {
            let _ = write!(pct_label, "--");
        }
        pct_label.draw(strip).unwrap();

        Region::new(icon_x, icon_y, icon_w, icon_h)
            .to_rect()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(strip)
            .unwrap();
        Region::new(icon_x + icon_w, icon_y + 3, 3, icon_h.saturating_sub(6))
            .to_rect()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(strip)
            .unwrap();

        if let Some(pct) = pct {
            let inner_w = icon_w.saturating_sub(4);
            let fill_w = ((u32::from(inner_w) * u32::from(pct)) / 100) as u16;
            if fill_w > 0 {
                Region::new(icon_x + 2, icon_y + 2, fill_w, icon_h.saturating_sub(4))
                    .to_rect()
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(strip)
                    .unwrap();
            }
        }
    }

    fn draw_menu(&self, strip: &mut StripBuffer) {
        let header_font = self.header_font();
        let title_region = Region::new(
            LARGE_MARGIN,
            HOME_HEADER_Y,
            FULL_CONTENT_W.saturating_sub(152),
            header_font.line_height,
        );
        BitmapLabel::new(title_region, "Vaachak", header_font)
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();

        self.draw_home_header_status(strip, HOME_HEADER_Y);

        let rule = Region::new(
            LARGE_MARGIN,
            HOME_HEADER_Y + header_font.line_height + HOME_HEADER_RULE_GAP,
            FULL_CONTENT_W,
            2,
        );
        rule.to_rect()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(strip)
            .unwrap();

        for i in 0..HOME_CARD_COUNT {
            self.draw_home_card(strip, i);
        }
    }

    fn draw_home_card(&self, strip: &mut StripBuffer, idx: usize) {
        let region = self.item_regions[idx];
        let selected = idx == self.selected;

        let fill = if selected {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };
        region
            .to_rect()
            .into_styled(PrimitiveStyle::with_fill(fill))
            .draw(strip)
            .unwrap();

        let stroke = if selected {
            BinaryColor::Off
        } else {
            BinaryColor::On
        };
        region
            .to_rect()
            .into_styled(PrimitiveStyle::with_stroke(stroke, 2))
            .draw(strip)
            .unwrap();

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let text_x = region.x + HOME_CARD_PAD_X;
        let text_w = region.w.saturating_sub(HOME_CARD_PAD_X * 2);
        let title_y = region.y + HOME_CARD_PAD_Y;
        let subtitle_y = title_y + title_font.line_height + HOME_CARD_TEXT_GAP;
        let detail_y = subtitle_y + meta_font.line_height + HOME_CARD_TEXT_GAP;

        BitmapLabel::new(
            Region::new(text_x, title_y, text_w, title_font.line_height),
            self.card_title(idx),
            title_font,
        )
        .alignment(Alignment::CenterLeft)
        .inverted(selected)
        .draw(strip)
        .unwrap();

        BitmapLabel::new(
            Region::new(text_x, subtitle_y, text_w, meta_font.line_height),
            self.card_subtitle(idx),
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .inverted(selected)
        .draw(strip)
        .unwrap();

        let detail = self.card_detail(idx);
        if !detail.is_empty() {
            let detail_region = Region::new(text_x, detail_y, text_w, meta_font.line_height);
            BitmapLabel::new(detail_region, detail.as_str(), meta_font)
                .alignment(Alignment::CenterLeft)
                .inverted(selected)
                .draw(strip)
                .unwrap();
        }
    }

    fn card_title(&self, idx: usize) -> &'static str {
        Self::category_title(idx)
    }

    fn card_subtitle(&self, idx: usize) -> &'static str {
        Self::category_detail(idx)
    }

    fn card_detail(&self, _idx: usize) -> String {
        String::new()
    }

    const fn category_count() -> usize {
        6
    }

    const fn category_title(idx: usize) -> &'static str {
        match idx {
            0 => "Network",
            1 => "Productivity",
            2 => "Games",
            3 => "Reader",
            4 => "System",
            5 => "Tools",
            _ => "",
        }
    }

    const fn category_detail(idx: usize) -> &'static str {
        match idx {
            0 => "Connect & diagnose",
            1 => "Daily utilities",
            2 => "Coming soon",
            3 => "Books & bookmarks",
            4 => "Device & settings",
            5 => "Files & helpers",
            _ => "",
        }
    }

    const fn category_item_count(category: usize) -> usize {
        match category {
            0 => 3,
            1 => 4,
            2 => 1,
            3 => 3,
            4 => 4,
            5 => TOOLS_CATEGORY_ITEM_COUNT,
            _ => 0,
        }
    }

    const fn category_item_title(category: usize, idx: usize) -> &'static str {
        match (category, idx) {
            (0, 0) => "Wi-Fi Networks",
            (0, 1) => "Wi-Fi Transfer",
            (0, 2) => "Network Status",
            (1, 0) => "Daily Mantra",
            (1, 1) => "Calendar",
            (1, 2) => "Panchang Lite",
            (1, 3) => "Lua Calendar",
            (2, 0) => "Coming soon",
            (3, 0) => "Continue Reading",
            (3, 1) => "Library",
            (3, 2) => "Bookmarks",
            (4, 0) => "Settings",
            (4, 1) => "Date & Time",
            (4, 2) => "Sleep Image",
            (4, 3) => "Device Info",
            (5, 0) => "File Browser",
            (5, 1) => "Lua Daily Mantra",
            (5, 2) => "Lua Panchang",
            (5, 3) => "QR Generator",
            _ => "",
        }
    }

    const fn category_item_detail(category: usize, idx: usize) -> &'static str {
        match (category, idx) {
            (0, 0) => "Scan SSID + password",
            (0, 1) => "Transfer + time sync",
            (0, 2) => "Wi-Fi, SD and runtime",
            (1, 0) => "Uses Date & Time",
            (1, 1) => "Offline monthly view",
            (1, 2) => "Tithi, Paksha, Month",
            (1, 3) => "Run SD Lua calendar",
            (2, 0) => "Placeholder",
            (3, 0) => "Resume last book",
            (3, 1) => "Open file library",
            (3, 2) => "Open bookmark list",
            (4, 0) => "Existing settings",
            (4, 1) => "Clock and sync",
            (4, 2) => "Placeholder",
            (4, 3) => "Placeholder",
            (5, 0) => "Open file library",
            (5, 1) => "Run SD Lua proof app",
            (5, 2) => "Placeholder",
            _ => "",
        }
    }

    fn draw_screen_header(&self, strip: &mut StripBuffer, title: &str, status: &str) {
        let header_region = Region::new(
            LARGE_MARGIN,
            BM_TITLE_Y,
            HEADER_W,
            self.ui_fonts.heading.line_height,
        );
        BitmapLabel::new(header_region, title, self.ui_fonts.heading)
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();

        let status_region = Region::new(
            SCREEN_W.saturating_sub(LARGE_MARGIN).saturating_sub(104),
            BM_TITLE_Y,
            104,
            self.card_meta_font().line_height,
        );
        BitmapLabel::new(status_region, status, self.card_meta_font())
            .alignment(Alignment::CenterRight)
            .draw(strip)
            .unwrap();
    }

    fn app_row_region(&self, idx: usize) -> Region {
        let row_y =
            BM_TITLE_Y + self.ui_fonts.heading.line_height + 24 + (idx as u16).saturating_mul(72);
        Region::new(LARGE_MARGIN, row_y, FULL_CONTENT_W, 58)
    }

    fn merged_redraw_region(first: Region, second: Region) -> Region {
        let x0 = first.x.min(second.x);
        let y0 = first.y.min(second.y);
        let x1 = first
            .x
            .saturating_add(first.w)
            .max(second.x.saturating_add(second.w));
        let y1 = first
            .y
            .saturating_add(first.h)
            .max(second.y.saturating_add(second.h));
        Region::new(x0, y0, x1.saturating_sub(x0), y1.saturating_sub(y0))
    }

    fn request_apps_list_selection_redraw(
        &self,
        ctx: &mut AppContext,
        previous: usize,
        next: usize,
    ) {
        if previous == next {
            return;
        }

        let previous_region = self.app_row_region(previous);
        let next_region = self.app_row_region(next);
        ctx.request_partial_redraw(Self::merged_redraw_region(previous_region, next_region));
    }

    fn draw_apps(&self, strip: &mut StripBuffer) {
        self.draw_screen_header(strip, "Vaachak", "");
        self.draw_home_header_status(strip, BM_TITLE_Y);

        for idx in 0..Self::category_count() {
            self.draw_category_card(strip, idx);
        }
    }

    fn draw_category_card(&self, strip: &mut StripBuffer, idx: usize) {
        let region = self.item_regions[idx];
        let selected = idx == self.selected.min(Self::category_count().saturating_sub(1));

        let fill = if selected {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };
        region
            .to_rect()
            .into_styled(PrimitiveStyle::with_fill(fill))
            .draw(strip)
            .unwrap();

        let stroke = if selected {
            BinaryColor::Off
        } else {
            BinaryColor::On
        };
        region
            .to_rect()
            .into_styled(PrimitiveStyle::with_stroke(stroke, 2))
            .draw(strip)
            .unwrap();

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let text_x = region.x + HOME_CARD_PAD_X;
        let text_w = region.w.saturating_sub(HOME_CARD_PAD_X * 2);
        let title_y = region.y + HOME_CARD_PAD_Y;
        let subtitle_y = title_y + title_font.line_height + HOME_CARD_TEXT_GAP;

        BitmapLabel::new(
            Region::new(text_x, title_y, text_w, title_font.line_height),
            Self::category_title(idx),
            title_font,
        )
        .alignment(Alignment::CenterLeft)
        .inverted(selected)
        .draw(strip)
        .unwrap();

        BitmapLabel::new(
            Region::new(text_x, subtitle_y, text_w, meta_font.line_height),
            Self::category_detail(idx),
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .inverted(selected)
        .draw(strip)
        .unwrap();
    }

    fn draw_category_items(&self, strip: &mut StripBuffer) {
        let count = Self::category_item_count(self.active_category);
        let mut status = BitmapDynLabel::<20>::new(
            Region::new(
                SCREEN_W.saturating_sub(LARGE_MARGIN).saturating_sub(104),
                BM_TITLE_Y,
                104,
                self.card_meta_font().line_height,
            ),
            self.card_meta_font(),
        )
        .alignment(Alignment::CenterRight);
        let _ = time_status::battery_label(self.home_battery_mv, &mut status);

        let header_region = Region::new(
            LARGE_MARGIN,
            BM_TITLE_Y,
            HEADER_W,
            self.ui_fonts.heading.line_height,
        );
        BitmapLabel::new(
            header_region,
            Self::category_title(self.active_category),
            self.ui_fonts.heading,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        status.draw(strip).unwrap();

        for idx in 0..count {
            self.draw_category_item_row(strip, idx);
        }
    }

    fn draw_category_item_row(&self, strip: &mut StripBuffer, idx: usize) {
        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let row = self.app_row_region(idx);
        let selected = idx
            == self
                .selected
                .min(Self::category_item_count(self.active_category).saturating_sub(1));

        let fill = if selected {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };
        row.to_rect()
            .into_styled(PrimitiveStyle::with_fill(fill))
            .draw(strip)
            .unwrap();

        let text_x = row.x + HOME_CARD_PAD_X;
        let text_w = row.w.saturating_sub(HOME_CARD_PAD_X * 2);
        BitmapLabel::new(
            Region::new(text_x, row.y + 8, text_w, title_font.line_height),
            Self::category_item_title(self.active_category, idx),
            title_font,
        )
        .alignment(Alignment::CenterLeft)
        .inverted(selected)
        .draw(strip)
        .unwrap();

        BitmapLabel::new(
            Region::new(
                text_x,
                row.y + 8 + title_font.line_height + HOME_CARD_TEXT_GAP,
                text_w,
                meta_font.line_height,
            ),
            Self::category_item_detail(self.active_category, idx),
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .inverted(selected)
        .draw(strip)
        .unwrap();
    }

    fn draw_panchang_lite(&self, strip: &mut StripBuffer) {
        let status = if !self.time_status_loaded {
            "Loading"
        } else {
            self.time_cache.freshness(self.time_uptime_secs).as_str()
        };
        self.draw_screen_header(strip, "Panchang Lite", status);

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 24;

        if !self.time_status_loaded {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Reading cached Date & Time...",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        let Some(panchang) = self.time_cache.display_panchang_lite(self.time_uptime_secs) else {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Sync Date & Time first",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += title_font.line_height + PANCHANG_LINE_GAP;
            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                "Panchang works offline after one successful sync",
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        };

        self.draw_panchang_line(strip, y, "Location", panchang.location, meta_font);
        y += meta_font.line_height + PANCHANG_LINE_GAP;
        self.draw_panchang_line(strip, y, "Timezone", panchang.timezone, meta_font);
        y += meta_font.line_height + PANCHANG_LINE_GAP;
        self.draw_panchang_line(strip, y, "Weekday", panchang.weekday, meta_font);
        y += meta_font.line_height + PANCHANG_LINE_GAP;
        self.draw_panchang_line(strip, y, "Tithi", panchang.tithi, meta_font);
        y += meta_font.line_height + PANCHANG_LINE_GAP;
        self.draw_panchang_line(strip, y, "Paksha", panchang.paksha, meta_font);
        y += meta_font.line_height + PANCHANG_LINE_GAP;
        self.draw_panchang_line(strip, y, "Month", panchang.month, meta_font);
        y += meta_font.line_height + PANCHANG_MANTRA_BLOCK_GAP;

        y = self.draw_panchang_mantra_block(
            strip,
            y,
            self.panchang_mantra_status(),
            title_font,
            meta_font,
        );
        y += PANCHANG_MANTRA_BLOCK_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            panchang.note,
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + PANCHANG_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "No network API used; festival notes later",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + PANCHANG_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Back returns to Productivity",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_panchang_mantra_block(
        &self,
        strip: &mut StripBuffer,
        mut y: u16,
        mantra: &str,
        mantra_font: &'static BitmapFont,
        label_font: &'static BitmapFont,
    ) -> u16 {
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;

        BitmapLabel::new(
            Region::new(x, y, w, label_font.line_height),
            "Day Mantra",
            label_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();

        y += label_font.line_height + 6;

        let mut current =
            BitmapDynLabel::<112>::new(Region::new(x, y, w, mantra_font.line_height), mantra_font)
                .alignment(Alignment::CenterLeft);

        let mut current_len = 0usize;
        let mut has_words = false;

        for word in mantra.split_whitespace() {
            let extra = if current_len == 0 { 0 } else { 1 };

            if current_len > 0 && current_len + word.len() + extra > 48 {
                current.draw(strip).unwrap();
                y += mantra_font.line_height + 4;

                current = BitmapDynLabel::<112>::new(
                    Region::new(x, y, w, mantra_font.line_height),
                    mantra_font,
                )
                .alignment(Alignment::CenterLeft);

                current_len = 0;
            }

            if current_len > 0 {
                let _ = write!(current, " ");
                current_len += 1;
            }

            let _ = write!(current, "{}", word);
            current_len += word.len();
            has_words = true;
        }

        if has_words && current_len > 0 {
            current.draw(strip).unwrap();
            y += mantra_font.line_height;
        }

        y
    }

    fn draw_panchang_line(
        &self,
        strip: &mut StripBuffer,
        y: u16,
        label: &str,
        value: &str,
        font: &'static BitmapFont,
    ) {
        let x = LARGE_MARGIN;
        let mut line =
            BitmapDynLabel::<96>::new(Region::new(x, y, PANCHANG_VALUE_W, font.line_height), font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(line, "{}: {}", label, value);
        line.draw(strip).unwrap();
    }

    fn draw_calendar(&self, strip: &mut StripBuffer) {
        let status = if !self.time_status_loaded {
            "Loading"
        } else {
            self.time_cache.freshness(self.time_uptime_secs).as_str()
        };
        self.draw_screen_header(strip, "Calendar", status);

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + CALENDAR_GRID_TOP_GAP;

        if !self.time_status_loaded {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Reading cached Date & Time...",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        let Some(today) = self.time_cache.display_date(self.time_uptime_secs) else {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Sync Date & Time first",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += title_font.line_height + NETWORK_STATUS_LINE_GAP;
            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                "Calendar works offline after one successful sync",
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        };

        let (year, month) = Self::calendar_month_from_offset(today, self.calendar_month_offset);
        let mut month_title =
            BitmapDynLabel::<40>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::Center);
        let _ = write!(
            month_title,
            "{} {}",
            time_status::calendar_month_name(month),
            year
        );
        month_title.draw(strip).unwrap();
        y += title_font.line_height + 10;

        let cell_w = (w.saturating_sub(CALENDAR_CELL_GAP * (CALENDAR_COLS as u16 - 1)))
            / CALENDAR_COLS as u16;
        let weekday_h = meta_font.line_height + 4;

        for (idx, label) in CALENDAR_WEEKDAY_LABELS.iter().enumerate() {
            let cell_x = x + (idx as u16).saturating_mul(cell_w + CALENDAR_CELL_GAP);
            BitmapLabel::new(
                Region::new(cell_x, y, cell_w, meta_font.line_height),
                label,
                meta_font,
            )
            .alignment(Alignment::Center)
            .draw(strip)
            .unwrap();
        }
        y += weekday_h;

        let first_weekday = time_status::calendar_weekday_for_date(year, month, 1);
        let days_in_month = time_status::calendar_days_in_month(year, month);
        let cell_h = 36;

        for row in 0..CALENDAR_ROWS {
            for col in 0..CALENDAR_COLS {
                let cell_x = x + (col as u16).saturating_mul(cell_w + CALENDAR_CELL_GAP);
                let cell_y = y + (row as u16).saturating_mul(cell_h + CALENDAR_CELL_GAP);
                let region = Region::new(cell_x, cell_y, cell_w, cell_h);
                let day = (row * CALENDAR_COLS + col) as i16 + 1 - i16::from(first_weekday);

                if day < 1 || day > i16::from(days_in_month) {
                    continue;
                }

                let day = day as u8;
                let is_today = year == today.year && month == today.month && day == today.day;
                self.draw_calendar_day(strip, region, day, is_today, meta_font);
            }
        }

        let footer_y = y + (CALENDAR_ROWS as u16).saturating_mul(cell_h + CALENDAR_CELL_GAP) + 8;
        let footer = if self.calendar_month_offset == 0 {
            "Prev/Next month   Back returns"
        } else {
            "Select today   Prev/Next month"
        };
        BitmapLabel::new(
            Region::new(x, footer_y, w, meta_font.line_height),
            footer,
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_calendar_day(
        &self,
        strip: &mut StripBuffer,
        region: Region,
        day: u8,
        is_today: bool,
        font: &'static BitmapFont,
    ) {
        let fill = if is_today {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };

        region
            .to_rect()
            .into_styled(PrimitiveStyle::with_fill(fill))
            .draw(strip)
            .unwrap();

        region
            .to_rect()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(strip)
            .unwrap();

        let mut label = BitmapDynLabel::<4>::new(
            Region::new(region.x, region.y + 9, region.w, font.line_height),
            font,
        )
        .alignment(Alignment::Center)
        .inverted(is_today);

        let _ = write!(label, "{}", day);
        label.draw(strip).unwrap();
    }

    fn calendar_month_from_offset(today: time_status::CalendarDate, offset: i16) -> (i32, u8) {
        let base = today
            .year
            .saturating_mul(12)
            .saturating_add(i32::from(today.month))
            .saturating_sub(1);
        let shifted = base.saturating_add(i32::from(offset));
        let year = shifted.div_euclid(12);
        let month = shifted.rem_euclid(12) + 1;

        (year, month as u8)
    }

    fn draw_daily_mantra(&self, strip: &mut StripBuffer) {
        let status = if !self.time_status_loaded {
            "Loading"
        } else {
            self.time_cache.freshness(self.time_uptime_secs).as_str()
        };
        self.draw_screen_header(strip, "Daily Mantra", status);

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 28;

        if !self.time_status_loaded {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Reading Date & Time...",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        if let Some(weekday) = self.time_cache.display_weekday_index(self.time_uptime_secs) {
            let idx = weekday as usize;
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                DAILY_MANTRA_TITLES[idx],
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                DAILY_MANTRA_DEDICATIONS[idx],
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                DAILY_MANTRA_ENGLISH[idx],
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

            let mut date_line =
                BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                    .alignment(Alignment::CenterLeft);
            let _ = write!(date_line, "Date: ");
            let _ = self
                .time_cache
                .write_date_value(self.time_uptime_secs, &mut date_line);
            date_line.draw(strip).unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

            let mut image_line =
                BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                    .alignment(Alignment::CenterLeft);
            let _ = write!(image_line, "Image: {}", DAILY_MANTRA_WEEKDAY_IMAGES[idx]);
            image_line.draw(strip).unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;
        } else {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Date unavailable",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                "Sync Date & Time to select today's mantra",
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

            let mut image_line =
                BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                    .alignment(Alignment::CenterLeft);
            let _ = write!(image_line, "Image: {}", DAILY_MANTRA_DEFAULT_IMAGE);
            image_line.draw(strip).unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;
        }

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Back returns to category",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_lua_daily_mantra(&self, strip: &mut StripBuffer) {
        self.draw_screen_header(strip, self.lua_daily_mantra_screen.title(), "Lua");

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 28;

        BitmapLabel::new(
            Region::new(x, y, w, title_font.line_height),
            self.lua_daily_mantra_screen.subtitle(),
            title_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        for line in [
            self.lua_daily_mantra_screen.line1(),
            self.lua_daily_mantra_screen.line2(),
            self.lua_daily_mantra_screen.line3(),
        ] {
            BitmapLabel::new(Region::new(x, y, w, meta_font.line_height), line, meta_font)
                .alignment(Alignment::CenterLeft)
                .draw(strip)
                .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;
        }

        let mut source_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            source_line,
            "Source: {}  Marker: {}",
            self.lua_daily_mantra_screen.source.label(),
            if self.lua_daily_mantra_screen.source.is_sd_loaded() {
                LUA_DAILY_MANTRA_SD_RUNTIME_MARKER
            } else {
                LUA_DAILY_MANTRA_APP_MARKER
            }
        );
        source_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            self.lua_daily_mantra_screen.footer(),
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Back returns to Tools",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_lua_panchang(&self, strip: &mut StripBuffer) {
        self.draw_screen_header(strip, self.lua_panchang_screen.title(), "Lua");

        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 28;

        for line in [
            self.lua_panchang_screen.subtitle(),
            self.lua_panchang_screen.line1(),
            self.lua_panchang_screen.line2(),
            self.lua_panchang_screen.line3(),
        ] {
            if !line.is_empty() {
                BitmapLabel::new(Region::new(x, y, w, meta_font.line_height), line, meta_font)
                    .alignment(Alignment::CenterLeft)
                    .draw(strip)
                    .unwrap();
                y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;
            }
        }

        let mut source_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            source_line,
            "Source: {}  Marker: {}",
            self.lua_panchang_screen.source.label(),
            if self.lua_panchang_screen.source.is_sd_loaded() {
                LUA_PANCHANG_SD_RUNTIME_MARKER
            } else {
                LUA_PANCHANG_APP_MARKER
            }
        );
        source_line.draw(strip).unwrap();
    }

    fn draw_lua_calendar(&self, strip: &mut StripBuffer) {
        self.draw_screen_header(strip, self.lua_calendar_screen.title(), "Lua");

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 28;

        BitmapLabel::new(
            Region::new(x, y, w, title_font.line_height),
            self.lua_calendar_screen.subtitle(),
            title_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        for line in [
            self.lua_calendar_screen.line1(),
            self.lua_calendar_screen.line2(),
            self.lua_calendar_screen.line3(),
        ] {
            BitmapLabel::new(Region::new(x, y, w, meta_font.line_height), line, meta_font)
                .alignment(Alignment::CenterLeft)
                .draw(strip)
                .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;
        }

        let mut source_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            source_line,
            "Source: {}  Marker: {}",
            self.lua_calendar_screen.source.label(),
            if self.lua_calendar_screen.source.is_sd_loaded() {
                LUA_CALENDAR_SD_RUNTIME_MARKER
            } else {
                LUA_CALENDAR_APP_MARKER
            }
        );
        source_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            self.lua_calendar_screen.footer(),
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Back returns to Productivity",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_wifi_connect(&self, strip: &mut StripBuffer) {
        let status = if !self.network_status_loaded {
            "Loading"
        } else if self.network_wifi_configured && self.network_wifi_password_saved {
            "Ready"
        } else {
            "Setup"
        };
        self.draw_screen_header(strip, "Wi-Fi Networks", status);

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 24;

        if !self.network_status_loaded {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Reading _x4/SETTINGS.TXT...",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        if self.is_wifi_text_editing() {
            let field_name = "Password";
            let mut edit_title =
                BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                    .alignment(Alignment::CenterLeft);
            let _ = write!(
                edit_title,
                "Edit {} · {} keyboard",
                field_name,
                text_keyboard::layout_name(self.wifi_keyboard_layout())
            );
            edit_title.draw(strip).unwrap();
            y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

            let mut value_line =
                BitmapDynLabel::<128>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                    .alignment(Alignment::CenterLeft);
            if self.network_pass_len == 0 {
                let _ = write!(value_line, "Password: not set");
            } else {
                let shown = self.network_pass_len.min(28);
                let _ = write!(value_line, "Password: ");
                for _ in 0..shown {
                    let _ = write!(value_line, "*");
                }
                if self.network_pass_len > shown {
                    let _ = write!(value_line, "...");
                }
                let _ = write!(value_line, " ({} chars)", self.network_pass_len);
            }
            value_line.draw(strip).unwrap();
            y += meta_font.line_height + 8;

            let keyboard_h = text_keyboard::TEXT_KEYBOARD_LARGE_HEIGHT;
            text_keyboard::draw(
                strip,
                Region::new(x, y, w, keyboard_h),
                self.wifi_keyboard_layout(),
                self.wifi_editor_char_idx,
                title_font,
                meta_font,
            );
            y += keyboard_h + NETWORK_STATUS_LINE_GAP;

            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                "Vol up/down rows · Left/Right keys · OK select",
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

            BitmapLabel::new(
                Region::new(x, y, w, meta_font.line_height),
                "Hold OK delete · done/back returns · Save persists",
                meta_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

            let mut msg_line =
                BitmapDynLabel::<96>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                    .alignment(Alignment::CenterLeft);
            let _ = write!(msg_line, "{}", self.wifi_editor_message);
            msg_line.draw(strip).unwrap();
            return;
        }

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Scan SSID, enter password, choose default, then Save.",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let profile_prefix = if self.wifi_editor_field == 0 {
            ">"
        } else {
            " "
        };
        let default_mark = if self.network_profile_slot == self.network_default_slot {
            "*"
        } else {
            " "
        };
        let mut profile_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            profile_line,
            "{} Profile {} {}",
            profile_prefix,
            default_mark,
            self.current_wifi_profile_name()
        );
        profile_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let scan_prefix = if self.wifi_editor_field == 1 {
            ">"
        } else {
            " "
        };
        let mut scan_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(scan_line, "{} Scan SSIDs", scan_prefix);
        scan_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let scanned = if self.network_scan_count > 0 {
            Self::ellipsize_ascii(self.wifi_scan_selected_str(), 28)
        } else if self.network_ssid_len > 0 {
            Self::ellipsize_ascii(self.network_ssid_str(), 28)
        } else {
            String::from("scan first")
        };
        let ssid_prefix = if self.wifi_editor_field == 2 {
            ">"
        } else {
            " "
        };
        let mut ssid_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        if self.network_scan_count > 0 {
            let _ = write!(
                ssid_line,
                "{} SSID       {} ({}/{})",
                ssid_prefix,
                scanned.as_str(),
                self.network_scan_selected + 1,
                self.network_scan_count
            );
        } else {
            let _ = write!(ssid_line, "{} SSID       {}", ssid_prefix, scanned.as_str());
        }
        ssid_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let pass_prefix = if self.wifi_editor_field == 3 {
            ">"
        } else {
            " "
        };
        let mut pass_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        if self.network_pass_len == 0 {
            let _ = write!(pass_line, "{} Password   not set", pass_prefix);
        } else {
            let _ = write!(
                pass_line,
                "{} Password   saved ({} chars)",
                pass_prefix, self.network_pass_len
            );
        }
        pass_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let default_prefix = if self.wifi_editor_field == 4 {
            ">"
        } else {
            " "
        };
        let mut default_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            default_line,
            "{} Set as default   {}",
            default_prefix,
            if self.network_profile_slot == self.network_default_slot {
                "yes"
            } else {
                "no"
            }
        );
        default_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let save_prefix = if self.wifi_editor_field == 5 {
            ">"
        } else {
            " "
        };
        let mut save_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(save_line, "{} Save to SETTINGS.TXT", save_prefix);
        save_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP + 4;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Select scans/applies/edits · Left/Right cycles SSIDs",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Larger keyboard is used for password entry",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut msg_line =
            BitmapDynLabel::<96>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(msg_line, "{}", self.wifi_editor_message);
        msg_line.draw(strip).unwrap();
    }

    fn draw_network_status(&self, strip: &mut StripBuffer) {
        let status = if self.network_status_loaded {
            "Ready"
        } else {
            "Loading"
        };
        self.draw_screen_header(strip, "Network Status", status);

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 28;

        if !self.network_status_loaded {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Reading local network settings...",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        let mut wifi =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            wifi,
            "Wi-Fi: {}",
            if self.network_wifi_configured {
                "Configured"
            } else {
                "Not configured"
            }
        );
        wifi.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let ssid = if self.network_wifi_configured {
            Self::ellipsize_ascii(self.network_ssid_str(), 28)
        } else {
            String::from("-")
        };
        let mut ssid_line =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            ssid_line,
            "Default: {} · SSID: {}",
            self.network_wifi
                .profile_name(self.network_default_slot as usize),
            ssid.as_str()
        );
        ssid_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut settings_line =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            settings_line,
            "Settings: {}",
            if self.network_settings_found {
                "_x4/SETTINGS.TXT"
            } else {
                "not found"
            }
        );
        settings_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "Radio: idle until Wi-Fi Transfer",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            "IP: shown when transfer starts",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut time_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(time_line, "Time Sync: ");
        let _ = self
            .time_cache
            .write_sync_summary(self.time_uptime_secs, &mut time_line);
        time_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut sd_line =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            sd_line,
            "SD Card: {}",
            if self.network_sd_ok { "OK" } else { "Missing" }
        );
        sd_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut power_line =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(power_line, "Battery: {} mV", self.network_battery_mv);
        power_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut uptime_line =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(
            uptime_line,
            "Uptime: {}s    Select refresh",
            self.network_uptime_secs
        );
        uptime_line.draw(strip).unwrap();
    }

    fn draw_date_time(&self, strip: &mut StripBuffer) {
        let status = if !self.time_status_loaded {
            "Loading"
        } else {
            self.time_cache.freshness(self.time_uptime_secs).as_str()
        };
        self.draw_screen_header(strip, "Date & Time", status);

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let mut y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 28;

        if !self.time_status_loaded {
            BitmapLabel::new(
                Region::new(x, y, w, title_font.line_height),
                "Reading cached time...",
                title_font,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        let mut time_line =
            BitmapDynLabel::<64>::new(Region::new(x, y, w, title_font.line_height), title_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(time_line, "Time: ");
        let _ = self
            .time_cache
            .write_time_value(self.time_uptime_secs, &mut time_line);
        time_line.draw(strip).unwrap();
        y += title_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut date_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(date_line, "Date: ");
        let _ = self
            .time_cache
            .write_date_value(self.time_uptime_secs, &mut date_line);
        date_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut tz_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(tz_line, "Timezone: {}", time_status::TIMEZONE_ID);
        tz_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut sync_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(sync_line, "Sync: ");
        let _ = self
            .time_cache
            .write_sync_summary(self.time_uptime_secs, &mut sync_line);
        sync_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut clock_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = self
            .time_cache
            .write_clock_detail(self.time_uptime_secs, &mut clock_line);
        clock_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut last_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(last_line, "Last sync: ");
        let _ = self.time_cache.write_last_sync(&mut last_line);
        last_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let mut result_line =
            BitmapDynLabel::<80>::new(Region::new(x, y, w, meta_font.line_height), meta_font)
                .alignment(Alignment::CenterLeft);
        let _ = write!(result_line, "Last result: ");
        if self.time_cache.last_sync_unix.is_none() && self.time_cache.last_sync_error.is_empty() {
            let _ = write!(result_line, "Never");
        } else if self.time_cache.last_sync_ok {
            let _ = write!(result_line, "OK");
        } else if self.time_cache.last_sync_error.is_empty() {
            let _ = write!(result_line, "Failed");
        } else {
            let _ = write!(result_line, "{}", self.time_cache.last_sync_error.as_str());
        }
        result_line.draw(strip).unwrap();
        y += meta_font.line_height + NETWORK_STATUS_LINE_GAP;

        let prompt = match self.time_cache.freshness(self.time_uptime_secs) {
            time_status::ClockFreshness::Live => "Select safely resyncs    Back returns",
            time_status::ClockFreshness::Cached => "Cached; Select safely retries",
            time_status::ClockFreshness::Unsynced => "Select syncs now    Back returns",
        };
        BitmapLabel::new(
            Region::new(x, y, w, meta_font.line_height),
            prompt,
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_placeholder(&self, strip: &mut StripBuffer) {
        self.draw_screen_header(strip, self.placeholder_title, "Soon");

        let title_font = self.card_title_font();
        let meta_font = self.card_meta_font();
        let x = LARGE_MARGIN;
        let w = FULL_CONTENT_W;
        let y = BM_TITLE_Y + self.ui_fonts.heading.line_height + 32;

        BitmapLabel::new(
            Region::new(x, y, w, title_font.line_height),
            self.placeholder_detail,
            title_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();

        BitmapLabel::new(
            Region::new(x, y + title_font.line_height + 16, w, meta_font.line_height),
            "Back returns to category",
            meta_font,
        )
        .alignment(Alignment::CenterLeft)
        .draw(strip)
        .unwrap();
    }

    fn draw_bookmarks(&self, strip: &mut StripBuffer) {
        let header_region = Region::new(
            LARGE_MARGIN,
            BM_TITLE_Y,
            HEADER_W,
            self.ui_fonts.heading.line_height,
        );
        BitmapLabel::new(header_region, "Bookmarks", self.ui_fonts.heading)
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();

        if self.bm_count > 0 {
            let mut status = BitmapDynLabel::<20>::new(self.bm_status_region(), self.ui_fonts.body)
                .alignment(Alignment::CenterRight);
            let _ = write!(status, "{}/{}", self.bm_selected + 1, self.bm_count);
            status.draw(strip).unwrap();
        }

        if self.bm_count == 0 {
            BitmapLabel::new(
                self.bm_row_region(0),
                "No bookmarks yet",
                self.ui_fonts.body,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
            return;
        }

        let grouped_title = self.bm_group_title();

        if let Some(title) = grouped_title {
            let subtitle_text = Self::ellipsize_ascii(title, BM_MAX_TITLE_CHARS);
            BitmapLabel::new(
                self.bm_subtitle_region(),
                &subtitle_text,
                self.ui_fonts.body,
            )
            .alignment(Alignment::CenterLeft)
            .draw(strip)
            .unwrap();
        }

        let vis = self.bm_visible_lines();
        let visible = vis.min(self.bm_count.saturating_sub(self.bm_scroll));

        for i in 0..vis {
            let region = self.bm_row_region(i);
            if i < visible {
                let idx = self.bm_scroll + i;
                let entry = &self.bm_entries[idx];
                let label = if grouped_title.is_some() {
                    Self::bm_detail_label(entry)
                } else {
                    Self::bm_mixed_book_label(entry)
                };

                BitmapLabel::new(region, &label, self.ui_fonts.body)
                    .alignment(Alignment::CenterLeft)
                    .inverted(idx == self.bm_selected)
                    .draw(strip)
                    .unwrap();
            }
        }
    }
}
