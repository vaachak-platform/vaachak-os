#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD" >&2
  exit 1
fi

for path in "$sd_root/sleep/light.bmp" "$sd_root/sleep.bmp"; do
  if [ ! -f "$path" ]; then
    echo "error: missing $path" >&2
    exit 1
  fi
done

echo "Active sleep image files verified:" >&2
echo "  $sd_root/sleep/light.bmp" >&2
echo "  $sd_root/sleep.bmp" >&2
