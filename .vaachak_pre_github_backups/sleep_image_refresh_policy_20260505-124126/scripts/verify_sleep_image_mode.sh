#!/usr/bin/env bash
set -euo pipefail

sd_root="${1:-}"
if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD" >&2
  exit 1
fi
mode_file="$sd_root/SLPMODE.TXT"
if [ ! -f "$mode_file" ]; then
  echo "Sleep image mode file not present; firmware will default to daily mode"
  exit 0
fi
mode="$(tr -d '\r\n ' < "$mode_file" | tr '[:upper:]' '[:lower:]')"
case "$mode" in
  daily|static|text|off)
    echo "Sleep image mode verified: $mode"
    ;;
  *)
    echo "error: invalid sleep image mode in $mode_file: $mode" >&2
    exit 1
    ;;
esac
