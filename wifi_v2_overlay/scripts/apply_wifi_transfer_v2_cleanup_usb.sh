#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-.}"
python3 "$(dirname "$0")/apply_wifi_transfer_v2_cleanup_usb.py" "$ROOT"
(
  cd "$ROOT"
  cargo fmt --all
)
