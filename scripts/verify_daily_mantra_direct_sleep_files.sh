#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD" >&2
  exit 1
fi

for file in "$sd_root/sleep/daily/today.txt" "$sd_root/sleep/daily/default.bmp"; do
  if [ ! -f "$file" ]; then
    echo "error: missing $file" >&2
    exit 1
  fi
done

key="$(tr -d '\r\n ' < "$sd_root/sleep/daily/today.txt" | tr '[:upper:]' '[:lower:]')"
case "$key" in
  mon|tue|wed|thu|fri|sat|sun) ;;
  *) echo "error: invalid weekday selector in today.txt: $key" >&2; exit 1 ;;
esac

if [ ! -f "$sd_root/sleep/daily/$key.bmp" ]; then
  echo "error: missing selected weekday bitmap: $sd_root/sleep/daily/$key.bmp" >&2
  exit 1
fi

echo "Daily Mantra direct sleep files verified: $key -> /sleep/daily/$key.bmp"
