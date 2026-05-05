#!/usr/bin/env bash
set -euo pipefail
repo="${1:-.}"
cd "$repo"

echo "== sleep bitmap prefetch cache audit =="
rg -n 'prefetch_sleep_bitmap|draw_prefetched_sleep_bitmap_strip|bmp_prefetch_ms|bmp_draw_ms|epd_refresh_ms' \
  vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs \
  vendor/pulp-os/kernel/src/kernel/scheduler.rs

test -x scripts/write_sleep_image_mode.sh
test -x scripts/verify_sleep_image_mode.sh
test -x scripts/write_sleep_image_cache_hint.sh
test -x scripts/clear_sleep_image_cache_hint.sh

echo "OK: sleep bitmap prefetch cache support is present"
