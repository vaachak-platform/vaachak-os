#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD" >&2
  exit 1
fi
if [ ! -d "$sd_root" ]; then
  echo "error: missing SD root: $sd_root" >&2
  exit 1
fi

key="$(date +%a | tr '[:upper:]' '[:lower:]')"
case "$key" in
  mon|tue|wed|thu|fri|sat|sun) ;;
  *) echo "error: unsupported weekday key: $key" >&2; exit 1 ;;
esac

mkdir -p "$sd_root/sleep/daily"
printf '%s\n' "$key" > "$sd_root/sleep/daily/today.txt"

echo "Wrote Daily Mantra weekday selector: $sd_root/sleep/daily/today.txt -> $key"
