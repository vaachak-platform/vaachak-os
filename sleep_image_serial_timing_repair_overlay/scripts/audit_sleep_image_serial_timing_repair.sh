#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

echo "== serial sleep-image timing audit =="

rg -n 'esp_println::println!\("sleep image:|mode_read_ms|resolve_ms|render_ms|display: deep sleep mode 1' \
  vendor/pulp-os/kernel/src/kernel/scheduler.rs

rg -n "pub const fn name\(self\) -> &'static str" \
  vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs

echo "OK: serial timing prints are present in the active sleep path"
