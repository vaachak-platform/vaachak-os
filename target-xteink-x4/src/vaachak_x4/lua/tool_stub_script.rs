//! SD-loaded Lua tools stub contract for Vaachak OS.
//!
//! Physical folders remain uppercase 8.3-safe under `/VAACHAK/APPS`, while
//! logical app ids remain descriptive snake_case in APP.TOM.

pub const LUA_TOOLS_DICTIONARY_UNIT_CONVERTER_STUB_PACK_MARKER: &str =
    "vaachak-lua-tools-dictionary-unit-converter-stub-pack-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LuaToolApp {
    pub folder: &'static str,
    pub logical_id: &'static str,
    pub display_name: &'static str,
    pub detail: &'static str,
    pub data_file: &'static str,
}

pub const LUA_TOOL_APPS: [LuaToolApp; 2] = [
    LuaToolApp {
        folder: "DICT",
        logical_id: "dictionary",
        display_name: "Dictionary",
        detail: "Offline prefix-shard word lookup",
        data_file: "INDEX.TXT",
    },
    LuaToolApp {
        folder: "UNITS",
        logical_id: "unit_converter",
        display_name: "Unit Converter",
        detail: "Offline units helper stub",
        data_file: "UNITS.TXT",
    },
];

pub const fn lua_tool_count() -> usize {
    LUA_TOOL_APPS.len()
}

pub fn lua_tool_app(index: usize) -> &'static LuaToolApp {
    &LUA_TOOL_APPS[if index < LUA_TOOL_APPS.len() {
        index
    } else {
        0
    }]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LuaToolStubSource {
    SdLuaScript,
    BuiltInFallback,
    MissingManifest,
    MissingEntry,
    ManifestInvalidUtf8,
    ScriptInvalidUtf8,
    InvalidManifestContract,
}

impl LuaToolStubSource {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SdLuaScript => "SD Lua",
            Self::BuiltInFallback => "Fallback",
            Self::MissingManifest => "Missing APP.TOM",
            Self::MissingEntry => "Missing MAIN.LUA",
            Self::ManifestInvalidUtf8 => "APP.TOM UTF-8 error",
            Self::ScriptInvalidUtf8 => "MAIN.LUA UTF-8 error",
            Self::InvalidManifestContract => "Bad APP.TOM",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LuaToolText<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> LuaToolText<N> {
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

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LuaToolStubScreen {
    pub source: LuaToolStubSource,
    pub title: LuaToolText<48>,
    pub line1: LuaToolText<144>,
    pub line2: LuaToolText<144>,
    pub line3: LuaToolText<144>,
    pub footer: LuaToolText<128>,
}

impl LuaToolStubScreen {
    pub fn fallback_for(index: usize) -> Self {
        let app = lua_tool_app(index);
        let mut footer = LuaToolText::from_str("Folder: /VAACHAK/APPS/");
        footer.push_str(app.folder);
        Self {
            source: LuaToolStubSource::BuiltInFallback,
            title: LuaToolText::from_str(app.display_name),
            line1: LuaToolText::from_str(app.detail),
            line2: LuaToolText::from_str("Upload APP.TOM + MAIN.LUA over Wi-Fi Transfer."),
            line3: LuaToolText::from_str("Back exits safely to Tools."),
            footer,
        }
    }

    pub fn diagnostic(
        index: usize,
        source: LuaToolStubSource,
        primary: &str,
        remediation: &str,
    ) -> Self {
        let app = lua_tool_app(index);
        let mut screen = Self::fallback_for(index);
        screen.source = source;
        screen.title.set(app.display_name);
        screen.line1.set(primary);
        screen.line2.set(remediation);
        screen.line3.set("Back exits safely to Tools.");
        screen.footer.set("Canonical root: /VAACHAK/APPS");
        screen
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
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

impl Default for LuaToolStubScreen {
    fn default() -> Self {
        Self::fallback_for(0)
    }
}

pub fn build_tool_stub_runtime(index: usize, manifest: &str, script: &str) -> LuaToolStubScreen {
    let app = lua_tool_app(index);
    if !manifest_declares_id(manifest, app.logical_id) {
        return LuaToolStubScreen::diagnostic(
            index,
            LuaToolStubSource::InvalidManifestContract,
            "APP.TOM app id does not match folder",
            "Fix manifest id and reopen app",
        );
    }

    let mut screen = evaluate_tool_stub_lua_subset(index, script);
    screen.source = LuaToolStubSource::SdLuaScript;
    screen.footer.set("Loaded from SD MAIN.LUA");
    screen
}

pub fn evaluate_tool_stub_lua_subset(index: usize, script: &str) -> LuaToolStubScreen {
    let mut screen = LuaToolStubScreen::fallback_for(index);
    for line in script.lines() {
        if let Some((key, value)) = parse_lua_string_assignment(line) {
            match key {
                "display_title" | "title" => screen.title.set(value),
                "display_line1" | "line1" => screen.line1.set(value),
                "display_line2" | "line2" => screen.line2.set(value),
                "display_line3" | "line3" => screen.line3.set(value),
                "display_footer" | "footer" => screen.footer.set(value),
                _ => {}
            }
        }
    }
    screen
}

fn manifest_declares_id(manifest: &str, expected: &str) -> bool {
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("id") {
            if let Some((key, value)) = parse_key_value(trimmed) {
                if key == "id" && value == expected {
                    return true;
                }
            }
        }
    }
    false
}

fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let line = line.split('#').next().unwrap_or("").trim();
    let (key, raw) = line.split_once('=')?;
    let key = key.trim();
    let value = unquote(raw.trim())?;
    Some((key, value))
}

fn parse_lua_string_assignment(line: &str) -> Option<(&str, &str)> {
    parse_key_value(line)
}

fn unquote(value: &str) -> Option<&str> {
    if value.len() < 2 {
        return None;
    }
    let bytes = value.as_bytes();
    let quote = bytes[0];
    if quote != b'\'' && quote != b'\"' {
        return None;
    }
    if bytes[value.len() - 1] != quote {
        return None;
    }
    Some(&value[1..value.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dictionary_tool_stub_screen_from_manifest_and_script() {
        let manifest = "id = \"dictionary\"\nname = \"Dictionary\"";
        let script = "display_title = \"Dictionary\"\ndisplay_line1 = \"Loaded from SD\"";
        let screen = build_tool_stub_runtime(0, manifest, script);
        assert_eq!(screen.source, LuaToolStubSource::SdLuaScript);
        assert_eq!(screen.title(), "Dictionary");
        assert_eq!(screen.line1(), "Loaded from SD");
    }

    #[test]
    fn rejects_bad_tool_manifest_id() {
        let manifest = "id = \"wrong\"";
        let screen = build_tool_stub_runtime(1, manifest, "display_title = \"Unit Converter\"");
        assert_eq!(screen.source, LuaToolStubSource::InvalidManifestContract);
    }
}
