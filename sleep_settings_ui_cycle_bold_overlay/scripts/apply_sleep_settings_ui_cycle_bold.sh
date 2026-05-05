#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from vaachak-os repo root or pass repo path" >&2
  exit 1
fi

settings="vendor/pulp-os/src/apps/settings.rs"
if [ ! -f "$settings" ]; then
  echo "error: missing $settings" >&2
  exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/sleep_settings_ui_cycle_bold_${stamp}"
mkdir -p "$backup_dir/$(dirname "$settings")"
cp "$settings" "$backup_dir/$settings"

python3 - <<'PY'
from pathlib import Path

path = Path("vendor/pulp-os/src/apps/settings.rs")
text = path.read_text()

# Fix cycle support for the Sleep image row. The first overlay added the row,
# value formatter, and persistence helpers, but did not add the editable branch
# when DeviceSleepImageMode was already present in the enum.
if "self.sleep_image_mode = cycle_index(self.sleep_image_mode, SLEEP_IMAGE_MODE_COUNT, delta)" not in text:
    old = '''            SettingsRowKind::DeviceSleepTimeout => {
                self.device_sleep_timeout = cycle_index(self.device_sleep_timeout, 4, delta);
                true
            }
            _ => false,
'''
    new = '''            SettingsRowKind::DeviceSleepTimeout => {
                self.device_sleep_timeout = cycle_index(self.device_sleep_timeout, 4, delta);
                true
            }
            SettingsRowKind::DeviceSleepImageMode => {
                self.sleep_image_mode = cycle_index(self.sleep_image_mode, SLEEP_IMAGE_MODE_COUNT, delta);
                true
            }
            _ => false,
'''
    if old not in text:
        raise SystemExit("Could not find DeviceSleepTimeout cycle branch in settings.rs")
    text = text.replace(old, new, 1)

# Render section names with the bold font for the current UI font size.
if "let section_font = fonts::FontSet::for_size(self.settings.ui_font_size_idx)" not in text:
    old = '''            if is_section {
                BitmapLabel::new(self.row_region(vi), row.label, self.ui_fonts.body)
                    .alignment(Alignment::CenterLeft)
                    .inverted(selected)
                    .draw(strip)
                    .unwrap();
            } else {
'''
    new = '''            if is_section {
                let section_font = fonts::FontSet::for_size(self.settings.ui_font_size_idx)
                    .font(fonts::Style::Bold);
                BitmapLabel::new(self.row_region(vi), row.label, section_font)
                    .alignment(Alignment::CenterLeft)
                    .inverted(selected)
                    .draw(strip)
                    .unwrap();
            } else {
'''
    if old not in text:
        raise SystemExit("Could not find Settings section render block")
    text = text.replace(old, new, 1)

path.write_text(text)
PY

cargo fmt --all

echo "Applied Sleep Image settings cycle and bold section UI fix. Backup: $backup_dir"
