#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from the vaachak-os repository root or pass the repository path" >&2
  exit 1
fi

settings="vendor/pulp-os/src/apps/settings.rs"
handle="vendor/pulp-os/kernel/src/kernel/handle.rs"

for path in "$settings" "$handle"; do
  if [ ! -f "$path" ]; then
    echo "error: missing expected file: $path" >&2
    exit 1
  fi
done

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/sleep_settings_ui_${stamp}"
mkdir -p "$backup_dir/$(dirname "$settings")" "$backup_dir/$(dirname "$handle")"
cp "$settings" "$backup_dir/$settings"
cp "$handle" "$backup_dir/$handle"

python3 - <<'PY'
from pathlib import Path
import re

settings_path = Path("vendor/pulp-os/src/apps/settings.rs")
handle_path = Path("vendor/pulp-os/kernel/src/kernel/handle.rs")

# Add root write support to KernelHandle so Settings can persist /SLPMODE.TXT.
handle = handle_path.read_text()
if "pub fn write_file(&mut self, name: &str, data: &[u8])" not in handle:
    anchor = '''    #[inline]\n    pub fn read_file_start(&mut self, name: &str, buf: &mut [u8]) -> Result<(u32, usize)> {\n        storage::read_file_start(&self.kernel.sd, name, buf)\n    }\n'''
    insert = anchor + '''\n    #[inline]\n    pub fn write_file(&mut self, name: &str, data: &[u8]) -> Result<()> {\n        storage::write_file(&self.kernel.sd, name, data)\n    }\n'''
    if anchor not in handle:
        raise SystemExit("Could not find KernelHandle::read_file_start anchor")
    handle = handle.replace(anchor, insert, 1)
    handle_path.write_text(handle)

text = settings_path.read_text()

# Constants and row count.
text = re.sub(r"const NUM_ROWS: usize = \d+;", "const NUM_ROWS: usize = 23;", text, count=1)
if "const SLEEP_IMAGE_MODE_FILE:" not in text:
    const_anchor = "const NUM_ROWS: usize = 23;\n"
    const_insert = const_anchor + '''\nconst SLEEP_IMAGE_MODE_FILE: &str = "SLPMODE.TXT";\nconst SLEEP_IMAGE_MODE_COUNT: u8 = 6;\nconst SLEEP_IMAGE_MODE_VALUES: [&str; SLEEP_IMAGE_MODE_COUNT as usize] = [\n    "daily",\n    "fast-daily",\n    "static",\n    "cached",\n    "text",\n    "no-redraw",\n];\nconst SLEEP_IMAGE_MODE_LABELS: [&str; SLEEP_IMAGE_MODE_COUNT as usize] = [\n    "Daily",\n    "Fast Daily",\n    "Static",\n    "Cached",\n    "Text",\n    "No Redraw",\n];\n'''
    if const_anchor not in text:
        raise SystemExit("Could not find NUM_ROWS anchor")
    text = text.replace(const_anchor, const_insert, 1)

# Row kind.
if "DeviceSleepImageMode" not in text:
    text = text.replace(
        "    DeviceSleepTimeout,\n",
        "    DeviceSleepTimeout,\n    DeviceSleepImageMode,\n",
        1,
    )

# Rows table: insert after Sleep timeout.
if 'label: "Sleep image"' not in text:
    timeout_row = '''    SettingsRow {\n        label: "Sleep timeout",\n        kind: SettingsRowKind::DeviceSleepTimeout,\n    },\n'''
    sleep_image_row = timeout_row + '''    SettingsRow {\n        label: "Sleep image",\n        kind: SettingsRowKind::DeviceSleepImageMode,\n    },\n'''
    if timeout_row not in text:
        raise SystemExit("Could not find Sleep timeout row anchor")
    text = text.replace(timeout_row, sleep_image_row, 1)

# Struct field and default.
if "sleep_image_mode: u8," not in text:
    text = text.replace(
        "    device_sleep_timeout: u8,\n",
        "    device_sleep_timeout: u8,\n    sleep_image_mode: u8,\n",
        1,
    )
    text = text.replace(
        "            device_sleep_timeout: 1,\n",
        "            device_sleep_timeout: 1,\n            sleep_image_mode: 0,\n",
        1,
    )

# Helper methods for /SLPMODE.TXT root file.
if "fn load_sleep_image_mode" not in text:
    helper_anchor = "    fn sync_local_from_system_settings(&mut self) {\n"
    helpers = '''    fn load_sleep_image_mode(&mut self, k: &mut KernelHandle<'_>) {\n        let mut buf = [0u8; 32];\n        self.sleep_image_mode = match k.read_file_start(SLEEP_IMAGE_MODE_FILE, &mut buf) {\n            Ok((_size, n)) if n > 0 => parse_sleep_image_mode(&buf[..n]),\n            _ => 0,\n        };\n    }\n\n    fn save_sleep_image_mode(&self, k: &mut KernelHandle<'_>) -> bool {\n        let mut buf = [0u8; 16];\n        let len = write_sleep_image_mode(self.sleep_image_mode, &mut buf);\n        match k.write_file(SLEEP_IMAGE_MODE_FILE, &buf[..len]) {\n            Ok(_) => true,\n            Err(e) => {\n                log::error!("settings: sleep image mode save failed: {}", e);\n                false\n            }\n        }\n    }\n\n'''
    if helper_anchor not in text:
        raise SystemExit("Could not find sync_local_from_system_settings anchor")
    text = text.replace(helper_anchor, helpers + helper_anchor, 1)

# Load root mode file after settings load and local sync.
if "self.load_sleep_image_mode(k);" not in text:
    text = text.replace(
        "        self.sync_local_from_system_settings();\n        self.loaded = true;\n",
        "        self.sync_local_from_system_settings();\n        self.load_sleep_image_mode(k);\n        self.loaded = true;\n",
        1,
    )

# Replace save function so it persists both SETTINGS.TXT and SLPMODE.TXT.
save_pattern = r"    fn save\(&self, k: &mut KernelHandle<'_>\) -> bool \{.*?    \}\n\n    fn visible_rows"
save_replacement = '''    fn save(&self, k: &mut KernelHandle<'_>) -> bool {\n        let mut buf = [0u8; 1024];\n        let len = write_settings_txt(&self.settings, &self.wifi, &mut buf);\n        let settings_saved = match k.write_app_data(config::SETTINGS_FILE, &buf[..len]) {\n            Ok(_) => {\n                log::info!("settings: saved to {}", config::SETTINGS_FILE);\n                true\n            }\n            Err(e) => {\n                log::error!("settings: save failed: {}", e);\n                false\n            }\n        };\n\n        let sleep_mode_saved = self.save_sleep_image_mode(k);\n        settings_saved && sleep_mode_saved\n    }\n\n    fn visible_rows'''
text, count = re.subn(save_pattern, save_replacement, text, count=1, flags=re.S)
if count != 1:
    raise SystemExit("Could not replace SettingsApp::save")

# Cycle handler.
if "SettingsRowKind::DeviceSleepImageMode" not in text:
    text = text.replace(
        '''            SettingsRowKind::DeviceSleepTimeout => {\n                self.device_sleep_timeout = cycle_index(self.device_sleep_timeout, 4, delta);\n                true\n            }\n''',
        '''            SettingsRowKind::DeviceSleepTimeout => {\n                self.device_sleep_timeout = cycle_index(self.device_sleep_timeout, 4, delta);\n                true\n            }\n            SettingsRowKind::DeviceSleepImageMode => {\n                self.sleep_image_mode = cycle_index(self.sleep_image_mode, SLEEP_IMAGE_MODE_COUNT, delta);\n                true\n            }\n''',
        1,
    )

# Format value handler.
if "sleep_image_mode_name(self.sleep_image_mode)" not in text:
    text = text.replace(
        '''            SettingsRowKind::DeviceSleepTimeout => {\n                let _ = write!(\n                    buf,\n                    "{}",\n                    ["5 min", "10 min", "30 min", "Never"][self.device_sleep_timeout as usize]\n                );\n            }\n''',
        '''            SettingsRowKind::DeviceSleepTimeout => {\n                let _ = write!(\n                    buf,\n                    "{}",\n                    ["5 min", "10 min", "30 min", "Never"][self.device_sleep_timeout as usize]\n                );\n            }\n            SettingsRowKind::DeviceSleepImageMode => {\n                let _ = write!(buf, "{}", sleep_image_mode_name(self.sleep_image_mode));\n            }\n''',
        1,
    )

# Global helpers near bottom.
if "fn sleep_image_mode_name" not in text:
    bottom_anchor = "fn cycle_index(value: u8, count: u8, delta: isize) -> u8 {\n"
    bottom_helpers = '''fn sleep_image_mode_name(idx: u8) -> &'static str {\n    SLEEP_IMAGE_MODE_LABELS\n        .get(idx as usize)\n        .copied()\n        .unwrap_or("Daily")\n}\n\nfn sleep_image_mode_value(idx: u8) -> &'static str {\n    SLEEP_IMAGE_MODE_VALUES\n        .get(idx as usize)\n        .copied()\n        .unwrap_or("daily")\n}\n\nfn parse_sleep_image_mode(data: &[u8]) -> u8 {\n    let Ok(text) = core::str::from_utf8(data) else {\n        return 0;\n    };\n    let trimmed = text.trim();\n    for (idx, value) in SLEEP_IMAGE_MODE_VALUES.iter().enumerate() {\n        if trimmed.eq_ignore_ascii_case(value) {\n            return idx as u8;\n        }\n    }\n    if trimmed.eq_ignore_ascii_case("off") {\n        return 5;\n    }\n    0\n}\n\nfn write_sleep_image_mode(idx: u8, out: &mut [u8]) -> usize {\n    let value = sleep_image_mode_value(idx).as_bytes();\n    let mut pos = 0usize;\n    while pos < value.len() && pos < out.len() {\n        out[pos] = value[pos];\n        pos += 1;\n    }\n    if pos < out.len() {\n        out[pos] = b'\\n';\n        pos += 1;\n    }\n    pos\n}\n\n'''
    if bottom_anchor not in text:
        raise SystemExit("Could not find cycle_index anchor")
    text = text.replace(bottom_anchor, bottom_helpers + bottom_anchor, 1)

settings_path.write_text(text)
PY

cargo fmt --all

echo "Applied Sleep Image Mode settings UI. Backup: $backup_dir"
