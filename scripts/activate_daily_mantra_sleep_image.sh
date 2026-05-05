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

daily_dir="$sd_root/sleep/daily"
if [ ! -d "$daily_dir" ]; then
  echo "error: missing daily sleep asset directory: $daily_dir" >&2
  exit 1
fi

weekday="$(date +%a | tr '[:upper:]' '[:lower:]')"
case "$weekday" in
  mon|tue|wed|thu|fri|sat|sun) ;;
  *) echo "error: unsupported weekday key: $weekday" >&2; exit 1 ;;
esac

src="$daily_dir/$weekday.bmp"
default_src="$daily_dir/default.bmp"
if [ ! -f "$src" ]; then
  echo "warning: missing $src; falling back to default.bmp" >&2
  src="$default_src"
fi
if [ ! -f "$src" ]; then
  echo "error: missing daily and default sleep bitmap assets" >&2
  exit 1
fi

mkdir -p "$sd_root/sleep"
cp -f "$src" "$sd_root/sleep/light.bmp"
cp -f "$src" "$sd_root/sleep.bmp"

echo "Activated daily mantra sleep image:" >&2
echo "  source: $src" >&2
echo "  wrote:  $sd_root/sleep/light.bmp" >&2
echo "  wrote:  $sd_root/sleep.bmp" >&2
