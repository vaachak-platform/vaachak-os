#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

settings="vendor/pulp-os/src/apps/settings.rs"
handle="vendor/pulp-os/kernel/src/kernel/handle.rs"

echo "== sleep settings UI audit =="

grep -n 'Sleep image' "$settings"
grep -n 'DeviceSleepImageMode' "$settings"
grep -n 'SLEEP_IMAGE_MODE_FILE' "$settings"
grep -n 'sleep_image_mode_name' "$settings"
grep -n 'pub fn write_file(&mut self, name: &str, data: &\[u8\])' "$handle"

echo "OK: Sleep Image Mode settings UI wiring is present"
