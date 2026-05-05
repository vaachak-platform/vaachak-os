#!/usr/bin/env bash
set -euo pipefail
sd_root="${1:-}"
if [ -z "$sd_root" ]; then
  echo "usage: $0 /Volumes/SD_CARD" >&2
  exit 1
fi
rm -f "$sd_root/SLPCACHE.TXT"
echo "Cleared sleep image cache hint: $sd_root/SLPCACHE.TXT"
