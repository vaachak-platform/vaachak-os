#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
mode="${2:-}"
if [ -z "$sd_root" ] || [ -z "$mode" ]; then
  echo "usage: $0 /Volumes/SD_CARD daily|fast-daily|static|cached|text|off|no-redraw" >&2
  exit 1
fi
if [ ! -d "$sd_root" ]; then
  echo "error: missing SD root: $sd_root" >&2
  exit 1
fi

case "$mode" in
  daily|fast-daily|static|cached|text|off|no-redraw) ;;
  fast_daily) mode="fast-daily" ;;
  no_redraw|none) mode="no-redraw" ;;
  *) echo "error: unsupported sleep image mode: $mode" >&2; exit 1 ;;
esac

printf '%s\n' "$mode" > "$sd_root/SLPMODE.TXT"
echo "Wrote sleep image mode: $mode -> $sd_root/SLPMODE.TXT"
