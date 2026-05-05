#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
key="${2:-}"
if [ -z "$sd_root" ] || [ -z "$key" ]; then
  echo "usage: $0 /Volumes/SD_CARD /sleep/daily/tue.bmp" >&2
  exit 1
fi
if [ ! -d "$sd_root" ]; then
  echo "error: missing SD root: $sd_root" >&2
  exit 1
fi
case "$key" in
  /sleep/daily/mon.bmp|/sleep/daily/tue.bmp|/sleep/daily/wed.bmp|/sleep/daily/thu.bmp|/sleep/daily/fri.bmp|/sleep/daily/sat.bmp|/sleep/daily/sun.bmp|/sleep/daily/default.bmp|/sleep.bmp) ;;
  *) echo "error: unsupported cache key: $key" >&2; exit 1 ;;
esac
printf '%s\n' "$key" > "$sd_root/SLPCACHE.TXT"
echo "Wrote sleep image cache hint: $key -> $sd_root/SLPCACHE.TXT"
