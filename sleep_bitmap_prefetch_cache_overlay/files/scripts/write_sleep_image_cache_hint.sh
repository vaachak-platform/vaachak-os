#!/usr/bin/env bash
set -euo pipefail
sd_root="${1:-}"
cache_key="${2:-}"
if [ -z "$sd_root" ] || [ -z "$cache_key" ]; then
  echo "usage: $0 /Volumes/SD_CARD /sleep/daily/tue.bmp" >&2
  exit 1
fi
if [ ! -d "$sd_root" ]; then
  echo "error: missing SD root: $sd_root" >&2
  exit 1
fi
printf '%s\n' "$cache_key" > "$sd_root/SLPCACHE.TXT"
echo "Wrote sleep image cache hint: $cache_key -> $sd_root/SLPCACHE.TXT"
