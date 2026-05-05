#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD" >&2
  exit 1
fi
file="$sd_root/SLPMODE.TXT"
if [ ! -f "$file" ]; then
  echo "error: missing $file" >&2
  exit 1
fi
mode="$(tr -d '\r\n ' < "$file" | tr '[:upper:]' '[:lower:]')"
case "$mode" in
  daily|fast-daily|static|cached|text|off|no-redraw) ;;
  *) echo "error: unsupported sleep image mode in $file: $mode" >&2; exit 1 ;;
esac

echo "Sleep image mode verified: $mode"
