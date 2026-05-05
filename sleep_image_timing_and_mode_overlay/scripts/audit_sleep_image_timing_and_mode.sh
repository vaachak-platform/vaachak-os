#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

echo "== sleep image timing and mode audit =="

grep -n "pub enum SleepImageMode" vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs
grep -n "SLEEP_IMAGE_MODE_FILE" vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs
grep -n "mode_read_ms" vendor/pulp-os/kernel/src/kernel/scheduler.rs
grep -n "render_ms" vendor/pulp-os/kernel/src/kernel/scheduler.rs
test -x scripts/write_sleep_image_mode.sh
test -x scripts/verify_sleep_image_mode.sh

echo "OK: sleep image timing and mode support present"
