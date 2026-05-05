#!/usr/bin/env bash
set -euo pipefail
repo="${1:-.}"
cd "$repo"

file="vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs"
for token in \
  "FastDaily" \
  "Cached" \
  "NoRedraw" \
  "resolve_sleep_bitmap_for_mode_timed" \
  "resolve_daily_mantra_sleep_bitmap" \
  "draw_sleep_bitmap_strip_timed" \
  "bmp_decode_ms" \
  "SLPCACHE.TXT"; do
  if ! grep -q "$token" "$file"; then
    echo "error: missing $token in $file" >&2
    exit 1
  fi
done

echo "OK: sleep image refresh policy helpers resynced"
