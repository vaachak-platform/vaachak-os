//! SD-backed Calendar Lua app renderer for Vaachak OS.
//!
//! This module intentionally keeps the current constrained Lua declaration
//! subset plus the feature-gated VM event-selection proof. Calendar app data is
//! read from `/VAACHAK/APPS/CALENDAR` using 8.3-safe physical names.
//!
//! Required SD files:
//!
//! - `/VAACHAK/APPS/CALENDAR/APP.TOM`
//! - `/VAACHAK/APPS/CALENDAR/MAIN.LUA`
//! - `/VAACHAK/APPS/CALENDAR/EVENTS.TXT`

/// Marker emitted/displayed by the Calendar Lua-backed app path.
pub const LUA_CALENDAR_APP_MARKER: &str = "vaachak-lua-calendar-app-ok";

/// Marker for the SD runtime proof that loads all required Calendar app files.
pub const LUA_CALENDAR_SD_RUNTIME_MARKER: &str = "vaachak-lua-calendar-sd-runtime-ok";

/// Optional VM marker for event-selection expression support.
pub const LUA_CALENDAR_VM_SELECTION_MARKER: &str = "vaachak-lua-calendar-vm-selection-ok";

/// Logical app id used by the accepted manifest/discovery model.
pub const LUA_CALENDAR_APP_ID: &str = "calendar";

/// 8.3-safe physical app folder under `/VAACHAK/APPS/` for X4 SD access.
pub const LUA_CALENDAR_APP_FOLDER: &str = "CALENDAR";

/// Manifest file read from `/VAACHAK/APPS/CALENDAR/APP.TOM`.
pub const LUA_CALENDAR_MANIFEST_FILE: &str = "APP.TOM";

/// Entry file read from `/VAACHAK/APPS/CALENDAR/MAIN.LUA`.
pub const LUA_CALENDAR_ENTRY_FILE: &str = "MAIN.LUA";

/// Data file read from `/VAACHAK/APPS/CALENDAR/EVENTS.TXT`.
pub const LUA_CALENDAR_EVENTS_FILE: &str = "EVENTS.TXT";

const TITLE_DEFAULT: &str = "Calendar";
const SUBTITLE_DEFAULT: &str = "Date: Today";
const LINE1_DEFAULT: &str = "Event: No event loaded yet";
const LINE2_DEFAULT: &str = "";
const LINE3_DEFAULT: &str = "Back exits safely to Productivity";
const FOOTER_DEFAULT: &str = "Loaded from /VAACHAK/APPS/CALENDAR";

/// Conservative wrap width for the current X4 bitmap-label Calendar view.
pub const LUA_CALENDAR_WRAP_CHARS: usize = 44;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LuaCalendarSource {
    SdLuaScript,
    BuiltInFallback,
    MissingManifest,
    MissingEntry,
    MissingEvents,
    ManifestInvalidUtf8,
    ScriptInvalidUtf8,
    EventsInvalidUtf8,
    InvalidManifestContract,
}

impl LuaCalendarSource {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SdLuaScript => "SD Lua",
            Self::BuiltInFallback => "Fallback",
            Self::MissingManifest => "Missing APP.TOM",
            Self::MissingEntry => "Missing MAIN.LUA",
            Self::MissingEvents => "Missing EVENTS.TXT",
            Self::ManifestInvalidUtf8 => "APP.TOM UTF-8 error",
            Self::ScriptInvalidUtf8 => "MAIN.LUA UTF-8 error",
            Self::EventsInvalidUtf8 => "EVENTS.TXT UTF-8 error",
            Self::InvalidManifestContract => "Bad APP.TOM",
        }
    }

    pub const fn is_sd_loaded(self) -> bool {
        matches!(self, Self::SdLuaScript)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LuaCalendarText<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> LuaCalendarText<N> {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }

    pub fn from_str(value: &str) -> Self {
        let mut text = Self::empty();
        text.set(value);
        text
    }

    pub fn clear(&mut self) {
        self.bytes = [0; N];
        self.len = 0;
    }

    pub fn set(&mut self, value: &str) {
        self.clear();
        self.push_str(value);
    }

    pub fn set_i32_line(&mut self, prefix: &str, value: i32) {
        self.clear();
        self.push_str(prefix);
        self.push_i32(value);
    }

    pub fn push_str(&mut self, value: &str) {
        for ch in value.chars() {
            self.push_char(ch);
        }
    }

    pub fn push_char(&mut self, value: char) {
        let mut buf = [0u8; 4];
        let encoded = value.encode_utf8(&mut buf);
        if self.len + encoded.len() <= N {
            self.bytes[self.len..self.len + encoded.len()].copy_from_slice(encoded.as_bytes());
            self.len += encoded.len();
        }
    }

    fn push_byte(&mut self, value: u8) {
        if self.len < N {
            self.bytes[self.len] = value;
            self.len += 1;
        }
    }

    fn push_i32(&mut self, value: i32) {
        if value == 0 {
            self.push_byte(b'0');
            return;
        }
        let mut number = if value < 0 {
            self.push_byte(b'-');
            (-(value as i64)) as u32
        } else {
            value as u32
        };
        let mut digits = [0u8; 10];
        let mut count = 0usize;
        while number > 0 && count < digits.len() {
            digits[count] = b'0' + (number % 10) as u8;
            number /= 10;
            count += 1;
        }
        while count > 0 {
            count -= 1;
            self.push_byte(digits[count]);
        }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LuaCalendarScreen {
    pub source: LuaCalendarSource,
    pub title: LuaCalendarText<48>,
    pub subtitle: LuaCalendarText<96>,
    pub line1: LuaCalendarText<160>,
    pub line2: LuaCalendarText<160>,
    pub line3: LuaCalendarText<160>,
    pub footer: LuaCalendarText<112>,
}

impl LuaCalendarScreen {
    pub fn fallback() -> Self {
        Self {
            source: LuaCalendarSource::BuiltInFallback,
            title: LuaCalendarText::from_str(TITLE_DEFAULT),
            subtitle: LuaCalendarText::from_str(SUBTITLE_DEFAULT),
            line1: LuaCalendarText::from_str(LINE1_DEFAULT),
            line2: LuaCalendarText::from_str(LINE2_DEFAULT),
            line3: LuaCalendarText::from_str(LINE3_DEFAULT),
            footer: LuaCalendarText::from_str(FOOTER_DEFAULT),
        }
    }

    pub fn missing_manifest() -> Self {
        diagnostic_screen(
            LuaCalendarSource::MissingManifest,
            "Missing /VAACHAK/APPS/CALENDAR/APP.TOM",
            "Upload APP.TOM with Wi-Fi Transfer",
        )
    }
    pub fn missing_entry() -> Self {
        diagnostic_screen(
            LuaCalendarSource::MissingEntry,
            "Missing /VAACHAK/APPS/CALENDAR/MAIN.LUA",
            "Upload MAIN.LUA with Wi-Fi Transfer",
        )
    }
    pub fn missing_events() -> Self {
        diagnostic_screen(
            LuaCalendarSource::MissingEvents,
            "Missing /VAACHAK/APPS/CALENDAR/EVENTS.TXT",
            "Upload EVENTS.TXT with Wi-Fi Transfer",
        )
    }
    pub fn invalid_manifest_utf8() -> Self {
        diagnostic_screen(
            LuaCalendarSource::ManifestInvalidUtf8,
            "APP.TOM is not valid UTF-8",
            "Replace APP.TOM on SD card",
        )
    }
    pub fn invalid_script_utf8() -> Self {
        diagnostic_screen(
            LuaCalendarSource::ScriptInvalidUtf8,
            "MAIN.LUA is not valid UTF-8",
            "Replace MAIN.LUA on SD card",
        )
    }
    pub fn invalid_events_utf8() -> Self {
        diagnostic_screen(
            LuaCalendarSource::EventsInvalidUtf8,
            "EVENTS.TXT is not valid UTF-8",
            "Replace EVENTS.TXT on SD card",
        )
    }
    pub fn invalid_manifest_contract() -> Self {
        diagnostic_screen(
            LuaCalendarSource::InvalidManifestContract,
            "APP.TOM must declare id = \"calendar\"",
            "Fix manifest and reopen the app",
        )
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn subtitle(&self) -> &str {
        self.subtitle.as_str()
    }
    pub fn line1(&self) -> &str {
        self.line1.as_str()
    }
    pub fn line2(&self) -> &str {
        self.line2.as_str()
    }
    pub fn line3(&self) -> &str {
        self.line3.as_str()
    }
    pub fn footer(&self) -> &str {
        self.footer.as_str()
    }
}

impl Default for LuaCalendarScreen {
    fn default() -> Self {
        Self::fallback()
    }
}

fn diagnostic_screen(
    source: LuaCalendarSource,
    primary: &str,
    remediation: &str,
) -> LuaCalendarScreen {
    let mut screen = LuaCalendarScreen::fallback();
    screen.source = source;
    screen.subtitle.set("Lua app SD files are incomplete");
    screen.line1.set(primary);
    screen.line2.set(remediation);
    screen.line3.set("Back exits safely to Productivity");
    screen.footer.set("Canonical root: /VAACHAK/APPS");
    screen
}

/// Build the Calendar screen from SD file contents.
///
/// This intentionally ignores visual text assignments in MAIN.LUA for the event
/// body. `EVENTS.TXT` is authoritative for date/event display so old SD scripts
/// cannot accidentally show raw `|` separators.
pub fn build_calendar_sd_runtime(manifest: &str, script: &str, events: &str) -> LuaCalendarScreen {
    if !manifest_declares_calendar(manifest) {
        return LuaCalendarScreen::invalid_manifest_contract();
    }

    let mut screen = LuaCalendarScreen::fallback();
    screen.source = LuaCalendarSource::SdLuaScript;
    screen.title.set("Calendar");

    let selected_index = calendar_selected_event_index(script).unwrap_or(0);
    if let Some(event) = nth_calendar_record(events, selected_index) {
        set_date_line(&mut screen, event.date);
        set_wrapped_event(&mut screen, event.title, event.detail);
    } else if first_non_empty_line(events).is_some() {
        screen.subtitle.set("Date: Today");
        screen.line1.set("Event: No matching event found");
        screen.line2.set("");
    } else {
        screen.subtitle.set("Date: Today");
        screen.line1.set("Event: EVENTS.TXT is present but empty");
        screen.line2.set("");
    }

    screen.line3.set("Back exits safely to Productivity");
    screen.footer.set("Loaded: APP.TOM + MAIN.LUA + EVENTS.TXT");
    screen
}

fn manifest_declares_calendar(manifest: &str) -> bool {
    manifest.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "id = \"calendar\"" || trimmed == "id=\"calendar\""
    })
}

fn calendar_selected_event_index(script: &str) -> Option<usize> {
    #[cfg(feature = "lua-vm")]
    {
        let expr = extract_calendar_vm_event_index(script)?;
        let mut vm = vaachak_lua_vm::LuaVm::new();
        match vm.eval_i32(expr) {
            Ok(value) if value > 0 => return Some((value as usize).saturating_sub(1)),
            _ => return None,
        }
    }
    #[cfg(not(feature = "lua-vm"))]
    {
        let _ = script;
        None
    }
}

pub fn extract_calendar_vm_event_index(script: &str) -> Option<&str> {
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(expr) = trimmed.strip_prefix("-- vaachak-vm-event:") {
            let expr = expr.trim();
            if !expr.is_empty() {
                return Some(expr);
            }
        }
        if let Some(expr) = parse_vm_assignment(trimmed, "vm_event_index_expression") {
            return Some(expr);
        }
    }
    None
}

fn parse_vm_assignment<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?.trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let value = strip_lua_trailing_comment(rest)
        .trim_end_matches(',')
        .trim();
    if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
        return None;
    }
    let inner = &value[1..value.len() - 1];
    if inner.trim().is_empty() {
        None
    } else {
        Some(inner.trim())
    }
}

fn strip_lua_trailing_comment(value: &str) -> &str {
    match value.find(" --") {
        Some(pos) => value[..pos].trim_end(),
        None => value,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarRecord<'a> {
    pub date: &'a str,
    pub title: &'a str,
    pub detail: &'a str,
}

fn nth_calendar_record(text: &str, index: usize) -> Option<CalendarRecord<'_>> {
    let mut seen = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("--") {
            continue;
        }
        if let Some(record) = parse_calendar_record(line) {
            if seen == index {
                return Some(record);
            }
            seen += 1;
        }
    }
    None
}

fn parse_calendar_record(line: &str) -> Option<CalendarRecord<'_>> {
    let mut parts = line.splitn(3, '|');
    let date = parts.next()?.trim();
    let title = parts.next().unwrap_or("").trim();
    let detail = parts.next().unwrap_or("").trim();
    if date.is_empty() || title.is_empty() {
        None
    } else {
        Some(CalendarRecord {
            date,
            title,
            detail,
        })
    }
}

fn first_non_empty_line(text: &str) -> Option<&str> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with("--"))
}

fn set_date_line(screen: &mut LuaCalendarScreen, date: &str) {
    screen.subtitle.set("Date: ");
    screen.subtitle.push_str(date);
}

fn set_wrapped_event(screen: &mut LuaCalendarScreen, title: &str, detail: &str) {
    let mut combined = LuaCalendarText::<220>::empty();
    combined.push_str("Event: ");
    combined.push_str(title);
    if !detail.trim().is_empty() {
        combined.push_str(" - ");
        combined.push_str(detail.trim());
    }
    wrap_two_lines(combined.as_str(), &mut screen.line1, &mut screen.line2);
}

fn wrap_two_lines(text: &str, line1: &mut LuaCalendarText<160>, line2: &mut LuaCalendarText<160>) {
    let trimmed = text.trim();
    line1.clear();
    line2.clear();
    if trimmed.chars().count() <= LUA_CALENDAR_WRAP_CHARS {
        line1.set(trimmed);
        return;
    }

    let mut split_byte = trimmed.len();
    let mut char_count = 0usize;
    for (idx, ch) in trimmed.char_indices() {
        if char_count >= LUA_CALENDAR_WRAP_CHARS {
            split_byte = idx;
            break;
        }
        char_count += 1;
        split_byte = idx + ch.len_utf8();
    }

    let prefix = &trimmed[..split_byte];
    let split = prefix.rfind(' ').unwrap_or(split_byte).max(1);
    line1.set(trimmed[..split].trim_end());
    line2.set(trimmed[split..].trim_start());
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"id = "calendar"
name = "Calendar"
category = "Productivity"
type = "activity"
version = "0.1.0"
entry = "MAIN.LUA"
capabilities = ["display", "input", "storage", "time"]
"#;

    #[test]
    fn sd_runtime_uses_events_txt_for_date_and_event_layout() {
        let script = r#"
display_title = "Old title from script"
display_line1 = "This must not override EVENTS.TXT"
"#;
        let screen = build_calendar_sd_runtime(
            MANIFEST,
            script,
            "2026-05-11|Vaachak Lua app milestone|Calendar app follow-up under Productivity\n",
        );
        assert_eq!(screen.source, LuaCalendarSource::SdLuaScript);
        assert_eq!(screen.title(), "Calendar");
        assert_eq!(screen.subtitle(), "Date: 2026-05-11");
        assert!(
            screen
                .line1()
                .starts_with("Event: Vaachak Lua app milestone")
        );
        assert!(screen.line2().contains("Productivity") || screen.line1().contains("Productivity"));
        assert!(!screen.line1().contains('|'));
        assert!(!screen.line2().contains('|'));
        assert_eq!(screen.line3(), "Back exits safely to Productivity");
    }

    #[test]
    fn invalid_manifest_produces_diagnostic_screen() {
        let screen = build_calendar_sd_runtime("id = \"bad\"", "", "2026-01-01|New Year");
        assert_eq!(screen.source, LuaCalendarSource::InvalidManifestContract);
        assert!(screen.line1().contains("id = \"calendar\""));
    }

    #[test]
    fn extracts_vm_event_index_expression() {
        assert_eq!(
            extract_calendar_vm_event_index("vm_event_index_expression = \"return 2 + 0\""),
            Some("return 2 + 0")
        );
        assert_eq!(
            extract_calendar_vm_event_index("-- vaachak-vm-event: return 3"),
            Some("return 3")
        );
    }
}
