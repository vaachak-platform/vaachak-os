#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
mode="${2:-daily}"

if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD [daily|static|text|off]" >&2
  exit 1
fi
if [ ! -d "$sd_root" ]; then
  echo "error: missing SD root: $sd_root" >&2
  exit 1
fi

case "$mode" in
  daily|daily-mantra|mantra) normalized="daily" ;;
  static|sleep-bmp|sleep) normalized="static" ;;
  text|fallback) normalized="text" ;;
  off|none|disabled) normalized="off" ;;
  *)
    echo "error: unsupported sleep image mode: $mode" >&2
    echo "supported: daily, static, text, off" >&2
    exit 1
    ;;
esac

printf '%s\n' "$normalized" > "$sd_root/SLPMODE.TXT"
echo "Wrote sleep image mode: $normalized -> $sd_root/SLPMODE.TXT"
