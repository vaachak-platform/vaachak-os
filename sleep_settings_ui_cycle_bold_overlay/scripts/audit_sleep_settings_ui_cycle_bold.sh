#!/usr/bin/env bash
set -euo pipefail
repo="${1:-.}"
cd "$repo"
settings="vendor/pulp-os/src/apps/settings.rs"

echo "== Sleep settings UI cycle/bold audit =="
rg -n 'SettingsRowKind::DeviceSleepImageMode => \{' "$settings"
rg -n 'self\.sleep_image_mode = cycle_index\(self\.sleep_image_mode, SLEEP_IMAGE_MODE_COUNT, delta\)' "$settings"
rg -n 'FontSet::for_size\(self\.settings\.ui_font_size_idx\)' "$settings"
rg -n 'font\(fonts::Style::Bold\)' "$settings"
echo "OK: Sleep Image row is editable and section labels use bold font"
